use std::io;

fn main() {
    // Vector of tuples storing (candidate_name, years_of_experience)
    let mut candidates: Vec<(&str, i32)> = Vec::new();

    // Adding candidates to the vector
    candidates.push(("Aisha", 4));
    candidates.push(("Chidi", 7));
    candidates.push(("Blessing", 2));
    candidates.push(("Femi", 9));
    candidates.push(("Grace", 5));

    // Variables to track the highest experience seen so far
    let mut highest_years = -1;
    let mut top_candidate = "";

    // Loop through the vector to find the developer with the highest experience
    for candidate in candidates.iter() {
        if candidate.1 > highest_years {
            highest_years = candidate.1;
            top_candidate = candidate.0;
        }
    }

    // Display the result
    println!("==================================================");
    println!("        EY NIGERIA RECRUITMENT RESULT             ");
    println!("==================================================");
    println!("Developer with highest experience: {}", top_candidate);
    println!("Years of programming experience: {} years", highest_years);
}