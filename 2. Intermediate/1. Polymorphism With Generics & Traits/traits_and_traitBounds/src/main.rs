trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting vehicle with color: {}", color);
    }
}
struct VehicleInfo {
    make: String,
    model: String,
    year: u32,
}
struct Car {
    // make: String,
    // model: String,
    // year: u32,
    info: VehicleInfo,
}
impl Park for Car {
    fn park(&self) {
        println!("Parking car");
    }
}

impl Paint for Car {}

struct Truck {
    // make: String,
    // model: String,
    // year: u32,
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("Unloading truck");
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("Parking truck");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting house with color: {}", color);
    }
}

fn main() {}
