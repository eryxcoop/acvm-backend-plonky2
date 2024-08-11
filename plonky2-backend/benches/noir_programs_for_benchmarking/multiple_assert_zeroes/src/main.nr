fn main(random_values: [Field; 100]) -> pub Field {
    let mut acc: Field = 0;
    for i in 0..random_values.len()-3 {
        let x = random_values[i] + random_values[i+1] + random_values[i+2] + random_values[i+3];
        let y = random_values[i] * random_values[i+1] + x;
        let z = random_values[i+2] * random_values[i+3] + x + y;
        acc = acc + x + y + z;
    }
    acc
}