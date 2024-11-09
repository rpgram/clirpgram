use crate::api_adapter::setup::SetupErrors;
use reqwest::blocking::Response;

pub fn parse_response<G: for<'a> serde::Deserialize<'a>>(
    response: reqwest::Result<Response>,
) -> Option<G> {
    match response {
        Ok(bf_response) => {
            if bf_response.status().as_u16() == 200 {
                bf_response.json().unwrap()
            } else {
                None
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

pub fn parse_setup_response<G: for<'a> serde::Deserialize<'a>>(
    response: reqwest::Result<Response>,
) -> Result<G, SetupErrors> {
    match response {
        Ok(response) => {
            let status_code = response.status().as_u16();
            if 200 <= status_code && status_code < 300 {
                Ok(response.json().unwrap())
            } else {
                Err(SetupErrors::UnknownError(
                    status_code,
                    response.text().unwrap(),
                ))
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(SetupErrors::ServiceError)
        }
    }
}
