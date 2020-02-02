
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the last time the user was online
pub trait TDUserStatus: Debug + RObject {}

/// Describes the last time the user was online
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserStatus {
  #[doc(hidden)] _Default(()),
  /// The user status was never changed
  Empty(UserStatusEmpty),
  /// The user is online
  Online(UserStatusOnline),
  /// The user is offline
  Offline(UserStatusOffline),
  /// The user was online recently
  Recently(UserStatusRecently),
  /// The user is offline, but was online last week
  LastWeek(UserStatusLastWeek),
  /// The user is offline, but was online last month
  LastMonth(UserStatusLastMonth),

}

impl Default for UserStatus {
  fn default() -> Self { UserStatus::_Default(()) }
}

impl<'de> Deserialize<'de> for UserStatus {
  fn deserialize<D>(deserializer: D) -> Result<UserStatus, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UserStatus,
      (userStatusEmpty, Empty);
      (userStatusOnline, Online);
      (userStatusOffline, Offline);
      (userStatusRecently, Recently);
      (userStatusLastWeek, LastWeek);
      (userStatusLastMonth, LastMonth);

    )(deserializer)
  }
}

impl RObject for UserStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      UserStatus::Empty(t) => t.td_name(),
      UserStatus::Online(t) => t.td_name(),
      UserStatus::Offline(t) => t.td_name(),
      UserStatus::Recently(t) => t.td_name(),
      UserStatus::LastWeek(t) => t.td_name(),
      UserStatus::LastMonth(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl UserStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let UserStatus::_Default(_) = self { true } else { false } }

  pub fn is_empty(&self) -> bool { if let UserStatus::Empty(_) = self { true } else { false } }
  pub fn is_online(&self) -> bool { if let UserStatus::Online(_) = self { true } else { false } }
  pub fn is_offline(&self) -> bool { if let UserStatus::Offline(_) = self { true } else { false } }
  pub fn is_recently(&self) -> bool { if let UserStatus::Recently(_) = self { true } else { false } }
  pub fn is_last_week(&self) -> bool { if let UserStatus::LastWeek(_) = self { true } else { false } }
  pub fn is_last_month(&self) -> bool { if let UserStatus::LastMonth(_) = self { true } else { false } }

  pub fn on_empty<F: FnOnce(&UserStatusEmpty)>(&self, fnc: F) -> &Self { if let UserStatus::Empty(t) = self { fnc(t) }; self }
  pub fn on_online<F: FnOnce(&UserStatusOnline)>(&self, fnc: F) -> &Self { if let UserStatus::Online(t) = self { fnc(t) }; self }
  pub fn on_offline<F: FnOnce(&UserStatusOffline)>(&self, fnc: F) -> &Self { if let UserStatus::Offline(t) = self { fnc(t) }; self }
  pub fn on_recently<F: FnOnce(&UserStatusRecently)>(&self, fnc: F) -> &Self { if let UserStatus::Recently(t) = self { fnc(t) }; self }
  pub fn on_last_week<F: FnOnce(&UserStatusLastWeek)>(&self, fnc: F) -> &Self { if let UserStatus::LastWeek(t) = self { fnc(t) }; self }
  pub fn on_last_month<F: FnOnce(&UserStatusLastMonth)>(&self, fnc: F) -> &Self { if let UserStatus::LastMonth(t) = self { fnc(t) }; self }

  pub fn as_empty(&self) -> Option<&UserStatusEmpty> { if let UserStatus::Empty(t) = self { return Some(t) } None }
  pub fn as_online(&self) -> Option<&UserStatusOnline> { if let UserStatus::Online(t) = self { return Some(t) } None }
  pub fn as_offline(&self) -> Option<&UserStatusOffline> { if let UserStatus::Offline(t) = self { return Some(t) } None }
  pub fn as_recently(&self) -> Option<&UserStatusRecently> { if let UserStatus::Recently(t) = self { return Some(t) } None }
  pub fn as_last_week(&self) -> Option<&UserStatusLastWeek> { if let UserStatus::LastWeek(t) = self { return Some(t) } None }
  pub fn as_last_month(&self) -> Option<&UserStatusLastMonth> { if let UserStatus::LastMonth(t) = self { return Some(t) } None }



  pub fn empty<T: AsRef<UserStatusEmpty>>(t: T) -> Self { UserStatus::Empty(t.as_ref().clone()) }

  pub fn online<T: AsRef<UserStatusOnline>>(t: T) -> Self { UserStatus::Online(t.as_ref().clone()) }

  pub fn offline<T: AsRef<UserStatusOffline>>(t: T) -> Self { UserStatus::Offline(t.as_ref().clone()) }

  pub fn recently<T: AsRef<UserStatusRecently>>(t: T) -> Self { UserStatus::Recently(t.as_ref().clone()) }

  pub fn last_week<T: AsRef<UserStatusLastWeek>>(t: T) -> Self { UserStatus::LastWeek(t.as_ref().clone()) }

  pub fn last_month<T: AsRef<UserStatusLastMonth>>(t: T) -> Self { UserStatus::LastMonth(t.as_ref().clone()) }

}

impl AsRef<UserStatus> for UserStatus {
  fn as_ref(&self) -> &UserStatus { self }
}







/// The user status was never changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for UserStatusEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusEmpty" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserStatus for UserStatusEmpty {}



