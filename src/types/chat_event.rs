
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents a chat event
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatEvent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Chat event identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")] id: i64,
  /// Point in time (Unix timestamp) when the event happened
  date: i64,
  /// Identifier of the user who performed the action that triggered the event
  user_id: i64,
  /// Action performed by the user
  action: ChatEventAction,
  
}

impl RObject for ChatEvent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEvent" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatEvent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatEventBuilder {
    let mut inner = ChatEvent::default();
    inner.td_name = "chatEvent".to_string();
    RTDChatEventBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn date(&self) -> i64 { self.date }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn action(&self) -> &ChatEventAction { &self.action }

}

#[doc(hidden)]
pub struct RTDChatEventBuilder {
  inner: ChatEvent
}

impl RTDChatEventBuilder {
  pub fn build(&self) -> ChatEvent { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn action<T: AsRef<ChatEventAction>>(&mut self, action: T) -> &mut Self {
    self.inner.action = action.as_ref().clone();
    self
  }

}

impl AsRef<ChatEvent> for ChatEvent {
  fn as_ref(&self) -> &ChatEvent { self }
}

impl AsRef<ChatEvent> for RTDChatEventBuilder {
  fn as_ref(&self) -> &ChatEvent { &self.inner }
}



