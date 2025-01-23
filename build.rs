use prost_build_config::{BuildConfig, Builder};

fn main() {
    let config: BuildConfig = serde_yaml::from_str(include_str!("build_config.yml")).unwrap();
    Builder::from(config).build_protos();
}
