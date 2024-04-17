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

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            year: 2018,
        },
    };

    let house = House {};

    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);
    paint_vehicle_red(&car);
    // paint_vehicle_red(&house); errors coz house does not implement Park
    // paint_vehicle_red(&object); errors coz house does not implement Park
}

fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_string());
}

fn paint_red2(object: &impl Paint) {
    object.paint("red".to_string());
}

fn paint_vehicle_red<T>(object: &T)
where
    T: Paint + Park,
{
    object.paint("red".to_string());
}

fn create_paintable_object() -> impl Paint {
    House {}
}
