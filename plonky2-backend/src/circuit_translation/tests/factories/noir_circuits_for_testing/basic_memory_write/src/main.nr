fn main(mut x: pub [Field; 2], y: pub Field, v: pub Field){
    x[y] = v;
    assert(x[0] == 1);
    assert(x[1] == 11);
}