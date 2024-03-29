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
pub struct BlockId {
  #[serde(rename = "hash")]
  hash: Option<::models::Hash>,
  #[serde(rename = "parts")]
  parts: Option<::models::BlockIdParts>
}

impl BlockId {
  pub fn new() -> BlockId {
    BlockId {
      hash: None,
      parts: None
    }
  }

  pub fn set_hash(&mut self, hash: ::models::Hash) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: ::models::Hash) -> BlockId {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&::models::Hash> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_parts(&mut self, parts: ::models::BlockIdParts) {
    self.parts = Some(parts);
  }

  pub fn with_parts(mut self, parts: ::models::BlockIdParts) -> BlockId {
    self.parts = Some(parts);
    self
  }

  pub fn parts(&self) -> Option<&::models::BlockIdParts> {
    self.parts.as_ref()
  }

  pub fn reset_parts(&mut self) {
    self.parts = None;
  }

}



