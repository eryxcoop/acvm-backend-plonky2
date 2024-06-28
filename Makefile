all: clone_custom_noir build_noir clone_custom_plonky2 build_plonky2 build_backend

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

.PHONY: all clone_custom_noir build_noir clone_custom_plonky2 build_plonky2 build_backend