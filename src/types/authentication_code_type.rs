
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Provides information about the method by which an authentication code is delivered to the user
pub trait TDAuthenticationCodeType: Debug + RObject {}

/// Provides information about the method by which an authentication code is delivered to the user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthenticationCodeType {
  #[doc(hidden)] _Default(()),
  /// An authentication code is delivered via a private Telegram message, which can be viewed in another client
  TelegramMessage(AuthenticationCodeTypeTelegramMessage),
  /// An authentication code is delivered via an SMS message to the specified phone number
  Sms(AuthenticationCodeTypeSms),
  /// An authentication code is delivered via a phone call to the specified phone number
  Call(AuthenticationCodeTypeCall),
  /// An authentication code is delivered by an immediately cancelled call to the specified phone number. The number from which the call was made is the code
  FlashCall(AuthenticationCodeTypeFlashCall),

}

impl Default for AuthenticationCodeType {
  fn default() -> Self { AuthenticationCodeType::_Default(()) }
}

impl<'de> Deserialize<'de> for AuthenticationCodeType {
  fn deserialize<D>(deserializer: D) -> Result<AuthenticationCodeType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AuthenticationCodeType,
      (authenticationCodeTypeTelegramMessage, TelegramMessage);
      (authenticationCodeTypeSms, Sms);
      (authenticationCodeTypeCall, Call);
      (authenticationCodeTypeFlashCall, FlashCall);

    )(deserializer)
  }
}

impl RObject for AuthenticationCodeType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      AuthenticationCodeType::TelegramMessage(t) => t.td_name(),
      AuthenticationCodeType::Sms(t) => t.td_name(),
      AuthenticationCodeType::Call(t) => t.td_name(),
      AuthenticationCodeType::FlashCall(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl AuthenticationCodeType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let AuthenticationCodeType::_Default(_) = self { true } else { false } }

  pub fn is_telegram_message(&self) -> bool { if let AuthenticationCodeType::TelegramMessage(_) = self { true } else { false } }
  pub fn is_sms(&self) -> bool { if let AuthenticationCodeType::Sms(_) = self { true } else { false } }
  pub fn is_call(&self) -> bool { if let AuthenticationCodeType::Call(_) = self { true } else { false } }
  pub fn is_flash_call(&self) -> bool { if let AuthenticationCodeType::FlashCall(_) = self { true } else { false } }

  pub fn on_telegram_message<F: FnOnce(&AuthenticationCodeTypeTelegramMessage)>(&self, fnc: F) -> &Self { if let AuthenticationCodeType::TelegramMessage(t) = self { fnc(t) }; self }
  pub fn on_sms<F: FnOnce(&AuthenticationCodeTypeSms)>(&self, fnc: F) -> &Self { if let AuthenticationCodeType::Sms(t) = self { fnc(t) }; self }
  pub fn on_call<F: FnOnce(&AuthenticationCodeTypeCall)>(&self, fnc: F) -> &Self { if let AuthenticationCodeType::Call(t) = self { fnc(t) }; self }
  pub fn on_flash_call<F: FnOnce(&AuthenticationCodeTypeFlashCall)>(&self, fnc: F) -> &Self { if let AuthenticationCodeType::FlashCall(t) = self { fnc(t) }; self }

  pub fn as_telegram_message(&self) -> Option<&AuthenticationCodeTypeTelegramMessage> { if let AuthenticationCodeType::TelegramMessage(t) = self { return Some(t) } None }
  pub fn as_sms(&self) -> Option<&AuthenticationCodeTypeSms> { if let AuthenticationCodeType::Sms(t) = self { return Some(t) } None }
  pub fn as_call(&self) -> Option<&AuthenticationCodeTypeCall> { if let AuthenticationCodeType::Call(t) = self { return Some(t) } None }
  pub fn as_flash_call(&self) -> Option<&AuthenticationCodeTypeFlashCall> { if let AuthenticationCodeType::FlashCall(t) = self { return Some(t) } None }



  pub fn telegram_message<T: AsRef<AuthenticationCodeTypeTelegramMessage>>(t: T) -> Self { AuthenticationCodeType::TelegramMessage(t.as_ref().clone()) }

  pub fn sms<T: AsRef<AuthenticationCodeTypeSms>>(t: T) -> Self { AuthenticationCodeType::Sms(t.as_ref().clone()) }

  pub fn call<T: AsRef<AuthenticationCodeTypeCall>>(t: T) -> Self { AuthenticationCodeType::Call(t.as_ref().clone()) }

  pub fn flash_call<T: AsRef<AuthenticationCodeTypeFlashCall>>(t: T) -> Self { AuthenticationCodeType::FlashCall(t.as_ref().clone()) }

}

impl AsRef<AuthenticationCodeType> for AuthenticationCodeType {
  fn as_ref(&self) -> &AuthenticationCodeType { self }
}







/// An authentication code is delivered via a private Telegram message, which can be viewed in another client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeTelegramMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Length of the code
  length: i64,
  
}

impl RObject for AuthenticationCodeTypeTelegramMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeTypeTelegramMessage" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthenticationCodeType for AuthenticationCodeTypeTelegramMessage {}



impl AuthenticationCodeTypeTelegramMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthenticationCodeTypeTelegramMessageBuilder {
    let mut inner = AuthenticationCodeTypeTelegramMessage::default();
    inner.td_name = "authenticationCodeTypeTelegramMessage".to_string();
    RTDAuthenticationCodeTypeTelegramMessageBuilder { inner }
  }

  pub fn length(&self) -> i64 { self.length }

}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeTelegramMessageBuilder {
  inner: AuthenticationCodeTypeTelegramMessage
}

impl RTDAuthenticationCodeTypeTelegramMessageBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeTelegramMessage { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

}

impl AsRef<AuthenticationCodeTypeTelegramMessage> for AuthenticationCodeTypeTelegramMessage {
  fn as_ref(&self) -> &AuthenticationCodeTypeTelegramMessage { self }
}

impl AsRef<AuthenticationCodeTypeTelegramMessage> for RTDAuthenticationCodeTypeTelegramMessageBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeTelegramMessage { &self.inner }
}







/// An authentication code is delivered via an SMS message to the specified phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeSms {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Length of the code
  length: i64,
  
}

impl RObject for AuthenticationCodeTypeSms {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeTypeSms" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthenticationCodeType for AuthenticationCodeTypeSms {}



impl AuthenticationCodeTypeSms {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthenticationCodeTypeSmsBuilder {
    let mut inner = AuthenticationCodeTypeSms::default();
    inner.td_name = "authenticationCodeTypeSms".to_string();
    RTDAuthenticationCodeTypeSmsBuilder { inner }
  }

  pub fn length(&self) -> i64 { self.length }

}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeSmsBuilder {
  inner: AuthenticationCodeTypeSms
}

impl RTDAuthenticationCodeTypeSmsBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeSms { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

}

impl AsRef<AuthenticationCodeTypeSms> for AuthenticationCodeTypeSms {
  fn as_ref(&self) -> &AuthenticationCodeTypeSms { self }
}

impl AsRef<AuthenticationCodeTypeSms> for RTDAuthenticationCodeTypeSmsBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeSms { &self.inner }
}







/// An authentication code is delivered via a phone call to the specified phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Length of the code
  length: i64,
  
}

impl RObject for AuthenticationCodeTypeCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeTypeCall" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthenticationCodeType for AuthenticationCodeTypeCall {}



impl AuthenticationCodeTypeCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthenticationCodeTypeCallBuilder {
    let mut inner = AuthenticationCodeTypeCall::default();
    inner.td_name = "authenticationCodeTypeCall".to_string();
    RTDAuthenticationCodeTypeCallBuilder { inner }
  }

  pub fn length(&self) -> i64 { self.length }

}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeCallBuilder {
  inner: AuthenticationCodeTypeCall
}

impl RTDAuthenticationCodeTypeCallBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeCall { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

}

impl AsRef<AuthenticationCodeTypeCall> for AuthenticationCodeTypeCall {
  fn as_ref(&self) -> &AuthenticationCodeTypeCall { self }
}

impl AsRef<AuthenticationCodeTypeCall> for RTDAuthenticationCodeTypeCallBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeCall { &self.inner }
}







/// An authentication code is delivered by an immediately cancelled call to the specified phone number. The number from which the call was made is the code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeFlashCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Pattern of the phone number from which the call will be made
  pattern: String,
  
}

impl RObject for AuthenticationCodeTypeFlashCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeTypeFlashCall" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthenticationCodeType for AuthenticationCodeTypeFlashCall {}



impl AuthenticationCodeTypeFlashCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthenticationCodeTypeFlashCallBuilder {
    let mut inner = AuthenticationCodeTypeFlashCall::default();
    inner.td_name = "authenticationCodeTypeFlashCall".to_string();
    RTDAuthenticationCodeTypeFlashCallBuilder { inner }
  }

  pub fn pattern(&self) -> &String { &self.pattern }

}

#[doc(hidden)]
pub struct RTDAuthenticationCodeTypeFlashCallBuilder {
  inner: AuthenticationCodeTypeFlashCall
}

impl RTDAuthenticationCodeTypeFlashCallBuilder {
  pub fn build(&self) -> AuthenticationCodeTypeFlashCall { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn pattern<T: AsRef<str>>(&mut self, pattern: T) -> &mut Self {
    self.inner.pattern = pattern.as_ref().to_string();
    self
  }

}

impl AsRef<AuthenticationCodeTypeFlashCall> for AuthenticationCodeTypeFlashCall {
  fn as_ref(&self) -> &AuthenticationCodeTypeFlashCall { self }
}

impl AsRef<AuthenticationCodeTypeFlashCall> for RTDAuthenticationCodeTypeFlashCallBuilder {
  fn as_ref(&self) -> &AuthenticationCodeTypeFlashCall { &self.inner }
}



