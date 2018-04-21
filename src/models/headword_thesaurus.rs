/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// HeadwordThesaurus : description of thesaurus information of a word

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeadwordThesaurus {
  /// The identifier of a word
  #[serde(rename = "id")]
  id: String,
  /// IANA language code
  #[serde(rename = "language")]
  language: String,
  /// A grouping of various senses in a specific language, and a lexical category that relates to a word
  #[serde(rename = "lexicalEntries")]
  lexical_entries: Vec<::models::ThesaurusLexicalEntry>,
  /// The json object type. Could be 'headword', 'inflection' or 'phrase'
  #[serde(rename = "type")]
  _type: Option<String>,
  /// A given written or spoken realisation of a an entry, lowercased.
  #[serde(rename = "word")]
  word: String
}

impl HeadwordThesaurus {
  /// description of thesaurus information of a word
  pub fn new(id: String, language: String, lexical_entries: Vec<::models::ThesaurusLexicalEntry>, word: String) -> HeadwordThesaurus {
    HeadwordThesaurus {
      id: id,
      language: language,
      lexical_entries: lexical_entries,
      _type: None,
      word: word
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = id;
  }

  pub fn with_id(mut self, id: String) -> HeadwordThesaurus {
    self.id = id;
    self
  }

  pub fn id(&self) -> &String {
    &self.id
  }


  pub fn set_language(&mut self, language: String) {
    self.language = language;
  }

  pub fn with_language(mut self, language: String) -> HeadwordThesaurus {
    self.language = language;
    self
  }

  pub fn language(&self) -> &String {
    &self.language
  }


  pub fn set_lexical_entries(&mut self, lexical_entries: Vec<::models::ThesaurusLexicalEntry>) {
    self.lexical_entries = lexical_entries;
  }

  pub fn with_lexical_entries(mut self, lexical_entries: Vec<::models::ThesaurusLexicalEntry>) -> HeadwordThesaurus {
    self.lexical_entries = lexical_entries;
    self
  }

  pub fn lexical_entries(&self) -> &Vec<::models::ThesaurusLexicalEntry> {
    &self.lexical_entries
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> HeadwordThesaurus {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

  pub fn set_word(&mut self, word: String) {
    self.word = word;
  }

  pub fn with_word(mut self, word: String) -> HeadwordThesaurus {
    self.word = word;
    self
  }

  pub fn word(&self) -> &String {
    &self.word
  }


}


