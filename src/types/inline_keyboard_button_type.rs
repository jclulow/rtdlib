
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the type of an inline keyboard button
pub trait TDInlineKeyboardButtonType: Debug + RObject {}

/// Describes the type of an inline keyboard button
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InlineKeyboardButtonType {
  #[doc(hidden)] _Default(()),
  /// A button that opens a specified URL
  Url(InlineKeyboardButtonTypeUrl),
  /// A button that opens a specified URL and automatically logs in in current user if they allowed to do that
  LoginUrl(InlineKeyboardButtonTypeLoginUrl),
  /// A button that sends a special callback query to a bot
  Callback(InlineKeyboardButtonTypeCallback),
  /// A button with a game that sends a special callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
  CallbackGame(InlineKeyboardButtonTypeCallbackGame),
  /// A button that forces an inline query to the bot to be inserted in the input field
  SwitchInline(InlineKeyboardButtonTypeSwitchInline),
  /// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
  Buy(InlineKeyboardButtonTypeBuy),

}

impl Default for InlineKeyboardButtonType {
  fn default() -> Self { InlineKeyboardButtonType::_Default(()) }
}

impl<'de> Deserialize<'de> for InlineKeyboardButtonType {
  fn deserialize<D>(deserializer: D) -> Result<InlineKeyboardButtonType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InlineKeyboardButtonType,
      (inlineKeyboardButtonTypeUrl, Url);
      (inlineKeyboardButtonTypeLoginUrl, LoginUrl);
      (inlineKeyboardButtonTypeCallback, Callback);
      (inlineKeyboardButtonTypeCallbackGame, CallbackGame);
      (inlineKeyboardButtonTypeSwitchInline, SwitchInline);
      (inlineKeyboardButtonTypeBuy, Buy);

    )(deserializer)
  }
}

impl RObject for InlineKeyboardButtonType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InlineKeyboardButtonType::Url(t) => t.td_name(),
      InlineKeyboardButtonType::LoginUrl(t) => t.td_name(),
      InlineKeyboardButtonType::Callback(t) => t.td_name(),
      InlineKeyboardButtonType::CallbackGame(t) => t.td_name(),
      InlineKeyboardButtonType::SwitchInline(t) => t.td_name(),
      InlineKeyboardButtonType::Buy(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InlineKeyboardButtonType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InlineKeyboardButtonType::_Default(_) = self { true } else { false } }

  pub fn is_url(&self) -> bool { if let InlineKeyboardButtonType::Url(_) = self { true } else { false } }
  pub fn is_login_url(&self) -> bool { if let InlineKeyboardButtonType::LoginUrl(_) = self { true } else { false } }
  pub fn is_callback(&self) -> bool { if let InlineKeyboardButtonType::Callback(_) = self { true } else { false } }
  pub fn is_callback_game(&self) -> bool { if let InlineKeyboardButtonType::CallbackGame(_) = self { true } else { false } }
  pub fn is_switch_inline(&self) -> bool { if let InlineKeyboardButtonType::SwitchInline(_) = self { true } else { false } }
  pub fn is_buy(&self) -> bool { if let InlineKeyboardButtonType::Buy(_) = self { true } else { false } }

  pub fn on_url<F: FnOnce(&InlineKeyboardButtonTypeUrl)>(&self, fnc: F) -> &Self { if let InlineKeyboardButtonType::Url(t) = self { fnc(t) }; self }
  pub fn on_login_url<F: FnOnce(&InlineKeyboardButtonTypeLoginUrl)>(&self, fnc: F) -> &Self { if let InlineKeyboardButtonType::LoginUrl(t) = self { fnc(t) }; self }
  pub fn on_callback<F: FnOnce(&InlineKeyboardButtonTypeCallback)>(&self, fnc: F) -> &Self { if let InlineKeyboardButtonType::Callback(t) = self { fnc(t) }; self }
  pub fn on_callback_game<F: FnOnce(&InlineKeyboardButtonTypeCallbackGame)>(&self, fnc: F) -> &Self { if let InlineKeyboardButtonType::CallbackGame(t) = self { fnc(t) }; self }
  pub fn on_switch_inline<F: FnOnce(&InlineKeyboardButtonTypeSwitchInline)>(&self, fnc: F) -> &Self { if let InlineKeyboardButtonType::SwitchInline(t) = self { fnc(t) }; self }
  pub fn on_buy<F: FnOnce(&InlineKeyboardButtonTypeBuy)>(&self, fnc: F) -> &Self { if let InlineKeyboardButtonType::Buy(t) = self { fnc(t) }; self }

  pub fn as_url(&self) -> Option<&InlineKeyboardButtonTypeUrl> { if let InlineKeyboardButtonType::Url(t) = self { return Some(t) } None }
  pub fn as_login_url(&self) -> Option<&InlineKeyboardButtonTypeLoginUrl> { if let InlineKeyboardButtonType::LoginUrl(t) = self { return Some(t) } None }
  pub fn as_callback(&self) -> Option<&InlineKeyboardButtonTypeCallback> { if let InlineKeyboardButtonType::Callback(t) = self { return Some(t) } None }
  pub fn as_callback_game(&self) -> Option<&InlineKeyboardButtonTypeCallbackGame> { if let InlineKeyboardButtonType::CallbackGame(t) = self { return Some(t) } None }
  pub fn as_switch_inline(&self) -> Option<&InlineKeyboardButtonTypeSwitchInline> { if let InlineKeyboardButtonType::SwitchInline(t) = self { return Some(t) } None }
  pub fn as_buy(&self) -> Option<&InlineKeyboardButtonTypeBuy> { if let InlineKeyboardButtonType::Buy(t) = self { return Some(t) } None }



  pub fn url<T: AsRef<InlineKeyboardButtonTypeUrl>>(t: T) -> Self { InlineKeyboardButtonType::Url(t.as_ref().clone()) }

  pub fn login_url<T: AsRef<InlineKeyboardButtonTypeLoginUrl>>(t: T) -> Self { InlineKeyboardButtonType::LoginUrl(t.as_ref().clone()) }

  pub fn callback<T: AsRef<InlineKeyboardButtonTypeCallback>>(t: T) -> Self { InlineKeyboardButtonType::Callback(t.as_ref().clone()) }

  pub fn callback_game<T: AsRef<InlineKeyboardButtonTypeCallbackGame>>(t: T) -> Self { InlineKeyboardButtonType::CallbackGame(t.as_ref().clone()) }

  pub fn switch_inline<T: AsRef<InlineKeyboardButtonTypeSwitchInline>>(t: T) -> Self { InlineKeyboardButtonType::SwitchInline(t.as_ref().clone()) }

  pub fn buy<T: AsRef<InlineKeyboardButtonTypeBuy>>(t: T) -> Self { InlineKeyboardButtonType::Buy(t.as_ref().clone()) }

}

