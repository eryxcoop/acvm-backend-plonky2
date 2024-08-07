all: clone_custom_noir build_noir clone_custom_plonky2 build_plonky2 build_backend precompile_tests

clone_custom_noir:
	git clone https://github.com/brweisz/noir

build_noir:
	cd noir && cargo build

clone_custom_plonky2:
	git clone https://github.com/brweisz/plonky2

build_plonky2:
	rustup override set nightly && cd plonky2 && cargo build && rustup override unset

build_backend:
	cd plonky2-backend && cargo build

precompile_tests:
	python prepare_compiled_noir_test_programs.py

build_backend_release:
	cd plonky2-backend && cargo build --profile=benchmarking

run_noir_example:
	cd noir_example && ../noir/target/debug/nargo execute witness --print-acir && \
	cd ../plonky2-backend && ./target/debug/plonky2-backend prove -c ../noir_example/target/noir_example.json -w  ../noir_example/target/witness -o ../noir_example/proof && \
	./target/debug/plonky2-backend write_vk -b ../noir_example/target/noir_example.json -o ../noir_example/target/vk && \
	./target/debug/plonky2-backend verify -k ../noir_example/target/vk -p ../noir_example/proof

run_noir_example_release:
	cd noir_example && ../noir/target/debug/nargo execute witness --print-acir && \
	cd ../plonky2-backend && ./target/release/plonky2-backend prove -c ../noir_example/target/noir_example.json -w  ../noir_example/target/witness -o ../noir_example/proof && \
	./target/release/plonky2-backend write_vk -b ../noir_example/target/noir_example.json -o ../noir_example/target/vk && \
	./target/release/plonky2-backend verify -k ../noir_example/target/vk -p ../noir_example/proof


.PHONY: all clone_custom_noir build_noir clone_custom_plonky2 build_plonky2 build_backend