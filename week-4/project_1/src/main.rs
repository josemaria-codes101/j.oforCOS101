use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter a value of b");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter a value of c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    // Calculate the discriminant
    let d:f64 = (b * b) - (4.0 * a * c);

    // Determine the nature of the roots
    if d > 0.0 {
        let root_1 = (-b + d.sqrt()) / (2.0 * a);
        let root_2 = (-b - d.sqrt()) / (2.0 * a);
        println!("There are two distinct real roots; {} and {}", root_1, root_2);
    }
    else if d == 0.0 {
        let root = (-b) / (2.0 * a);
        println!("There is only one real root; {}", root);
    }
    else // If the discriminant is neither greater or equal to 0, therefore it is lesser than 0
    {
        println!("There are no real roots");
    }

}
