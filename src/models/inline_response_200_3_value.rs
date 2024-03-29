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
pub struct InlineResponse2003Value {
  #[serde(rename = "account_number")]
  account_number: Option<String>,
  #[serde(rename = "address")]
  address: Option<String>,
  #[serde(rename = "coins")]
  coins: Option<Vec<::models::Coin>>,
  #[serde(rename = "public_key")]
  public_key: Option<String>,
  #[serde(rename = "sequence")]
  sequence: Option<String>
}

impl InlineResponse2003Value {
  pub fn new() -> InlineResponse2003Value {
    InlineResponse2003Value {
      account_number: None,
      address: None,
      coins: None,
      public_key: None,
      sequence: None
    }
  }

  pub fn set_account_number(&mut self, account_number: String) {
    self.account_number = Some(account_number);
  }

  pub fn with_account_number(mut self, account_number: String) -> InlineResponse2003Value {
    self.account_number = Some(account_number);
    self
  }

  pub fn account_number(&self) -> Option<&String> {
    self.account_number.as_ref()
  }

  pub fn reset_account_number(&mut self) {
    self.account_number = None;
  }

  pub fn set_address(&mut self, address: String) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: String) -> InlineResponse2003Value {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&String> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

  pub fn set_coins(&mut self, coins: Vec<::models::Coin>) {
    self.coins = Some(coins);
  }

  pub fn with_coins(mut self, coins: Vec<::models::Coin>) -> InlineResponse2003Value {
    self.coins = Some(coins);
    self
  }

  pub fn coins(&self) -> Option<&Vec<::models::Coin>> {
    self.coins.as_ref()
  }

  pub fn reset_coins(&mut self) {
    self.coins = None;
  }

  pub fn set_public_key(&mut self, public_key: String) {
    self.public_key = Some(public_key);
  }

  pub fn with_public_key(mut self, public_key: String) -> InlineResponse2003Value {
    self.public_key = Some(public_key);
    self
  }

  pub fn public_key(&self) -> Option<&String> {
    self.public_key.as_ref()
  }

  pub fn reset_public_key(&mut self) {
    self.public_key = None;
  }

  pub fn set_sequence(&mut self, sequence: String) {
    self.sequence = Some(sequence);
  }

  pub fn with_sequence(mut self, sequence: String) -> InlineResponse2003Value {
    self.sequence = Some(sequence);
    self
  }

  pub fn sequence(&self) -> Option<&String> {
    self.sequence.as_ref()
  }

  pub fn reset_sequence(&mut self) {
    self.sequence = None;
  }

}



