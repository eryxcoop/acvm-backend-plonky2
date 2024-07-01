fn main() -> pub [u8; 32]{
   let x = [163, 117, 178, 149];
   std::sha256::sha256_var(x, 4)
}
