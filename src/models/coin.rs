/* 
 * Gaia-Lite for Cosmos
 *
 * A REST interface for state queries, transaction generation and broadcasting.
 *
 * OpenAPI spec version: 3.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coin {
  #[serde(rename = "denom")]
  denom: Option<String>,
  #[serde(rename = "amount")]
  amount: Option<String>
}

impl Coin {
  pub fn new() -> Coin {
    Coin {
      denom: None,
      amount: None
    }
  }

  pub fn set_denom(&mut self, denom: String) {
    self.denom = Some(denom);
  }

  pub fn with_denom(mut self, denom: String) -> Coin {
    self.denom = Some(denom);
    self
  }

  pub fn denom(&self) -> Option<&String> {
    self.denom.as_ref()
  }

  pub fn reset_denom(&mut self) {
    self.denom = None;
  }

  pub fn set_amount(&mut self, amount: String) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: String) -> Coin {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&String> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

}


