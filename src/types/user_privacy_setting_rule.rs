
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a single rule for managing privacy settings
pub trait TDUserPrivacySettingRule: Debug + RObject {}

/// Represents a single rule for managing privacy settings
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserPrivacySettingRule {
  #[doc(hidden)] _Default(()),
  /// A rule to allow all users to do something
  AllowAll(UserPrivacySettingRuleAllowAll),
  /// A rule to allow all of a user's contacts to do something
  AllowContacts(UserPrivacySettingRuleAllowContacts),
  /// A rule to allow certain specified users to do something
  AllowUsers(UserPrivacySettingRuleAllowUsers),
  /// A rule to allow all members of certain specified basic groups and supergroups to doing something
  AllowChatMembers(UserPrivacySettingRuleAllowChatMembers),
  /// A rule to restrict all users from doing something
  RestrictAll(UserPrivacySettingRuleRestrictAll),
  /// A rule to restrict all contacts of a user from doing something
  RestrictContacts(UserPrivacySettingRuleRestrictContacts),
  /// A rule to restrict all specified users from doing something
  RestrictUsers(UserPrivacySettingRuleRestrictUsers),
  /// A rule to restrict all members of specified basic groups and supergroups from doing something
  RestrictChatMembers(UserPrivacySettingRuleRestrictChatMembers),

}

impl Default for UserPrivacySettingRule {
  fn default() -> Self { UserPrivacySettingRule::_Default(()) }
}

impl<'de> Deserialize<'de> for UserPrivacySettingRule {
  fn deserialize<D>(deserializer: D) -> Result<UserPrivacySettingRule, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UserPrivacySettingRule,
      (userPrivacySettingRuleAllowAll, AllowAll);
      (userPrivacySettingRuleAllowContacts, AllowContacts);
      (userPrivacySettingRuleAllowUsers, AllowUsers);
      (userPrivacySettingRuleAllowChatMembers, AllowChatMembers);
      (userPrivacySettingRuleRestrictAll, RestrictAll);
      (userPrivacySettingRuleRestrictContacts, RestrictContacts);
      (userPrivacySettingRuleRestrictUsers, RestrictUsers);
      (userPrivacySettingRuleRestrictChatMembers, RestrictChatMembers);

    )(deserializer)
  }
}

