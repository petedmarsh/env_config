use std::convert::AsRef;
use std::env;
use std::ffi::{OsStr, OsString};
use std::str::FromStr;

pub use env_config_derive::*;

#[derive(Clone, Debug, PartialEq)]
pub enum EnvVarError {
    NotUnicode{ env_var_name: OsString, env_var_raw_value: OsString },
    NotParsable{ env_var_name: OsString, env_var_value: String },
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnvError {
    InvalidValue(EnvVarError),
    MandatoryEnvVarNotSet{ env_var_name: OsString },
}

pub trait FromEnv {
    fn from_env() -> Result<Self, EnvError> where Self: Sized;
}

pub trait FromEnvVar: FromStr {
    fn from_env_var<S: AsRef<OsStr> + Into<OsString>>(env_var_name: S) -> Result<Option<Self>, EnvVarError> {
        match env::var(&env_var_name) {
            Ok(env_var_value) => {
                match FromStr::from_str(&env_var_value) {
                    Ok(parsed_value) => Ok(Some(parsed_value)),
                    Err(_) => Err(EnvVarError::NotParsable{ env_var_name: env_var_name.into(), env_var_value }),
                }
            },
            Err(e) => {
                match e {
                    env::VarError::NotPresent => Ok(None),
                    env::VarError::NotUnicode(env_var_raw_value) => Err(EnvVarError::NotUnicode{env_var_name: env_var_name.into(), env_var_raw_value }),
                }
            }
        }
    }
} 

impl FromEnvVar for bool {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use assert2::{check, let_assert};

    static ENV_VAR: &str = "9d8e8467";

    mod fromenvvar {
        use super::*;

        #[test]
        fn when_env_var_not_present() {
            let unset: Option<String> = None;
            temp_env::with_var(ENV_VAR, unset, || {
                assert_eq!(<bool as FromEnvVar>::from_env_var(ENV_VAR).unwrap(), None);
            });
        }

        #[test]
        fn when_env_var_name_is_borrowed_string() {
            let env_var_name_string = ENV_VAR.to_string();
            temp_env::with_var(ENV_VAR, Some("false"), || {
                assert_eq!(<bool as FromEnvVar>::from_env_var(&env_var_name_string).unwrap(), Some(false));
            });
        }

        #[test]
        fn when_env_var_name_is_osstring() {
            temp_env::with_var(ENV_VAR, Some("false"), || {
                let env_var_name_string: OsString = ENV_VAR.into();
                assert_eq!(<bool as FromEnvVar>::from_env_var(env_var_name_string).unwrap(), Some(false));
            });
        }

        #[test]
        fn when_env_var_name_is_borrowed_osstring() {
            temp_env::with_var(ENV_VAR, Some("false"), || {
                let env_var_name_string: OsString = ENV_VAR.into();
                assert_eq!(<bool as FromEnvVar>::from_env_var(&env_var_name_string).unwrap(), Some(false));
            });
        }

        #[test]
        fn when_env_var_is_valid_unicode_but_not_parsable() {
            let invalid = "this is not a bool";
            temp_env::with_var(ENV_VAR, Some(invalid), || {
                let_assert!(Err(EnvVarError::NotParsable{ env_var_name, env_var_value }) = <bool as FromEnvVar>::from_env_var(ENV_VAR));
                check!(env_var_name == ENV_VAR);
                check!(env_var_value == invalid);
            });
        }

        #[test]
        fn when_env_var_is_not_valid_unicode() {
            // Here, the values 0x66 and 0x6f correspond to 'f' and 'o'
            // respectively. The value 0x80 is a lone continuation byte, invalid
            // in a UTF-8 sequence.
            let os_str = OsStr::from_bytes(&[0x66, 0x6f, 0x80, 0x6f]);
            temp_env::with_var(ENV_VAR, Some(os_str.clone()), || {
                let_assert!(Err(EnvVarError::NotUnicode{ env_var_name, env_var_raw_value }) = <bool as FromEnvVar>::from_env_var(ENV_VAR));
                check!(env_var_name == ENV_VAR);
                check!(env_var_raw_value == os_str);
            });
        }

        mod for_bool {
            use super::*;
            use temp_env;

            #[test]
            fn when_env_var_is_false() {
                temp_env::with_var(ENV_VAR, Some("false"), || {
                    assert_eq!(<bool as FromEnvVar>::from_env_var(ENV_VAR).unwrap(), Some(false));
                });
            }

            #[test]
            fn when_env_var_is_true() {
                temp_env::with_var(ENV_VAR, Some("true"), || {
                    assert_eq!(<bool as FromEnvVar>::from_env_var(ENV_VAR).unwrap(), Some(true));
                });
            }

            #[test]
            fn when_env_var_is_improperly_cased_true() {
                let improperly_cased_true = "tRuE";
                temp_env::with_var(ENV_VAR, Some(improperly_cased_true.clone()), || {
                    let_assert!(Err(EnvVarError::NotParsable{ env_var_name, env_var_value }) = <bool as FromEnvVar>::from_env_var(ENV_VAR));
                    check!(env_var_name == ENV_VAR);
                    check!(env_var_value == improperly_cased_true);
                });
            }

            #[test]
            fn when_env_var_is_improperly_cased_false() {
                let improperly_cased_false = "fAlSe";
                temp_env::with_var(ENV_VAR, Some(improperly_cased_false.clone()), || {
                    let_assert!(Err(EnvVarError::NotParsable{ env_var_name, env_var_value }) = <bool as FromEnvVar>::from_env_var(ENV_VAR));
                    check!(env_var_name == ENV_VAR);
                    check!(env_var_value == improperly_cased_false);
                });
            }
        }
    }
}
