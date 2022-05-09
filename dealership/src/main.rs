use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs,fmt, path::Path};

#[derive(Serialize, Deserialize, Debug)]
struct Car {
    brand: String,
    model: String,
    year: u32,
    colour: String,
}

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "brand: {}, model: {}, year: {}, colour: {}", self.brand, self.model, self.year, self.colour)
    }
}

fn read_file() -> Option<Vec<Car>> {

    let data: String = fs::read_to_string("./cars.json").ok()?;

    let cars: Option<Vec<Car>> = serde_json::from_str(&data).ok()?;

    cars
}

fn add_car(car: Car) {
    let data = read_file();

    if let Some(mut cars) = data {
        cars.push(car);
        let data = serde_json::to_string(&cars);

        if let Ok(json_string) = data {
            let path = Path::new("./cars.json");
            fs::write(&path, json_string.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let car = Car {
        brand: "Jeep".to_owned(),
        model: "Wrangler".to_owned(),
        year: 2022,
        colour: "Gray".to_owned(),
    };

    add_car(car);

    let data = read_file();
    if let Some(cars) = data { 
        for c in cars.iter(){
            println!("{}", c);
        }
    }
}
