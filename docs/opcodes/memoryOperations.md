### Memory Operations
There are 2 memory operations: MemoryInit and MemoryOp (read or write). This opcode exists to handle the case where we read or write values from arrays with an unknown position in circuit-building time. 

#### MemoryInit
It represents the creation of an array in memory. What does this mean for a prover? Well, it depends on the prover. This opcode will be translated into a set of constraints that will be used to generate the Plonky2 circuit. It has 2 fields:
* block_id: the memory block index that is being created. This index will be global within our program. It's numbered from 0 to N.
* witnesses: an array of the witnesses that hold the initial values of the memory block.

Our way of translating this to Plonky2 was simply creating a global mapping between indexes and arrays of Plonky2 targets, like ```HashMap<usize,Vec<Target>>```.

#### MemoryOp
It represents either a memory read operation or a memory write operation. 

##### Memory Read
The opcode has the following fields:
* block_id: the memory block index where we're reading from. 
* index: witness that holds the value of the index where we want to read. For example, assume we have a witness w_0 and we want to read on index w_0. In this case is simpler to think of it as a variable and that we want to read from ```array[w_0]```.
* value: witness where the value of the memory read will be stored. Assume we have a value = w_1. To complete the example, this would be like doing ```w_1 = array[w_0]```.

To implement this we used the Plonky2 RandomAccessMemory gate through the CircuitBuilder's ```random_access()``` method.   

##### Memory Write
The opcode has the following fields:
* block_id: the memory block index we're writing into.
* index: witness that holds the value of the index we want to write into. 
* value: witness that holds the value we want to write. Assume we have a value = w_1. 

This would be equivalent as doing ```array[w_0] = w_1```.

Now, this operation is a bit tricky since we don't know the values we're writing while building the circuit. During the circuit construction, we have arrays of targets representing our memory blocks, and these targets have unchangeable values during circuit execution. We need to create a static circuit, but any slot's value could change. Therefore, we need to create an entirely new array and constrain its values to be the same as the previous ones, except for the position we're changing. As you can imagine, this operation is rather expensive. 

 We iterate over all the targets, using the CircuitBuilder's ```is_equal()``` method to figure out which position are we changing.  