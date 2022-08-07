use chrono::{Date, Utc, TimeZone};

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
}