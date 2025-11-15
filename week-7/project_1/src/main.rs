use std::io;
use std::f64::consts::PI;

fn main() {
    println!("======================================");
    println!("          SHAPE CALCULATOR            ");
    println!("======================================");
    println!("  Select what you want to calculate   ");
    println!("--------------------------------------");
    println!("1.| Area of Trapezium"); // h/2 * (b1 + b2)
    println!("2.| Area of Rhombus"); // 1/2 * d1 * d2
    println!("3.| Area of Parallelogram"); // base * alt
    println!("4.| Area of Cube"); // 6 * (side^2)
    println!("5.| Volume of Cylinder\n"); // pi * r^2 * h

    // made a choice variable
    // read_input will be a function for reading input

    let choice = read_input("Enter your choice (1-5):").trim().parse::<u32>().unwrap();

    if choice == 1 {

    // read_number will be a function for asking a prompt and reading input

       let h = read_number("Enter the height:");
       let b1 = read_number("Enter base1:");
       let b2 = read_number("Enter base2:");

    // area_trapezium will be a function for calculating the area of trapezium with 3 parameters

       println!("Area of Trampezium = {}", area_trapezium(h, b1, b2)); 

    }

    else if choice == 2 {
        let d1 = read_number("Enter diagonal1:");
        let d2 = read_number("Enter diagonal2:");

    // area_rhombus will be a function for calculating the area of a rhombus

        println!("Area of Rhombus = {}", area_rhombus(d1, d2));
    }

    else if choice == 3 {
        let base = read_number("Enter length of base:");
        let alt = read_number("Enter altitude height:");
        println!("Area of Parallelogram = {}", area_parallelogram(base, alt));
    }

    else if choice == 4 {
        let side = read_number("Enter length of side:");
        println!("Surface Area of Cube = {}", area_cube(side));
    }

    else if choice == 5 {
        let r = read_number("Enter radius of cylinder:");
        let h = read_number("Enter height of cylinder:");
        println!("Volume of cylinder is = {}", volume_cylinder(r, h));
    }

    else {
        println!("Invalid choice");
    }

    //---INPUT FUNCTIONS---
    fn read_input(prompt:&str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input // or return input;
    }

    fn read_number(prompt:&str) -> f64 {
        read_input(prompt).trim().parse::<f64>().unwrap()
    }

    //---SHAPE FUNCTIONS--
    fn area_trapezium(h:f64, b1:f64, b2:f64) -> f64 {
        (h / 2.0) * (b1 + b2)
    }
    fn area_rhombus(d1:f64, d2:f64) -> f64 {
        0.5 * d1 * d2
    }
    fn area_parallelogram(base:f64, alt:f64) -> f64 {
        base * alt
    }
    fn area_cube(side:f64) -> f64 {
        6.0 * side * side
    }
    fn volume_cylinder(r:f64, h:f64) -> f64 {
        PI * r * r * h
    }

}
