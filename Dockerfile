from python:3.13.0rc2-slim-bookworm as initial_build
run apt upgrade -y
run apt update -y
run apt install -y git build-essential make curl

run curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
env PATH="/root/.cargo/bin:${PATH}"

arg CACHEBUST=1
run git clone https://github.com/eryxcoop/acvm-backend-plonky2.git
workdir acvm-backend-plonky2
run make build_external
run make build_backend

from python:3.13.0rc2-slim-bookworm as final_build
copy --from=initial_build /acvm-backend-plonky2/noir/target/release/nargo /acvm-backend-plonky2/noir/target/release/nargo
copy --from=initial_build /acvm-backend-plonky2/plonky2-backend /acvm-backend-plonky2/plonky2-backend
copy --from=initial_build /acvm-backend-plonky2/Makefile /acvm-backend-plonky2/Makefile
copy --from=initial_build /acvm-backend-plonky2/prepare_compiled_noir_test_programs.py /acvm-backend-plonky2/prepare_compiled_noir_test_programs.py

run apt upgrade -y
run apt update -y
run apt install -y build-essential make
workdir acvm-backend-plonky2

run make precompile_tests
