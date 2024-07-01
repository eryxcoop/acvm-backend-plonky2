fn main() -> pub Field{
    let mut a = 0;
    let mut b = 1;
    for _ in 0..13 {
        let aux = a + b;
        a = b;
        b = aux;
    }
    b
}
