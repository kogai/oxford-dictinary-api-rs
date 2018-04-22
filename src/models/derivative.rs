#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Derivative {
  /// The identifier of the word
  #[serde(rename = "id")]
  id: Option<String>,
  /// A note text
  #[serde(rename = "text")]
  text: String,
}
