
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Options to be used when a message is send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessageOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Pass true to disable notification for the message. Must be false if the message is sent to a secret chat
  disable_notification: bool,
  /// Pass true if the message is sent from the background
  from_background: bool,
  /// Message scheduling state. Messages sent to a secret chat, live location messages and self-destructing messages can't be scheduled
  scheduling_state: MessageSchedulingState,
  
}

impl RObject for SendMessageOptions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sendMessageOptions" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl SendMessageOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSendMessageOptionsBuilder {
    let mut inner = SendMessageOptions::default();
    inner.td_name = "sendMessageOptions".to_string();
    RTDSendMessageOptionsBuilder { inner }
  }

  pub fn disable_notification(&self) -> bool { self.disable_notification }

  pub fn from_background(&self) -> bool { self.from_background }

  pub fn scheduling_state(&self) -> &MessageSchedulingState { &self.scheduling_state }

}

#[doc(hidden)]
pub struct RTDSendMessageOptionsBuilder {
  inner: SendMessageOptions
}

impl RTDSendMessageOptionsBuilder {
  pub fn build(&self) -> SendMessageOptions { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.disable_notification = disable_notification;
    self
  }

   
  pub fn from_background(&mut self, from_background: bool) -> &mut Self {
    self.inner.from_background = from_background;
    self
  }

   
  pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(&mut self, scheduling_state: T) -> &mut Self {
    self.inner.scheduling_state = scheduling_state.as_ref().clone();
    self
  }

}

impl AsRef<SendMessageOptions> for SendMessageOptions {
  fn as_ref(&self) -> &SendMessageOptions { self }
}

impl AsRef<SendMessageOptions> for RTDSendMessageOptionsBuilder {
  fn as_ref(&self) -> &SendMessageOptions { &self.inner }
}



