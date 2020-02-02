
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a description of the required Telegram Passport element that was requested by a service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportRequiredElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of Telegram Passport elements any of which is enough to provide
  suitable_elements: Vec<PassportSuitableElement>,
  
}

impl RObject for PassportRequiredElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportRequiredElement" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PassportRequiredElement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportRequiredElementBuilder {
    let mut inner = PassportRequiredElement::default();
    inner.td_name = "passportRequiredElement".to_string();
    RTDPassportRequiredElementBuilder { inner }
  }

  pub fn suitable_elements(&self) -> &Vec<PassportSuitableElement> { &self.suitable_elements }

}

#[doc(hidden)]
pub struct RTDPassportRequiredElementBuilder {
  inner: PassportRequiredElement
}

impl RTDPassportRequiredElementBuilder {
  pub fn build(&self) -> PassportRequiredElement { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn suitable_elements(&mut self, suitable_elements: Vec<PassportSuitableElement>) -> &mut Self {
    self.inner.suitable_elements = suitable_elements;
    self
  }

}

impl AsRef<PassportRequiredElement> for PassportRequiredElement {
  fn as_ref(&self) -> &PassportRequiredElement { self }
}

impl AsRef<PassportRequiredElement> for RTDPassportRequiredElementBuilder {
  fn as_ref(&self) -> &PassportRequiredElement { &self.inner }
}



