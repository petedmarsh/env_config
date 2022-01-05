use assert2::{check, let_assert};
use env_config::{env_config, EnvConfig, FromEnv};

#[env_config(prefix = "TEST")]
#[derive(Debug, EnvConfig, PartialEq)]
struct MyLovelyConfig {
    some_mandatory_bool: bool,
}

static TEST_SOME_MANDATORY_BOOL: &str = "TEST_SOME_MANDATORY_BOOL";

#[test]
fn test_env_var_set_to_true() {
    temp_env::with_var(TEST_SOME_MANDATORY_BOOL, Some("true"), || {
        let c = MyLovelyConfig::from_env().unwrap();
        assert_eq!(c, MyLovelyConfig { some_mandatory_bool: true });
    });
}

#[test]
fn test_env_var_set_to_false() {
    temp_env::with_var(TEST_SOME_MANDATORY_BOOL, Some("false"), || {
        let c = MyLovelyConfig::from_env().unwrap();
        assert_eq!(c, MyLovelyConfig { some_mandatory_bool: false });
    });
}

#[test]
fn test_env_var_not_set() {
    let unset: Option<String> = None;
    temp_env::with_var(TEST_SOME_MANDATORY_BOOL, unset, || {
        let_assert!(Err(env_config::EnvError::MandatoryEnvVarNotSet{ env_var_name }) = MyLovelyConfig::from_env());
        check!(env_var_name == TEST_SOME_MANDATORY_BOOL);
    });
}
