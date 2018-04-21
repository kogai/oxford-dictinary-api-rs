/* 
 * 
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 1.11.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
  /// The origin of the word and the way in which its meaning has changed throughout history
  #[serde(rename = "etymologies")]
  etymologies: Option<::models::Arrayofstrings>,
  #[serde(rename = "grammaticalFeatures")]
  grammatical_features: Option<::models::GrammaticalFeaturesList>,
  /// Identifies the homograph grouping. The last two digits identify different entries of the same homograph. The first one/two digits identify the homograph number.
  #[serde(rename = "homographNumber")]
  homograph_number: Option<String>,
  #[serde(rename = "notes")]
  notes: Option<::models::CategorizedTextList>,
  #[serde(rename = "pronunciations")]
  pronunciations: Option<::models::PronunciationsList>,
  /// Complete list of senses
  #[serde(rename = "senses")]
  senses: Option<Vec<::models::Sense>>,
  /// Various words that are used interchangeably depending on the context, e.g 'a' and 'an'
  #[serde(rename = "variantForms")]
  variant_forms: Option<::models::VariantFormsList>
}

impl Entry {
  pub fn new() -> Entry {
    Entry {
      etymologies: None,
      grammatical_features: None,
      homograph_number: None,
      notes: None,
      pronunciations: None,
      senses: None,
      variant_forms: None
    }
  }

  pub fn set_etymologies(&mut self, etymologies: ::models::Arrayofstrings) {
    self.etymologies = Some(etymologies);
  }

  pub fn with_etymologies(mut self, etymologies: ::models::Arrayofstrings) -> Entry {
    self.etymologies = Some(etymologies);
    self
  }

  pub fn etymologies(&self) -> Option<&::models::Arrayofstrings> {
    self.etymologies.as_ref()
  }

  pub fn reset_etymologies(&mut self) {
    self.etymologies = None;
  }

  pub fn set_grammatical_features(&mut self, grammatical_features: ::models::GrammaticalFeaturesList) {
    self.grammatical_features = Some(grammatical_features);
  }

  pub fn with_grammatical_features(mut self, grammatical_features: ::models::GrammaticalFeaturesList) -> Entry {
    self.grammatical_features = Some(grammatical_features);
    self
  }

  pub fn grammatical_features(&self) -> Option<&::models::GrammaticalFeaturesList> {
    self.grammatical_features.as_ref()
  }

  pub fn reset_grammatical_features(&mut self) {
    self.grammatical_features = None;
  }

  pub fn set_homograph_number(&mut self, homograph_number: String) {
    self.homograph_number = Some(homograph_number);
  }

  pub fn with_homograph_number(mut self, homograph_number: String) -> Entry {
    self.homograph_number = Some(homograph_number);
    self
  }

  pub fn homograph_number(&self) -> Option<&String> {
    self.homograph_number.as_ref()
  }

  pub fn reset_homograph_number(&mut self) {
    self.homograph_number = None;
  }

  pub fn set_notes(&mut self, notes: ::models::CategorizedTextList) {
    self.notes = Some(notes);
  }

  pub fn with_notes(mut self, notes: ::models::CategorizedTextList) -> Entry {
    self.notes = Some(notes);
    self
  }

  pub fn notes(&self) -> Option<&::models::CategorizedTextList> {
    self.notes.as_ref()
  }

  pub fn reset_notes(&mut self) {
    self.notes = None;
  }

  pub fn set_pronunciations(&mut self, pronunciations: ::models::PronunciationsList) {
    self.pronunciations = Some(pronunciations);
  }

  pub fn with_pronunciations(mut self, pronunciations: ::models::PronunciationsList) -> Entry {
    self.pronunciations = Some(pronunciations);
    self
  }

  pub fn pronunciations(&self) -> Option<&::models::PronunciationsList> {
    self.pronunciations.as_ref()
  }

  pub fn reset_pronunciations(&mut self) {
    self.pronunciations = None;
  }

  pub fn set_senses(&mut self, senses: Vec<::models::Sense>) {
    self.senses = Some(senses);
  }

  pub fn with_senses(mut self, senses: Vec<::models::Sense>) -> Entry {
    self.senses = Some(senses);
    self
  }

  pub fn senses(&self) -> Option<&Vec<::models::Sense>> {
    self.senses.as_ref()
  }

  pub fn reset_senses(&mut self) {
    self.senses = None;
  }

  pub fn set_variant_forms(&mut self, variant_forms: ::models::VariantFormsList) {
    self.variant_forms = Some(variant_forms);
  }

  pub fn with_variant_forms(mut self, variant_forms: ::models::VariantFormsList) -> Entry {
    self.variant_forms = Some(variant_forms);
    self
  }

  pub fn variant_forms(&self) -> Option<&::models::VariantFormsList> {
    self.variant_forms.as_ref()
  }

  pub fn reset_variant_forms(&mut self) {
    self.variant_forms = None;
  }

}


