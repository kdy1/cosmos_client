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
pub struct BlockLastCommit {
  #[serde(rename = "block_id")]
  block_id: Option<::models::BlockId>,
  #[serde(rename = "precommits")]
  precommits: Option<Vec<::models::BlockLastCommitPrecommits>>
}

impl BlockLastCommit {
  pub fn new() -> BlockLastCommit {
    BlockLastCommit {
      block_id: None,
      precommits: None
    }
  }

  pub fn set_block_id(&mut self, block_id: ::models::BlockId) {
    self.block_id = Some(block_id);
  }

  pub fn with_block_id(mut self, block_id: ::models::BlockId) -> BlockLastCommit {
    self.block_id = Some(block_id);
    self
  }

  pub fn block_id(&self) -> Option<&::models::BlockId> {
    self.block_id.as_ref()
  }

  pub fn reset_block_id(&mut self) {
    self.block_id = None;
  }

  pub fn set_precommits(&mut self, precommits: Vec<::models::BlockLastCommitPrecommits>) {
    self.precommits = Some(precommits);
  }

  pub fn with_precommits(mut self, precommits: Vec<::models::BlockLastCommitPrecommits>) -> BlockLastCommit {
    self.precommits = Some(precommits);
    self
  }

  pub fn precommits(&self) -> Option<&Vec<::models::BlockLastCommitPrecommits>> {
    self.precommits.as_ref()
  }

  pub fn reset_precommits(&mut self) {
    self.precommits = None;
  }

}



