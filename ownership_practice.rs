// https://igaguri.hatenablog.com/entry/2019/08/17/184205
// ownership means a structure instance is responsible also for
// releasing resource (memory) assigned to its components,
// not only for itself.

use std::fmt;// stands for format

struct Coords {
    x: f64,
    y: f64,
}

impl Coords {
    fn boxed(x: f64, y: f64) -> Box<Coords> {
        Box::new(Coords {x,y})
    }
}
//-- implementation for fmt's Display trait
impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:.4}, {:.4}]", self.x, self.y)
    }
}

struct Location {
    name: String,
    coords: Box<Coords>,
}

impl Location {
    fn boxed(name: String, coords: Box<Coords>) -> Box<Location> {
        Box::new(Location{name, coords})
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name, self.coords)
    }
}

fn sample_function() {
    let coords = Coords::boxed(35.6290249, 139.7943864);
    let name = "Tokyo Big Sight".to_owned();
    let loc = Location::boxed(name, coords);

    println!("{}", loc);

    // スコープを抜けることで自動で`loc`が破棄され、その中身である`coords`と`name`も自動で破棄される
}

fn main() {
    sample_function()
}

