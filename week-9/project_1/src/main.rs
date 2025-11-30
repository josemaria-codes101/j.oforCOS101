use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let mut file = File::create("drinks.csv")?;

    // Write the header
    writeln!(file, "Lager,Stout,Non-Alcoholic")?;

    // The drink data from the table
    let drinks = vec![
        ("33 Export", "Legend", "Maltina"),
        ("Desperados", "Turbo King", "Amstel Malta"),
        ("Goldberg", "Williams", "Malta Gold"),
        ("Gulder", "", "Fayrouz"),
        ("Heineken", "", ""),
        ("Star", "", ""),
    ];

    // Write each row into the CSV
    for (lager, stout, non_alcoholic) in drinks {
        writeln!(file, "{},{},{}", lager, stout, non_alcoholic)?;
    }

    println!("CSV file 'drinks.csv' created successfully!");

    Ok(())
}
