cargo build
if ! test -d ~/.nargo/backends/acvm-backend-plonky2; then
  mkdir ~/.nargo/backends/acvm-backend-plonky2
fi
cp target/debug/plonky2-backend ~/.nargo/backends/acvm-backend-plonky2/backend_binary