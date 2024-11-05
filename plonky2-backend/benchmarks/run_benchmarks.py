import os
import subprocess
import pandas as pd
import time

origin_path = os.path.abspath(os.getcwd())
custom_backend_path = origin_path + "/plonky2-backend/target/release/plonky2-backend"
base_test_programs_path = origin_path + "/noir_programs_for_benchmarking/"

NUMBER_OF_ITERATIONS = 5
rows = []

def generate_proofs(noir_project_name):
    if noir_project_name == "multiple_xor_u8":
        return

    cur_dir = base_test_programs_path + noir_project_name
    os.chdir(cur_dir)
    print(cur_dir)
    try:
        backend_noirky2 = "../../../target/release/plonky2-backend"
        command_noirky2 = f"time {backend_noirky2} prove -b ./target/noirky2_program.json -w ./target/witness_noirky2.gz -o ./target/proof_noirky2"
        for i in range(NUMBER_OF_ITERATIONS):
            initial_time = time.time()
            result = subprocess.check_output(command_noirky2, shell=True, text=True)
            print(result)
            final_time = time.time()
            elapsed_time = final_time - initial_time
            row = {"backend": "Noirky2", "program_name": noir_project_name, "iteration_number": i,
                   "elapsed_time": elapsed_time}
            rows.append(row)


        backend_bb = f"bb"
        command_bb = f"time {backend_bb} prove -b ./target/bb_program.json -w ./target/witness_bb.gz -o ./target/proof_bb"
        for i in range(NUMBER_OF_ITERATIONS):
            initial_time = time.time()
            result = subprocess.check_output(command_bb, shell=True, text=True)
            print(result)
            final_time = time.time()
            elapsed_time = final_time - initial_time
            row = {"backend": "Barretenberg", "program_name": noir_project_name, "iteration_number": i,
                   "elapsed_time": elapsed_time}
            rows.append(row)



    except Exception as e:
        print(f"An error has occurred while trying to execute the Noir program {noir_project_name}: {e}")
        return


for noir_project_name in os.listdir(base_test_programs_path):
    subdir_path = os.path.join(base_test_programs_path, noir_project_name)
    if os.path.isdir(subdir_path):
        generate_proofs(noir_project_name)

df = pd.DataFrame(rows, columns=["backend", "program_name", "iteration_number", "elapsed_time"])
print(df)
df.to_csv(origin_path + "/last_meditions.csv", index = False)
