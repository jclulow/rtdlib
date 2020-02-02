
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents one language pack string
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// String key
  key: String,
  /// String value
  value: LanguagePackStringValue,
  
}

impl RObject for LanguagePackString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackString" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LanguagePackString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLanguagePackStringBuilder {
    let mut inner = LanguagePackString::default();
    inner.td_name = "languagePackString".to_string();
    RTDLanguagePackStringBuilder { inner }
  }

  pub fn key(&self) -> &String { &self.key }

  pub fn value(&self) -> &LanguagePackStringValue { &self.value }

}

#[doc(hidden)]
pub struct RTDLanguagePackStringBuilder {
  inner: LanguagePackString
}

impl RTDLanguagePackStringBuilder {
  pub fn build(&self) -> LanguagePackString { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn key<T: AsRef<str>>(&mut self, key: T) -> &mut Self {
    self.inner.key = key.as_ref().to_string();
    self
  }

   
  pub fn value<T: AsRef<LanguagePackStringValue>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().clone();
    self
  }

}

impl AsRef<LanguagePackString> for LanguagePackString {
  fn as_ref(&self) -> &LanguagePackString { self }
}

impl AsRef<LanguagePackString> for RTDLanguagePackStringBuilder {
  fn as_ref(&self) -> &LanguagePackString { &self.inner }
}



