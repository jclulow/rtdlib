
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents one member of a JSON object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonObjectMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Member's key
  key: String,
  /// Member's value
  value: JsonValue,
  
}

impl RObject for JsonObjectMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonObjectMember" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl JsonObjectMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonObjectMemberBuilder {
    let mut inner = JsonObjectMember::default();
    inner.td_name = "jsonObjectMember".to_string();
    RTDJsonObjectMemberBuilder { inner }
  }

  pub fn key(&self) -> &String { &self.key }

  pub fn value(&self) -> &JsonValue { &self.value }

}

#[doc(hidden)]
pub struct RTDJsonObjectMemberBuilder {
  inner: JsonObjectMember
}

impl RTDJsonObjectMemberBuilder {
  pub fn build(&self) -> JsonObjectMember { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn key<T: AsRef<str>>(&mut self, key: T) -> &mut Self {
    self.inner.key = key.as_ref().to_string();
    self
  }

   
  pub fn value<T: AsRef<JsonValue>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().clone();
    self
  }

}

impl AsRef<JsonObjectMember> for JsonObjectMember {
  fn as_ref(&self) -> &JsonObjectMember { self }
}

impl AsRef<JsonObjectMember> for RTDJsonObjectMemberBuilder {
  fn as_ref(&self) -> &JsonObjectMember { &self.inner }
}



