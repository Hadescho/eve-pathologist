#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    id: usize,
    name: String
}

impl System {
    pub fn new(id: usize, name: String) -> Self {
        System { id: id, name: name }
    }
}

#[cfg(test)]
mod test {
    use System;
    use serde_json::*;

    #[test]
    fn system_new_test(){
        let system = System::new(0, String::from("Test"));
        assert_eq!(system.id, 0);
        assert_eq!(system.name, "Test");
    }

    #[test]
    fn serialize_system_test() {
        let system = System::new(0, String::from("Test"));
        let result = ::serde_json::to_string(&system).unwrap();
        let deser_system: System = ::serde_json::from_str(&result).unwrap();

        assert_eq!(system.id, deser_system.id);
        assert_eq!(system.name, deser_system.name);

    }

    #[test]
    fn deserialize_system_test() {
        let data = r#"{
                        "name": "Test",
                        "id": 0
                      }"#;
        let system: System = ::serde_json::from_str(data).unwrap();
        assert_eq!(system.id, 0);
        assert_eq!(system.name, "Test");
    }
}
