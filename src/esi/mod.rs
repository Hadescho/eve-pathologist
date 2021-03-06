use hyper::status::StatusCode;
use std::error::Error;
use std::fmt;
use std::io::Read;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::client::response::Response;
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
    use hyper::Client;
    use std::error::Error;
    use std::io::Read;
    use hyper::status::StatusCode;
    use ::models::System;

    /// Fetches the ids of all of the systems in the EVE universe.
    /// Uses `client` as https client for the request. Returns `Result`
    /// containing a `Vec` of system ids. If a problem was encountered returns
    /// a boxed `Error`
    ///
    /// Arguments:
    ///
    /// client: &hyper::Client - reference to HTTPS enabled hyper client
    pub fn fetch_all_ids(client: &Client) -> Result<Vec<usize>, Box<Error>>  {
        let uri = "https://esi.tech.ccp.is/latest/universe/systems/";
        let mut body: Vec<u8> = vec![];
        let mut response = esi::get_request(client, uri)?;
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
    /// Returns `Result` containing a `models::System`. If a problem was
    /// encountered returns a boxed `Error`
    ///
    /// Arguments:
    ///
    /// client: &hyper::Client - reference to HTTPS enabled hyper client
    /// id: usize - Solar system id
    pub fn fetch_system_info(client: &Client, id: usize) -> Result<System, Box<Error>> {
        let uri = format!("https://esi.tech.ccp.is/latest/universe/systems/{}/", id);
        let mut response = esi::get_request(client, &uri)?;
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

}
/// Builds a hyper client. Exists if unable to get OpenSSL or the equivalent
/// for the underlying operation system.
pub fn build_client() -> Client {
    let ssl = grab_ssl();
    let connector = HttpsConnector::new(ssl);
    Client::with_connector(connector)
}

/// Tries to use native tls client, like OpenSSL and returns object used
/// by hyper_native_tls. Exists if unable to get OpenSSL or the equivalent
/// for the running operation system.
// [TODO] Rename to TLS, since we actually don't use SSL.
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
fn get_request(client: &Client, uri: &str) -> Result<Response, Box<Error>> {
    // Why does this work this way?!? Actually, I know.  Why should it work this way ?!?
    // HACKS <3
    Ok(client.get(uri).send()?)
}

// It's really bad practice to make actual call to the remote API in tests
// but I will skip using a mocking hyper client for now. Also there isn't
// limit on the correct requests sent to the ESI, so I won't end up blocked.
#[cfg(test)]
mod test {
    use esi;
    use ::esi::systems;
    #[test]
    fn fetch_all_ids_test(){
        let client = esi::build_client();
        assert_eq!(systems::fetch_all_ids(&client).unwrap()[0], 30000001);
    }

    #[test]
    fn fetch_system_info(){
        let client = esi::build_client();
        let id = 30005311;
        let sys = systems::fetch_system_info(&client, id).unwrap();
        assert_eq!(sys.name, "Amygnon");
        assert_eq!(sys.system_id, id)
    }

    #[test]
    fn fetch_system_info_for_system_with_invalid_id() {
        let client = esi::build_client();
        let id = 0;
        let sys = systems::fetch_system_info(&client ,id);
        assert!(sys.is_err());
    }
}
