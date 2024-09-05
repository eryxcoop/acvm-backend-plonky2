### Memory Operations
There are 2 memory operations: MemoryInit and MemoryOp (read or write). This opcode exists to handle the case where we read or write values from arrays with an unknown position in circuit-building time. For example, the Noir program...

```rust
fn main(array: [Field; 5]) -> pub Field {
    array[3]
}
```
...resolves to AssertZero opcodes, since the input Witness are w0, w1, w2, w3 & w4, the output witness is w5, and the code just makes an AssertZero to ensure that w3 is equal to w5. Instead, when the Noir code looks like this...

```rust
fn main(array: [Field; 5], index: Field) -> pub Field {
    array[index]
}
```
...then we're making a random access into an array in a position unknown at the time the circuit is being built. This requires the concept of Memory blocks (or memory arrays). 


#### MemoryInit
It represents the creation of an array in memory. What does this mean for a prover? Well, it depends on the prover. This opcode will be translated into a set of constraints that will be used to generate the Plonky2 circuit. It has 2 fields:
* block_id: the memory block index that is being created. This index will be global within our program. It's numbered from 0 to N.
* witnesses: an array of the witnesses that hold the initial values of the memory block.

Our way of translating this to Plonky2 was simply creating a global mapping between indexes and arrays of Plonky2 targets, like ```HashMap<usize,Vec<Target>>```.

#### MemoryOp
It represents either a memory read operation or a memory write operation. 

##### Memory Read
The opcode has the following fields:
* block_id: the index of the memory block we're reading from. 
* index: witness that holds the value of the index where we want to read from. It's like  ```array[index]```.
* value: witness where the value of the memory read will be stored. It's like ```value = array[index]```.

To implement this we used the Plonky2 RandomAccessMemory gate through the CircuitBuilder's ```random_access()``` method.   

##### Memory Write
The opcode has the following fields:
* block_id: the memory block index we're writing into.
* index: witness that holds the value of the index we want to write into. 
* value: witness that holds the value we want to write. Assume we have a value = w_1. 

This would be equivalent as doing ```array[index] = value```.

Now, this operation is a bit tricky since we don't know the position we're writing while building the circuit. During the circuit construction, we have arrays of targets representing our memory blocks, and these targets have unchangeable values during circuit execution. We need to create a static circuit, but any slot's value could change. Therefore, we need to create an entirely new array and constrain its values to be the same as the previous ones, except for the position we're changing. As you can imagine, this operation is rather expensive. 

To visualize this: imagine we need to write position 2 with some value v. Then we'll need to create a new set of targets and attach them to the corresponding value. 

Previous memory block: | t0  | t1  | t2  | t3  | t4  |
                          ↓     ↓           ↓     ↓
New memory block:      | t0' | t1' | t2' | t3' | t4' |
                                      ↑
New value:                            v


We iterate over all the targets, using the CircuitBuilder's ```is_equal()``` method to figure out which position are we changing. If the position doesn't match the index, we link it to the target in the previous version of the memory block on the same position. If the position matches the index, then we create a new target with the value we want to write and link it to the new memory array.    