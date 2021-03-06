/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatsWordResultResult : Frequency information for a given entity

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsWordResultResult {
  /// The number of times a word appears in the entire corpus
  #[serde(rename = "frequency")]
  frequency: i32,
  /// A lemma of the word (e.g., wordforms \"lay\", \"laid\" and \"laying\" have all lemma \"lay\")
  #[serde(rename = "lemma")]
  lemma: Option<String>,
  /// A lexical category such as 'verb' or 'noun'
  #[serde(rename = "lexicalCategory")]
  lexical_category: Option<String>,
  /// The number of database records that matched the query params (stated frequency is the sum of the individual frequencies)
  #[serde(rename = "matchCount")]
  match_count: i32,
  /// The number of times a word appears on average in 1 million words
  #[serde(rename = "normalizedFrequency")]
  normalized_frequency: i32,
  /// A given written realisation of a an entry (e.g., \"lay\") usually lower case
  #[serde(rename = "trueCase")]
  true_case: Option<String>,
  /// A given written realisation of a an entry (e.g., \"Lay\") preserving case
  #[serde(rename = "wordform")]
  wordform: Option<String>
}

impl StatsWordResultResult {
  /// Frequency information for a given entity
  pub fn new(frequency: i32, match_count: i32, normalized_frequency: i32) -> StatsWordResultResult {
    StatsWordResultResult {
      frequency: frequency,
      lemma: None,
      lexical_category: None,
      match_count: match_count,
      normalized_frequency: normalized_frequency,
      true_case: None,
      wordform: None
    }
  }

  pub fn set_frequency(&mut self, frequency: i32) {
    self.frequency = frequency;
  }

  pub fn with_frequency(mut self, frequency: i32) -> StatsWordResultResult {
    self.frequency = frequency;
    self
  }

  pub fn frequency(&self) -> &i32 {
    &self.frequency
  }


  pub fn set_lemma(&mut self, lemma: String) {
    self.lemma = Some(lemma);
  }

  pub fn with_lemma(mut self, lemma: String) -> StatsWordResultResult {
    self.lemma = Some(lemma);
    self
  }

  pub fn lemma(&self) -> Option<&String> {
    self.lemma.as_ref()
  }

  pub fn reset_lemma(&mut self) {
    self.lemma = None;
  }

  pub fn set_lexical_category(&mut self, lexical_category: String) {
    self.lexical_category = Some(lexical_category);
  }

  pub fn with_lexical_category(mut self, lexical_category: String) -> StatsWordResultResult {
    self.lexical_category = Some(lexical_category);
    self
  }

  pub fn lexical_category(&self) -> Option<&String> {
    self.lexical_category.as_ref()
  }

  pub fn reset_lexical_category(&mut self) {
    self.lexical_category = None;
  }

  pub fn set_match_count(&mut self, match_count: i32) {
    self.match_count = match_count;
  }

  pub fn with_match_count(mut self, match_count: i32) -> StatsWordResultResult {
    self.match_count = match_count;
    self
  }

  pub fn match_count(&self) -> &i32 {
    &self.match_count
  }


  pub fn set_normalized_frequency(&mut self, normalized_frequency: i32) {
    self.normalized_frequency = normalized_frequency;
  }

  pub fn with_normalized_frequency(mut self, normalized_frequency: i32) -> StatsWordResultResult {
    self.normalized_frequency = normalized_frequency;
    self
  }

  pub fn normalized_frequency(&self) -> &i32 {
    &self.normalized_frequency
  }


  pub fn set_true_case(&mut self, true_case: String) {
    self.true_case = Some(true_case);
  }

  pub fn with_true_case(mut self, true_case: String) -> StatsWordResultResult {
    self.true_case = Some(true_case);
    self
  }

  pub fn true_case(&self) -> Option<&String> {
    self.true_case.as_ref()
  }

  pub fn reset_true_case(&mut self) {
    self.true_case = None;
  }

  pub fn set_wordform(&mut self, wordform: String) {
    self.wordform = Some(wordform);
  }

  pub fn with_wordform(mut self, wordform: String) -> StatsWordResultResult {
    self.wordform = Some(wordform);
    self
  }

  pub fn wordform(&self) -> Option<&String> {
    self.wordform.as_ref()
  }

  pub fn reset_wordform(&mut self) {
    self.wordform = None;
  }

}



