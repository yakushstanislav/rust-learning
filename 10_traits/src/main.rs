pub trait Car {
    fn name(&self) -> &'static str;

    fn speed(&self) -> u32;

    fn beep(&self) -> &'static str {
        return "Beep!";
    }
}

pub trait Sale {
    fn get_cost(&self) -> &u32;
}

struct Audi {
    pub cost: u32,
}

struct Mercedes;
struct BMW;

impl Sale for Audi {
    fn get_cost(&self) -> &u32 {
        return &self.cost;
    }
}

impl Car for Audi {
    fn name(&self) -> &'static str {
        return "Audi";
    }

    fn speed(&self) -> u32 {
        return 320;
    }

    fn beep(&self) -> &'static str {
        return "Bip! Bip!";
    }
}

impl Car for Mercedes {
    fn name(&self) -> &'static str {
        return "Mercedes";
    }

    fn speed(&self) -> u32 {
        return 180;
    }

    fn beep(&self) -> &'static str {
        return "Beep! Beep!";
    }
}

impl Car for BMW {
    fn name(&self) -> &'static str {
        return "BMW";
    }

    fn speed(&self) -> u32 {
        return 250;
    }
}

fn display_car(car: &impl Car) {
    println!("{}: speed: {}, {}!", car.name(), car.speed(), car.beep());
}

fn display_car_extended<T: Car + Sale>(car: &T) {
    println!(
        "{}: speed: {}, cost: {}$, {}!",
        car.name(),
        car.speed(),
        car.get_cost(),
        car.beep(),
    );
}

fn enable_signal_car<T>(car: &T)
where
    T: Car,
{
    println!("{}", car.beep());
}

fn main() {
    let audi = Audi { cost: 25000 };
    let mercedes = Mercedes;
    let bmw = BMW;

    display_car(&audi);
    display_car(&mercedes);
    display_car(&bmw);

    display_car_extended(&audi);

    enable_signal_car(&audi);
}
