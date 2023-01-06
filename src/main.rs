#[derive(Debug)] // This is needed to print the structure
struct Employees{
    id: i32,
    name: String,
    employeed: bool,
}

fn main() {
    //run_arrays(); // Arrays
    //run_vector(); // Vector
    //run_slice(); // Slice
    //run_tuples(); // Tuples
    //run_structures(); // Structures
    //run_enums(); // Enums
    run_generics(); // Generics
}
fn run_arrays(){
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
fn run_vector(){
    let mut primes: Vec<i32> = Vec::new();
    let mut primes2 = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    println!("The Vector First Example: {:?}", primes);
    println!("The Vector Second Example: {:?}", primes2);

    primes.push(2);
    primes.push(3);
    primes.push(5);
    primes.push(7);

    println!("The Vector First Example: {:?}", primes);
    println!("The Vector Second Example: {:?}", primes2);

    primes.remove(0);
    println!("The Vector First Example: {:?}", primes);

    //Vectors Default Values
    let mut numbers = vec![0; 10]; // 10 zeros
    println!("The Vector First Example: {:?}", numbers);
    const DEFAULT: i32 = 3;
    let numbers2 = vec![DEFAULT; 10]; // 10 threes
    println!("The Vector First Example: {:?}", numbers2);

    //Accessing Vector Elements
    let mut add_numbers = vec![2, 3, 5, 7, 11, 13];
    println!("The first number is: {}", add_numbers[0]);
    add_numbers[0] = 7; // This is allowed
    println!("The first number is: {}", add_numbers[0]);

    for number in add_numbers.iter() {
        println!("The number is: {}", number);
    }

}

fn run_slice() {
    let numbers: [i32;5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &numbers[1..4];
    println!("The slice is: {:?}", slice);
    println!("The slice is: {}", slice[0]);

    let mut colors = ["red", "green", "blue", "orange"];
    println!("The slice is: {:?}", colors);
    update_color(&mut colors[2..4]);
}
fn update_color(colors: &mut [&str]) {
    colors[0] = "yellow";
    colors[1] = "pink";
    //colors[2] = "black"; // This fails because it exceeds the slice
    println!("The slice is: {:?}", colors);
}

fn run_tuples(){
    let tuple: (i32, &str, bool) = (500, "John", true);
    let (id, name, employeed) = tuple;
    println!("The Employee ID: {}", id);
    println!("The Name is: {}", name);
    println!("The Employment Status is: {}", employeed);
    println!("The value of tuple.0 is: {}", tuple.0);
    println!("The value of tuple.1 is: {}", tuple.1);
    println!("The value of tuple.2 is: {}", tuple.2);
}
fn run_structures(){
    let employee = Employees{
        id: 500,
        name: String::from("John"),
        employeed: true,
    };
    println!("The Employee ID: {}", employee.id);
    println!("The Name is: {}", employee.name);
    println!("The Employment Status is: {}", employee.employeed);
    //println!("The Employee: {?:}", employee); // This fails by default
    println!("The Employee: {:#?}", employee); // This works with the derive(Debug) statement
    println!("The Employee: {:#?}", employee.get_employment());
    println!("The Employee: {:#?}", Employees::get_employment_static()); //don't need to Instantiate
}

//Employee implementation
impl Employees{
    fn get_employment(&self) -> String {
        format!("Is {0} Employeed? {1}", self.name, self.employeed)
    }
}
//Employee implementation static
impl Employees{
    fn get_employment_static() -> String {
        String::from("They are Employeed")
    }
}

enum Colors {
    Red,
    Green,
    Blue,
}
#[derive(Debug)]
enum Person {
    Employee(i32, String, bool),
    Manager_Name(String),
    Age(u32),
}
fn run_enums(){
    let color = Colors::Red; 
    match color {
        Colors::Red => println!("The color is Red"), //This needs to be setup with how its defined in the enum
        Colors::Green => println!("The color is Green"),
        Colors::Blue => println!("The color is Blue"),
    }

    let person = Person::Employee(500, String::from("John"), true);
    println!("The Person is: {:#?}", person);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
enum Color<T> {
    Red(T),
    Green(T),
    Blue(T),
}
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

fn run_generics() {
    
    let p1: Point<i32> = Point { x: 5, y: 10 };
    let p2: Point<f64> = Point { x: 1.0, y: 4.0 };
    println!("The Point is: {:#?}", p1);
    println!("The Point is: {:#?}", p2);

    let c1: Color<String> = Color::Red("#FF0000".to_string());
    let c2: Color<String> = Color::Green("#00FF00".to_string());
    let c3: Color<String> = Color::Blue("#0000FF".to_string());
    let c1a: Color<i32> = Color::Red(255);
    println!("The Color is: {:#?}", c1);
    println!("The Color is: {:#?}", c1a);
    println!("The Color is: {:#?}", c2);
    println!("The Color is: {:#?}", c3);
    let p3 = Point2::<i32, f64> { x: 5, y: 10.4 };
    println!("The Point is: {:#?}", p3);

}
