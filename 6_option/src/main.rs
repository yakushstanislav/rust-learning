#[derive(Debug)]
enum Car {
    Audi(u32),
    Mercedes(u32),
    Ford,
}

fn handle_car_price(car: &Car) -> Option<&u32> {
    match car {
        Car::Audi(price) => Some(price),
        Car::Mercedes(price) => Some(price),
        _ => None,
    }
}

fn main() {
    let car1 = Car::Audi(25000);
    let car2 = Car::Mercedes(23000);
    let car3 = Car::Ford;

    let p1 = handle_car_price(&car1);
    let p2 = handle_car_price(&car2);
    let p3 = handle_car_price(&car3);

    println!("Car {:?} has price {:?}", car1, p1);
    println!("Car {:?} has price {:?}", car2, p2);
    println!("Car {:?} has price {:?}", car3, p3);
}
