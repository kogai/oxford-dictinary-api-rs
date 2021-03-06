/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatsWordResultList : Schema for lexi-stats results for a word/trueCase/lemma/lexicalCategory returned as a list of frequencies per wordform-trueCase-lemma-lexicalCategory entry.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsWordResultList {
  /// Additional Information provided by OUP
  #[serde(rename = "metadata")]
  metadata: Option<Value>,
  /// A list of found words along with their frequencies
  #[serde(rename = "results")]
  results: Option<Vec<::models::StatsWordResultListResults>>
}

impl StatsWordResultList {
  /// Schema for lexi-stats results for a word/trueCase/lemma/lexicalCategory returned as a list of frequencies per wordform-trueCase-lemma-lexicalCategory entry.
  pub fn new() -> StatsWordResultList {
    StatsWordResultList {
      metadata: None,
      results: None
    }
  }

  pub fn set_metadata(&mut self, metadata: Value) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: Value) -> StatsWordResultList {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&Value> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_results(&mut self, results: Vec<::models::StatsWordResultListResults>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<::models::StatsWordResultListResults>) -> StatsWordResultList {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<::models::StatsWordResultListResults>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

}



