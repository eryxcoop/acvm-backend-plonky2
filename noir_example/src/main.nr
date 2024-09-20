fn main(x: u8, y: pub u8, x1: u32, y1: u32) -> pub (u8, u32){
     let z = x^y;
     let z1 = x1^y1;
     (z, z1)
}
