use std::error::Error;

/// Structure containing the `x`, `y` and `z` "coordinates" of the system
/// in space
#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// Structure containing the information for a single solar system
#[derive(Debug, Deserialize)]
pub struct System {
    pub system_id: usize,
    pub name: String,
    pub position: ::models::Position,
    pub security_status: f32,
    pub stargates: Vec<usize>
}

impl System {
    /// Converts json array of system ids into a Result containing vector of
    /// system ids. If the process of parsing the json wasn't succesfull returns
    /// a boxed error.
    ///
    /// Arguments:
    ///
    /// ids_string: &[u8] - Reference to json array of Integers
    pub fn parse_ids(ids_string: &[u8]) -> Result<Vec<usize>, Box<Error>> {
        Ok(::serde_json::from_slice(ids_string)?)
    }

    /// Converts json object containing information about a solar system into
    /// `System` object. If the process of parsing the json wasn't successfull
    /// returns a boxed error.
    ///
    /// Arguments:
    ///
    /// json: &[u8] - Reference to json object structured like the System struct
    pub fn parse(json: &[u8]) -> Result<System, Box<Error>> {
        Ok(::serde_json::from_slice(json)?)
    }
}

#[cfg(test)]
mod test {
    use ::models::System;
    use spectral::prelude::*;

    #[test]
    fn parse_ids_test() {
        let data = b"[1, 2, 3]";
        let result = System::parse_ids(data);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1,2,3]);
    }

    #[test]
    fn parse_ids_with_invalid_input_test() {
        let data = b"invalid json";
        let result = System::parse_ids(data);

        assert!(result.is_err());
    }

    #[test]
    fn parse_system_test() {
        let data = br#"{"star_id":40335883,"system_id":30005311,"name":"Amygnon","position":{"x":-2.437652065551942e+17,"y":3.610789959702216e+16,"z":5.16497631623793e+16},"security_status":0.6375686526298523,"constellation_id":20000777,"planets":[{"planet_id":40335884},{"planet_id":40335886,"moons":[40335887]},{"planet_id":40335888,"moons":[40335889]},{"planet_id":40335890,"moons":[40335891]},{"planet_id":40335892,"moons":[40335893]},{"planet_id":40335894,"moons":[40335895,40335896,40335897,40335898,40335899,40335901,40335902,40335903,40335904,40335905,40335906,40335907,40335908,40335909,40335910,40335911]},{"planet_id":40335912,"moons":[40335913,40335914,40335915,40335916,40335917,40335918,40335919,40335920,40335921,40335922,40335923,40335924,40335925,40335926,40335927,40335928,40335929,40335930,40335931,40335932,40335933,40335934]},{"planet_id":40335936,"moons":[40335939,40335940,40335941,40335942,40335943,40335944,40335945,40335946,40335947,40335948,40335950,40335952,40335953,40335954,40335955,40335956,40335957,40335959,40335960,40335961,40335962,40335964]}],"security_class":"D1","stargates":[50007433],"stations":[60011203,60011209,60011530,60011533,60014479]}"#;
        let result = System::parse(data);

        assert!(result.is_ok());

        let system = result.unwrap();

        assert_eq!(system.system_id, 30005311);
        assert_eq!(system.name, "Amygnon");
        assert_that(&system.security_status).is_close_to(0.63, 0.01);
        assert_that(&system.position.x).is_close_to(-2.43e+17, 0.01e+17);
        assert_that(&system.stargates).has_length(1);
    }

    #[test]
    fn parse_system_invalid_input_test() {
        let data = b"invalid json";
        let result = System::parse(data);

        assert!(result.is_err());
    }
}
