all: clone_noir build_noir build_plonky2 build_plonky2 build_backend

clone_noir:
	git clone $(https://github.com/brweisz/noir)
	cd noir && make build_noir

build_noir:
	cd noir && cargo build

clone_plonky2:
	git clone $(https://github.com/brweisz/plonky2)
	cd plonky2 && make build_plonky2

build_plonky2:
	cd plonky2 && cargo build

build_backend:
	cd plonky2-backend && cargo build
