use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod default_api;
pub use self::default_api::{ DefaultApi, DefaultApiClient };
mod ics0_api;
pub use self::ics0_api::{ ICS0Api, ICS0ApiClient };
mod ics1_api;
pub use self::ics1_api::{ ICS1Api, ICS1ApiClient };
mod ics20_api;
pub use self::ics20_api::{ ICS20Api, ICS20ApiClient };
mod ics21_api;
pub use self::ics21_api::{ ICS21Api, ICS21ApiClient };
mod ics22_api;
pub use self::ics22_api::{ ICS22Api, ICS22ApiClient };
mod ics23_api;
pub use self::ics23_api::{ ICS23Api, ICS23ApiClient };
mod ics24_api;
pub use self::ics24_api::{ ICS24Api, ICS24ApiClient };
mod version_api;
pub use self::version_api::{ VersionApi, VersionApiClient };

pub mod configuration;
pub mod client;
