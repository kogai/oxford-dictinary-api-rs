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
pub struct GrammaticalFeaturesListInner {
  #[serde(rename = "text")]
  text: String,
  #[serde(rename = "type")]
  _type: String
}

impl GrammaticalFeaturesListInner {
  pub fn new(text: String, _type: String) -> GrammaticalFeaturesListInner {
    GrammaticalFeaturesListInner {
      text: text,
      _type: _type
    }
  }

  pub fn set_text(&mut self, text: String) {
    self.text = text;
  }

  pub fn with_text(mut self, text: String) -> GrammaticalFeaturesListInner {
    self.text = text;
    self
  }

  pub fn text(&self) -> &String {
    &self.text
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> GrammaticalFeaturesListInner {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



