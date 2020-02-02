
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a list of language pack strings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStrings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// A list of language pack strings
  strings: Vec<LanguagePackString>,
  
}

impl RObject for LanguagePackStrings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackStrings" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LanguagePackStrings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLanguagePackStringsBuilder {
    let mut inner = LanguagePackStrings::default();
    inner.td_name = "languagePackStrings".to_string();
    RTDLanguagePackStringsBuilder { inner }
  }

  pub fn strings(&self) -> &Vec<LanguagePackString> { &self.strings }

}

#[doc(hidden)]
pub struct RTDLanguagePackStringsBuilder {
  inner: LanguagePackStrings
}

impl RTDLanguagePackStringsBuilder {
  pub fn build(&self) -> LanguagePackStrings { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self {
    self.inner.strings = strings;
    self
  }

}

impl AsRef<LanguagePackStrings> for LanguagePackStrings {
  fn as_ref(&self) -> &LanguagePackStrings { self }
}

impl AsRef<LanguagePackStrings> for RTDLanguagePackStringsBuilder {
  fn as_ref(&self) -> &LanguagePackStrings { &self.inner }
}



