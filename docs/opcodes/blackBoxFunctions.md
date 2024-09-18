### BlackBoxFunction opcode
This opcode is different from Brillig Opcode. The idea is similar, but in this case we do want to constrain the calculations, and that's the key difference. The idea behind blackbox functions is to delegate to the proving backend the responsibility of how that calculation should be represented within its paradigm, so it can produce an optimal circuit.
There are lots of blackbox functions. The ones done so far for Plonky2 are:

#### RangeCheck
The purpose is to constrain a value to be in certain range [0, 2^x), in other words, we want to make sure some value can be represented with x bits. For this we used the CircuitBuilder's ```range_check()```, which ultimately uses the ```BaseSumGate```. 

#### AND and XOR
Performs a bitwise AND or XOR operation. For this we need to split some target into bits and then perform a traditional operation between bits. This also ends up using the ```BaseSumGate```. 

#### Sha256Compression
This opcode represents a step in the hash function SHA256. Is not the whole hashing function, but one iteration of the main loop. 
