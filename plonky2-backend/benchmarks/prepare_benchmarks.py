import os
import subprocess

origin_path = os.path.abspath(os.getcwd())
custom_nargo_path = origin_path + "/../../noir/target/release/nargo"
custom_backend_path = origin_path + "../target/release/plonky2-backend"
base_test_programs_path = origin_path + "/noir_programs_for_benchmarking/"


def execute_noir_project(noir_project_name):
    cur_dir = base_test_programs_path + noir_project_name
    os.chdir(cur_dir)
    try:
        command_noirky2 = f"{custom_nargo_path} execute witness_noirky2"
        result = subprocess.check_output(command_noirky2, shell=True, text=True)
        rename_circuit_command_noirky2 = f"mv {cur_dir}/target/{noir_project_name}.json {cur_dir}/target/noirky2_program.json"
        subprocess.check_output(rename_circuit_command_noirky2, shell=True, text=True)
        print(result)

        command_bb = f"nargo execute witness_bb"
        result = subprocess.check_output(command_bb, shell=True, text=True)
        rename_circuit_command_bb = f"mv {cur_dir}/target/{noir_project_name}.json {cur_dir}/target/bb_program.json"
        subprocess.check_output(rename_circuit_command_bb, shell=True, text=True)

        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to execute the Noir program {noir_project_name}: {e}")
        return

for noir_project_name in os.listdir(base_test_programs_path):
    subdir_path = os.path.join(base_test_programs_path, noir_project_name)
    if os.path.isdir(subdir_path):
        execute_noir_project(noir_project_name)
# execute_noir_project("memory_writes")