impl RObject for UserPrivacySettingRule {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      UserPrivacySettingRule::AllowAll(t) => t.td_name(),
      UserPrivacySettingRule::AllowContacts(t) => t.td_name(),
      UserPrivacySettingRule::AllowUsers(t) => t.td_name(),
      UserPrivacySettingRule::AllowChatMembers(t) => t.td_name(),
      UserPrivacySettingRule::RestrictAll(t) => t.td_name(),
      UserPrivacySettingRule::RestrictContacts(t) => t.td_name(),
      UserPrivacySettingRule::RestrictUsers(t) => t.td_name(),
      UserPrivacySettingRule::RestrictChatMembers(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl UserPrivacySettingRule {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let UserPrivacySettingRule::_Default(_) = self { true } else { false } }

  pub fn is_allow_all(&self) -> bool { if let UserPrivacySettingRule::AllowAll(_) = self { true } else { false } }
  pub fn is_allow_contacts(&self) -> bool { if let UserPrivacySettingRule::AllowContacts(_) = self { true } else { false } }
  pub fn is_allow_users(&self) -> bool { if let UserPrivacySettingRule::AllowUsers(_) = self { true } else { false } }
  pub fn is_allow_chat_members(&self) -> bool { if let UserPrivacySettingRule::AllowChatMembers(_) = self { true } else { false } }
  pub fn is_restrict_all(&self) -> bool { if let UserPrivacySettingRule::RestrictAll(_) = self { true } else { false } }
  pub fn is_restrict_contacts(&self) -> bool { if let UserPrivacySettingRule::RestrictContacts(_) = self { true } else { false } }
  pub fn is_restrict_users(&self) -> bool { if let UserPrivacySettingRule::RestrictUsers(_) = self { true } else { false } }
  pub fn is_restrict_chat_members(&self) -> bool { if let UserPrivacySettingRule::RestrictChatMembers(_) = self { true } else { false } }

  pub fn on_allow_all<F: FnOnce(&UserPrivacySettingRuleAllowAll)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::AllowAll(t) = self { fnc(t) }; self }
  pub fn on_allow_contacts<F: FnOnce(&UserPrivacySettingRuleAllowContacts)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::AllowContacts(t) = self { fnc(t) }; self }
  pub fn on_allow_users<F: FnOnce(&UserPrivacySettingRuleAllowUsers)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::AllowUsers(t) = self { fnc(t) }; self }
  pub fn on_allow_chat_members<F: FnOnce(&UserPrivacySettingRuleAllowChatMembers)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::AllowChatMembers(t) = self { fnc(t) }; self }
  pub fn on_restrict_all<F: FnOnce(&UserPrivacySettingRuleRestrictAll)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::RestrictAll(t) = self { fnc(t) }; self }
  pub fn on_restrict_contacts<F: FnOnce(&UserPrivacySettingRuleRestrictContacts)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::RestrictContacts(t) = self { fnc(t) }; self }
  pub fn on_restrict_users<F: FnOnce(&UserPrivacySettingRuleRestrictUsers)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::RestrictUsers(t) = self { fnc(t) }; self }
  pub fn on_restrict_chat_members<F: FnOnce(&UserPrivacySettingRuleRestrictChatMembers)>(&self, fnc: F) -> &Self { if let UserPrivacySettingRule::RestrictChatMembers(t) = self { fnc(t) }; self }

  pub fn as_allow_all(&self) -> Option<&UserPrivacySettingRuleAllowAll> { if let UserPrivacySettingRule::AllowAll(t) = self { return Some(t) } None }
  pub fn as_allow_contacts(&self) -> Option<&UserPrivacySettingRuleAllowContacts> { if let UserPrivacySettingRule::AllowContacts(t) = self { return Some(t) } None }
  pub fn as_allow_users(&self) -> Option<&UserPrivacySettingRuleAllowUsers> { if let UserPrivacySettingRule::AllowUsers(t) = self { return Some(t) } None }
  pub fn as_allow_chat_members(&self) -> Option<&UserPrivacySettingRuleAllowChatMembers> { if let UserPrivacySettingRule::AllowChatMembers(t) = self { return Some(t) } None }
  pub fn as_restrict_all(&self) -> Option<&UserPrivacySettingRuleRestrictAll> { if let UserPrivacySettingRule::RestrictAll(t) = self { return Some(t) } None }
  pub fn as_restrict_contacts(&self) -> Option<&UserPrivacySettingRuleRestrictContacts> { if let UserPrivacySettingRule::RestrictContacts(t) = self { return Some(t) } None }
  pub fn as_restrict_users(&self) -> Option<&UserPrivacySettingRuleRestrictUsers> { if let UserPrivacySettingRule::RestrictUsers(t) = self { return Some(t) } None }
  pub fn as_restrict_chat_members(&self) -> Option<&UserPrivacySettingRuleRestrictChatMembers> { if let UserPrivacySettingRule::RestrictChatMembers(t) = self { return Some(t) } None }



  pub fn allow_all<T: AsRef<UserPrivacySettingRuleAllowAll>>(t: T) -> Self { UserPrivacySettingRule::AllowAll(t.as_ref().clone()) }

  pub fn allow_contacts<T: AsRef<UserPrivacySettingRuleAllowContacts>>(t: T) -> Self { UserPrivacySettingRule::AllowContacts(t.as_ref().clone()) }

  pub fn allow_users<T: AsRef<UserPrivacySettingRuleAllowUsers>>(t: T) -> Self { UserPrivacySettingRule::AllowUsers(t.as_ref().clone()) }

  pub fn allow_chat_members<T: AsRef<UserPrivacySettingRuleAllowChatMembers>>(t: T) -> Self { UserPrivacySettingRule::AllowChatMembers(t.as_ref().clone()) }

  pub fn restrict_all<T: AsRef<UserPrivacySettingRuleRestrictAll>>(t: T) -> Self { UserPrivacySettingRule::RestrictAll(t.as_ref().clone()) }

  pub fn restrict_contacts<T: AsRef<UserPrivacySettingRuleRestrictContacts>>(t: T) -> Self { UserPrivacySettingRule::RestrictContacts(t.as_ref().clone()) }

  pub fn restrict_users<T: AsRef<UserPrivacySettingRuleRestrictUsers>>(t: T) -> Self { UserPrivacySettingRule::RestrictUsers(t.as_ref().clone()) }

  pub fn restrict_chat_members<T: AsRef<UserPrivacySettingRuleRestrictChatMembers>>(t: T) -> Self { UserPrivacySettingRule::RestrictChatMembers(t.as_ref().clone()) }

}

