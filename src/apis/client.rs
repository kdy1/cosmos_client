use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  default_api: Box<::apis::DefaultApi>,
  ics0_api: Box<::apis::ICS0Api>,
  ics1_api: Box<::apis::ICS1Api>,
  ics20_api: Box<::apis::ICS20Api>,
  ics21_api: Box<::apis::ICS21Api>,
  ics22_api: Box<::apis::ICS22Api>,
  ics23_api: Box<::apis::ICS23Api>,
  ics24_api: Box<::apis::ICS24Api>,
  version_api: Box<::apis::VersionApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      default_api: Box::new(::apis::DefaultApiClient::new(rc.clone())),
      ics0_api: Box::new(::apis::ICS0ApiClient::new(rc.clone())),
      ics1_api: Box::new(::apis::ICS1ApiClient::new(rc.clone())),
      ics20_api: Box::new(::apis::ICS20ApiClient::new(rc.clone())),
      ics21_api: Box::new(::apis::ICS21ApiClient::new(rc.clone())),
      ics22_api: Box::new(::apis::ICS22ApiClient::new(rc.clone())),
      ics23_api: Box::new(::apis::ICS23ApiClient::new(rc.clone())),
      ics24_api: Box::new(::apis::ICS24ApiClient::new(rc.clone())),
      version_api: Box::new(::apis::VersionApiClient::new(rc.clone())),
    }
  }

  pub fn default_api(&self) -> &::apis::DefaultApi{
    self.default_api.as_ref()
  }

  pub fn ics0_api(&self) -> &::apis::ICS0Api{
    self.ics0_api.as_ref()
  }

  pub fn ics1_api(&self) -> &::apis::ICS1Api{
    self.ics1_api.as_ref()
  }

  pub fn ics20_api(&self) -> &::apis::ICS20Api{
    self.ics20_api.as_ref()
  }

  pub fn ics21_api(&self) -> &::apis::ICS21Api{
    self.ics21_api.as_ref()
  }

  pub fn ics22_api(&self) -> &::apis::ICS22Api{
    self.ics22_api.as_ref()
  }

  pub fn ics23_api(&self) -> &::apis::ICS23Api{
    self.ics23_api.as_ref()
  }

  pub fn ics24_api(&self) -> &::apis::ICS24Api{
    self.ics24_api.as_ref()
  }

  pub fn version_api(&self) -> &::apis::VersionApi{
    self.version_api.as_ref()
  }


}
