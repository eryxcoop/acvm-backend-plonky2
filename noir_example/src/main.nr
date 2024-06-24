// use dep::std;

fn main(){
   let x = [163, 117, 178, 149]; // some random bytes
   let hash = std::sha256::sha256_var(x, 4);
}

