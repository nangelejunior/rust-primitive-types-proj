fn main() {
	let mut numbers = [1.1, 2.0, 3.3];
	println!("{:?}", numbers);
	
	//slice
	println!("{:?}", &numbers[1..3]);
	println!("{:?}", &numbers[1..]);
	println!("{:?}", &numbers[..2]);
}
