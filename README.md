# acvm-backend-plonky2
This is an open source backend for the ACIR standard as implemented in the Noir programming languaje, written in Rust.

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

## Manual testing

For some manual testing, the workflow is as follows: in the ```noir_example``` folder there's a Noir project. In the ```noir_example/src/main.nr``` file you can write the main function of any noir program you want to prove.  
Run ```make run_noir_example``` from the root directory. The following explanation is similar to the official [Noir docs](https://noir-lang.org/docs/dev/getting_started/hello_noir/#execute-our-noir-program), but using the custom plonky2 backend instead of barretenberg, and it's what the command will execute.

1) From the ```noir_example``` directory run:
* ```../noir/target/debug/nargo execute witness```. This will execute the noir program through the nargo acvm, generating:
   * The ACIR circuit in ```target/noir_example.json```
   * The witness in ```target/witness-name.gz```
2) From the ```plonky2-backend``` directory run: 
* ```./target/debug/plonky2-backend prove -c ../noir_example/target/noir_example.json -w  ../noir_example/target/witness -o ../noir_example/proof```. This will create a Plonky2 proof in ```../noir_example/proof```.
* ```./target/debug/plonky2-backend write_vk -b ../noir_example/target/noir_example.json -o ../noir_example/target/vk```. This will create the verification key in ```../noir_example/target/vk```
* ```./target/debug/plonky2-backend verify -k ../noir_example/target/vk -p ../noir_example/proof```. This will verify the Plonky2 proof. An empty output is sign of verification success. 

    
## Running some predefined examples
If you want to try out some Noir examples, execute the python script ```run_examples.py``` with the name of the example as the only parameter from the ```plonky2-backend``` directory:
* ```basic_if```
* ```fibonacci```
* ```basic_div```

## Contact Us
Feel free to join our telegram group for suggestions, report bugs or any question you might have!
https://t.me/+2sYwLSy1SEVhYjlh