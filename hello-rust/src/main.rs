use rand::Rng;

fn main() {
    println!("Change the world!");

    // Generate a random number between 0 and 20
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..=20);
    println!("Random number: {}", random_number);

    if random_number > 10 {
        println!("The number is greater than 10: {}", random_number);
    } else {
        println!("The number is less than or equal to 10: {}", random_number);
    }
}
