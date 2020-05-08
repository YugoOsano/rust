//use std::io;

// A function to practice 'slice' transcribed from Practical Rust p-166 (190 in PDF)
fn print_info(name: &str, s1: &[f64]) {
    println!(" {:9} - {}, {:?}, {:?}, {:?}",
             name,
             s1.len(),
             s1.first(),
             s1[1],
             s1.last()
    );
}

fn main (){
    // a phrase which ends by ! is a macro.
    // It can accepts variable number of args while
    // the number of a function's arguments has to be fixed.
    // println! macro calls std::io::_print
    println!("Hello, World!");
    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    println!("Hello, pointer {:?}", c1_ptr);

    // vec! is a macro to initialize a Vec.
    let v2: std::vec::Vec<f64> = vec![0.0, -1.0, 1.0];
    assert_eq!(v2.len(), 3);
    println!("{:?}", v2);
    print_info("&v2[..]", &v2[..]);
}


