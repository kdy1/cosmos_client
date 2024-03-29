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
pub struct InlineResponse200ProtocolVersion {
  #[serde(rename = "p2p")]
  p2p: Option<String>,
  #[serde(rename = "block")]
  block: Option<String>,
  #[serde(rename = "app")]
  app: Option<String>
}

impl InlineResponse200ProtocolVersion {
  pub fn new() -> InlineResponse200ProtocolVersion {
    InlineResponse200ProtocolVersion {
      p2p: None,
      block: None,
      app: None
    }
  }

  pub fn set_p2p(&mut self, p2p: String) {
    self.p2p = Some(p2p);
  }

  pub fn with_p2p(mut self, p2p: String) -> InlineResponse200ProtocolVersion {
    self.p2p = Some(p2p);
    self
  }

  pub fn p2p(&self) -> Option<&String> {
    self.p2p.as_ref()
  }

  pub fn reset_p2p(&mut self) {
    self.p2p = None;
  }

  pub fn set_block(&mut self, block: String) {
    self.block = Some(block);
  }

  pub fn with_block(mut self, block: String) -> InlineResponse200ProtocolVersion {
    self.block = Some(block);
    self
  }

  pub fn block(&self) -> Option<&String> {
    self.block.as_ref()
  }

  pub fn reset_block(&mut self) {
    self.block = None;
  }

  pub fn set_app(&mut self, app: String) {
    self.app = Some(app);
  }

  pub fn with_app(mut self, app: String) -> InlineResponse200ProtocolVersion {
    self.app = Some(app);
    self
  }

  pub fn app(&self) -> Option<&String> {
    self.app.as_ref()
  }

  pub fn reset_app(&mut self) {
    self.app = None;
  }

}



