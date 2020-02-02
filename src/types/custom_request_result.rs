
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains the result of a custom request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomRequestResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// A JSON-serialized result
  result: String,
  
}

impl RObject for CustomRequestResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "customRequestResult" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl CustomRequestResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCustomRequestResultBuilder {
    let mut inner = CustomRequestResult::default();
    inner.td_name = "customRequestResult".to_string();
    RTDCustomRequestResultBuilder { inner }
  }

  pub fn result(&self) -> &String { &self.result }

}

#[doc(hidden)]
pub struct RTDCustomRequestResultBuilder {
  inner: CustomRequestResult
}

impl RTDCustomRequestResultBuilder {
  pub fn build(&self) -> CustomRequestResult { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn result<T: AsRef<str>>(&mut self, result: T) -> &mut Self {
    self.inner.result = result.as_ref().to_string();
    self
  }

}

impl AsRef<CustomRequestResult> for CustomRequestResult {
  fn as_ref(&self) -> &CustomRequestResult { self }
}

impl AsRef<CustomRequestResult> for RTDCustomRequestResultBuilder {
  fn as_ref(&self) -> &CustomRequestResult { &self.inner }
}



