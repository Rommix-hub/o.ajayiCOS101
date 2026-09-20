use std::io;

fn main() {
    println!("Enter the value of a ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Not a string");
    let a:f32 = a.trim().parse().expect("Not a variable");

    println!("Enter the value of b ");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Not a string");
    let b:f32 = b.trim().parse().expect("Not a variable");

    println!("Enter the value of c ");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Not a string");
    let c:f32 = c.trim().parse().expect("Not a variable");

    let d = b * b - 4.0 * a * c;
    if d > 0.0 {
        let root1 = -b + d.sqrt() / (2.0 * a);
        let root2 = -b - d.sqrt() / (2.0 * a);

        println!("There are two roots");
        println!("Root1 = {}", root1);
        println!("Root2 = {}", root2);
    }
    else if d ==0.0{
    let root = -b / (2.0 * a);
    println!("1root = {}", root);
    }

    else {
        println!("There is no root");
    }
}
