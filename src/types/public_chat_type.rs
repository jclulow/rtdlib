
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a type of public chats
pub trait TDPublicChatType: Debug + RObject {}

/// Describes a type of public chats
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PublicChatType {
  #[doc(hidden)] _Default(()),
  /// The chat is public, because it has username
  HasUsername(PublicChatTypeHasUsername),
  /// The chat is public, because it is a location-based supergroup
  IsLocationBased(PublicChatTypeIsLocationBased),

}

impl Default for PublicChatType {
  fn default() -> Self { PublicChatType::_Default(()) }
}

impl<'de> Deserialize<'de> for PublicChatType {
  fn deserialize<D>(deserializer: D) -> Result<PublicChatType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PublicChatType,
      (publicChatTypeHasUsername, HasUsername);
      (publicChatTypeIsLocationBased, IsLocationBased);

    )(deserializer)
  }
}

impl RObject for PublicChatType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PublicChatType::HasUsername(t) => t.td_name(),
      PublicChatType::IsLocationBased(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PublicChatType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PublicChatType::_Default(_) = self { true } else { false } }

  pub fn is_has_username(&self) -> bool { if let PublicChatType::HasUsername(_) = self { true } else { false } }
  pub fn is_is_location_based(&self) -> bool { if let PublicChatType::IsLocationBased(_) = self { true } else { false } }

  pub fn on_has_username<F: FnOnce(&PublicChatTypeHasUsername)>(&self, fnc: F) -> &Self { if let PublicChatType::HasUsername(t) = self { fnc(t) }; self }
  pub fn on_is_location_based<F: FnOnce(&PublicChatTypeIsLocationBased)>(&self, fnc: F) -> &Self { if let PublicChatType::IsLocationBased(t) = self { fnc(t) }; self }

  pub fn as_has_username(&self) -> Option<&PublicChatTypeHasUsername> { if let PublicChatType::HasUsername(t) = self { return Some(t) } None }
  pub fn as_is_location_based(&self) -> Option<&PublicChatTypeIsLocationBased> { if let PublicChatType::IsLocationBased(t) = self { return Some(t) } None }



  pub fn has_username<T: AsRef<PublicChatTypeHasUsername>>(t: T) -> Self { PublicChatType::HasUsername(t.as_ref().clone()) }

  pub fn is_location_based<T: AsRef<PublicChatTypeIsLocationBased>>(t: T) -> Self { PublicChatType::IsLocationBased(t.as_ref().clone()) }

}

impl AsRef<PublicChatType> for PublicChatType {
  fn as_ref(&self) -> &PublicChatType { self }
}







/// The chat is public, because it has username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicChatTypeHasUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PublicChatTypeHasUsername {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "publicChatTypeHasUsername" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPublicChatType for PublicChatTypeHasUsername {}



impl PublicChatTypeHasUsername {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPublicChatTypeHasUsernameBuilder {
    let mut inner = PublicChatTypeHasUsername::default();
    inner.td_name = "publicChatTypeHasUsername".to_string();
    RTDPublicChatTypeHasUsernameBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPublicChatTypeHasUsernameBuilder {
  inner: PublicChatTypeHasUsername
}

impl RTDPublicChatTypeHasUsernameBuilder {
  pub fn build(&self) -> PublicChatTypeHasUsername { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PublicChatTypeHasUsername> for PublicChatTypeHasUsername {
  fn as_ref(&self) -> &PublicChatTypeHasUsername { self }
}

impl AsRef<PublicChatTypeHasUsername> for RTDPublicChatTypeHasUsernameBuilder {
  fn as_ref(&self) -> &PublicChatTypeHasUsername { &self.inner }
}







/// The chat is public, because it is a location-based supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicChatTypeIsLocationBased {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for PublicChatTypeIsLocationBased {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "publicChatTypeIsLocationBased" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPublicChatType for PublicChatTypeIsLocationBased {}



impl PublicChatTypeIsLocationBased {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPublicChatTypeIsLocationBasedBuilder {
    let mut inner = PublicChatTypeIsLocationBased::default();
    inner.td_name = "publicChatTypeIsLocationBased".to_string();
    RTDPublicChatTypeIsLocationBasedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPublicChatTypeIsLocationBasedBuilder {
  inner: PublicChatTypeIsLocationBased
}

impl RTDPublicChatTypeIsLocationBasedBuilder {
  pub fn build(&self) -> PublicChatTypeIsLocationBased { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<PublicChatTypeIsLocationBased> for PublicChatTypeIsLocationBased {
  fn as_ref(&self) -> &PublicChatTypeIsLocationBased { self }
}

impl AsRef<PublicChatTypeIsLocationBased> for RTDPublicChatTypeIsLocationBasedBuilder {
  fn as_ref(&self) -> &PublicChatTypeIsLocationBased { &self.inner }
}



