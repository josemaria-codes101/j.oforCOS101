use std::io;

#[derive(Debug)]
    struct Developer {
        name: String,
        years: u32,
    }

fn main() {
    println!("==================================================================");
    println!("               ERNST & YOUNG (EY) GLOBAL LIMITED                  ");
    println!("==================================================================");
    
    let mut interviewees = String::new();
    println!("Enter number of interviewees:");
    io::stdin().read_line(&mut interviewees).expect("Failed to read input");
    let n:u32 = interviewees.trim().parse().expect("Invalid input");

    let mut developers: Vec<Developer> = Vec::new();
    for i in 1..=n {
        println!("Enter name of interviewee {}:", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        name = name.trim().to_string();

        println!("Enter years of programming experience of interviewee {}:", i);
        let mut years = String::new();
        io::stdin().read_line(&mut years).expect("Failed to read input");
        let years:u32 = years.trim().parse().expect("Invalid input");

        developers.push(Developer {name, years}); // push value into empty vector
    }
    let most_experienced = developers
        .iter() // doesn't take the values out the vector (borrows)
        .max_by_key(|dev| dev.years) // take the value with the highest key and compares with years(.years)
        .expect("No interviewees found"); // message if key is empty

        println!("Interviewee with the highest years of programming experience is:\n{} ({} years)", most_experienced.name, most_experienced.years);
}
