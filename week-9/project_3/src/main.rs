use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efcc_merged.txt").expect("create failed");

    for i in 0..commissioners.len() {
        let record = format!("{} | {} | {}\n", commissioners[i], ministries[i], zones[i]);
        file.write_all(record.as_bytes()).expect("write failed");
    }

    println!("\nData written to file.");
}