impl UserStatusEmpty {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserStatusEmptyBuilder {
    let mut inner = UserStatusEmpty::default();
    inner.td_name = "userStatusEmpty".to_string();
    RTDUserStatusEmptyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserStatusEmptyBuilder {
  inner: UserStatusEmpty
}

impl RTDUserStatusEmptyBuilder {
  pub fn build(&self) -> UserStatusEmpty { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<UserStatusEmpty> for UserStatusEmpty {
  fn as_ref(&self) -> &UserStatusEmpty { self }
}

impl AsRef<UserStatusEmpty> for RTDUserStatusEmptyBuilder {
  fn as_ref(&self) -> &UserStatusEmpty { &self.inner }
}







/// The user is online
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusOnline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Point in time (Unix timestamp) when the user's online status will expire
  expires: i64,
  
}

impl RObject for UserStatusOnline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusOnline" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserStatus for UserStatusOnline {}



impl UserStatusOnline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserStatusOnlineBuilder {
    let mut inner = UserStatusOnline::default();
    inner.td_name = "userStatusOnline".to_string();
    RTDUserStatusOnlineBuilder { inner }
  }

  pub fn expires(&self) -> i64 { self.expires }

}

#[doc(hidden)]
pub struct RTDUserStatusOnlineBuilder {
  inner: UserStatusOnline
}

impl RTDUserStatusOnlineBuilder {
  pub fn build(&self) -> UserStatusOnline { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn expires(&mut self, expires: i64) -> &mut Self {
    self.inner.expires = expires;
    self
  }

}

impl AsRef<UserStatusOnline> for UserStatusOnline {
  fn as_ref(&self) -> &UserStatusOnline { self }
}

impl AsRef<UserStatusOnline> for RTDUserStatusOnlineBuilder {
  fn as_ref(&self) -> &UserStatusOnline { &self.inner }
}







/// The user is offline
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusOffline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Point in time (Unix timestamp) when the user was last online
  was_online: i64,
  
}

impl RObject for UserStatusOffline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusOffline" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserStatus for UserStatusOffline {}



impl UserStatusOffline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserStatusOfflineBuilder {
    let mut inner = UserStatusOffline::default();
    inner.td_name = "userStatusOffline".to_string();
    RTDUserStatusOfflineBuilder { inner }
  }

  pub fn was_online(&self) -> i64 { self.was_online }

}

#[doc(hidden)]
pub struct RTDUserStatusOfflineBuilder {
  inner: UserStatusOffline
}

impl RTDUserStatusOfflineBuilder {
  pub fn build(&self) -> UserStatusOffline { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn was_online(&mut self, was_online: i64) -> &mut Self {
    self.inner.was_online = was_online;
    self
  }

}

impl AsRef<UserStatusOffline> for UserStatusOffline {
  fn as_ref(&self) -> &UserStatusOffline { self }
}

impl AsRef<UserStatusOffline> for RTDUserStatusOfflineBuilder {
  fn as_ref(&self) -> &UserStatusOffline { &self.inner }
}







/// The user was online recently
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusRecently {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for UserStatusRecently {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusRecently" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserStatus for UserStatusRecently {}



impl UserStatusRecently {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserStatusRecentlyBuilder {
    let mut inner = UserStatusRecently::default();
    inner.td_name = "userStatusRecently".to_string();
    RTDUserStatusRecentlyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserStatusRecentlyBuilder {
  inner: UserStatusRecently
}

impl RTDUserStatusRecentlyBuilder {
  pub fn build(&self) -> UserStatusRecently { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<UserStatusRecently> for UserStatusRecently {
  fn as_ref(&self) -> &UserStatusRecently { self }
}

impl AsRef<UserStatusRecently> for RTDUserStatusRecentlyBuilder {
  fn as_ref(&self) -> &UserStatusRecently { &self.inner }
}







/// The user is offline, but was online last week
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusLastWeek {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for UserStatusLastWeek {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusLastWeek" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserStatus for UserStatusLastWeek {}



impl UserStatusLastWeek {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserStatusLastWeekBuilder {
    let mut inner = UserStatusLastWeek::default();
    inner.td_name = "userStatusLastWeek".to_string();
    RTDUserStatusLastWeekBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserStatusLastWeekBuilder {
  inner: UserStatusLastWeek
}

impl RTDUserStatusLastWeekBuilder {
  pub fn build(&self) -> UserStatusLastWeek { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<UserStatusLastWeek> for UserStatusLastWeek {
  fn as_ref(&self) -> &UserStatusLastWeek { self }
}

impl AsRef<UserStatusLastWeek> for RTDUserStatusLastWeekBuilder {
  fn as_ref(&self) -> &UserStatusLastWeek { &self.inner }
}







/// The user is offline, but was online last month
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusLastMonth {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for UserStatusLastMonth {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userStatusLastMonth" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserStatus for UserStatusLastMonth {}



impl UserStatusLastMonth {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserStatusLastMonthBuilder {
    let mut inner = UserStatusLastMonth::default();
    inner.td_name = "userStatusLastMonth".to_string();
    RTDUserStatusLastMonthBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserStatusLastMonthBuilder {
  inner: UserStatusLastMonth
}

impl RTDUserStatusLastMonthBuilder {
  pub fn build(&self) -> UserStatusLastMonth { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<UserStatusLastMonth> for UserStatusLastMonth {
  fn as_ref(&self) -> &UserStatusLastMonth { self }
}

impl AsRef<UserStatusLastMonth> for RTDUserStatusLastMonthBuilder {
  fn as_ref(&self) -> &UserStatusLastMonth { &self.inner }
}



