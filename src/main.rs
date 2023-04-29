fn main() {
	//tupla
	let numbers = (1, 2, 3);
	println!("{:?}", numbers);
	
	let numbers: (i32, i32, i32) = (1, 2, 3);
	println!("{:?}", numbers);
	
	let numbers: (i32, i32, f64) = (1, 2, 3.5);
	println!("{:?}", numbers);
}
