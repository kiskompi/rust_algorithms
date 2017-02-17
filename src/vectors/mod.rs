use rand::Rng;

pub mod sorts;

/// This function return a vector with a size given as a parameter, filled up with random i32 numbers.
/// # Example:
/// let vector:Vec<i32> = algorithms::vectors::random(30);
pub fn random(size: usize) -> Vec<i32> {
    let mut vector = Vec::new();
    for i in 0..size {
        let j:i32 = i as i32;
        let mut rng = super::rand::thread_rng();
        vector.push(rng.gen::<i32>() % 1000 + j);
    }
    return vector;
}

/// This method prints all of the element of a given vector.
/// # Example:
/// let vector:Vec<i32> = algorithms::vectors::random(30);
/// algorithms::vectors::printv(vector);
pub fn printv(vector: &Vec<i32>) {
    for i in vector {
        print!("{}", i);
    }
    println!("---");
}
