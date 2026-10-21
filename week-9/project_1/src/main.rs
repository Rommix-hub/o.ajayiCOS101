use std::fs::File;
use std::io::Write;

fn main() {
    // 1. Create the output file (overwrites if it already exists)
    let mut file = File::create("nigerian_breweries.txt").expect("Failed to create file");

    // 2. Define the header and content based on the table in Project I
    let content = "==========================================================\n\
                   NIGERIAN BREWERIES PLC - CATEGORIES OF DRINKS             \n\
                   ==========================================================\n\
                   Lager               | Stout               | Non-Alcoholic \n\
                   ----------------------------------------------------------\n\
                   33 Export           | Legend              | Maltina       \n\
                   Desperados          | Turbo King          | Amstel Malta  \n\
                   Goldberg            | Williams            | Malta Gold    \n\
                   Gulder              |                     | Fayrouz       \n\
                   Heineken            |                     |               \n\
                   Star                |                     |               \n\
                   ==========================================================\n";

    // 3. Write the formatted string data into the file as bytes
    file.write_all(content.as_bytes())
        .expect("Failed to write data to file");

    // 4. Print confirmation to console
    println!("Successfully saved Nigerian Breweries drink categories to 'nigerian_breweries.txt'.");
}