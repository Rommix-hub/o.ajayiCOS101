use std::io;

fn main() {
    // Vectors matching the table columns
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    
    // Vector matching the APS level rows
    let aps_levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // Prompt user for job title
    println!("Enter job title:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let job_title = input.trim();

    // Loop through the vector indices to find a match
    for i in 0..aps_levels.len() {
        if job_title == office_admin[i] || job_title == academic[i] || job_title == lawyer[i] || job_title == teacher[i] {
            println!("Staff Level: {}", aps_levels[i]);
            return;
        }
    }
}    
