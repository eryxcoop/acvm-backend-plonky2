fn main(mut v: pub [Field; 5], idx: pub Field){
    let value_that_surely_is_different_than_0 = v[idx];
    assert(value_that_surely_is_different_than_0 == 0);
}
