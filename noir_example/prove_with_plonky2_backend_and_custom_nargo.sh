cargo build --manifest-path ../noir/Cargo.toml
nargo check
NARGO_BACKEND_PATH=~/.nargo/backends/acvm-backend-plonky2/backend_binary ../noir/target/debug/nargo prove