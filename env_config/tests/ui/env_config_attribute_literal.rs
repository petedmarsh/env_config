use env_config::{env_config, EnvConfig};

#[env_config("MY_LOVELY_PREFIX")]
#[derive(EnvConfig)]
struct MyLovelyConfig {
    something: bool,
}

fn main() {}
