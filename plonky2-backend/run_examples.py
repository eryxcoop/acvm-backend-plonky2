import os
import sys
import subprocess

def main(argc, argv):
    example_name = argv[1]
    os.chdir(f"example_programs/{example_name}")
    custom_nargo_path = "../../../noir/target/debug/nargo"
    custom_backend_path = "../../../plonky2-backend/target/debug/plonky2-backend"

    try:
        command = f"{custom_nargo_path} execute witness"
        result = subprocess.check_output(command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to execute the Noir program: {e}")
        return

    try:
        command = f"{custom_backend_path} -c ./target/noir_example.json -w  ./target/witness -o proof"
        result = subprocess.check_output(command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to generate the plonky2 proof: {e}")
        return

    try:
        with open(f"./proof", 'r') as f:
            print(f.read())
    except Exception as e:
        print(f"An error has occurred while trying to read the proof: {e}")

    try:
        command = f"{custom_backend_path} write_vk -b ./target/{example_name}.json -o ./target/vk"
        result = subprocess.check_output(command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to generate verification key: {e}")
        return

    try:
        command = f"{custom_backend_path} verify -k ./target/vk -p ./proof"
        result = subprocess.check_output(command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to verify: {e}")
        return


if __name__ == '__main__':
    main(len(sys.argv), sys.argv)
