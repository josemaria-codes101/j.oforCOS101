use std::io::Read;

fn open_file(x: &str) {
    let mut file = std::fs::File::open(x).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    loop {
        println!("Choose user status");
        println!("1. Administrator");
        println!("2. Project Manager");
        println!("3. Employee");
        println!("4. Customer");
        println!("5. Vendor");
        println!("0. Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("FAILED TO READ INPUT");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid choice\n");
                continue;
            }
        };

        match input {
            1 => open_file("globacom_db.sql"),
            2 => open_file("project_tb.sql"),
            3 => open_file("staff_tb.sql"),
            4 => open_file("customer_tb.sql"),
            5 => open_file("dataplan_tb.sql"),
            0 => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid status entered."),
        }

        println!(); // spacing between iterations
    }
}
