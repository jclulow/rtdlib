
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains Telegram terms of service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TermsOfService {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Text of the terms of service
  text: FormattedText,
  /// Minimum age of a user to be able to accept the terms; 0 if any
  min_user_age: i64,
  /// True, if a blocking popup with terms of service must be shown to the user
  show_popup: bool,
  
}

impl RObject for TermsOfService {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "termsOfService" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TermsOfService {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTermsOfServiceBuilder {
    let mut inner = TermsOfService::default();
    inner.td_name = "termsOfService".to_string();
    RTDTermsOfServiceBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText { &self.text }

  pub fn min_user_age(&self) -> i64 { self.min_user_age }

  pub fn show_popup(&self) -> bool { self.show_popup }

}

#[doc(hidden)]
pub struct RTDTermsOfServiceBuilder {
  inner: TermsOfService
}

impl RTDTermsOfServiceBuilder {
  pub fn build(&self) -> TermsOfService { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn min_user_age(&mut self, min_user_age: i64) -> &mut Self {
    self.inner.min_user_age = min_user_age;
    self
  }

   
  pub fn show_popup(&mut self, show_popup: bool) -> &mut Self {
    self.inner.show_popup = show_popup;
    self
  }

}

impl AsRef<TermsOfService> for TermsOfService {
  fn as_ref(&self) -> &TermsOfService { self }
}

impl AsRef<TermsOfService> for RTDTermsOfServiceBuilder {
  fn as_ref(&self) -> &TermsOfService { &self.inner }
}



