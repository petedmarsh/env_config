use env_config::{env_config, EnvConfig};

#[env_config(A, B, C)]
#[derive(EnvConfig)]
struct MyLovelyConfig {
    something: bool,
}

fn main() {}
