/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatsWordResultListResults : Statistical information about a word

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsWordResultListResults {
  /// The number of times a word appears in the entire corpus
  #[serde(rename = "frequency")]
  frequency: i32,
  /// A lemma of the word.
  #[serde(rename = "lemma")]
  lemma: String,
  /// A lexical category such as 'verb' or 'noun'
  #[serde(rename = "lexicalCategory")]
  lexical_category: String,
  /// The number of times a word appears on average in 1 million words
  #[serde(rename = "normalizedFrequency")]
  normalized_frequency: i32,
  /// A given written realisation of a an entry (e.g., \"lay\") usually lower case
  #[serde(rename = "trueCase")]
  true_case: String,
  /// A given written realisation of a an entry (e.g., \"lay\") preserving case
  #[serde(rename = "wordform")]
  wordform: String
}

impl StatsWordResultListResults {
  /// Statistical information about a word
  pub fn new(frequency: i32, lemma: String, lexical_category: String, normalized_frequency: i32, true_case: String, wordform: String) -> StatsWordResultListResults {
    StatsWordResultListResults {
      frequency: frequency,
      lemma: lemma,
      lexical_category: lexical_category,
      normalized_frequency: normalized_frequency,
      true_case: true_case,
      wordform: wordform
    }
  }

  pub fn set_frequency(&mut self, frequency: i32) {
    self.frequency = frequency;
  }

  pub fn with_frequency(mut self, frequency: i32) -> StatsWordResultListResults {
    self.frequency = frequency;
    self
  }

  pub fn frequency(&self) -> &i32 {
    &self.frequency
  }


  pub fn set_lemma(&mut self, lemma: String) {
    self.lemma = lemma;
  }

  pub fn with_lemma(mut self, lemma: String) -> StatsWordResultListResults {
    self.lemma = lemma;
    self
  }

  pub fn lemma(&self) -> &String {
    &self.lemma
  }


  pub fn set_lexical_category(&mut self, lexical_category: String) {
    self.lexical_category = lexical_category;
  }

  pub fn with_lexical_category(mut self, lexical_category: String) -> StatsWordResultListResults {
    self.lexical_category = lexical_category;
    self
  }

  pub fn lexical_category(&self) -> &String {
    &self.lexical_category
  }


  pub fn set_normalized_frequency(&mut self, normalized_frequency: i32) {
    self.normalized_frequency = normalized_frequency;
  }

  pub fn with_normalized_frequency(mut self, normalized_frequency: i32) -> StatsWordResultListResults {
    self.normalized_frequency = normalized_frequency;
    self
  }

  pub fn normalized_frequency(&self) -> &i32 {
    &self.normalized_frequency
  }


  pub fn set_true_case(&mut self, true_case: String) {
    self.true_case = true_case;
  }

  pub fn with_true_case(mut self, true_case: String) -> StatsWordResultListResults {
    self.true_case = true_case;
    self
  }

  pub fn true_case(&self) -> &String {
    &self.true_case
  }


  pub fn set_wordform(&mut self, wordform: String) {
    self.wordform = wordform;
  }

  pub fn with_wordform(mut self, wordform: String) -> StatsWordResultListResults {
    self.wordform = wordform;
    self
  }

  pub fn wordform(&self) -> &String {
    &self.wordform
  }


}



