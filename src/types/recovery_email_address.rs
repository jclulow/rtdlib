
use crate::types::*;
use crate::errors::*;




/// Contains information about the current recovery email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecoveryEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Recovery email address
  recovery_email_address: String,
  
}

impl RObject for RecoveryEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "recoveryEmailAddress" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl RecoveryEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDRecoveryEmailAddressBuilder {
    let mut inner = RecoveryEmailAddress::default();
    inner.td_name = "recoveryEmailAddress".to_string();
    RTDRecoveryEmailAddressBuilder { inner }
  }

  pub fn recovery_email_address(&self) -> &String { &self.recovery_email_address }

}

#[doc(hidden)]
pub struct RTDRecoveryEmailAddressBuilder {
  inner: RecoveryEmailAddress
}

impl RTDRecoveryEmailAddressBuilder {
  pub fn build(&self) -> RecoveryEmailAddress { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn recovery_email_address<T: AsRef<str>>(&mut self, recovery_email_address: T) -> &mut Self {
    self.inner.recovery_email_address = recovery_email_address.as_ref().to_string();
    self
  }

}

impl AsRef<RecoveryEmailAddress> for RecoveryEmailAddress {
  fn as_ref(&self) -> &RecoveryEmailAddress { self }
}

impl AsRef<RecoveryEmailAddress> for RTDRecoveryEmailAddressBuilder {
  fn as_ref(&self) -> &RecoveryEmailAddress { &self.inner }
}



