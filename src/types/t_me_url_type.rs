
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the type of a URL linking to an internal Telegram entity
pub trait TDTMeUrlType: Debug + RObject {}

/// Describes the type of a URL linking to an internal Telegram entity
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TMeUrlType {
  #[doc(hidden)] _Default(()),
  /// A URL linking to a user
  User(TMeUrlTypeUser),
  /// A URL linking to a public supergroup or channel
  Supergroup(TMeUrlTypeSupergroup),
  /// A chat invite link
  ChatInvite(TMeUrlTypeChatInvite),
  /// A URL linking to a sticker set
  StickerSet(TMeUrlTypeStickerSet),

}

impl Default for TMeUrlType {
  fn default() -> Self { TMeUrlType::_Default(()) }
}

impl<'de> Deserialize<'de> for TMeUrlType {
  fn deserialize<D>(deserializer: D) -> Result<TMeUrlType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TMeUrlType,
      (tMeUrlTypeUser, User);
      (tMeUrlTypeSupergroup, Supergroup);
      (tMeUrlTypeChatInvite, ChatInvite);
      (tMeUrlTypeStickerSet, StickerSet);

    )(deserializer)
  }
}

impl RObject for TMeUrlType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      TMeUrlType::User(t) => t.td_name(),
      TMeUrlType::Supergroup(t) => t.td_name(),
      TMeUrlType::ChatInvite(t) => t.td_name(),
      TMeUrlType::StickerSet(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl TMeUrlType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let TMeUrlType::_Default(_) = self { true } else { false } }

  pub fn is_user(&self) -> bool { if let TMeUrlType::User(_) = self { true } else { false } }
  pub fn is_supergroup(&self) -> bool { if let TMeUrlType::Supergroup(_) = self { true } else { false } }
  pub fn is_chat_invite(&self) -> bool { if let TMeUrlType::ChatInvite(_) = self { true } else { false } }
  pub fn is_sticker_set(&self) -> bool { if let TMeUrlType::StickerSet(_) = self { true } else { false } }

  pub fn on_user<F: FnOnce(&TMeUrlTypeUser)>(&self, fnc: F) -> &Self { if let TMeUrlType::User(t) = self { fnc(t) }; self }
  pub fn on_supergroup<F: FnOnce(&TMeUrlTypeSupergroup)>(&self, fnc: F) -> &Self { if let TMeUrlType::Supergroup(t) = self { fnc(t) }; self }
  pub fn on_chat_invite<F: FnOnce(&TMeUrlTypeChatInvite)>(&self, fnc: F) -> &Self { if let TMeUrlType::ChatInvite(t) = self { fnc(t) }; self }
  pub fn on_sticker_set<F: FnOnce(&TMeUrlTypeStickerSet)>(&self, fnc: F) -> &Self { if let TMeUrlType::StickerSet(t) = self { fnc(t) }; self }

  pub fn as_user(&self) -> Option<&TMeUrlTypeUser> { if let TMeUrlType::User(t) = self { return Some(t) } None }
  pub fn as_supergroup(&self) -> Option<&TMeUrlTypeSupergroup> { if let TMeUrlType::Supergroup(t) = self { return Some(t) } None }
  pub fn as_chat_invite(&self) -> Option<&TMeUrlTypeChatInvite> { if let TMeUrlType::ChatInvite(t) = self { return Some(t) } None }
  pub fn as_sticker_set(&self) -> Option<&TMeUrlTypeStickerSet> { if let TMeUrlType::StickerSet(t) = self { return Some(t) } None }



  pub fn user<T: AsRef<TMeUrlTypeUser>>(t: T) -> Self { TMeUrlType::User(t.as_ref().clone()) }

  pub fn supergroup<T: AsRef<TMeUrlTypeSupergroup>>(t: T) -> Self { TMeUrlType::Supergroup(t.as_ref().clone()) }

  pub fn chat_invite<T: AsRef<TMeUrlTypeChatInvite>>(t: T) -> Self { TMeUrlType::ChatInvite(t.as_ref().clone()) }

  pub fn sticker_set<T: AsRef<TMeUrlTypeStickerSet>>(t: T) -> Self { TMeUrlType::StickerSet(t.as_ref().clone()) }

}

impl AsRef<TMeUrlType> for TMeUrlType {
  fn as_ref(&self) -> &TMeUrlType { self }
}







/// A URL linking to a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Identifier of the user
  user_id: i64,
  
}

impl RObject for TMeUrlTypeUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeUser" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTMeUrlType for TMeUrlTypeUser {}



