use std::fs::File;
use std::io::{Write, Result};

struct Student {
    name: &'static str,
    matric: &'static str,
    department: &'static str,
    level: u32,
}

fn main() -> Result<()> {
    // Array / Vector of students (as seen in the screenshot)
    let students = vec![
        Student { name: "Oluchi Nnodi",   matric: "ACC10211111", department: "Accounting", level: 300 },
        Student { name: "Adams Aliyu",    matric: "ECO10110101", department: "Economics", level: 200 },
        Student { name: "Shania Bolade",  matric: "CSC10328828", department: "Computer",   level: 200 },
        Student { name: "Adekule Gold",   matric: "EEE10202020", department: "Electrical", level: 200 },
        Student { name: "Blanca Edemoi",  matric: "MEE10200201", department: "Mechanical", level: 100 },
    ];

    // Display the student details on screen
    println!("PAU SMIS – Student Records");
    println!("--------------------------------------------");
    for s in &students {
        println!(
            "Name: {:20}  Matric: {:12}  Dept: {:12}  Level: {}",
            s.name, s.matric, s.department, s.level
        );
    }

    // Create CSV file
    let mut file = File::create("students.csv")?;

    // Header row
    writeln!(file, "Student Name,Matric Number,Department,Level")?;

    // Write student data
    for s in students {
        writeln!(
            file,
            "{},{},{},{}",
            s.name, s.matric, s.department, s.level
        )?;
    }

    println!("\nFile 'students.csv' created successfully!");

    Ok(())
}