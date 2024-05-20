cargo build --manifest-path ../noir/Cargo.toml
../noir/target/debug/nargo check
NARGO_BACKEND_PATH=~/.nargo/backends/acvm-backend-plonky2/backend_binary ../noir/target/debug/nargo prove