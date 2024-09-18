### AssertZero
AssertZero is the main opcode in almost any circuit: it allows arithmetic operations over an arbitrary set of variables. This arithmetic operations are represented through equations that must hold throughout the program. For example, if we want to say

```x := y*y + z - 2``` 

we should express it like an equation 

```y*y + z - x - 2 == 0```

that ultimately will translate into an AssertZero opcode of the form

```rust 
{
    quadratic_terms: [(1,y,y)]
    linear_combinations: [(1,z), (-1, x)]
    constant: -2
}
```

Any assignment of a variable done in Noir, or any comparison by equality will end up being an AssertZero opcode. 

#### Equivalence in Plonky2

Plonky2 has an ```ArithmeticGate``` that we can access through the CircuitBuilder API. There are some methods that we can use to facilitate the translation without the need of using the gate directly:
* ```add(t1: Target, t2: Target)```
* ```mul(t1: Target, t2: Target)```
* ```mul_const(t1: Target, c: FieldElement)```
* ```assert_zero(t: Target)``` -> Restricts the value being held by the target to equal 0.

The ArithmeticGate accepts equations of the form

```k_0 * x * y + k_1 * z```

so to translate a single AssertZero we need to generate a Plonky2 circuit that potentially uses many Arithmetic Gates. So, to sum it up, a single AssertZero opcode might be translated to many arithmetic operations. 