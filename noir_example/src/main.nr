//fn main(x: u8, y: pub u8, x1: u32, y1: u32) -> pub (u8, u32){
//     let z = x^y;
//     let z1 = x1^y1;
//     (z, z1)
//}

fn main(x: Field) -> pub Field {
    let a: [Field; 10] = [0,1,2,3,4,5,6,7,8,9];
    let mut c = 0;
    for _ in 0..10 {
        let b = a[x];
        c += b;
    }
    c
}
