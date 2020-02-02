
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Information about the email address authentication code that was sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationCodeInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Pattern of the email address to which an authentication code was sent
  email_address_pattern: String,
  /// Length of the code; 0 if unknown
  length: i64,
  
}

impl RObject for EmailAddressAuthenticationCodeInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "emailAddressAuthenticationCodeInfo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl EmailAddressAuthenticationCodeInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEmailAddressAuthenticationCodeInfoBuilder {
    let mut inner = EmailAddressAuthenticationCodeInfo::default();
    inner.td_name = "emailAddressAuthenticationCodeInfo".to_string();
    RTDEmailAddressAuthenticationCodeInfoBuilder { inner }
  }

  pub fn email_address_pattern(&self) -> &String { &self.email_address_pattern }

  pub fn length(&self) -> i64 { self.length }

}

#[doc(hidden)]
pub struct RTDEmailAddressAuthenticationCodeInfoBuilder {
  inner: EmailAddressAuthenticationCodeInfo
}

impl RTDEmailAddressAuthenticationCodeInfoBuilder {
  pub fn build(&self) -> EmailAddressAuthenticationCodeInfo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn email_address_pattern<T: AsRef<str>>(&mut self, email_address_pattern: T) -> &mut Self {
    self.inner.email_address_pattern = email_address_pattern.as_ref().to_string();
    self
  }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

}

impl AsRef<EmailAddressAuthenticationCodeInfo> for EmailAddressAuthenticationCodeInfo {
  fn as_ref(&self) -> &EmailAddressAuthenticationCodeInfo { self }
}

impl AsRef<EmailAddressAuthenticationCodeInfo> for RTDEmailAddressAuthenticationCodeInfoBuilder {
  fn as_ref(&self) -> &EmailAddressAuthenticationCodeInfo { &self.inner }
}



