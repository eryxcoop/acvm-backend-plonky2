### Memory Operations
There are 2 memory operations: MemoryInit and MemoryOp (read or write). 

#### MemoryInit
This opcode has the form
```rust
{
    index: block_idx,
    witnesses: [w_0, ..., w_m]
}
```
It represents the creation of an array in memory. What does this mean for a prover? Well, it depends on the prover. 
* index: represents the "index" of the memory array, they will be numerated from 0 to N. 
* witnesses: is an array of witnesses. The values of these witnesses will be the initial values of the memory block. 

#### MemoryOp
This opcode has the form
```rust
{

    block_id: block_idx,
    op: {
        operation: 1/0,
        index: w_0
        value: w_1,
    }   
}
```
* block_id: index of the memory array
* operation: takes the value 0 for a read operation and 1 for a write operation. 
* 