/*fn main(
    x: [u32; 5],
    mut z: u32,
    t: u32,
    index: [Field; 5],
    index2: [Field; 5],
    offset: Field,
    sublen: Field
) {
    let idx = (z - 5 * t - 5) as Field;
    //dynamic array test
    dyn_array(x, idx, idx - 3);
    //regression for issue 1283
    let mut s = 0;
    let x3 = [246, 159, 32, 176, 8];
    for i in 0..5 {
        s += x3[index[i]];
    }
    assert(s != 0);

    if 3 < (sublen as u32) {
        assert(index[offset + 3] == index2[3]);
    }
}*/


fn main(mut x: [Field; 1], mut y: Field){
        x[y] = 1;
        assert(x[0] == 1);
}

fn uuu(mut x: [u32; 5], mut y: Field) -> u32{
    y = 0;
    x[y] = 2;
    assert(x[y] == 2);
    x[y]
}