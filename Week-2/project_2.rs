fn main() {
	let toshiba:f64 = 450_000.0 * 2.0;
	let mac:f64 = 1_000_000.0 * 1.0;
	let hp:f64 = 750_000.0 * 3.0;
	let dell:f64 = 2_850_000.0 * 3.0;
	let acer:f64 = 250_000.0 * 1.0;

	//Sum and Average
	let total_sum = toshiba + mac + hp + dell + acer;
	let total_qty = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	println!("Total Sum is {:.2}", total_sum);
	let average = total_sum / total_qty;
	println!("Average Sales is {:.2}", average);
}