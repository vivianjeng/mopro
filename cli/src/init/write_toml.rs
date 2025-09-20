pub fn init_toml() -> &'static str {
    r#"
[package]
name = "MOPRO_TEMPLATE_PROJECT_NAME"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["lib", "cdylib", "staticlib"]

# Adapters for different proof systems
[features]
default = ["<FEATURES>"]

[dependencies]
mopro-wasm = { git = "https://github.com/zkmopro/mopro.git" }
mopro-ffi = { path = "../mopro-ffi", features = ["uniffi"] }
thiserror = "2.0.12"
anyhow = "1.0.99"

# CIRCOM_DEPENDENCIES

# HALO2_DEPENDENCIES

# NOIR_DEPENDENCIES

[build-dependencies]
# CIRCOM_BUILD_DEPENDENCIES
    "# // TODO - make build dependencies also configurable
}