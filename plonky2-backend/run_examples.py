import os
import sys
import subprocess

def main(argc, argv):
    example_name = argv[1]
    try:
        command = "./build_and_deploy_backend.sh"
        result = subprocess.check_output(command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to compile backend: {e}")
        return

    os.chdir(f"example_programs/{example_name}")

    env_set = "NARGO_BACKEND_PATH=~/.nargo/backends/acvm-backend-plonky2/backend_binary"
    custom_nargo_path = "../../../noir/target/debug/nargo"

    try:
        command = f"{env_set} {custom_nargo_path} prove"
        result = subprocess.check_output(command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to generate prove: {e}")
        return

    try:
        with open(f"proofs/{example_name}.proof", 'r') as f:
            print(f.read())
    except Exception as e:
        print(f"An error has occurred while trying to read the proof: {e}")

    try:
        command = f"{env_set} {custom_nargo_path} verify"
        result = subprocess.check_output(command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to verify: {e}")
        return


if __name__ == '__main__':
    main(len(sys.argv), sys.argv)