impl AsRef<InlineKeyboardButtonType> for InlineKeyboardButtonType {
  fn as_ref(&self) -> &InlineKeyboardButtonType { self }
}







/// A button that opens a specified URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// HTTP or tg:// URL to open
  url: String,
  
}

impl RObject for InlineKeyboardButtonTypeUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeUrl" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeUrl {}



impl InlineKeyboardButtonTypeUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineKeyboardButtonTypeUrlBuilder {
    let mut inner = InlineKeyboardButtonTypeUrl::default();
    inner.td_name = "inlineKeyboardButtonTypeUrl".to_string();
    RTDInlineKeyboardButtonTypeUrlBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeUrlBuilder {
  inner: InlineKeyboardButtonTypeUrl
}

impl RTDInlineKeyboardButtonTypeUrlBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeUrl { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<InlineKeyboardButtonTypeUrl> for InlineKeyboardButtonTypeUrl {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeUrl { self }
}

impl AsRef<InlineKeyboardButtonTypeUrl> for RTDInlineKeyboardButtonTypeUrlBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeUrl { &self.inner }
}







/// A button that opens a specified URL and automatically logs in in current user if they allowed to do that
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeLoginUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// An HTTP URL to open
  url: String,
  /// Unique button identifier
  id: i64,
  /// If non-empty, new text of the button in forwarded messages
  forward_text: String,
  
}

impl RObject for InlineKeyboardButtonTypeLoginUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeLoginUrl" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeLoginUrl {}



impl InlineKeyboardButtonTypeLoginUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineKeyboardButtonTypeLoginUrlBuilder {
    let mut inner = InlineKeyboardButtonTypeLoginUrl::default();
    inner.td_name = "inlineKeyboardButtonTypeLoginUrl".to_string();
    RTDInlineKeyboardButtonTypeLoginUrlBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

  pub fn id(&self) -> i64 { self.id }

  pub fn forward_text(&self) -> &String { &self.forward_text }

}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeLoginUrlBuilder {
  inner: InlineKeyboardButtonTypeLoginUrl
}

impl RTDInlineKeyboardButtonTypeLoginUrlBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeLoginUrl { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn forward_text<T: AsRef<str>>(&mut self, forward_text: T) -> &mut Self {
    self.inner.forward_text = forward_text.as_ref().to_string();
    self
  }

}

impl AsRef<InlineKeyboardButtonTypeLoginUrl> for InlineKeyboardButtonTypeLoginUrl {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeLoginUrl { self }
}

