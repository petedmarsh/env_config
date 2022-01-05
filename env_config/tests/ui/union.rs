use env_config::{EnvConfig};

#[derive(EnvConfig)]
union MyLovelyUnion {
    f1: u32,
    f2: f32,
}

fn main() {}
