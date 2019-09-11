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
pub struct UnbondingDelegation {
  #[serde(rename = "delegator_address")]
  delegator_address: Option<String>,
  #[serde(rename = "validator_address")]
  validator_address: Option<String>,
  #[serde(rename = "initial_balance")]
  initial_balance: Option<String>,
  #[serde(rename = "balance")]
  balance: Option<String>,
  #[serde(rename = "creation_height")]
  creation_height: Option<i32>,
  #[serde(rename = "min_time")]
  min_time: Option<i32>
}

impl UnbondingDelegation {
  pub fn new() -> UnbondingDelegation {
    UnbondingDelegation {
      delegator_address: None,
      validator_address: None,
      initial_balance: None,
      balance: None,
      creation_height: None,
      min_time: None
    }
  }

  pub fn set_delegator_address(&mut self, delegator_address: String) {
    self.delegator_address = Some(delegator_address);
  }

  pub fn with_delegator_address(mut self, delegator_address: String) -> UnbondingDelegation {
    self.delegator_address = Some(delegator_address);
    self
  }

  pub fn delegator_address(&self) -> Option<&String> {
    self.delegator_address.as_ref()
  }

  pub fn reset_delegator_address(&mut self) {
    self.delegator_address = None;
  }

  pub fn set_validator_address(&mut self, validator_address: String) {
    self.validator_address = Some(validator_address);
  }

  pub fn with_validator_address(mut self, validator_address: String) -> UnbondingDelegation {
    self.validator_address = Some(validator_address);
    self
  }

  pub fn validator_address(&self) -> Option<&String> {
    self.validator_address.as_ref()
  }

  pub fn reset_validator_address(&mut self) {
    self.validator_address = None;
  }

  pub fn set_initial_balance(&mut self, initial_balance: String) {
    self.initial_balance = Some(initial_balance);
  }

  pub fn with_initial_balance(mut self, initial_balance: String) -> UnbondingDelegation {
    self.initial_balance = Some(initial_balance);
    self
  }

  pub fn initial_balance(&self) -> Option<&String> {
    self.initial_balance.as_ref()
  }

  pub fn reset_initial_balance(&mut self) {
    self.initial_balance = None;
  }

  pub fn set_balance(&mut self, balance: String) {
    self.balance = Some(balance);
  }

  pub fn with_balance(mut self, balance: String) -> UnbondingDelegation {
    self.balance = Some(balance);
    self
  }

  pub fn balance(&self) -> Option<&String> {
    self.balance.as_ref()
  }

  pub fn reset_balance(&mut self) {
    self.balance = None;
  }

  pub fn set_creation_height(&mut self, creation_height: i32) {
    self.creation_height = Some(creation_height);
  }

  pub fn with_creation_height(mut self, creation_height: i32) -> UnbondingDelegation {
    self.creation_height = Some(creation_height);
    self
  }

  pub fn creation_height(&self) -> Option<&i32> {
    self.creation_height.as_ref()
  }

  pub fn reset_creation_height(&mut self) {
    self.creation_height = None;
  }

  pub fn set_min_time(&mut self, min_time: i32) {
    self.min_time = Some(min_time);
  }

  pub fn with_min_time(mut self, min_time: i32) -> UnbondingDelegation {
    self.min_time = Some(min_time);
    self
  }

  pub fn min_time(&self) -> Option<&i32> {
    self.min_time.as_ref()
  }

  pub fn reset_min_time(&mut self) {
    self.min_time = None;
  }

}


