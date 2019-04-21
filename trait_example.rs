// https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/traits.html
use std::io;

struct Circle {
	x: f64,
	y: f64,
	radius: f64,
}

trait HasArea {
	fn area(&self) -> f64;
}

impl HasArea for Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}
}

fn main () {
	let c = Circle {
		x: 0.0f64,
		y: 0.0f64,
		radius: 1.0f64,
	};
	println!("Area is: {}", c.area()); 
}

