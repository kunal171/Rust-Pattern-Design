// Let's consider building a Car object using the Builder Pattern:

// Product
#[derive(Debug, PartialEq, Clone)]
struct Car {
    model: String,
    color: String,
    wheels: usize,
}

//car Builder
trait CarBuilder {
    fn set_model(&mut self, model: String);
    fn set_color(&mut self, color: String);
    fn set_wheels(&mut self, wheels: usize);
    fn build(&self) -> Car;
}

// Concrete Builder
struct SedanBuilder {
    car: Car,
}

impl SedanBuilder {
    fn new() -> Self {
        Self {
            car: Car {
                model: String::new(),
                color: String::new(),
                wheels: 4,
            },
        }
    }
}

impl CarBuilder for SedanBuilder {
    fn set_model(&mut self, model: String) {
        self.car.model = model;
    }

    fn set_color(&mut self, color: String) {
        self.car.color = color;
    }

    fn set_wheels(&mut self, wheels: usize) {
        self.car.wheels = wheels;
    }

    fn build(&self) -> Car {
        self.car.clone()
    }
}

// Director (optional)
struct CarDirector;

impl CarDirector {
    fn construct<T: CarBuilder>(builder: &mut T, model: String, color: String, wheels: usize) -> Car {
        builder.set_model(model);
        builder.set_color(color);
        builder.set_wheels(wheels);
        builder.build()
    }
}

fn main() {
    let mut sedan_builder = SedanBuilder::new();
    let car = CarDirector::construct(&mut sedan_builder, "Toyota Camry".to_string(), "Blue".to_string(), 4);
    println!("{:?}", car);
}