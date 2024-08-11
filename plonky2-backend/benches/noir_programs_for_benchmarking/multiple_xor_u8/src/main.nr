fn main(random_values: [u8; 100000]) -> pub u8 {
    let mut acc: u8 = 0;
    for i in 0..100000 {
        acc = acc ^ random_values[i];
    }
    acc
}
