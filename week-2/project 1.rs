fn main () {
	let p:f64 = 520000000;
	let r:f64 = 10;
	let n:f64 = 5;

	// compound interest
	let a = p * (1.0 + (r / 100)).powf (n)
	println!("Amount is {}", a);
	let ci = a - p
	println!("Compound Interest is {}", ci);
}