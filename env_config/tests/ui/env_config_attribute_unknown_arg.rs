use env_config::{env_config, EnvConfig};

#[env_config(some_unknown_arg = "WHOOPS")]
#[derive(EnvConfig)]
struct MyLovelyConfig {
    something: bool,
}

fn main() {}
