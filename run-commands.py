import os
import sys


def build_and_deploy():
    os.chdir("./plonky2-backend")
    os.system("./build_and_deploy_backend.sh")
    os.chdir("..")


def generate_proof():
    os.chdir("./noir_example")
    os.system("./prove_with_plonky2_backend_and_custom_nargo.sh")
    os.chdir("..")


def read_proof():
    path_to_proof = "./noir_example/proofs/noir_example.proof"
    with open(path_to_proof, 'r') as f:
        return f.read()


def verify_proof():
    os.chdir("./noir_example")
    # os.system("cargo build --manifest-path ../noir/Cargo.toml")
    # os.system("../noir/target/debug/nargo check")
    os.system("NARGO_BACKEND_PATH=~/.nargo/backends/acvm-backend-plonky2/backend_binary ../noir/target/debug/nargo verify")
    os.chdir("..")


def hex_to_string(hex_values):
    byte_values = bytes.fromhex(hex_values)
    return byte_values.decode('utf-8')


def main(argc, argv):
    if "build" in argv:
        build_and_deploy()

    if "prove" in argv:
        generate_proof()

    if "provetest" in argv:
        generate_proof()
        print(hex_to_string(read_proof()))

    if "verify" in argv:
        verify_proof()


if __name__ == '__main__':
    main(len(sys.argv), sys.argv)
