fn main() {
	//tupla
	let mut numbers = (1, 2, 3);
	println!("{:?}", numbers);
	
	numbers.0 = 50;
	println!("{:?}", numbers);
}
