
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about the time when a scheduled message will be sent
pub trait TDMessageSchedulingState: Debug + RObject {}

/// Contains information about the time when a scheduled message will be sent
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageSchedulingState {
  #[doc(hidden)] _Default(()),
  /// The message will be sent at the specified date
  SendAtDate(MessageSchedulingStateSendAtDate),
  /// The message will be sent when the peer will be online. Applicable to private chats only and when the exact online status of the peer is known
  SendWhenOnline(MessageSchedulingStateSendWhenOnline),

}

impl Default for MessageSchedulingState {
  fn default() -> Self { MessageSchedulingState::_Default(()) }
}

impl<'de> Deserialize<'de> for MessageSchedulingState {
  fn deserialize<D>(deserializer: D) -> Result<MessageSchedulingState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageSchedulingState,
      (messageSchedulingStateSendAtDate, SendAtDate);
      (messageSchedulingStateSendWhenOnline, SendWhenOnline);

    )(deserializer)
  }
}

impl RObject for MessageSchedulingState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      MessageSchedulingState::SendAtDate(t) => t.td_name(),
      MessageSchedulingState::SendWhenOnline(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl MessageSchedulingState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let MessageSchedulingState::_Default(_) = self { true } else { false } }

  pub fn is_send_at_date(&self) -> bool { if let MessageSchedulingState::SendAtDate(_) = self { true } else { false } }
  pub fn is_send_when_online(&self) -> bool { if let MessageSchedulingState::SendWhenOnline(_) = self { true } else { false } }

  pub fn on_send_at_date<F: FnOnce(&MessageSchedulingStateSendAtDate)>(&self, fnc: F) -> &Self { if let MessageSchedulingState::SendAtDate(t) = self { fnc(t) }; self }
  pub fn on_send_when_online<F: FnOnce(&MessageSchedulingStateSendWhenOnline)>(&self, fnc: F) -> &Self { if let MessageSchedulingState::SendWhenOnline(t) = self { fnc(t) }; self }

  pub fn as_send_at_date(&self) -> Option<&MessageSchedulingStateSendAtDate> { if let MessageSchedulingState::SendAtDate(t) = self { return Some(t) } None }
  pub fn as_send_when_online(&self) -> Option<&MessageSchedulingStateSendWhenOnline> { if let MessageSchedulingState::SendWhenOnline(t) = self { return Some(t) } None }



  pub fn send_at_date<T: AsRef<MessageSchedulingStateSendAtDate>>(t: T) -> Self { MessageSchedulingState::SendAtDate(t.as_ref().clone()) }

  pub fn send_when_online<T: AsRef<MessageSchedulingStateSendWhenOnline>>(t: T) -> Self { MessageSchedulingState::SendWhenOnline(t.as_ref().clone()) }

}

impl AsRef<MessageSchedulingState> for MessageSchedulingState {
  fn as_ref(&self) -> &MessageSchedulingState { self }
}







/// The message will be sent at the specified date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSchedulingStateSendAtDate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Date the message will be sent. The date must be within 367 days in the future
  send_date: i64,
  
}

impl RObject for MessageSchedulingStateSendAtDate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSchedulingStateSendAtDate" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageSchedulingState for MessageSchedulingStateSendAtDate {}



impl MessageSchedulingStateSendAtDate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSchedulingStateSendAtDateBuilder {
    let mut inner = MessageSchedulingStateSendAtDate::default();
    inner.td_name = "messageSchedulingStateSendAtDate".to_string();
    RTDMessageSchedulingStateSendAtDateBuilder { inner }
  }

  pub fn send_date(&self) -> i64 { self.send_date }

}

#[doc(hidden)]
pub struct RTDMessageSchedulingStateSendAtDateBuilder {
  inner: MessageSchedulingStateSendAtDate
}

impl RTDMessageSchedulingStateSendAtDateBuilder {
  pub fn build(&self) -> MessageSchedulingStateSendAtDate { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn send_date(&mut self, send_date: i64) -> &mut Self {
    self.inner.send_date = send_date;
    self
  }

}

impl AsRef<MessageSchedulingStateSendAtDate> for MessageSchedulingStateSendAtDate {
  fn as_ref(&self) -> &MessageSchedulingStateSendAtDate { self }
}

impl AsRef<MessageSchedulingStateSendAtDate> for RTDMessageSchedulingStateSendAtDateBuilder {
  fn as_ref(&self) -> &MessageSchedulingStateSendAtDate { &self.inner }
}







/// The message will be sent when the peer will be online. Applicable to private chats only and when the exact online status of the peer is known
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSchedulingStateSendWhenOnline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageSchedulingStateSendWhenOnline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSchedulingStateSendWhenOnline" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageSchedulingState for MessageSchedulingStateSendWhenOnline {}



impl MessageSchedulingStateSendWhenOnline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSchedulingStateSendWhenOnlineBuilder {
    let mut inner = MessageSchedulingStateSendWhenOnline::default();
    inner.td_name = "messageSchedulingStateSendWhenOnline".to_string();
    RTDMessageSchedulingStateSendWhenOnlineBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageSchedulingStateSendWhenOnlineBuilder {
  inner: MessageSchedulingStateSendWhenOnline
}

impl RTDMessageSchedulingStateSendWhenOnlineBuilder {
  pub fn build(&self) -> MessageSchedulingStateSendWhenOnline { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageSchedulingStateSendWhenOnline> for MessageSchedulingStateSendWhenOnline {
  fn as_ref(&self) -> &MessageSchedulingStateSendWhenOnline { self }
}

impl AsRef<MessageSchedulingStateSendWhenOnline> for RTDMessageSchedulingStateSendWhenOnlineBuilder {
  fn as_ref(&self) -> &MessageSchedulingStateSendWhenOnline { &self.inner }
}



