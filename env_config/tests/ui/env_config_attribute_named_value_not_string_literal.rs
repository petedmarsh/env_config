use env_config::{env_config, EnvConfig};

#[env_config(prefix = 123)]
#[derive(EnvConfig)]
struct MyLovelyConfig {
    something: bool,
}

fn main() {}
