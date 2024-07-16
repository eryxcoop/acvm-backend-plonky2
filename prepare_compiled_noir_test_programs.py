import os
import subprocess

origin_path = os.path.abspath(os.getcwd())
custom_nargo_path = origin_path + "/noir/target/debug/nargo"
custom_backend_path = origin_path + "/plonky2-backend/target/debug/plonky2-backend"
base_test_programs_path = origin_path + "/plonky2-backend/src/circuit_translation/tests/factories/precompiled_circuits_0.47.0/"
print(origin_path)

def execute_noir_project(noir_project_name):
    cur_dir = base_test_programs_path + noir_project_name
    os.chdir(cur_dir)
    try:
        command = f"{custom_nargo_path} execute witness"
        result = subprocess.check_output(command, shell=True, text=True)
        rename_circuit_command = f"mv {cur_dir}/target/{noir_project_name}.json {cur_dir}/target/circuit.json"
        subprocess.check_output(rename_circuit_command, shell=True, text=True)
        print(result)
    except Exception as e:
        print(f"An error has occurred while trying to execute the Noir program {noir_project_name}: {e}")
        return

for noir_project_name in os.listdir(base_test_programs_path):
    subdir_path = os.path.join(base_test_programs_path, noir_project_name)
    if os.path.isdir(subdir_path):
        execute_noir_project(noir_project_name)