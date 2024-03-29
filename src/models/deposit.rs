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
pub struct Deposit {
  #[serde(rename = "amount")]
  amount: Option<Vec<::models::Coin>>,
  #[serde(rename = "proposal_id")]
  proposal_id: Option<i32>,
  #[serde(rename = "depositor")]
  depositor: Option<::models::Address>
}

impl Deposit {
  pub fn new() -> Deposit {
    Deposit {
      amount: None,
      proposal_id: None,
      depositor: None
    }
  }

  pub fn set_amount(&mut self, amount: Vec<::models::Coin>) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: Vec<::models::Coin>) -> Deposit {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&Vec<::models::Coin>> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_proposal_id(&mut self, proposal_id: i32) {
    self.proposal_id = Some(proposal_id);
  }

  pub fn with_proposal_id(mut self, proposal_id: i32) -> Deposit {
    self.proposal_id = Some(proposal_id);
    self
  }

  pub fn proposal_id(&self) -> Option<&i32> {
    self.proposal_id.as_ref()
  }

  pub fn reset_proposal_id(&mut self) {
    self.proposal_id = None;
  }

  pub fn set_depositor(&mut self, depositor: ::models::Address) {
    self.depositor = Some(depositor);
  }

  pub fn with_depositor(mut self, depositor: ::models::Address) -> Deposit {
    self.depositor = Some(depositor);
    self
  }

  pub fn depositor(&self) -> Option<&::models::Address> {
    self.depositor.as_ref()
  }

  pub fn reset_depositor(&mut self) {
    self.depositor = None;
  }

}



