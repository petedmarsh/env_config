use env_config::{EnvConfig};

struct StructWithoutFromEnvVarImpl;

#[derive(EnvConfig)]
struct MyLovelyConfig {
    some_optional_bool: StructWithoutFromEnvVarImpl,
}

fn main() {}
