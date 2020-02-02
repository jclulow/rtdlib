
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains information about saved Telegram Passport elements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElements {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Telegram Passport elements
  elements: Vec<PassportElement>,
  
}

impl RObject for PassportElements {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportElements" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PassportElements {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPassportElementsBuilder {
    let mut inner = PassportElements::default();
    inner.td_name = "passportElements".to_string();
    RTDPassportElementsBuilder { inner }
  }

  pub fn elements(&self) -> &Vec<PassportElement> { &self.elements }

}

#[doc(hidden)]
pub struct RTDPassportElementsBuilder {
  inner: PassportElements
}

impl RTDPassportElementsBuilder {
  pub fn build(&self) -> PassportElements { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn elements(&mut self, elements: Vec<PassportElement>) -> &mut Self {
    self.inner.elements = elements;
    self
  }

}

impl AsRef<PassportElements> for PassportElements {
  fn as_ref(&self) -> &PassportElements { self }
}

impl AsRef<PassportElements> for RTDPassportElementsBuilder {
  fn as_ref(&self) -> &PassportElements { &self.inner }
}