impl AsRef<UserPrivacySettingRule> for UserPrivacySettingRule {
  fn as_ref(&self) -> &UserPrivacySettingRule { self }
}







/// A rule to allow all users to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for UserPrivacySettingRuleAllowAll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleAllowAll" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowAll {}



impl UserPrivacySettingRuleAllowAll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowAllBuilder {
    let mut inner = UserPrivacySettingRuleAllowAll::default();
    inner.td_name = "userPrivacySettingRuleAllowAll".to_string();
    RTDUserPrivacySettingRuleAllowAllBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowAllBuilder {
  inner: UserPrivacySettingRuleAllowAll
}

impl RTDUserPrivacySettingRuleAllowAllBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowAll { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<UserPrivacySettingRuleAllowAll> for UserPrivacySettingRuleAllowAll {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowAll { self }
}

impl AsRef<UserPrivacySettingRuleAllowAll> for RTDUserPrivacySettingRuleAllowAllBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowAll { &self.inner }
}







/// A rule to allow all of a user's contacts to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for UserPrivacySettingRuleAllowContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleAllowContacts" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowContacts {}



impl UserPrivacySettingRuleAllowContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowContactsBuilder {
    let mut inner = UserPrivacySettingRuleAllowContacts::default();
    inner.td_name = "userPrivacySettingRuleAllowContacts".to_string();
    RTDUserPrivacySettingRuleAllowContactsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowContactsBuilder {
  inner: UserPrivacySettingRuleAllowContacts
}

impl RTDUserPrivacySettingRuleAllowContactsBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowContacts { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<UserPrivacySettingRuleAllowContacts> for UserPrivacySettingRuleAllowContacts {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowContacts { self }
}

impl AsRef<UserPrivacySettingRuleAllowContacts> for RTDUserPrivacySettingRuleAllowContactsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowContacts { &self.inner }
}







/// A rule to allow certain specified users to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The user identifiers, total number of users in all rules must not exceed 1000
  user_ids: Vec<i64>,
  
}

impl RObject for UserPrivacySettingRuleAllowUsers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleAllowUsers" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowUsers {}



impl UserPrivacySettingRuleAllowUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowUsersBuilder {
    let mut inner = UserPrivacySettingRuleAllowUsers::default();
    inner.td_name = "userPrivacySettingRuleAllowUsers".to_string();
    RTDUserPrivacySettingRuleAllowUsersBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> { &self.user_ids }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowUsersBuilder {
  inner: UserPrivacySettingRuleAllowUsers
}

impl RTDUserPrivacySettingRuleAllowUsersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowUsers { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

}

impl AsRef<UserPrivacySettingRuleAllowUsers> for UserPrivacySettingRuleAllowUsers {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowUsers { self }
}

impl AsRef<UserPrivacySettingRuleAllowUsers> for RTDUserPrivacySettingRuleAllowUsersBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowUsers { &self.inner }
}







/// A rule to allow all members of certain specified basic groups and supergroups to doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The chat identifiers, total number of chats in all rules must not exceed 20
  chat_ids: Vec<i64>,
  
}

impl RObject for UserPrivacySettingRuleAllowChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleAllowChatMembers" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowChatMembers {}



impl UserPrivacySettingRuleAllowChatMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleAllowChatMembersBuilder {
    let mut inner = UserPrivacySettingRuleAllowChatMembers::default();
    inner.td_name = "userPrivacySettingRuleAllowChatMembers".to_string();
    RTDUserPrivacySettingRuleAllowChatMembersBuilder { inner }
  }

  pub fn chat_ids(&self) -> &Vec<i64> { &self.chat_ids }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowChatMembersBuilder {
  inner: UserPrivacySettingRuleAllowChatMembers
}

impl RTDUserPrivacySettingRuleAllowChatMembersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleAllowChatMembers { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }

}

impl AsRef<UserPrivacySettingRuleAllowChatMembers> for UserPrivacySettingRuleAllowChatMembers {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowChatMembers { self }
}

