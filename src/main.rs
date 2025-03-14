use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }

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

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Accommodation for Hotel {
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

fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1)
}

fn main() {
    let mut hotel: Hotel = Hotel::new(String::from("The Luxe"));
    book_for_one_night(&mut hotel, "Greg");
    println!("{:#?}", hotel);
    let mut airbnb: AirBnB = AirBnB::new(String::from("Peter"), 45);
    book_for_one_night(&mut airbnb, "Fucko");
    println!("{:#?}", airbnb);
}
