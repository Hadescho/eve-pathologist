use std::error::Error;
/// Structure containing the information for a single solar system
#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    pub system_id: usize,
    pub name: String
}

impl System {
    /// Creates a new system from id and name
    /// # Examples
    ///
    /// ```
    /// let system = System::new(30005311, "Amygnon")
    /// ```
    pub fn new(id: usize, name: String) -> Self {
        System { system_id: id, name: name }
    }

    pub fn parse_ids(ids_string: &[u8]) -> Result<Vec<usize>, Box<Error>> {
        Ok(::serde_json::from_slice(ids_string)?)
    }

    pub fn parse(json: &[u8]) -> Result<System, Box<Error>> {
        Ok(::serde_json::from_slice(json)?)
    }
}

#[cfg(test)]
mod test {
    use ::models::System;

    #[test]
    fn system_new_test(){
        let system = System::new(0, String::from("Test"));
        assert_eq!(system.system_id, 0);
        assert_eq!(system.name, "Test");
    }

    #[test]
    fn serialize_system_test() {
        let system = System::new(0, String::from("Test"));
        let result = ::serde_json::to_string(&system).unwrap();
        let deser_system: System = ::serde_json::from_str(&result).unwrap();

        assert_eq!(system.system_id, deser_system.system_id);
        assert_eq!(system.name, deser_system.name);

    }

    #[test]
    fn deserialize_system_test() {
        let data = r#"{
                        "name": "Test",
                        "system_id": 0
                      }"#;
        let system: System = ::serde_json::from_str(data).unwrap();
        assert_eq!(system.system_id, 0);
        assert_eq!(system.name, "Test");
    }

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
    }

    #[test]
    fn parse_system_invalid_input_test() {
        let data = b"invalid json";
        let result = System::parse(data);

        assert!(result.is_err());
    }
}
