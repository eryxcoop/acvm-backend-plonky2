## General overview
The path from writing a Noir program and verifying a Plonky2 proof has many steps. This repo aims to touch only some of those, since the modularity provided by Aztec's stack allows it. We want to create a new backend for Noir using Plonky2 prover instead of Barretenberg. Essentially, these are the steps:
* Read the ACIR circuit and the trace that solve it. These are the outputs of compiling and executing a Noir program. 
* Create a Plonky2 circuit equivalent to the ACIR circuit.
* Give concrete values to the Plonky2 circuit and generate the proof. 

The ACIR circuit is composed by Opcodes, a set of abstract operations over variables. These variables are called witnesses in this context. 

The backend as an executable has 3 operations:
* prove
* write_vk
* verify

All the mentioned steps are performed in the ```prove``` command. The ```write_vk``` and ```verify``` mimics the Barretenberg behaviour.  