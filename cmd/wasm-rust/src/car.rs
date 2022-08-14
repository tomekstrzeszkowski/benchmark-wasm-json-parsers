use std::io::Error;
use std::fs::read_to_string;
use chrono::{Date, Utc, TimeZone};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

mod deserialize {
    use serde_json::Value;
    use chrono::{Date, Utc, NaiveDate};
    use serde::{Deserialize, Deserializer};
    use regex::Regex;

    pub fn year_deserialize<'de, D>(deserializer: D) -> Result<Option<Date<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_str: String = Deserialize::deserialize(deserializer)?;
        let date = NaiveDate::parse_from_str(&date_str[..], "%Y-%m-%d").unwrap();
        return Ok(Some(Date::<Utc>::from_utc(date, Utc)));
    }

    pub fn horsepower_deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<u8> = Deserialize::deserialize(deserializer)?;
        match value {
            Some(value) => return Ok(value),
            None => return Ok(0),
        }
    }
    
    pub fn miles_per_gallon_deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<f64> = Deserialize::deserialize(deserializer)?;
        match value {
            Some(value) => return Ok(value),
            None => return Ok(0.0),
        }
    }
    
    pub fn displacement_deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value.as_str() {
            Some(value) => return Ok(Some(String::from(value))),
            None => {return Ok(None)},
        }
    }
    pub fn acceleration_deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
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
}

mod serialize {
    use chrono::{Date, Utc};
    use serde::{Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn year_serialize<S>(
        year: &Option<Date<Utc>>, 
        serializer: S
    ) -> Result<S::Ok, S::Error> 
    where
        S: Serializer {
        match year {
            Some(year) => {
                let formatted = format!("{}", year.format(FORMAT));
                serializer.serialize_str(&formatted)
            },
            _ => serializer.serialize_none(),
        }
    }
}

mod default_values {
    pub fn horsepower() -> u8 {
        0
    }
    pub fn cylinders() -> i32 {
        0
    }
    pub fn miles_per_gallon() -> f64 {
        0.0
    }
    pub fn displacement() -> Option<String> {
        None
    }
    pub fn acceleration() -> i64 {
        0
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Car {
    #[serde(default)]
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(default="default_values::miles_per_gallon")]
    #[serde(deserialize_with = "deserialize::miles_per_gallon_deserialize")]
    #[serde(rename(deserialize = "Miles_per_Gallon"))]
    pub miles_per_galon: f64,
    #[serde(default="default_values::displacement")]
    #[serde(deserialize_with = "deserialize::displacement_deserialize")]
    #[serde(rename(deserialize = "Displacement"))]
    pub displacement: Option<String>,
    #[serde(default="default_values::horsepower")]
    #[serde(deserialize_with = "deserialize::horsepower_deserialize")]
    #[serde(rename(deserialize = "Horsepower"))]
    pub horsepower: u8,
    #[serde(default)]
    #[serde(rename(deserialize = "Weight_in_lbs"))]
    pub weight_in_lbs: u16,
    #[serde(default="default_values::cylinders")]
    #[serde(rename(deserialize = "Cylinders"))]
    pub cylinders: i32,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize::year_deserialize", serialize_with="serialize::year_serialize")]
    #[serde(rename(deserialize = "Year"))]
    pub year: Option<Date<Utc>>,
    #[serde(default="default_values::acceleration")]
    #[serde(deserialize_with = "deserialize::acceleration_deserialize")]
    #[serde(rename(deserialize = "Acceleration"))]
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
fn make_cars(content: &str) -> Vec<Car> {
    let mut cars = parse_content(&content).unwrap();
    sort_content(&mut cars);
    return cars;
}

/// Parse items from given path
fn parse_from_file(path: &str) -> Vec<Car> {
    let json_str = read_content(path);
    let cars = make_cars(&json_str);
    return cars;
}

/// Parse JSON content to Car instances
pub fn parse_json(content: &str) -> String {
    let cars = make_cars(&content);
    serde_json::to_string(&cars).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let content = r#"
        [
            {
                "Name":"Wagon",
                "Miles_per_Gallon":18,
                "Cylinders":8,
                "Displacement":"307",
                "Horsepower":130,
                "Weight_in_lbs":3504,
                "Acceleration":"-]2-",
                "Year":"1970-01-01",
                "Origin":"USA"
            },
            {
                "Name":"amc rebel sst",
                "Miles_per_Gallon":16,
                "Cylinders":8,
                "Displacement":"304",
                "Horsepower":150,
                "Weight_in_lbs":3433,
                "Acceleration":12,
                "Origin":"USA"
            }
        ]"#;
        let serialized_data = parse_json(&content);
        assert_eq!(
            serialized_data, 
            concat!(
                r#"[{"name":"amc rebel sst","miles_per_galon":16.0,"displacement":"304","horsepower":150,"#,
                r#""weight_in_lbs":3433,"cylinders":8,"year":null,"acceleration":12},"#,
                r#"{"name":"Wagon","miles_per_galon":18.0,"displacement":"307","horsepower":130,"#,
                r#""weight_in_lbs":3504,"cylinders":8,"year":"1970-01-01","acceleration":2}]"#
            )
        )
    }

    #[test]
    fn test_parse_from_file() {
        let cars = parse_from_file("src/testdata/cars.json");
        assert_eq!(cars.len(), 406);
    }

    #[test]
    fn test_make_cars() {
        let content = r#"
        [
            {
                "Name":"",
                "Miles_per_Gallon":18,
                "Cylinders":8,
                "Displacement":"307",
                "Horsepower":130,
                "Weight_in_lbs":3504,
                "Acceleration":"-]2-",
                "Year":"1970-01-01",
                "Origin":"USA"
            },
            {
                "Name":"amc rebel sst",
                "Miles_per_Gallon":16,
                "Cylinders":8,
                "Displacement":"304",
                "Horsepower":150,
                "Weight_in_lbs":3433,
                "Acceleration":12,
                "Origin":"USA"
            },
            {
                "Name":"amc rebel sst",
                "Miles_per_Gallon":16,
                "Cylinders":8,
                "Displacement":"304",
                "Horsepower":150,
                "Weight_in_lbs":3433,
                "Acceleration":18.8,
                "Origin":"USA"
            }
        ]"#;
        let content = content;
        let cars = make_cars(&content);
        assert_eq!(cars.len(), 3);
    }

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