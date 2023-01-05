fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    
    println!("The first prime is: {}", primes[0]);
    println!("The second prime is: {}", primes[1]); 

    println!("The first double is: {}", doubles[0]);
    println!("The second double is: {}", doubles[1]);
    println!("{:?}", doubles); // Prints the entire array

    //Create Arrays with Default Values
    let mut numbers = [0; 10]; // 10 zeros
    println!("{:?}", numbers); // Prints the entire array

    const DEFAULT: i32 = 3;
    let numbers2 = [DEFAULT; 10]; // 10 threes
    println!("{:?}", numbers2); // Prints the entire array

    //Accessing Array Elements
    let mut add_numbers = [2, 3, 5, 7, 11, 13];
    println!("The first number is: {}", add_numbers[0]);
    add_numbers[0] = 7; // This is allowed

    for number in add_numbers.iter() {
        println!("The number is: {}", number);
    }
}

