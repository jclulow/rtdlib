
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl
pub trait TDLoginUrlInfo: Debug + RObject {}

/// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LoginUrlInfo {
  #[doc(hidden)] _Default(()),
  /// An HTTP url needs to be open
  Open(LoginUrlInfoOpen),
  /// An authorization confirmation dialog needs to be shown to the user
  RequestConfirmation(LoginUrlInfoRequestConfirmation),
  /// Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
  GetLoginUrlInfo(GetLoginUrlInfo),

}

impl Default for LoginUrlInfo {
  fn default() -> Self { LoginUrlInfo::_Default(()) }
}

impl<'de> Deserialize<'de> for LoginUrlInfo {
  fn deserialize<D>(deserializer: D) -> Result<LoginUrlInfo, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      LoginUrlInfo,
      (loginUrlInfoOpen, Open);
      (loginUrlInfoRequestConfirmation, RequestConfirmation);
      (getLoginUrlInfo, GetLoginUrlInfo);

    )(deserializer)
  }
}

impl RObject for LoginUrlInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      LoginUrlInfo::Open(t) => t.td_name(),
      LoginUrlInfo::RequestConfirmation(t) => t.td_name(),
      LoginUrlInfo::GetLoginUrlInfo(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl LoginUrlInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let LoginUrlInfo::_Default(_) = self { true } else { false } }

  pub fn is_open(&self) -> bool { if let LoginUrlInfo::Open(_) = self { true } else { false } }
  pub fn is_request_confirmation(&self) -> bool { if let LoginUrlInfo::RequestConfirmation(_) = self { true } else { false } }
  pub fn is_get_login_url_info(&self) -> bool { if let LoginUrlInfo::GetLoginUrlInfo(_) = self { true } else { false } }

  pub fn on_open<F: FnOnce(&LoginUrlInfoOpen)>(&self, fnc: F) -> &Self { if let LoginUrlInfo::Open(t) = self { fnc(t) }; self }
  pub fn on_request_confirmation<F: FnOnce(&LoginUrlInfoRequestConfirmation)>(&self, fnc: F) -> &Self { if let LoginUrlInfo::RequestConfirmation(t) = self { fnc(t) }; self }
  pub fn on_get_login_url_info<F: FnOnce(&GetLoginUrlInfo)>(&self, fnc: F) -> &Self { if let LoginUrlInfo::GetLoginUrlInfo(t) = self { fnc(t) }; self }

  pub fn as_open(&self) -> Option<&LoginUrlInfoOpen> { if let LoginUrlInfo::Open(t) = self { return Some(t) } None }
  pub fn as_request_confirmation(&self) -> Option<&LoginUrlInfoRequestConfirmation> { if let LoginUrlInfo::RequestConfirmation(t) = self { return Some(t) } None }
  pub fn as_get_login_url_info(&self) -> Option<&GetLoginUrlInfo> { if let LoginUrlInfo::GetLoginUrlInfo(t) = self { return Some(t) } None }



  pub fn open<T: AsRef<LoginUrlInfoOpen>>(t: T) -> Self { LoginUrlInfo::Open(t.as_ref().clone()) }

  pub fn request_confirmation<T: AsRef<LoginUrlInfoRequestConfirmation>>(t: T) -> Self { LoginUrlInfo::RequestConfirmation(t.as_ref().clone()) }

  pub fn get_login_url_info<T: AsRef<GetLoginUrlInfo>>(t: T) -> Self { LoginUrlInfo::GetLoginUrlInfo(t.as_ref().clone()) }

}

impl AsRef<LoginUrlInfo> for LoginUrlInfo {
  fn as_ref(&self) -> &LoginUrlInfo { self }
}







/// An HTTP url needs to be open
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginUrlInfoOpen {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The URL to open
  url: String,
  /// True, if there is no need to show an ordinary open URL confirm
  skip_confirm: bool,
  
}

impl RObject for LoginUrlInfoOpen {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "loginUrlInfoOpen" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLoginUrlInfo for LoginUrlInfoOpen {}



impl LoginUrlInfoOpen {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLoginUrlInfoOpenBuilder {
    let mut inner = LoginUrlInfoOpen::default();
    inner.td_name = "loginUrlInfoOpen".to_string();
    RTDLoginUrlInfoOpenBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn skip_confirm(&self) -> bool { self.skip_confirm }

}

#[doc(hidden)]
pub struct RTDLoginUrlInfoOpenBuilder {
  inner: LoginUrlInfoOpen
}

impl RTDLoginUrlInfoOpenBuilder {
  pub fn build(&self) -> LoginUrlInfoOpen { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn skip_confirm(&mut self, skip_confirm: bool) -> &mut Self {
    self.inner.skip_confirm = skip_confirm;
    self
  }

}

impl AsRef<LoginUrlInfoOpen> for LoginUrlInfoOpen {
  fn as_ref(&self) -> &LoginUrlInfoOpen { self }
}

impl AsRef<LoginUrlInfoOpen> for RTDLoginUrlInfoOpenBuilder {
  fn as_ref(&self) -> &LoginUrlInfoOpen { &self.inner }
}







/// An authorization confirmation dialog needs to be shown to the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoginUrlInfoRequestConfirmation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// An HTTP URL to be opened
  url: String,
  /// A domain of the URL
  domain: String,
  /// User identifier of a bot linked with the website
  bot_user_id: i64,
  /// True, if the user needs to be requested to give the permission to the bot to send them messages
  request_write_access: bool,
  
}

impl RObject for LoginUrlInfoRequestConfirmation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "loginUrlInfoRequestConfirmation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLoginUrlInfo for LoginUrlInfoRequestConfirmation {}



impl LoginUrlInfoRequestConfirmation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLoginUrlInfoRequestConfirmationBuilder {
    let mut inner = LoginUrlInfoRequestConfirmation::default();
    inner.td_name = "loginUrlInfoRequestConfirmation".to_string();
    RTDLoginUrlInfoRequestConfirmationBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn domain(&self) -> &String { &self.domain }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn request_write_access(&self) -> bool { self.request_write_access }

}

#[doc(hidden)]
pub struct RTDLoginUrlInfoRequestConfirmationBuilder {
  inner: LoginUrlInfoRequestConfirmation
}

impl RTDLoginUrlInfoRequestConfirmationBuilder {
  pub fn build(&self) -> LoginUrlInfoRequestConfirmation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn domain<T: AsRef<str>>(&mut self, domain: T) -> &mut Self {
    self.inner.domain = domain.as_ref().to_string();
    self
  }

   
  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

   
  pub fn request_write_access(&mut self, request_write_access: bool) -> &mut Self {
    self.inner.request_write_access = request_write_access;
    self
  }

}

impl AsRef<LoginUrlInfoRequestConfirmation> for LoginUrlInfoRequestConfirmation {
  fn as_ref(&self) -> &LoginUrlInfoRequestConfirmation { self }
}

impl AsRef<LoginUrlInfoRequestConfirmation> for RTDLoginUrlInfoRequestConfirmationBuilder {
  fn as_ref(&self) -> &LoginUrlInfoRequestConfirmation { &self.inner }
}



