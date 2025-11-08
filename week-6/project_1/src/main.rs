use std::io;

fn main() {
    println!("===========================================");
    println!("                FOOD MENU                  ");
    println!("===========================================");
    println!("           MENU                  |   PRICE ");
    println!("---------------------------------|---------");
    println!("P = Poundo Yam/Edinkaiko Soup    |  -N3,200");
    println!("F = Fried Rice & Chicken         |  -N3,000");
    println!("A = Amala & Ewedu Soup           |  -N2,500");
    println!("E = Eba & Egusi Soup             |  -N2,000");
    println!("W = White Rice & Stew            |  -N2,500");

    let mut code = String::new();
    let mut quantity = String::new();

    println!("\nWELCOME!\nPlease input food type (P, F, A, E, W): ");
    io::stdin().read_line(&mut code).expect("Failed to read input");
    let code = code.trim().to_uppercase();

    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:u64 = quantity.trim().parse().expect("Not a valid input");

    let price:u64;
    let food: &str;
    if code == "P" {
        price = 3200;
        food = "Poundo Yam/Edinkaiko Soup";
    }
    else if code == "F" {
        price = 3000;
        food = "Fried Rice & Chicken";
    }
    else if code == "A" {
        price = 2500;
        food = "Amala & Ewedu Soup";
    }
    else if code == "E" {
        price = 2000;
        food = "Eba & Egusi Soup";
    }
    else if code == "W" {
        price = 2500;
        food = "White Rice & Stew";
    }
    else {
        println!("Not a valid food type");
        return;
    }
    let total_price:u64 = price * quantity;
    
    println!("============================");
    println!("        ORDER SUMMARY       ");
    println!("============================");
    println!("x{} {}",quantity, food);
    println!("TOTAL PRICE = N{}",total_price);

    if total_price > 10000 {
        let final_price:f64 = 0.95 * (total_price as f64);
        println!("FINAL PRICE (5% discount) = N{}",final_price);
    }
}
