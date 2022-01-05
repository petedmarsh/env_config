use env_config::{EnvConfig};

#[derive(EnvConfig)]
enum MyLovelyEnum {
    Something,
    SomethingElse,
}

fn main() {}
