
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the current secret chat state
pub trait TDSecretChatState: Debug + RObject {}

/// Describes the current secret chat state
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SecretChatState {
  #[doc(hidden)] _Default(()),
  /// The secret chat is not yet created; waiting for the other user to get online
  Pending(SecretChatStatePending),
  /// The secret chat is ready to use
  Ready(SecretChatStateReady),
  /// The secret chat is closed
  Closed(SecretChatStateClosed),

}

impl Default for SecretChatState {
  fn default() -> Self { SecretChatState::_Default(()) }
}

impl<'de> Deserialize<'de> for SecretChatState {
  fn deserialize<D>(deserializer: D) -> Result<SecretChatState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SecretChatState,
      (secretChatStatePending, Pending);
      (secretChatStateReady, Ready);
      (secretChatStateClosed, Closed);

    )(deserializer)
  }
}

impl RObject for SecretChatState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      SecretChatState::Pending(t) => t.td_name(),
      SecretChatState::Ready(t) => t.td_name(),
      SecretChatState::Closed(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl SecretChatState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let SecretChatState::_Default(_) = self { true } else { false } }

  pub fn is_pending(&self) -> bool { if let SecretChatState::Pending(_) = self { true } else { false } }
  pub fn is_ready(&self) -> bool { if let SecretChatState::Ready(_) = self { true } else { false } }
  pub fn is_closed(&self) -> bool { if let SecretChatState::Closed(_) = self { true } else { false } }

  pub fn on_pending<F: FnOnce(&SecretChatStatePending)>(&self, fnc: F) -> &Self { if let SecretChatState::Pending(t) = self { fnc(t) }; self }
  pub fn on_ready<F: FnOnce(&SecretChatStateReady)>(&self, fnc: F) -> &Self { if let SecretChatState::Ready(t) = self { fnc(t) }; self }
  pub fn on_closed<F: FnOnce(&SecretChatStateClosed)>(&self, fnc: F) -> &Self { if let SecretChatState::Closed(t) = self { fnc(t) }; self }

  pub fn as_pending(&self) -> Option<&SecretChatStatePending> { if let SecretChatState::Pending(t) = self { return Some(t) } None }
  pub fn as_ready(&self) -> Option<&SecretChatStateReady> { if let SecretChatState::Ready(t) = self { return Some(t) } None }
  pub fn as_closed(&self) -> Option<&SecretChatStateClosed> { if let SecretChatState::Closed(t) = self { return Some(t) } None }



  pub fn pending<T: AsRef<SecretChatStatePending>>(t: T) -> Self { SecretChatState::Pending(t.as_ref().clone()) }

  pub fn ready<T: AsRef<SecretChatStateReady>>(t: T) -> Self { SecretChatState::Ready(t.as_ref().clone()) }

  pub fn closed<T: AsRef<SecretChatStateClosed>>(t: T) -> Self { SecretChatState::Closed(t.as_ref().clone()) }

}

impl AsRef<SecretChatState> for SecretChatState {
  fn as_ref(&self) -> &SecretChatState { self }
}







/// The secret chat is not yet created; waiting for the other user to get online
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SecretChatStatePending {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "secretChatStatePending" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSecretChatState for SecretChatStatePending {}



impl SecretChatStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSecretChatStatePendingBuilder {
    let mut inner = SecretChatStatePending::default();
    inner.td_name = "secretChatStatePending".to_string();
    RTDSecretChatStatePendingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSecretChatStatePendingBuilder {
  inner: SecretChatStatePending
}

impl RTDSecretChatStatePendingBuilder {
  pub fn build(&self) -> SecretChatStatePending { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SecretChatStatePending> for SecretChatStatePending {
  fn as_ref(&self) -> &SecretChatStatePending { self }
}

impl AsRef<SecretChatStatePending> for RTDSecretChatStatePendingBuilder {
  fn as_ref(&self) -> &SecretChatStatePending { &self.inner }
}







/// The secret chat is ready to use
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SecretChatStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "secretChatStateReady" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSecretChatState for SecretChatStateReady {}



impl SecretChatStateReady {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSecretChatStateReadyBuilder {
    let mut inner = SecretChatStateReady::default();
    inner.td_name = "secretChatStateReady".to_string();
    RTDSecretChatStateReadyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSecretChatStateReadyBuilder {
  inner: SecretChatStateReady
}

impl RTDSecretChatStateReadyBuilder {
  pub fn build(&self) -> SecretChatStateReady { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SecretChatStateReady> for SecretChatStateReady {
  fn as_ref(&self) -> &SecretChatStateReady { self }
}

impl AsRef<SecretChatStateReady> for RTDSecretChatStateReadyBuilder {
  fn as_ref(&self) -> &SecretChatStateReady { &self.inner }
}







/// The secret chat is closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChatStateClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SecretChatStateClosed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "secretChatStateClosed" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSecretChatState for SecretChatStateClosed {}



impl SecretChatStateClosed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSecretChatStateClosedBuilder {
    let mut inner = SecretChatStateClosed::default();
    inner.td_name = "secretChatStateClosed".to_string();
    RTDSecretChatStateClosedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSecretChatStateClosedBuilder {
  inner: SecretChatStateClosed
}

impl RTDSecretChatStateClosedBuilder {
  pub fn build(&self) -> SecretChatStateClosed { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SecretChatStateClosed> for SecretChatStateClosed {
  fn as_ref(&self) -> &SecretChatStateClosed { self }
}

impl AsRef<SecretChatStateClosed> for RTDSecretChatStateClosedBuilder {
  fn as_ref(&self) -> &SecretChatStateClosed { &self.inner }
}



