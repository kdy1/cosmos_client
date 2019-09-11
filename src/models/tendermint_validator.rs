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
pub struct TendermintValidator {
  #[serde(rename = "address")]
  address: Option<::models::ValidatorAddress>,
  #[serde(rename = "pub_key")]
  pub_key: Option<String>,
  #[serde(rename = "voting_power")]
  voting_power: Option<String>,
  #[serde(rename = "proposer_priority")]
  proposer_priority: Option<String>
}

impl TendermintValidator {
  pub fn new() -> TendermintValidator {
    TendermintValidator {
      address: None,
      pub_key: None,
      voting_power: None,
      proposer_priority: None
    }
  }

  pub fn set_address(&mut self, address: ::models::ValidatorAddress) {
    self.address = Some(address);
  }

  pub fn with_address(mut self, address: ::models::ValidatorAddress) -> TendermintValidator {
    self.address = Some(address);
    self
  }

  pub fn address(&self) -> Option<&::models::ValidatorAddress> {
    self.address.as_ref()
  }

  pub fn reset_address(&mut self) {
    self.address = None;
  }

  pub fn set_pub_key(&mut self, pub_key: String) {
    self.pub_key = Some(pub_key);
  }

  pub fn with_pub_key(mut self, pub_key: String) -> TendermintValidator {
    self.pub_key = Some(pub_key);
    self
  }

  pub fn pub_key(&self) -> Option<&String> {
    self.pub_key.as_ref()
  }

  pub fn reset_pub_key(&mut self) {
    self.pub_key = None;
  }

  pub fn set_voting_power(&mut self, voting_power: String) {
    self.voting_power = Some(voting_power);
  }

  pub fn with_voting_power(mut self, voting_power: String) -> TendermintValidator {
    self.voting_power = Some(voting_power);
    self
  }

  pub fn voting_power(&self) -> Option<&String> {
    self.voting_power.as_ref()
  }

  pub fn reset_voting_power(&mut self) {
    self.voting_power = None;
  }

  pub fn set_proposer_priority(&mut self, proposer_priority: String) {
    self.proposer_priority = Some(proposer_priority);
  }

  pub fn with_proposer_priority(mut self, proposer_priority: String) -> TendermintValidator {
    self.proposer_priority = Some(proposer_priority);
    self
  }

  pub fn proposer_priority(&self) -> Option<&String> {
    self.proposer_priority.as_ref()
  }

  pub fn reset_proposer_priority(&mut self) {
    self.proposer_priority = None;
  }

}



