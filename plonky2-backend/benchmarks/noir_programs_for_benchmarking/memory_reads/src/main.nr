fn main(x: [Field; 100]) -> pub Field {
    let a: [Field; 10] = [0,1,2,3,4,5,6,7,8,9];
    let mut c = 0;
    for i in 0..100 {
        let b = a[x[i]];
        c += b;
    }
    c
}
