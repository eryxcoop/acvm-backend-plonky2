fn main(x: [u8; 100000]) /*-> pub [u8; 1000] */{

}

// 6.000.000
// Nargo execute (plonky2): 48s
// Plonky2 prove: (muere por uso excesivo de memoria) 7s
// Nargo execute (barretenberg): 67s
// Barretenberg prove: (muere por uso excesivo de memoria) 1s

// 100.000
//Plonky2 prove without lookups: 10s
//Plonky2 prove with lookups: 1,5s
