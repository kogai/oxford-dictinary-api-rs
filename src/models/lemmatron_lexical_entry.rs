/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// LemmatronLexicalEntry : Description of an entry for a particular part of speech and grammatical features

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LemmatronLexicalEntry {
  #[serde(rename = "grammaticalFeatures")]
  grammatical_features: Option<Vec<::models::GrammaticalFeaturesListInner>>,
  /// The canonical form of words for which the entry is an inflection
  #[serde(rename = "inflectionOf")]
  inflection_of: ::models::InflectionsList,
  /// IANA language code
  #[serde(rename = "language")]
  language: String,
  /// A linguistic category of words (or more precisely lexical items), generally defined by the syntactic or morphological behaviour of the lexical item in question, such as noun or verb
  #[serde(rename = "lexicalCategory")]
  lexical_category: String,
  /// A given written or spoken realisation of a an entry.
  #[serde(rename = "text")]
  text: String
}

impl LemmatronLexicalEntry {
  /// Description of an entry for a particular part of speech and grammatical features
  pub fn new(inflection_of: ::models::InflectionsList, language: String, lexical_category: String, text: String) -> LemmatronLexicalEntry {
    LemmatronLexicalEntry {
      grammatical_features: None,
      inflection_of: inflection_of,
      language: language,
      lexical_category: lexical_category,
      text: text
    }
  }

  pub fn set_grammatical_features(&mut self, grammatical_features: Vec<::models::GrammaticalFeaturesListInner>) {
    self.grammatical_features = Some(grammatical_features);
  }

  pub fn with_grammatical_features(mut self, grammatical_features: Vec<::models::GrammaticalFeaturesListInner>) -> LemmatronLexicalEntry {
    self.grammatical_features = Some(grammatical_features);
    self
  }

  pub fn grammatical_features(&self) -> Option<&Vec<::models::GrammaticalFeaturesListInner>> {
    self.grammatical_features.as_ref()
  }

  pub fn reset_grammatical_features(&mut self) {
    self.grammatical_features = None;
  }

  pub fn set_inflection_of(&mut self, inflection_of: ::models::InflectionsList) {
    self.inflection_of = inflection_of;
  }

  pub fn with_inflection_of(mut self, inflection_of: ::models::InflectionsList) -> LemmatronLexicalEntry {
    self.inflection_of = inflection_of;
    self
  }

  pub fn inflection_of(&self) -> &::models::InflectionsList {
    &self.inflection_of
  }


  pub fn set_language(&mut self, language: String) {
    self.language = language;
  }

  pub fn with_language(mut self, language: String) -> LemmatronLexicalEntry {
    self.language = language;
    self
  }

  pub fn language(&self) -> &String {
    &self.language
  }


  pub fn set_lexical_category(&mut self, lexical_category: String) {
    self.lexical_category = lexical_category;
  }

  pub fn with_lexical_category(mut self, lexical_category: String) -> LemmatronLexicalEntry {
    self.lexical_category = lexical_category;
    self
  }

  pub fn lexical_category(&self) -> &String {
    &self.lexical_category
  }


  pub fn set_text(&mut self, text: String) {
    self.text = text;
  }

  pub fn with_text(mut self, text: String) -> LemmatronLexicalEntry {
    self.text = text;
    self
  }

  pub fn text(&self) -> &String {
    &self.text
  }


}



