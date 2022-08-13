use std::io::Error;
use serde_json::Value;
use std::fs::read_to_string;
use chrono::{Date, Utc, TimeZone, NaiveDate};
use std::cmp::Ordering;
use serde::{Deserialize, Deserializer, de};
use regex::Regex;

fn year_deserialize<'de, D>(deserializer: D) -> Result<Option<Date<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    let date = NaiveDate::parse_from_str(&date_str[..], "%Y-%m-%d").unwrap();
    return Ok(Some(Date::<Utc>::from_utc(date, Utc)));
}

fn acceleration_deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let acc_value: Value = Deserialize::deserialize(deserializer)?;
    let acc_str: String = match acc_value.as_str() {
        Some(val) => String::from(val),
        None => {
            match acc_value.as_f64() {
                Some(val) => return Ok(val as i64),
                None => {
                    return Ok(acc_value.as_i64().unwrap());
                },
            };
        },
    };
    let re = Regex::new(r"[^\d]").unwrap();
    let safe_str = re.replace_all(&acc_str[..], "");
    let parsed = safe_str.parse::<i64>().unwrap();
    return Ok(parsed);
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Car {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Miles_per_Gallon"))]
    pub miles_per_galon: f64,
    #[serde(rename(deserialize = "Displacement"))]
    pub displacement: Option<String>,
    #[serde(rename(deserialize = "Horsepower"))]
    pub horsepower: u8,
    #[serde(rename(deserialize = "Weight_in_lbs"))]
    pub weight_in_lbs: u16,
    #[serde(rename(deserialize = "Cylinders"))]
    pub cylinders: i32,
    #[serde(default)]
    #[serde(deserialize_with = "year_deserialize")]
    #[serde(rename(deserialize = "Year"))]
    pub year: Option<Date<Utc>>,
    #[serde(rename(deserialize = "Acceleration"))]
    #[serde(deserialize_with = "acceleration_deserialize")]
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

impl Eq for Car {}

/// Read file content by given path
fn read_content(path: &str) -> String {
    read_to_string(path).expect("unable to read the file")
}

/// Parse JSON content
pub fn parse_content(content: &str) -> Result<Vec<Car>, Error> {
    let cars: Vec<Car> = serde_json::from_str(content)?;
    Ok(cars)
}

/// Sort by year, horsepower and name respectively
fn sort_content(cars: &mut Vec<Car>) {
    cars.sort();
}

/// Create struct, format fields' values, parse them into specific
/// types and sort.
fn make_cars(content: &String) -> Vec<Car> {
    let mut cars = parse_content(&content).unwrap();
    sort_content(&mut cars);
    return cars;
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
    fn test_parse_content() {
        let json_str = read_content("src/testdata/cars-tiny.json");
        let parsed = parse_content(&json_str).unwrap();
        assert_eq!(parsed[0].acceleration, 2);
        assert_eq!(parsed[1].acceleration, 12);
        assert_eq!(parsed[2].acceleration, 18);
    }
    #[test]
    fn reading_file() {
        let file_content = read_content("src/testdata/cars.json");
        assert_eq!(file_content.len(), 101399);
    }
    #[test]
    fn car_can_be_created() {
        let car = Car{
            name: String::from("Honda"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 140,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Some(Utc.ymd(1970, 1, 1)),
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
            year: Some(Utc.ymd(1990, 1, 1)),
            acceleration: 100,
        };
        let car2 = Car{
            name: String::from("Acura"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 110,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Some(Utc.ymd(1970, 1, 1)),
            acceleration: 100,
        };
        let car3 = Car{
            name: String::from("VW"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 100,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Some(Utc.ymd(1970, 1, 1)),
            acceleration: 100,
        };        
        let car4 = Car{
            name: String::from("Bentley"),
            miles_per_galon: 12.6,
            displacement: None,
            horsepower: 110,
            weight_in_lbs: 1900,
            cylinders: 4,
            year: Some(Utc.ymd(1970, 1, 1)),
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