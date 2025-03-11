use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String;
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservation: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: String) -> Self {
        Self {
            name,
            reservation: HashMap::new(),
        }
    }
}

impl Accommodation for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxery", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservation.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: String, guests: u32) -> Self {
        Self {
            host,
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnB {
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment!", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights))
    }
}

fn main() {
    let mut hotel: Hotel = Hotel::new(String::from("The Luxe"));
    println!("{:#?}", hotel.get_description());

    let mut airbnb: AirBnB = AirBnB::new(String::from("Peter"), 45);
    println!("{:?}", airbnb.get_description());

    hotel.book("Robert", 23);
    airbnb.book("Taco", 74);

    println!("{hotel:?}");
    println!("{airbnb:?}");
}
