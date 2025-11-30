fn main () {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    let zones = vec![
        "South West",
        "North West",
        "South South",
        "South West",
        "South East"
    ];

    println!("=====================================================================");   
    println!("             CONVICTED MINISTERS MERGED EFCC DATASET                 ");
    println!("=====================================================================");
    println!("{:<5} {:<30} {:<20} {:<15}", "S/N", "COMMISSIONER", "MINISTRY", "ZONE");

    for i in 0..commissioners.len() {
        let name = commissioners[i];
        let ministry = ministries[i];
        let zone = zones[i];

        println!("{:<5} {:<30} {:<20} {:<15}", i + 1, name, ministry, zone);
    }
}