impl AsRef<InlineKeyboardButtonTypeLoginUrl> for RTDInlineKeyboardButtonTypeLoginUrlBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeLoginUrl { &self.inner }
}







/// A button that sends a special callback query to a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallback {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Data to be sent to the bot via a callback query
  data: String,
  
}

impl RObject for InlineKeyboardButtonTypeCallback {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeCallback" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallback {}



impl InlineKeyboardButtonTypeCallback {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackBuilder {
    let mut inner = InlineKeyboardButtonTypeCallback::default();
    inner.td_name = "inlineKeyboardButtonTypeCallback".to_string();
    RTDInlineKeyboardButtonTypeCallbackBuilder { inner }
  }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackBuilder {
  inner: InlineKeyboardButtonTypeCallback
}

impl RTDInlineKeyboardButtonTypeCallbackBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeCallback { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<InlineKeyboardButtonTypeCallback> for InlineKeyboardButtonTypeCallback {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallback { self }
}

impl AsRef<InlineKeyboardButtonTypeCallback> for RTDInlineKeyboardButtonTypeCallbackBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallback { &self.inner }
}







/// A button with a game that sends a special callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageGame
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallbackGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for InlineKeyboardButtonTypeCallbackGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeCallbackGame" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackGame {}



impl InlineKeyboardButtonTypeCallbackGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineKeyboardButtonTypeCallbackGameBuilder {
    let mut inner = InlineKeyboardButtonTypeCallbackGame::default();
    inner.td_name = "inlineKeyboardButtonTypeCallbackGame".to_string();
    RTDInlineKeyboardButtonTypeCallbackGameBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeCallbackGameBuilder {
  inner: InlineKeyboardButtonTypeCallbackGame
}

impl RTDInlineKeyboardButtonTypeCallbackGameBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeCallbackGame { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<InlineKeyboardButtonTypeCallbackGame> for InlineKeyboardButtonTypeCallbackGame {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackGame { self }
}

impl AsRef<InlineKeyboardButtonTypeCallbackGame> for RTDInlineKeyboardButtonTypeCallbackGameBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeCallbackGame { &self.inner }
}







/// A button that forces an inline query to the bot to be inserted in the input field
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeSwitchInline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Inline query to be sent to the bot
  query: String,
  /// True, if the inline query should be sent from the current chat
  in_current_chat: bool,
  
}

impl RObject for InlineKeyboardButtonTypeSwitchInline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeSwitchInline" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeSwitchInline {}



impl InlineKeyboardButtonTypeSwitchInline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
    let mut inner = InlineKeyboardButtonTypeSwitchInline::default();
    inner.td_name = "inlineKeyboardButtonTypeSwitchInline".to_string();
    RTDInlineKeyboardButtonTypeSwitchInlineBuilder { inner }
  }

  pub fn query(&self) -> &String { &self.query }

  pub fn in_current_chat(&self) -> bool { self.in_current_chat }

}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
  inner: InlineKeyboardButtonTypeSwitchInline
}

impl RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeSwitchInline { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
    self.inner.query = query.as_ref().to_string();
    self
  }

   
  pub fn in_current_chat(&mut self, in_current_chat: bool) -> &mut Self {
    self.inner.in_current_chat = in_current_chat;
    self
  }

}

impl AsRef<InlineKeyboardButtonTypeSwitchInline> for InlineKeyboardButtonTypeSwitchInline {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeSwitchInline { self }
}

impl AsRef<InlineKeyboardButtonTypeSwitchInline> for RTDInlineKeyboardButtonTypeSwitchInlineBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeSwitchInline { &self.inner }
}







/// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type messageInvoice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeBuy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for InlineKeyboardButtonTypeBuy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeBuy" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineKeyboardButtonType for InlineKeyboardButtonTypeBuy {}



impl InlineKeyboardButtonTypeBuy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineKeyboardButtonTypeBuyBuilder {
    let mut inner = InlineKeyboardButtonTypeBuy::default();
    inner.td_name = "inlineKeyboardButtonTypeBuy".to_string();
    RTDInlineKeyboardButtonTypeBuyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonTypeBuyBuilder {
  inner: InlineKeyboardButtonTypeBuy
}

impl RTDInlineKeyboardButtonTypeBuyBuilder {
  pub fn build(&self) -> InlineKeyboardButtonTypeBuy { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<InlineKeyboardButtonTypeBuy> for InlineKeyboardButtonTypeBuy {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeBuy { self }
}

impl AsRef<InlineKeyboardButtonTypeBuy> for RTDInlineKeyboardButtonTypeBuyBuilder {
  fn as_ref(&self) -> &InlineKeyboardButtonTypeBuy { &self.inner }
}



