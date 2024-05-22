# acvm-backend-plonky2
This is an open source backend for the ACIR standard as implemented in the Noir programming languaje, written in Rust.

For now, untill the corresponding PRs are made in the plonky2 and the noir repositories, you have to clone this repositories inside the project root.
* https://github.com/brweisz/noir 
* https://github.com/brweisz/plonky2

Those are forks of the official noir and plonky2 repositories respectively, with a couple modifications.
The resulting project tree must therefore look like:

```
plonky-2-backend-for-acir
|_ noir
|_ noir_example
|_ plonky2
|_ plonky2-backend
|_ run-commands.py
```

## Soon...
For some manual testing, the workflow is as follows:
* In the ```noir_example``` folder there's a Noir project. In the ```noir_example/src/main.nr``` file you can write the main function of any noir program you want to prove.
* Back in the root directory, you can run ```python run-commands.py build prove``` to generate a custom plonky2 proof
  * ```build``` builds the backend and copies the executable in the folder Noir expects it to be
  * ```prove``` uses the 'customized' Noir project to run the ```prove``` command over the corresponding backend
  * ```verify``` is not implemented yet. 

The stdout in the custom plonky2 backend is used in the noir workflow as the return value, but in our custom noir project it is also printed by stdout.
