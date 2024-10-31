# acvm-backend-plonky2
This is an open source backend for the ACIR standard as implemented in the Noir programming languaje, written in Rust. Check out [the docs](https://eryxcoop.github.io/acvm-backend-plonky2/foreword.html) for more detail on what this repo does.

## How to generate Plonky2 proofs for your Noir circuit with docker
1. You need to install docker, if you haven't already
2. From a terminal, run ```docker pull bweisz/acvm-backend-plonky2:0.3``` to pull the docker image from dockerhub

Fast version (execute, prove, write_vk and verify all at once)
3. Run ```docker run -v /full/path/to/your/noir/project:/acvm-backend-plonky2/noir_example bweisz/acvm-backend-plonky2 make verification_happy_path```. This will create a container from the image 'bweisz/acvm-backend-plonky2', copy the contents of your noir project into the 'noir_example' folder inside the container and run a complete workflow of executing the circuit, generating the proof, writing the vk and verifying the proof. 


Alternatively, if you want to run the commands separately and know something about docker (or not), follow the following steps. 
3. Run ```docker run -d --name noir_with_plonky2 -v /full/path/to/your/noir/project:/acvm-backend-plonky2/noir_example bweisz/acvm-backend-plonky2 tail -f /dev/null```. This will create a container named 'noir_with_plonky2'
4. Run ```docker exec -it noir_with_plonky2 bash```. This will allow you to execute commands in the container. 
5. From the container terminal, run separately:
   1. ```make nargo_execute```
   2. ```make prove```
   3. ```make write_vk```
   4. ```make verify ```


## How to set up the project locally (without docker)

For the setup, run ```make``` on the root directory. This will do the following:

For now, until the corresponding PRs are made in the Plonky2 and the Noir repositories, the command will clone these repositories inside the project root.
* https://github.com/brweisz/noir 
* https://github.com/brweisz/plonky2

Those are forks of the official [Noir](https://github.com/noir-lang/noir) and [Plonky2](https://github.com/0xPolygonZero/plonky2) repositories respectively, with a couple modifications.
The resulting project tree must therefore look like:

```
plonky-2-backend-for-acir
|_ noir
|_ noir_example
|_ plonky2
|_ plonky2-backend
|_ Makefile
```

Then it'll build noir and plonky2. The latter with the nightly toolchain. Lastly, it'll build the custom plonky2 backend. 

## Running some predefined examples
If you want to try out some Noir examples, execute the python script ```run_examples.py``` with the name of the example as the only parameter from the ```plonky2-backend``` directory:
* ```basic_if```
* ```fibonacci```
* ```basic_div```


## Manual testing

For some manual testing (local), the workflow is as follows: in the ```noir_example``` folder there's a Noir project. In the ```noir_example/src/main.nr``` file you can write the main function of any noir program you want to prove.  
Run ```make run_noir_example``` from the root directory. The following explanation is similar to the official [Noir docs](https://noir-lang.org/docs/getting_started/hello_noir/#4-execute-the-noir-program), but using the custom plonky2 backend instead of barretenberg, and it's what the command will execute.

1) From the ```noir_example``` directory run:
* ```../noir/target/release/nargo execute witness```. This will execute the noir program through the nargo acvm, generating:
   * The ACIR circuit in ```target/noir_example.json```
   * The witness in ```target/witness.gz```
2) From the ```plonky2-backend``` directory run: 
* ```./target/release/plonky2-backend prove -b ../noir_example/target/noir_example.json -w  ../noir_example/target/witness.gz -o ../noir_example/proof```. This will create a Plonky2 proof in ```../noir_example/proof```.
* ```./target/release/plonky2-backend write_vk -b ../noir_example/target/noir_example.json -o ../noir_example/target/vk```. This will create the verification key in ```../noir_example/target/vk```
* ```./target/release/plonky2-backend verify -k ../noir_example/target/vk -p ../noir_example/proof```. This will verify the Plonky2 proof. An empty output is sign of verification success.

## Contact Us
Feel free to join our telegram group for suggestions, report bugs or any question you might have!
https://t.me/+HeUDkQPX_w0yMDQx

## Things already implemented in this version
The Plonky2 backend for ACIR is still in a development phase. As for now, these are the implemented functionalities:
* AssertZero Opcode ✓
* MemoryInit Opcode ✓
* MemoryOp Opcode ✓
* BrilligCall Opcode ✓
* BlackBoxFunction ✓
  * RANGE (up to 33 bits) ✓
  * XOR ✓
  * AND ✓
  * SHA256 ✓
  * EcdsaSecp256k1 ✓

Things not implemented yet are mostly BlackBoxFunctions.

## Credits
We used some code from repos for the implementation of ECDSA verification and made some modifications to them:
* https://github.com/0xPolygonZero/plonky2-ecdsa
* https://github.com/0xPolygonZero/plonky2-u32

If you want to try this backend with a medium size Noir project here's an example of a [context free grammar checker](https://codeberg.org/pdallegri/zk-grammar) by Pablo Dallegri, the first Plonky2 backend user. 