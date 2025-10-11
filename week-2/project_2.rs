fn main () {
    // Individual sales amounts
    let toshiba = 450_000.0;
    let mac = 1_500_000.0;
    let hp = 750_000.0;
    let dell = 2_850_000.0;
    let acer = 250_000.0;

    // Calculate total
    let total = toshiba + mac + hp + dell + acer;
     println!("Total Sales is {}", total);

    // Calculate average
    let average = total / 10.0; // 10 items total
    println!("Average Sales is {}", average);
}