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
pub struct TallyResult {
  #[serde(rename = "yes")]
  yes: Option<String>,
  #[serde(rename = "abstain")]
  abstain: Option<String>,
  #[serde(rename = "no")]
  no: Option<String>,
  #[serde(rename = "no_with_veto")]
  no_with_veto: Option<String>
}

impl TallyResult {
  pub fn new() -> TallyResult {
    TallyResult {
      yes: None,
      abstain: None,
      no: None,
      no_with_veto: None
    }
  }

  pub fn set_yes(&mut self, yes: String) {
    self.yes = Some(yes);
  }

  pub fn with_yes(mut self, yes: String) -> TallyResult {
    self.yes = Some(yes);
    self
  }

  pub fn yes(&self) -> Option<&String> {
    self.yes.as_ref()
  }

  pub fn reset_yes(&mut self) {
    self.yes = None;
  }

  pub fn set_abstain(&mut self, abstain: String) {
    self.abstain = Some(abstain);
  }

  pub fn with_abstain(mut self, abstain: String) -> TallyResult {
    self.abstain = Some(abstain);
    self
  }

  pub fn abstain(&self) -> Option<&String> {
    self.abstain.as_ref()
  }

  pub fn reset_abstain(&mut self) {
    self.abstain = None;
  }

  pub fn set_no(&mut self, no: String) {
    self.no = Some(no);
  }

  pub fn with_no(mut self, no: String) -> TallyResult {
    self.no = Some(no);
    self
  }

  pub fn no(&self) -> Option<&String> {
    self.no.as_ref()
  }

  pub fn reset_no(&mut self) {
    self.no = None;
  }

  pub fn set_no_with_veto(&mut self, no_with_veto: String) {
    self.no_with_veto = Some(no_with_veto);
  }

  pub fn with_no_with_veto(mut self, no_with_veto: String) -> TallyResult {
    self.no_with_veto = Some(no_with_veto);
    self
  }

  pub fn no_with_veto(&self) -> Option<&String> {
    self.no_with_veto.as_ref()
  }

  pub fn reset_no_with_veto(&mut self) {
    self.no_with_veto = None;
  }

}



