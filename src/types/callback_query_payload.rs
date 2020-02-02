
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a payload of a callback query
pub trait TDCallbackQueryPayload: Debug + RObject {}

/// Represents a payload of a callback query
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallbackQueryPayload {
  #[doc(hidden)] _Default(()),
  /// The payload from a general callback button
  Data(CallbackQueryPayloadData),
  /// The payload from a game callback button
  Game(CallbackQueryPayloadGame),

}

impl Default for CallbackQueryPayload {
  fn default() -> Self { CallbackQueryPayload::_Default(()) }
}

impl<'de> Deserialize<'de> for CallbackQueryPayload {
  fn deserialize<D>(deserializer: D) -> Result<CallbackQueryPayload, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CallbackQueryPayload,
      (callbackQueryPayloadData, Data);
      (callbackQueryPayloadGame, Game);

    )(deserializer)
  }
}

impl RObject for CallbackQueryPayload {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      CallbackQueryPayload::Data(t) => t.td_name(),
      CallbackQueryPayload::Game(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl CallbackQueryPayload {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let CallbackQueryPayload::_Default(_) = self { true } else { false } }

  pub fn is_data(&self) -> bool { if let CallbackQueryPayload::Data(_) = self { true } else { false } }
  pub fn is_game(&self) -> bool { if let CallbackQueryPayload::Game(_) = self { true } else { false } }

  pub fn on_data<F: FnOnce(&CallbackQueryPayloadData)>(&self, fnc: F) -> &Self { if let CallbackQueryPayload::Data(t) = self { fnc(t) }; self }
  pub fn on_game<F: FnOnce(&CallbackQueryPayloadGame)>(&self, fnc: F) -> &Self { if let CallbackQueryPayload::Game(t) = self { fnc(t) }; self }

  pub fn as_data(&self) -> Option<&CallbackQueryPayloadData> { if let CallbackQueryPayload::Data(t) = self { return Some(t) } None }
  pub fn as_game(&self) -> Option<&CallbackQueryPayloadGame> { if let CallbackQueryPayload::Game(t) = self { return Some(t) } None }



  pub fn data<T: AsRef<CallbackQueryPayloadData>>(t: T) -> Self { CallbackQueryPayload::Data(t.as_ref().clone()) }

  pub fn game<T: AsRef<CallbackQueryPayloadGame>>(t: T) -> Self { CallbackQueryPayload::Game(t.as_ref().clone()) }

}

impl AsRef<CallbackQueryPayload> for CallbackQueryPayload {
  fn as_ref(&self) -> &CallbackQueryPayload { self }
}







/// The payload from a general callback button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackQueryPayloadData {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Data that was attached to the callback button
  data: String,
  
}

impl RObject for CallbackQueryPayloadData {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callbackQueryPayloadData" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallbackQueryPayload for CallbackQueryPayloadData {}



impl CallbackQueryPayloadData {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallbackQueryPayloadDataBuilder {
    let mut inner = CallbackQueryPayloadData::default();
    inner.td_name = "callbackQueryPayloadData".to_string();
    RTDCallbackQueryPayloadDataBuilder { inner }
  }

  pub fn data(&self) -> &String { &self.data }

}

#[doc(hidden)]
pub struct RTDCallbackQueryPayloadDataBuilder {
  inner: CallbackQueryPayloadData
}

impl RTDCallbackQueryPayloadDataBuilder {
  pub fn build(&self) -> CallbackQueryPayloadData { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
    self.inner.data = data.as_ref().to_string();
    self
  }

}

impl AsRef<CallbackQueryPayloadData> for CallbackQueryPayloadData {
  fn as_ref(&self) -> &CallbackQueryPayloadData { self }
}

impl AsRef<CallbackQueryPayloadData> for RTDCallbackQueryPayloadDataBuilder {
  fn as_ref(&self) -> &CallbackQueryPayloadData { &self.inner }
}







/// The payload from a game callback button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackQueryPayloadGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// A short name of the game that was attached to the callback button
  game_short_name: String,
  
}

impl RObject for CallbackQueryPayloadGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callbackQueryPayloadGame" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallbackQueryPayload for CallbackQueryPayloadGame {}



impl CallbackQueryPayloadGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallbackQueryPayloadGameBuilder {
    let mut inner = CallbackQueryPayloadGame::default();
    inner.td_name = "callbackQueryPayloadGame".to_string();
    RTDCallbackQueryPayloadGameBuilder { inner }
  }

  pub fn game_short_name(&self) -> &String { &self.game_short_name }

}

#[doc(hidden)]
pub struct RTDCallbackQueryPayloadGameBuilder {
  inner: CallbackQueryPayloadGame
}

impl RTDCallbackQueryPayloadGameBuilder {
  pub fn build(&self) -> CallbackQueryPayloadGame { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
    self.inner.game_short_name = game_short_name.as_ref().to_string();
    self
  }

}

impl AsRef<CallbackQueryPayloadGame> for CallbackQueryPayloadGame {
  fn as_ref(&self) -> &CallbackQueryPayloadGame { self }
}

impl AsRef<CallbackQueryPayloadGame> for RTDCallbackQueryPayloadGameBuilder {
  fn as_ref(&self) -> &CallbackQueryPayloadGame { &self.inner }
}



