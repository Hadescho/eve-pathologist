use hyper::status::StatusCode;
use std::error::Error;
use std::fmt;
/// Enum for custom errors related to hyper
#[derive(Debug)]
enum HttpError {
    BadStatusCode(StatusCode, String)
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpError::BadStatusCode(_, _) => write!(f, "BasStatusCode"),
        }
    }
}

impl Error for HttpError {
    fn description(&self) -> &str {
        match *self {
            HttpError::BadStatusCode(_, _) =>
                "We recieved non successful response code for the given reqeust"
        }
    }
}

pub mod systems {
    use esi;
    use std::io::Read;
    use hyper::Client;
    use std::error::Error;
    use hyper::status::StatusCode;
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;
    use hyper::client::response::Response;
    use ::models::System;

    /// Fetches the ids of all of the systems in the EVE universe.
    /// Uses `client` as https client for the request
    pub fn fetch_all_ids(client: Client) -> Result<Vec<usize>, Box<Error>>  {
        let uri = "https://esi.tech.ccp.is/latest/universe/systems/";
        let mut body: Vec<u8> = vec![];
        let mut response = get_request(client, uri)?;
        response.read_to_end(&mut body)?;
        if response.status.is_success() {
            System::parse_ids(body.as_ref())
        } else {
            eprintln!("Unable to download system ids from ESI");
            let err = esi::HttpError::BadStatusCode(response.status,
                                                    String::from_utf8(body).unwrap());
            Err(Box::new(err))
        }
    }

    /// Fetches information for the system with the given `id` from ESI.
    pub fn fetch_system_info(client: Client, id: usize) -> Result<System, Box<Error>> {
        let uri = format!("https://esi.tech.ccp.is/latest/universe/systems/{}/", id);
        let mut response = get_request(client, &uri)?;
        let mut body: Vec<u8> = vec![];

        response.read_to_end(&mut body)?;
        if response.status == StatusCode::Ok {
            System::parse(&body)
        } else {
            eprintln!("Unable to fetch information for system with id: {}", id);
            let err = esi::HttpError::BadStatusCode(response.status,
                                                    String::from_utf8(body).unwrap());
            Err(Box::new(err))
        }
    }

    /// Build a client to be used for a request
    pub fn build_client() -> Client {
        let ssl = grab_ssl();
        let connector = HttpsConnector::new(ssl);
        Client::with_connector(connector)
    }

    /// Tries to use native tls client, like OpenSSL and gives a wrapper
    /// around it.
    fn grab_ssl() -> NativeTlsClient {
        match NativeTlsClient::new() {
            Ok(ssl) => ssl,
            Err(_) => {
                eprintln!("Unable to grab Native TLS client.");
                eprintln!("If you are using Linux, please install OpenSSL");
                ::std::process::exit(1);
            }
        }
    }

    /// Uses `client` to create and send `GET` request to `uri`.
    /// If everything went correctly returns `Ok(response)`, otherwise
    /// prints information about what went wrong and returns `None`
    fn get_request(client: Client, uri: &str) -> Result<Response, Box<Error>> {
        // Why does this work this way?!? Actually, I know.  Why should it work this way ?!?
        // HACKS <3
        Ok(client.get(uri).send()?)
    }
}

#[cfg(test)]
mod test {
    use ::esi::systems;
    // It's really bad practice to make actual call to the remote API in tests
    // but I will skip using a mocking hyper client for now. Also there isn't
    // limit on the correct requests sent to the ESI, so I won't end up blocked.
    #[test]
    fn fetch_all_ids_test(){
        let client = systems::build_client();
        assert_eq!(systems::fetch_all_ids(client).unwrap()[0], 30000001);
    }

    #[test]
    fn fetch_system_info(){
        let client = systems::build_client();
        let id = 30005311;
        let sys = systems::fetch_system_info(client, id).unwrap();
        assert_eq!(sys.name, "Amygnon");
        assert_eq!(sys.system_id, id)
    }
}