impl AsRef<UserPrivacySettingRuleAllowChatMembers> for RTDUserPrivacySettingRuleAllowChatMembersBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleAllowChatMembers { &self.inner }
}







/// A rule to restrict all users from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for UserPrivacySettingRuleRestrictAll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleRestrictAll" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictAll {}



impl UserPrivacySettingRuleRestrictAll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictAllBuilder {
    let mut inner = UserPrivacySettingRuleRestrictAll::default();
    inner.td_name = "userPrivacySettingRuleRestrictAll".to_string();
    RTDUserPrivacySettingRuleRestrictAllBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictAllBuilder {
  inner: UserPrivacySettingRuleRestrictAll
}

impl RTDUserPrivacySettingRuleRestrictAllBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictAll { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<UserPrivacySettingRuleRestrictAll> for UserPrivacySettingRuleRestrictAll {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictAll { self }
}

impl AsRef<UserPrivacySettingRuleRestrictAll> for RTDUserPrivacySettingRuleRestrictAllBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictAll { &self.inner }
}







/// A rule to restrict all contacts of a user from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictContacts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for UserPrivacySettingRuleRestrictContacts {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleRestrictContacts" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictContacts {}



impl UserPrivacySettingRuleRestrictContacts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictContactsBuilder {
    let mut inner = UserPrivacySettingRuleRestrictContacts::default();
    inner.td_name = "userPrivacySettingRuleRestrictContacts".to_string();
    RTDUserPrivacySettingRuleRestrictContactsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictContactsBuilder {
  inner: UserPrivacySettingRuleRestrictContacts
}

impl RTDUserPrivacySettingRuleRestrictContactsBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictContacts { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<UserPrivacySettingRuleRestrictContacts> for UserPrivacySettingRuleRestrictContacts {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictContacts { self }
}

impl AsRef<UserPrivacySettingRuleRestrictContacts> for RTDUserPrivacySettingRuleRestrictContactsBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictContacts { &self.inner }
}







/// A rule to restrict all specified users from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The user identifiers, total number of users in all rules must not exceed 1000
  user_ids: Vec<i64>,
  
}

impl RObject for UserPrivacySettingRuleRestrictUsers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleRestrictUsers" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictUsers {}



impl UserPrivacySettingRuleRestrictUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictUsersBuilder {
    let mut inner = UserPrivacySettingRuleRestrictUsers::default();
    inner.td_name = "userPrivacySettingRuleRestrictUsers".to_string();
    RTDUserPrivacySettingRuleRestrictUsersBuilder { inner }
  }

  pub fn user_ids(&self) -> &Vec<i64> { &self.user_ids }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictUsersBuilder {
  inner: UserPrivacySettingRuleRestrictUsers
}

impl RTDUserPrivacySettingRuleRestrictUsersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictUsers { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

}

impl AsRef<UserPrivacySettingRuleRestrictUsers> for UserPrivacySettingRuleRestrictUsers {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictUsers { self }
}

impl AsRef<UserPrivacySettingRuleRestrictUsers> for RTDUserPrivacySettingRuleRestrictUsersBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictUsers { &self.inner }
}







/// A rule to restrict all members of specified basic groups and supergroups from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictChatMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The chat identifiers, total number of chats in all rules must not exceed 20
  chat_ids: Vec<i64>,
  
}

impl RObject for UserPrivacySettingRuleRestrictChatMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRuleRestrictChatMembers" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictChatMembers {}



impl UserPrivacySettingRuleRestrictChatMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
    let mut inner = UserPrivacySettingRuleRestrictChatMembers::default();
    inner.td_name = "userPrivacySettingRuleRestrictChatMembers".to_string();
    RTDUserPrivacySettingRuleRestrictChatMembersBuilder { inner }
  }

  pub fn chat_ids(&self) -> &Vec<i64> { &self.chat_ids }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
  inner: UserPrivacySettingRuleRestrictChatMembers
}

impl RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
  pub fn build(&self) -> UserPrivacySettingRuleRestrictChatMembers { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }

}

impl AsRef<UserPrivacySettingRuleRestrictChatMembers> for UserPrivacySettingRuleRestrictChatMembers {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictChatMembers { self }
}

impl AsRef<UserPrivacySettingRuleRestrictChatMembers> for RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRuleRestrictChatMembers { &self.inner }
}



