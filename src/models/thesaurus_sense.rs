/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ThesaurusSense : A lexical sense represents the lexical meaning of a lexical entry when interpreted as referring to the corresponding ontology element

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThesaurusSense {
  /// antonym of word
  #[serde(rename = "antonyms")]
  antonyms: Option<::models::SynonymsAntonyms>,
  /// A subject, discipline, or branch of knowledge particular to the Sense
  #[serde(rename = "domains")]
  domains: Option<::models::Arrayofstrings>,
  #[serde(rename = "examples")]
  examples: Option<::models::ExamplesList>,
  /// The id of the sense that is required for the delete procedure
  #[serde(rename = "id")]
  id: Option<String>,
  /// A particular area in which the Sense occurs, e.g. 'Great Britain'
  #[serde(rename = "regions")]
  regions: Option<::models::Arrayofstrings>,
  /// A level of language usage, typically with respect to formality. e.g. 'offensive', 'informal'
  #[serde(rename = "registers")]
  registers: Option<::models::Arrayofstrings>,
  /// subsenses of word
  #[serde(rename = "subsenses")]
  subsenses: Option<Vec<::models::ThesaurusSense>>,
  /// synonym of word
  #[serde(rename = "synonyms")]
  synonyms: Option<::models::SynonymsAntonyms>
}

impl ThesaurusSense {
  /// A lexical sense represents the lexical meaning of a lexical entry when interpreted as referring to the corresponding ontology element
  pub fn new() -> ThesaurusSense {
    ThesaurusSense {
      antonyms: None,
      domains: None,
      examples: None,
      id: None,
      regions: None,
      registers: None,
      subsenses: None,
      synonyms: None
    }
  }

  pub fn set_antonyms(&mut self, antonyms: ::models::SynonymsAntonyms) {
    self.antonyms = Some(antonyms);
  }

  pub fn with_antonyms(mut self, antonyms: ::models::SynonymsAntonyms) -> ThesaurusSense {
    self.antonyms = Some(antonyms);
    self
  }

  pub fn antonyms(&self) -> Option<&::models::SynonymsAntonyms> {
    self.antonyms.as_ref()
  }

  pub fn reset_antonyms(&mut self) {
    self.antonyms = None;
  }

  pub fn set_domains(&mut self, domains: ::models::Arrayofstrings) {
    self.domains = Some(domains);
  }

  pub fn with_domains(mut self, domains: ::models::Arrayofstrings) -> ThesaurusSense {
    self.domains = Some(domains);
    self
  }

  pub fn domains(&self) -> Option<&::models::Arrayofstrings> {
    self.domains.as_ref()
  }

  pub fn reset_domains(&mut self) {
    self.domains = None;
  }

  pub fn set_examples(&mut self, examples: ::models::ExamplesList) {
    self.examples = Some(examples);
  }

  pub fn with_examples(mut self, examples: ::models::ExamplesList) -> ThesaurusSense {
    self.examples = Some(examples);
    self
  }

  pub fn examples(&self) -> Option<&::models::ExamplesList> {
    self.examples.as_ref()
  }

  pub fn reset_examples(&mut self) {
    self.examples = None;
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> ThesaurusSense {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_regions(&mut self, regions: ::models::Arrayofstrings) {
    self.regions = Some(regions);
  }

  pub fn with_regions(mut self, regions: ::models::Arrayofstrings) -> ThesaurusSense {
    self.regions = Some(regions);
    self
  }

  pub fn regions(&self) -> Option<&::models::Arrayofstrings> {
    self.regions.as_ref()
  }

  pub fn reset_regions(&mut self) {
    self.regions = None;
  }

  pub fn set_registers(&mut self, registers: ::models::Arrayofstrings) {
    self.registers = Some(registers);
  }

  pub fn with_registers(mut self, registers: ::models::Arrayofstrings) -> ThesaurusSense {
    self.registers = Some(registers);
    self
  }

  pub fn registers(&self) -> Option<&::models::Arrayofstrings> {
    self.registers.as_ref()
  }

  pub fn reset_registers(&mut self) {
    self.registers = None;
  }

  pub fn set_subsenses(&mut self, subsenses: Vec<::models::ThesaurusSense>) {
    self.subsenses = Some(subsenses);
  }

  pub fn with_subsenses(mut self, subsenses: Vec<::models::ThesaurusSense>) -> ThesaurusSense {
    self.subsenses = Some(subsenses);
    self
  }

  pub fn subsenses(&self) -> Option<&Vec<::models::ThesaurusSense>> {
    self.subsenses.as_ref()
  }

  pub fn reset_subsenses(&mut self) {
    self.subsenses = None;
  }

  pub fn set_synonyms(&mut self, synonyms: ::models::SynonymsAntonyms) {
    self.synonyms = Some(synonyms);
  }

  pub fn with_synonyms(mut self, synonyms: ::models::SynonymsAntonyms) -> ThesaurusSense {
    self.synonyms = Some(synonyms);
    self
  }

  pub fn synonyms(&self) -> Option<&::models::SynonymsAntonyms> {
    self.synonyms.as_ref()
  }

  pub fn reset_synonyms(&mut self) {
    self.synonyms = None;
  }

}


