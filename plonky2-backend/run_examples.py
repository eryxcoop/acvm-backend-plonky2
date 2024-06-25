import os
import sys

def main(argc, argv):
    example_name = argv[1]
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
