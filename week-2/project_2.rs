fn main () {
	/* Toshiba=_item1 = Q*A = 450000*2 = 900,000
		mac=_item2 = Q*A = 1,500,000*1 = 1,500,000
HP=_item3 = Q*A = 750,000*3 = 2,250,000
Dell=_item4 = Q*A = 2,850,000*3 = 8,550,000
Acer=_item5 = Q*A = 250,000*1 = 250,000 
 Q is Quantity
 and A is Amount*/

	
	let _item1:f64 = 900_000.00 ;
	let _item2: f64 = 1500_000.00;
	let _item3: f64 = 2_250_000.00;
	let _item4: f64 = 8_550_000.00;
	let _item5: f64 = 250_000.00;


	let sum = _item1 + _item2 + _item3 + _item4 + _item5;
	println!("SUM = {}", sum);
	let average = sum / 5.0;
	println!("AVERAGE = {}", average);
}