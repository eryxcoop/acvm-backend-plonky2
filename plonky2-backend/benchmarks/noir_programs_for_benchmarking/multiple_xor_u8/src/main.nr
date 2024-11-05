fn main(random_values: [u8; 1000]) -> pub u8 {
    let mut acc: u8 = 0;
    for i in 0..1000 {
        acc = acc ^ random_values[i];
    }
    acc
}
