use chrono::{Date, Utc, TimeZone};
use std::cmp::Ordering;

pub struct Car {
    pub name: String,
    pub miles_per_galon: f64,
    pub displacement: Option<u16>,
    pub horsepower: u8,
    pub weight_in_lbs: u16,
    pub cylinders: i32,
    pub year: Date<Utc>,
    pub acceleration: i64,
}

impl Car {
    fn is_less(&self, other: &Self) -> bool {
        if self.year == other.year {
            if self.horsepower == other.horsepower {
                return self.name.to_lowercase() < other.name.to_lowercase();
            }
            return self.horsepower < other.horsepower;
        }
        self.year < other.year
    }
}

impl Ord for Car {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_less(&other) {
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.is_less(&other)
    }
}

impl Eq for Car {
    
}

/// Read file content by given path
fn read_content(path: &str) {
    todo!();
}

/// Parse JSON content
pub fn parse_content(content: &str) -> &[Car] {
    todo!()
}

/// Sort by year, horsepower and name respectively
fn sort_content(cars: &mut [Car]) {
    cars.sort();
}

/// Create struct, format fields' values, parse them into specific
/// types and sort.
fn make_cars(content: &str) -> &[Car] {
    let cars = parse_content(&content);
    //sort_content(&cars);
    return &cars;
}

/// Parse items from given path
fn parse_from_file(path: &str) -> &[Car] {
    todo!()
}

/// Parse JSON content to Car instances
pub fn parse_json(content: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn car_can_be_created() {
        let car = Car{
            name: String::from("Honda"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 140,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Utc.ymd(1970, 1, 1),
            acceleration: 100,
        };
        assert_eq!(car.cylinders, 4);
    }
    #[test]
    fn sort_cars() {
        let car1 = Car{
            name: String::from("Honda"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 140,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Utc.ymd(1990, 1, 1),
            acceleration: 100,
        };
        let car2 = Car{
            name: String::from("Acura"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 110,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Utc.ymd(1970, 1, 1),
            acceleration: 100,
        };
        let car3 = Car{
            name: String::from("VW"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 100,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Utc.ymd(1970, 1, 1),
            acceleration: 100,
        };        
        let car4 = Car{
            name: String::from("Bentley"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 110,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Utc.ymd(1970, 1, 1),
            acceleration: 100,
        };
        let mut cars = vec![car1, car2, car3, car4];
        sort_content(&mut cars);
        for c in &cars {
            println!("{}", c.name);
        }
        assert_eq!(cars[0].name, "VW");
        assert_eq!(cars[1].name, "Acura");
        assert_eq!(cars[2].name, "Bentley");
        assert_eq!(cars[3].name, "Honda");

    }
}