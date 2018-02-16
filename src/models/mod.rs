use std::error::Error;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use hyper::Client;
use esi;

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

/// Struct containing the available information for the whole ingame universe.
#[derive(Debug)]
pub struct Universe {
    pub system_count: usize,
    pub systems_by_id: HashMap<usize, Rc<Box<System>>>,
    pub systems_by_name: HashMap<String, Rc<Box<System>>>
}

#[derive(Debug)]
pub enum UniverseError {
    SystemNameNotFound(String),
    SystemIdNotFound(usize)
}

impl fmt::Display for UniverseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UniverseError::SystemNameNotFound(ref name) => {
                write!(f, "System with name \"{}\" could not be found", name)
            },
            UniverseError::SystemIdNotFound(ref id) => {
                write!(f, "System with id \"{}\" could not be found", id)
            }
        }
    }
}

impl Error for UniverseError {
    fn description(&self) -> &str {
        match *self {
            UniverseError::SystemNameNotFound(_) =>
                "System with the given name wasn't found in the universe object",
            UniverseError::SystemIdNotFound(_) =>
                "System with the given id wasn't found in the universe object"
        }
    }
}

impl Universe {
    /// Uses the given client to fetch information for the game universe
    /// Returns `Ok(Universe)` if everything went well and boxed error
    /// otherwise.
    ///
    /// Arguments:
    ///
    /// client: hyper::Client - HTTPS enabled hyper client
    pub fn fetch_universe(client: Client) -> Result<Universe, Box<Error>> {
        let mut universe = Self::new();
        let system_ids = esi::systems::fetch_all_ids(&client)?;
        system_ids
            .into_iter()
            .map(|system_id| esi::systems::fetch_system_info(&client, system_id))
            .for_each(|system_result|
                 match system_result {
                     Ok(system) => universe.fill_system_info(system),
                     Err(error) => {
                         eprintln!("Encountered the following error:\n{}", error);
                         // WUT DO?!?
                     }
                 }
             );
        Ok(universe)
    }

    /// Creates new Universe, with empty hashes
    pub fn new() -> Self {
        Self {
            system_count: 0,
            systems_by_id: HashMap::new(),
            systems_by_name: HashMap::new()
        }
    }

    /// Inserts the given system in the structures containing information for
    /// systems in the Universe.
    ///
    /// Arguments:
    ///
    /// system: models::System - The system you want to push in
    fn fill_system_info(&mut self, system: System) {
        let system_name  = system.name.clone();
        let system_id    = system.system_id;
        let boxed_system = Box::new(system);
        let system_rc    = Rc::new(boxed_system);
        self.systems_by_name.insert(system_name, system_rc.clone());
        self.systems_by_id.insert(system_id, system_rc.clone());
        self.system_count += 1;
    }

    /// Finds a system in the current `Universe` by name. If everything is
    /// successful returns a `Ok` containing reference to the system. Otherwise
    /// returns `UniverseError::SystemNameNotFound`
    ///
    /// Arguments:
    ///
    /// name: &str - The name of the system we are searching for.
    pub fn get_system_by_name(&self, name: &str) -> Result<&System, UniverseError> {
        match self.systems_by_name.get(name) {
            Some(system) => Ok(system),
            None => Err(UniverseError::SystemNameNotFound(String::from(name)))
        }
    }

    /// Finds a system in the current `Universe` by id. If everything is
    /// successful returns a `Ok` containing reference to the system. Otherwise
    /// returns `UniverseError::SystemIdNotFound`
    ///
    /// Arguments:
    ///
    /// id: usize - The id of the system we are searching for.
    pub fn get_system_by_id(&self, id: usize) -> Result<&System, UniverseError> {
        match self.systems_by_id.get(&id) {
            Some(system) => Ok(system),
            None => Err(UniverseError::SystemIdNotFound(id))
        }
    }
}

#[cfg(test)]
mod test {
    use ::models::*;
    use spectral::prelude::*;

    const AMY_DATA: &'static [u8] =  br#"{"star_id":40335883,"system_id":30005311,"name":"Amygnon","position":{"x":-2.437652065551942e+17,"y":3.610789959702216e+16,"z":5.16497631623793e+16},"security_status":0.6375686526298523,"constellation_id":20000777,"planets":[{"planet_id":40335884},{"planet_id":40335886,"moons":[40335887]},{"planet_id":40335888,"moons":[40335889]},{"planet_id":40335890,"moons":[40335891]},{"planet_id":40335892,"moons":[40335893]},{"planet_id":40335894,"moons":[40335895,40335896,40335897,40335898,40335899,40335901,40335902,40335903,40335904,40335905,40335906,40335907,40335908,40335909,40335910,40335911]},{"planet_id":40335912,"moons":[40335913,40335914,40335915,40335916,40335917,40335918,40335919,40335920,40335921,40335922,40335923,40335924,40335925,40335926,40335927,40335928,40335929,40335930,40335931,40335932,40335933,40335934]},{"planet_id":40335936,"moons":[40335939,40335940,40335941,40335942,40335943,40335944,40335945,40335946,40335947,40335948,40335950,40335952,40335953,40335954,40335955,40335956,40335957,40335959,40335960,40335961,40335962,40335964]}],"security_class":"D1","stargates":[50007433],"stations":[60011203,60011209,60011530,60011533,60014479]}"#;
    const AMY_ID: usize = 30005311;

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
        let result = System::parse(AMY_DATA);

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

    #[test]
    fn universe_new_test() {
        let universe = Universe::new();
        assert!(universe.systems_by_name.is_empty());
        assert!(universe.systems_by_id.is_empty());
    }

    #[test]
    fn fill_system_info_test() {
        let mut universe = Universe::new();
        let system = System::parse(AMY_DATA).unwrap();

        universe.fill_system_info(system);
        assert!(universe.systems_by_name.contains_key("Amygnon"));
        assert!(universe.systems_by_id.contains_key(&30005311));
    }

    #[test]
    fn get_system_by_name_test() {
        let mut universe = Universe::new();
        let system = System::parse(AMY_DATA).unwrap();
        universe.fill_system_info(system);

        let result = universe.get_system_by_name("Amygnon");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Amygnon");

        assert!(universe.get_system_by_name("Invalid").is_err());
    }

    #[test]
    fn get_system_by_id_test() {
        let mut universe = Universe::new();
        let system = System::parse(AMY_DATA).unwrap();
        universe.fill_system_info(system);

        let result = universe.get_system_by_id(AMY_ID);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Amygnon")
    }

    // This one might run really slowly ...
    // PS: Actually, it runs extremely slowly. Really gotta fetch it only once
    // and save it locally. Maybe even distribute it with the binary or something
    // PPS: I belive I saw way too many tls handshakes on wireshark while
    // running. Maybe for some reason I don't keep the connection alive, even
    // if the default behaviour for the client is to send keep-alive. Gotta
    // play a little bit more with wireshark.
    #[ignore] #[test]
    fn fetch_universe_test() {
        let client = esi::build_client();
        let universe_result = Universe::fetch_universe(client);

        let universe = universe_result.unwrap();
        assert!(universe.get_system_by_id(AMY_ID).is_ok());
        assert!(universe.get_system_by_name("PC9-AY").is_ok());
    }
}
