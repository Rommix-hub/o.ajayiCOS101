use std::fs::File;
use std::io::Write;

fn main() {
    let students = vec![
        "Oluchi Mordi | ACC10211111 | Accounting | 300\n",
        "Adams Aliyu | ECO10110101 | Economics | 100\n",
        "Shania Bolade | CSC10328828 | Computer | 200\n",
        "Adekunle Gold | EEE11020202 | Electrical | 200\n",
        "Blanca Edemoh | MEE10202001 | Mechanical | 100\n",
    ];

    let mut file = File::create("pau_students.txt").expect("create failed");

    for student in students.iter() {
        print!("{}", student);
        file.write_all(student.as_bytes()).expect("write failed");
    }

    println!("\nData written to file.");
}