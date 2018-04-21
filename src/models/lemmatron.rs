/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Lemmatron : Schema for the inflections endpoint.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lemmatron {
  /// Additional Information provided by OUP
  #[serde(rename = "metadata")]
  metadata: Option<Value>,
  /// A list of inflections matching a given word
  #[serde(rename = "results")]
  results: Option<Vec<::models::HeadwordLemmatron>>
}

impl Lemmatron {
  /// Schema for the inflections endpoint.
  pub fn new() -> Lemmatron {
    Lemmatron {
      metadata: None,
      results: None
    }
  }

  pub fn set_metadata(&mut self, metadata: Value) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: Value) -> Lemmatron {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&Value> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_results(&mut self, results: Vec<::models::HeadwordLemmatron>) {
    self.results = Some(results);
  }

  pub fn with_results(mut self, results: Vec<::models::HeadwordLemmatron>) -> Lemmatron {
    self.results = Some(results);
    self
  }

  pub fn results(&self) -> Option<&Vec<::models::HeadwordLemmatron>> {
    self.results.as_ref()
  }

  pub fn reset_results(&mut self) {
    self.results = None;
  }

}


