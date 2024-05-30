use rand::Rng;

fn main() {
let two_dim_array: [[i32; 16]; 16] = [[0; 16]; 16];

let mut rng = rand::thread_rng();
let mut two_dim_array: [[i32; 16]; 16] = [[0; 16]; 16];

for i in 0..16 {
    for j in 0..16 {
        two_dim_array[i][j] = rng.gen_range(0..100); // Generating random numbers between 0 and 99
    }
}

for row in &two_dim_array {
    println!("{:?}", row);
}

}
