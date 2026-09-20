use std::io;

fn main() {

    println!("\nThe Incentive Calculator");

    println!("\nAre you experienced? (Enter 1 for yes or 0 for no):");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Wrong answer");
let  experienced:u64 = answer.trim().parse().expect("wrong input");
    if experienced == 1{

        println!("\nHow old are you?");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Wrong age");
        let _age:u64 = age.trim().parse().expect("Wrong age");

        if age >=40.to_string(){
            println!("Annual Incentive = 1_560_000");
        }

        else if age >=30 .to_string() && age<=39.to_string(){
            println!("Annual Incentive = 1_480_000");
        }

        else if age<=28.to_string(){
            println!("Annual Incentive = 1_300_000");
        }

        

    }

    else if experienced == 0{
        println!("Annual Incentive = 100_000");
    }
    
}
