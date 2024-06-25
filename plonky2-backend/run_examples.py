import os
import sys
#
# def generate_proof():
#     os.chdir("./noir_example")
#     os.system("./prove_with_plonky2_backend_and_custom_nargo.sh")
#     os.chdir("..")
#
#

#
#
# def verify_proof():
#     os.chdir("./noir_example")
#     # os.system("cargo build --manifest-path ../noir/Cargo.toml")
#     # os.system("../noir/target/debug/nargo check")
#     os.system("NARGO_BACKEND_PATH=~/.nargo/backends/acvm-backend-plonky2/backend_binary ../noir/target/debug/nargo verify")
#     os.chdir("..")

# def read_and_print_proof():
#     path_to_proof = "./noir_example/proofs/noir_example.proof"
#     with open(path_to_proof, 'r') as f:
#         return f.read()

def hex_to_string(hex_values):
    byte_values = bytes.fromhex(hex_values)
    return byte_values.decode('utf-8')


def main(argc, argv):
    example_name = "basic_if"
    os.system("./build_and_deploy_backend.sh")
    os.chdir(f"example_programs/{example_name}")

    env_set = "NARGO_BACKEND_PATH=~/.nargo/backends/acvm-backend-plonky2/backend_binary"
    custom_nargo_path = "../../../noir/target/debug/nargo"

    os.system(f"{env_set} {custom_nargo_path} prove")
    with open(f"proofs/{example_name}.proof", 'r') as f:
        print(f.read())

    os.system(f"{env_set} {custom_nargo_path} verify")


if __name__ == '__main__':
    main(len(sys.argv), sys.argv)
