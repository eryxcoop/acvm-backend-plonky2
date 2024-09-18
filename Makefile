all: build_external build_backend precompile_tests

# Cloning and building external resources

build_external:
	$(MAKE) clone_custom_noir
	$(MAKE) build_noir
	$(MAKE) clone_custom_plonky2
	$(MAKE) build_plonky2

clone_custom_noir:
	git clone https://github.com/brweisz/noir

build_noir:
	cd noir && cargo build --release

clone_custom_plonky2:
	git clone https://github.com/brweisz/plonky2

build_plonky2:
	rustup override set nightly && cd plonky2 && cargo build --release


# Building plonky2-backend

build_backend:
	cd plonky2-backend && cargo build --release


# Tests

precompile_tests:
	python prepare_compiled_noir_test_programs.py


# Execution

verification_happy_path:
	$(MAKE) nargo_execute
	$(MAKE) prove
	$(MAKE) write_vk
	$(MAKE) verify

nargo_execute:
	cd noir_example && ../noir/target/release/nargo execute witness --print-acir

prove:
	cd plonky2-backend && ./target/release/plonky2-backend prove -c ../noir_example/target/noir_example.json -w  ../noir_example/target/witness -o ../noir_example/proof

write_vk:
	cd plonky2-backend && ./target/release/plonky2-backend write_vk -b ../noir_example/target/noir_example.json -o ../noir_example/target/vk

verify:
	cd plonky2-backend && ./target/release/plonky2-backend verify -k ../noir_example/target/vk -p ../noir_example/proof

.PHONY: all clone_custom_noir build_noir clone_custom_plonky2 build_plonky2 build_backend