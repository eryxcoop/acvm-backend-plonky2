fn main(x: [Field; 1000]) -> pub Field{
    let mut acc: Field = 0;
    for i in x {
        acc = acc + i;
    }
    acc
}
