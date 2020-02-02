
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about the sending state of the message
pub trait TDMessageSendingState: Debug + RObject {}

/// Contains information about the sending state of the message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSendingState {
  #[doc(hidden)] _Default(()),
  /// The message is being sent now, but has not yet been delivered to the server
  Pending(MessageSendingStatePending),
  /// The message failed to be sent
  Failed(MessageSendingStateFailed),

}

impl Default for MessageSendingState {
  fn default() -> Self { MessageSendingState::_Default(()) }
}

impl<'de> Deserialize<'de> for MessageSendingState {
  fn deserialize<D>(deserializer: D) -> Result<MessageSendingState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageSendingState,
      (messageSendingStatePending, Pending);
      (messageSendingStateFailed, Failed);

    )(deserializer)
  }
}

impl RObject for MessageSendingState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      MessageSendingState::Pending(t) => t.td_name(),
      MessageSendingState::Failed(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl MessageSendingState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let MessageSendingState::_Default(_) = self { true } else { false } }

  pub fn is_pending(&self) -> bool { if let MessageSendingState::Pending(_) = self { true } else { false } }
  pub fn is_failed(&self) -> bool { if let MessageSendingState::Failed(_) = self { true } else { false } }

  pub fn on_pending<F: FnOnce(&MessageSendingStatePending)>(&self, fnc: F) -> &Self { if let MessageSendingState::Pending(t) = self { fnc(t) }; self }
  pub fn on_failed<F: FnOnce(&MessageSendingStateFailed)>(&self, fnc: F) -> &Self { if let MessageSendingState::Failed(t) = self { fnc(t) }; self }

  pub fn as_pending(&self) -> Option<&MessageSendingStatePending> { if let MessageSendingState::Pending(t) = self { return Some(t) } None }
  pub fn as_failed(&self) -> Option<&MessageSendingStateFailed> { if let MessageSendingState::Failed(t) = self { return Some(t) } None }



  pub fn pending<T: AsRef<MessageSendingStatePending>>(t: T) -> Self { MessageSendingState::Pending(t.as_ref().clone()) }

  pub fn failed<T: AsRef<MessageSendingStateFailed>>(t: T) -> Self { MessageSendingState::Failed(t.as_ref().clone()) }

}

impl AsRef<MessageSendingState> for MessageSendingState {
  fn as_ref(&self) -> &MessageSendingState { self }
}







/// The message is being sent now, but has not yet been delivered to the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendingStatePending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for MessageSendingStatePending {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSendingStatePending" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageSendingState for MessageSendingStatePending {}



impl MessageSendingStatePending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSendingStatePendingBuilder {
    let mut inner = MessageSendingStatePending::default();
    inner.td_name = "messageSendingStatePending".to_string();
    RTDMessageSendingStatePendingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageSendingStatePendingBuilder {
  inner: MessageSendingStatePending
}

impl RTDMessageSendingStatePendingBuilder {
  pub fn build(&self) -> MessageSendingStatePending { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<MessageSendingStatePending> for MessageSendingStatePending {
  fn as_ref(&self) -> &MessageSendingStatePending { self }
}

impl AsRef<MessageSendingStatePending> for RTDMessageSendingStatePendingBuilder {
  fn as_ref(&self) -> &MessageSendingStatePending { &self.inner }
}







/// The message failed to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendingStateFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// An error code; 0 if unknown
  error_code: i64,
  /// Error message
  error_message: String,
  /// True, if the message can be re-sent
  can_retry: bool,
  /// Time left before the message can be re-sent, in seconds. No update is sent when this field changes
  retry_after: f32,
  
}

impl RObject for MessageSendingStateFailed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSendingStateFailed" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageSendingState for MessageSendingStateFailed {}



impl MessageSendingStateFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSendingStateFailedBuilder {
    let mut inner = MessageSendingStateFailed::default();
    inner.td_name = "messageSendingStateFailed".to_string();
    RTDMessageSendingStateFailedBuilder { inner }
  }

  pub fn error_code(&self) -> i64 { self.error_code }

  pub fn error_message(&self) -> &String { &self.error_message }

  pub fn can_retry(&self) -> bool { self.can_retry }

  pub fn retry_after(&self) -> f32 { self.retry_after }

}

#[doc(hidden)]
pub struct RTDMessageSendingStateFailedBuilder {
  inner: MessageSendingStateFailed
}

impl RTDMessageSendingStateFailedBuilder {
  pub fn build(&self) -> MessageSendingStateFailed { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn error_code(&mut self, error_code: i64) -> &mut Self {
    self.inner.error_code = error_code;
    self
  }

   
  pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
    self.inner.error_message = error_message.as_ref().to_string();
    self
  }

   
  pub fn can_retry(&mut self, can_retry: bool) -> &mut Self {
    self.inner.can_retry = can_retry;
    self
  }

   
  pub fn retry_after(&mut self, retry_after: f32) -> &mut Self {
    self.inner.retry_after = retry_after;
    self
  }

}

impl AsRef<MessageSendingStateFailed> for MessageSendingStateFailed {
  fn as_ref(&self) -> &MessageSendingStateFailed { self }
}

impl AsRef<MessageSendingStateFailed> for RTDMessageSendingStateFailedBuilder {
  fn as_ref(&self) -> &MessageSendingStateFailed { &self.inner }
}



