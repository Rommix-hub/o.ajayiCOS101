use std::io;

fn main() {
    // 1. Display the Restaurant Menu
    println!("==================================================");
    println!("               THE RESTAURANT MENU                ");
    println!("==================================================");
    println!("Code  Item                                Price   ");
    println!("P     Poundo Yam / Edinkaiko Soup         N3,200  ");
    println!("F     Fried Rice & Chicken                N3,000  ");
    println!("A     Amala & Ewedu Soup                  N2,500  ");
    println!("E     Eba & Egusi Soup                    N2,000  ");
    println!("W     White Rice & Stew                   N2,500  ");
    println!("==================================================\n");

    // 2. Read the food type choice from customer input
    println!("Enter the code for your food choice (P, F, A, E, W):");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim().to_uppercase();

    // 3. Read the quantity from customer input
    println!("Enter the quantity:");
    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");
    let quantity: f32 = quantity_input
        .trim()
        .parse()
        .expect("Please enter a valid number for quantity");

    // 4. Determine price per item based on the selected letter
    let price: f32 = if choice == "P" {
        3200.0
    } else if choice == "F" {
        3000.0
    } else if choice == "A" {
        2500.0
    } else if choice == "E" {
        2000.0
    } else if choice == "W" {
        2500.0
    } else {
        println!("Invalid food selection code!");
        return;
    };

    // 5. Compute the total charge
    let mut total: f32 = price * quantity;
    println!("\nSubtotal: N{:.2}", total);

    // 6. Apply 5% discount if the total charge exceeds N10,000
    if total > 10000.0 {
        let discount = total * 0.05;
        total -= discount;
        println!("Discount (5%): -N{:.2}", discount);
    } else {
        println!("No discount applied.");
    }

    // 7. Output final amount
    println!("Final Total Charge: N{:.2}", total);
}
