# env_config

env_config provides a derive macro which simplifies parsing configuration from
environment variables into typed struct fields.

```rust
use std::env;
use env_config::{EnvConfig, EnvError, FromEnv};

#[derive(EnvConfig)]
struct MyLovelyConfig {
    user: Option<String>
}

fn main() -> Result<(), EnvError> {
  // the manual way
  let user = match env::var("USER") {
    Ok(user) => Some(user),
    Err(_) => None,
  };

  // with from_env() derived using EnvConfig
  let config = MyLovelyConfig::from_env()?;

  assert_eq!(user, config.user);
  
  Ok(())
} 
```

## Mandatory Variables

If a particular environment variable must be present then simply omit `Option`
as part of your field type. In this case if the relevant environment variable
is not set then `from_env()` will result in
`Err(EnvError::MandatoryEnvVarNotSet("SOME_MANDATORY_VARIABLE"))`.

```rust
use env_config::{EnvConfig, EnvError, FromEnv};

#[derive(EnvConfig)]
struct MyLovelyConfig {
    some_mandatory_variable: String
}

fn main() {
  match MyLovelyConfig::from_env() {
    Ok(_) => panic!("SOME_MANDATORY_VARIABLE was set!"),
    Err(e) => {
      match e {
        EnvError::MandatoryEnvVarNotSet { env_var_name } => assert_eq!(env_var_name, "SOME_MANDATORY_VARIABLE"),
        _ => panic!("unexpected error"),
      }
    }
  };
}
```

## Namespacing Environment Variable Names

Your application might have several different environment variables to read for
its configuration. Typically these look like `${APPLICATION NAME}_SOME_VARIABLE`
e.g. for an application named `HORSE`:

```ignore
HORSE_SOME_VARIABLE=abc
HORSE_SOME_OTHER_VARIABLE=def
```

You can prefix `HORSE` all of environment variable names derived from your
struct fields names using the `env_config` attribute macro, like so:

```rust
use env_config::{env_config, EnvConfig};

#[env_config(prefix = "HORSE")]
#[derive(EnvConfig)]
struct MyLovelyConfig {
    some_variable: Option<String>, // value of HORSE_SOME_VARIABLE
    some_other_variable: Option<String>, // value of HORSE_SOME_OTHER_VARIABLE
}
```
