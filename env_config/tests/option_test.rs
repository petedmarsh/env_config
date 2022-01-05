use env_config::{env_config, EnvConfig, FromEnv};

#[env_config(prefix = "TEST")]
#[derive(Debug, EnvConfig, PartialEq)]
struct MyLovelyConfig {
    some_optional_bool: Option<bool>,
}

static TEST_SOME_OPTIONAL_BOOL: &str = "TEST_SOME_OPTIONAL_BOOL";

#[test]
fn test_env_var_set_to_true() {
    temp_env::with_var(TEST_SOME_OPTIONAL_BOOL, Some("true"), || {
        let c = MyLovelyConfig::from_env().unwrap();
        assert_eq!(c, MyLovelyConfig { some_optional_bool: Some(true) });
    });
}

#[test]
fn test_env_var_set_to_false() {
    temp_env::with_var(TEST_SOME_OPTIONAL_BOOL, Some("false"), || {
        let c = MyLovelyConfig::from_env().unwrap();
        assert_eq!(c, MyLovelyConfig { some_optional_bool: Some(false) });
    });
}

#[test]
fn test_env_var_not_set() {
    let unset: Option<String> = None;
    temp_env::with_var(TEST_SOME_OPTIONAL_BOOL, unset, || {
        let c = MyLovelyConfig::from_env().unwrap();
        assert_eq!(c, MyLovelyConfig { some_optional_bool: None });
    });
}
