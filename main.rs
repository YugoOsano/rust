//use std::io;

fn main (){
	println!("Hello, World!");
	let c1 = 'A';
	let c1_ptr: *const char = &c1;
	println!("Hello, pointer {:?}", c1_ptr);

	let v2: std::vec::Vec<f64> = vec![0.0, -1.0, 1.0];
	assert_eq!(v2.len(), 3);
}


