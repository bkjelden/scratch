fn main() {
    println!("Hello, world!");
    let mut mutable_binding = 1;
    println!("before mut: {}", mutable_binding);
    mutable_binding = 2;
	println!("after mut: {}", mutable_binding);   

	let _long_lived_binding = 1;

	
		let _long_lived_binding = 2;
	 

	println!("{}", _long_lived_binding);
}
