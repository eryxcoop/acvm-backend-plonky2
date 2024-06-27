# acvm-backend-plonky2
This is an open source backend for the ACIR standard as implemented in the Noir programming languaje, written in Rust.

For now, until the corresponding PRs are made in the Plonky2 and the Noir repositories, you have to clone this repositories inside the project root.
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
|_ run-commands.py
```

Then you'll have to build noir and plonky2. To do so, step into ```noir```/```plonky2``` and run ```cargo build```. Optionally (for example, for performance evaluation) you can use the release build. Also, Plonky2 requires you to use the nightly version of cargo. 

## Manual testing (up to acvm version 0.45.0)
_The Noir workflow regarding proof generation and verification has changed recently, so the following instructions are deprecated in the latest Noir version. However, for now you should be using the fork of Noir referenced earlier, which has a previous version of the code and therefore it's compatible with what follows:_

For some manual testing, the workflow is as follows:
* In the ```noir_example``` folder there's a Noir project. In the ```noir_example/src/main.nr``` file you can write the main function of any noir program you want to prove.
* Back in the root directory, you can run ```python run-commands.py build prove verify``` to generate a custom plonky2 proof
  * ```build``` builds the backend and copies the executable in the folder Noir expects it to be
  * ```prove``` uses the 'customized' Noir project to run the ```prove``` command on the corresponding backend
  * ```verify``` uses the 'customized' Noir project to run the ```vrite_vk``` and ```verify``` command on the corresponding backend. 

The stdout in the custom plonky2 backend is used in the noir workflow as the return value, but in our custom noir project it is also printed by stdout for debugging.

## Running some examples
If you want to try out some Noir examples, execute the python script ```run_examples.py``` with the name of the example as the only parameter from the ```plonky2-backend``` directory:
* ```basic_if```
* ```fibonacci```
* ```basic_div```