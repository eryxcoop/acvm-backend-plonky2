fn main(hash_input: pub [u8; 4]) -> pub [u8; 32]{
    std::sha256::sha256_var(hash_input, 4)
}