impl TMeUrlTypeUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTMeUrlTypeUserBuilder {
    let mut inner = TMeUrlTypeUser::default();
    inner.td_name = "tMeUrlTypeUser".to_string();
    RTDTMeUrlTypeUserBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDTMeUrlTypeUserBuilder {
  inner: TMeUrlTypeUser
}

impl RTDTMeUrlTypeUserBuilder {
  pub fn build(&self) -> TMeUrlTypeUser { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<TMeUrlTypeUser> for TMeUrlTypeUser {
  fn as_ref(&self) -> &TMeUrlTypeUser { self }
}

impl AsRef<TMeUrlTypeUser> for RTDTMeUrlTypeUserBuilder {
  fn as_ref(&self) -> &TMeUrlTypeUser { &self.inner }
}







/// A URL linking to a public supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Identifier of the supergroup or channel
  supergroup_id: i64,
  
}

impl RObject for TMeUrlTypeSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeSupergroup" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTMeUrlType for TMeUrlTypeSupergroup {}



impl TMeUrlTypeSupergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTMeUrlTypeSupergroupBuilder {
    let mut inner = TMeUrlTypeSupergroup::default();
    inner.td_name = "tMeUrlTypeSupergroup".to_string();
    RTDTMeUrlTypeSupergroupBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

}

#[doc(hidden)]
pub struct RTDTMeUrlTypeSupergroupBuilder {
  inner: TMeUrlTypeSupergroup
}

impl RTDTMeUrlTypeSupergroupBuilder {
  pub fn build(&self) -> TMeUrlTypeSupergroup { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

}

impl AsRef<TMeUrlTypeSupergroup> for TMeUrlTypeSupergroup {
  fn as_ref(&self) -> &TMeUrlTypeSupergroup { self }
}

impl AsRef<TMeUrlTypeSupergroup> for RTDTMeUrlTypeSupergroupBuilder {
  fn as_ref(&self) -> &TMeUrlTypeSupergroup { &self.inner }
}







/// A chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeChatInvite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Chat invite link info
  info: ChatInviteLinkInfo,
  
}

impl RObject for TMeUrlTypeChatInvite {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeChatInvite" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTMeUrlType for TMeUrlTypeChatInvite {}



impl TMeUrlTypeChatInvite {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTMeUrlTypeChatInviteBuilder {
    let mut inner = TMeUrlTypeChatInvite::default();
    inner.td_name = "tMeUrlTypeChatInvite".to_string();
    RTDTMeUrlTypeChatInviteBuilder { inner }
  }

  pub fn info(&self) -> &ChatInviteLinkInfo { &self.info }

}

#[doc(hidden)]
pub struct RTDTMeUrlTypeChatInviteBuilder {
  inner: TMeUrlTypeChatInvite
}

impl RTDTMeUrlTypeChatInviteBuilder {
  pub fn build(&self) -> TMeUrlTypeChatInvite { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn info<T: AsRef<ChatInviteLinkInfo>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().clone();
    self
  }

}

impl AsRef<TMeUrlTypeChatInvite> for TMeUrlTypeChatInvite {
  fn as_ref(&self) -> &TMeUrlTypeChatInvite { self }
}

impl AsRef<TMeUrlTypeChatInvite> for RTDTMeUrlTypeChatInviteBuilder {
  fn as_ref(&self) -> &TMeUrlTypeChatInvite { &self.inner }
}







/// A URL linking to a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeStickerSet {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Identifier of the sticker set
  sticker_set_id: isize,
  
}

impl RObject for TMeUrlTypeStickerSet {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeStickerSet" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTMeUrlType for TMeUrlTypeStickerSet {}



impl TMeUrlTypeStickerSet {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTMeUrlTypeStickerSetBuilder {
    let mut inner = TMeUrlTypeStickerSet::default();
    inner.td_name = "tMeUrlTypeStickerSet".to_string();
    RTDTMeUrlTypeStickerSetBuilder { inner }
  }

  pub fn sticker_set_id(&self) -> isize { self.sticker_set_id }

}

#[doc(hidden)]
pub struct RTDTMeUrlTypeStickerSetBuilder {
  inner: TMeUrlTypeStickerSet
}

impl RTDTMeUrlTypeStickerSetBuilder {
  pub fn build(&self) -> TMeUrlTypeStickerSet { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn sticker_set_id(&mut self, sticker_set_id: isize) -> &mut Self {
    self.inner.sticker_set_id = sticker_set_id;
    self
  }

}

impl AsRef<TMeUrlTypeStickerSet> for TMeUrlTypeStickerSet {
  fn as_ref(&self) -> &TMeUrlTypeStickerSet { self }
}

impl AsRef<TMeUrlTypeStickerSet> for RTDTMeUrlTypeStickerSetBuilder {
  fn as_ref(&self) -> &TMeUrlTypeStickerSet { &self.inner }
}



