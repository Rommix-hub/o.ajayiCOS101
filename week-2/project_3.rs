fn main() {
	let p:f64 = 210000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	// amount
	let a = p*(  1.00 - (r / 100.00)).powf (n);
	println!(" amount is {}", a);
}