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
pub struct ThesaurusEntry {
  /// Identifies the homograph grouping. The last two digits identify different entries of the same homograph. The first one/two digits identify the homograph number.
  #[serde(rename = "homographNumber")]
  homograph_number: Option<String>,
  /// Complete list of senses
  #[serde(rename = "senses")]
  senses: Option<Vec<::models::ThesaurusSense>>,
  /// Various words that are used interchangeably depending on the context, e.g 'a' and 'an'
  #[serde(rename = "variantForms")]
  variant_forms: Option<Vec<::models::VariantFormsListInner>>
}

impl ThesaurusEntry {
  pub fn new() -> ThesaurusEntry {
    ThesaurusEntry {
      homograph_number: None,
      senses: None,
      variant_forms: None
    }
  }

  pub fn set_homograph_number(&mut self, homograph_number: String) {
    self.homograph_number = Some(homograph_number);
  }

  pub fn with_homograph_number(mut self, homograph_number: String) -> ThesaurusEntry {
    self.homograph_number = Some(homograph_number);
    self
  }

  pub fn homograph_number(&self) -> Option<&String> {
    self.homograph_number.as_ref()
  }

  pub fn reset_homograph_number(&mut self) {
    self.homograph_number = None;
  }

  pub fn set_senses(&mut self, senses: Vec<::models::ThesaurusSense>) {
    self.senses = Some(senses);
  }

  pub fn with_senses(mut self, senses: Vec<::models::ThesaurusSense>) -> ThesaurusEntry {
    self.senses = Some(senses);
    self
  }

  pub fn senses(&self) -> Option<&Vec<::models::ThesaurusSense>> {
    self.senses.as_ref()
  }

  pub fn reset_senses(&mut self) {
    self.senses = None;
  }

  pub fn set_variant_forms(&mut self, variant_forms: Vec<::models::VariantFormsListInner>) {
    self.variant_forms = Some(variant_forms);
  }

  pub fn with_variant_forms(mut self, variant_forms: Vec<::models::VariantFormsListInner>) -> ThesaurusEntry {
    self.variant_forms = Some(variant_forms);
    self
  }

  pub fn variant_forms(&self) -> Option<&Vec<::models::VariantFormsListInner>> {
    self.variant_forms.as_ref()
  }

  pub fn reset_variant_forms(&mut self) {
    self.variant_forms = None;
  }

}



