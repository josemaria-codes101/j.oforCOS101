use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Is the employee experienced? (yes or no)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience = input1.trim().to_lowercase();

    println!("What is the employee's age? ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:u32 = input2.trim().parse().expect("Not a valid age");

    let incentive:i64;

    if experience == "yes" {

        if age >= 40 {
            incentive = 1_560_000;
        }
        else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        }
        else if age < 28 {
            incentive = 1_300_000;
        }
        else // There's no incentive for experienced between 28 and 29 so i provided a close incentive.
        {
            incentive = 1_300_000;
        }
    }
    else // if the employee is not experienced it automatically means the employee is inexperienced 
    {
        incentive = 100_000; 
    }
    println!("The annual incentive for this employee is N{}", incentive);
}
