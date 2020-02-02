
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains detailed information about a notification
pub trait TDNotificationType: Debug + RObject {}

/// Contains detailed information about a notification
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NotificationType {
  #[doc(hidden)] _Default(()),
  /// New message was received
  NewMessage(NotificationTypeNewMessage),
  /// New secret chat was created
  NewSecretChat(NotificationTypeNewSecretChat),
  /// New call was received
  NewCall(NotificationTypeNewCall),
  /// New message was received through a push notification
  NewPushMessage(NotificationTypeNewPushMessage),

}

impl Default for NotificationType {
  fn default() -> Self { NotificationType::_Default(()) }
}

impl<'de> Deserialize<'de> for NotificationType {
  fn deserialize<D>(deserializer: D) -> Result<NotificationType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      NotificationType,
      (notificationTypeNewMessage, NewMessage);
      (notificationTypeNewSecretChat, NewSecretChat);
      (notificationTypeNewCall, NewCall);
      (notificationTypeNewPushMessage, NewPushMessage);

    )(deserializer)
  }
}

impl RObject for NotificationType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      NotificationType::NewMessage(t) => t.td_name(),
      NotificationType::NewSecretChat(t) => t.td_name(),
      NotificationType::NewCall(t) => t.td_name(),
      NotificationType::NewPushMessage(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl NotificationType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let NotificationType::_Default(_) = self { true } else { false } }

  pub fn is_new_message(&self) -> bool { if let NotificationType::NewMessage(_) = self { true } else { false } }
  pub fn is_new_secret_chat(&self) -> bool { if let NotificationType::NewSecretChat(_) = self { true } else { false } }
  pub fn is_new_call(&self) -> bool { if let NotificationType::NewCall(_) = self { true } else { false } }
  pub fn is_new_push_message(&self) -> bool { if let NotificationType::NewPushMessage(_) = self { true } else { false } }

  pub fn on_new_message<F: FnOnce(&NotificationTypeNewMessage)>(&self, fnc: F) -> &Self { if let NotificationType::NewMessage(t) = self { fnc(t) }; self }
  pub fn on_new_secret_chat<F: FnOnce(&NotificationTypeNewSecretChat)>(&self, fnc: F) -> &Self { if let NotificationType::NewSecretChat(t) = self { fnc(t) }; self }
  pub fn on_new_call<F: FnOnce(&NotificationTypeNewCall)>(&self, fnc: F) -> &Self { if let NotificationType::NewCall(t) = self { fnc(t) }; self }
  pub fn on_new_push_message<F: FnOnce(&NotificationTypeNewPushMessage)>(&self, fnc: F) -> &Self { if let NotificationType::NewPushMessage(t) = self { fnc(t) }; self }

  pub fn as_new_message(&self) -> Option<&NotificationTypeNewMessage> { if let NotificationType::NewMessage(t) = self { return Some(t) } None }
  pub fn as_new_secret_chat(&self) -> Option<&NotificationTypeNewSecretChat> { if let NotificationType::NewSecretChat(t) = self { return Some(t) } None }
  pub fn as_new_call(&self) -> Option<&NotificationTypeNewCall> { if let NotificationType::NewCall(t) = self { return Some(t) } None }
  pub fn as_new_push_message(&self) -> Option<&NotificationTypeNewPushMessage> { if let NotificationType::NewPushMessage(t) = self { return Some(t) } None }



  pub fn new_message<T: AsRef<NotificationTypeNewMessage>>(t: T) -> Self { NotificationType::NewMessage(t.as_ref().clone()) }

  pub fn new_secret_chat<T: AsRef<NotificationTypeNewSecretChat>>(t: T) -> Self { NotificationType::NewSecretChat(t.as_ref().clone()) }

  pub fn new_call<T: AsRef<NotificationTypeNewCall>>(t: T) -> Self { NotificationType::NewCall(t.as_ref().clone()) }

  pub fn new_push_message<T: AsRef<NotificationTypeNewPushMessage>>(t: T) -> Self { NotificationType::NewPushMessage(t.as_ref().clone()) }

}

impl AsRef<NotificationType> for NotificationType {
  fn as_ref(&self) -> &NotificationType { self }
}







/// New message was received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The message
  message: Message,
  
}

impl RObject for NotificationTypeNewMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationTypeNewMessage" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNotificationType for NotificationTypeNewMessage {}



impl NotificationTypeNewMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationTypeNewMessageBuilder {
    let mut inner = NotificationTypeNewMessage::default();
    inner.td_name = "notificationTypeNewMessage".to_string();
    RTDNotificationTypeNewMessageBuilder { inner }
  }

  pub fn message(&self) -> &Message { &self.message }

}

#[doc(hidden)]
pub struct RTDNotificationTypeNewMessageBuilder {
  inner: NotificationTypeNewMessage
}

impl RTDNotificationTypeNewMessageBuilder {
  pub fn build(&self) -> NotificationTypeNewMessage { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }

}

impl AsRef<NotificationTypeNewMessage> for NotificationTypeNewMessage {
  fn as_ref(&self) -> &NotificationTypeNewMessage { self }
}

impl AsRef<NotificationTypeNewMessage> for RTDNotificationTypeNewMessageBuilder {
  fn as_ref(&self) -> &NotificationTypeNewMessage { &self.inner }
}







/// New secret chat was created
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for NotificationTypeNewSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationTypeNewSecretChat" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNotificationType for NotificationTypeNewSecretChat {}



impl NotificationTypeNewSecretChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationTypeNewSecretChatBuilder {
    let mut inner = NotificationTypeNewSecretChat::default();
    inner.td_name = "notificationTypeNewSecretChat".to_string();
    RTDNotificationTypeNewSecretChatBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNotificationTypeNewSecretChatBuilder {
  inner: NotificationTypeNewSecretChat
}

impl RTDNotificationTypeNewSecretChatBuilder {
  pub fn build(&self) -> NotificationTypeNewSecretChat { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<NotificationTypeNewSecretChat> for NotificationTypeNewSecretChat {
  fn as_ref(&self) -> &NotificationTypeNewSecretChat { self }
}

impl AsRef<NotificationTypeNewSecretChat> for RTDNotificationTypeNewSecretChatBuilder {
  fn as_ref(&self) -> &NotificationTypeNewSecretChat { &self.inner }
}







/// New call was received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Call identifier
  call_id: i64,
  
}

impl RObject for NotificationTypeNewCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationTypeNewCall" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNotificationType for NotificationTypeNewCall {}



impl NotificationTypeNewCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationTypeNewCallBuilder {
    let mut inner = NotificationTypeNewCall::default();
    inner.td_name = "notificationTypeNewCall".to_string();
    RTDNotificationTypeNewCallBuilder { inner }
  }

  pub fn call_id(&self) -> i64 { self.call_id }

}

#[doc(hidden)]
pub struct RTDNotificationTypeNewCallBuilder {
  inner: NotificationTypeNewCall
}

impl RTDNotificationTypeNewCallBuilder {
  pub fn build(&self) -> NotificationTypeNewCall { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn call_id(&mut self, call_id: i64) -> &mut Self {
    self.inner.call_id = call_id;
    self
  }

}

impl AsRef<NotificationTypeNewCall> for NotificationTypeNewCall {
  fn as_ref(&self) -> &NotificationTypeNewCall { self }
}

impl AsRef<NotificationTypeNewCall> for RTDNotificationTypeNewCallBuilder {
  fn as_ref(&self) -> &NotificationTypeNewCall { &self.inner }
}







/// New message was received through a push notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NotificationTypeNewPushMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The message identifier. The message will not be available in the chat history, but the ID can be used in viewMessages and as reply_to_message_id
  message_id: i64,
  /// Sender of the message. Corresponding user may be inaccessible
  sender_user_id: i64,
  /// Push message content
  content: PushMessageContent,
  
}

impl RObject for NotificationTypeNewPushMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "notificationTypeNewPushMessage" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNotificationType for NotificationTypeNewPushMessage {}



impl NotificationTypeNewPushMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNotificationTypeNewPushMessageBuilder {
    let mut inner = NotificationTypeNewPushMessage::default();
    inner.td_name = "notificationTypeNewPushMessage".to_string();
    RTDNotificationTypeNewPushMessageBuilder { inner }
  }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn content(&self) -> &PushMessageContent { &self.content }

}

#[doc(hidden)]
pub struct RTDNotificationTypeNewPushMessageBuilder {
  inner: NotificationTypeNewPushMessage
}

impl RTDNotificationTypeNewPushMessageBuilder {
  pub fn build(&self) -> NotificationTypeNewPushMessage { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn content<T: AsRef<PushMessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

}

impl AsRef<NotificationTypeNewPushMessage> for NotificationTypeNewPushMessage {
  fn as_ref(&self) -> &NotificationTypeNewPushMessage { self }
}

impl AsRef<NotificationTypeNewPushMessage> for RTDNotificationTypeNewPushMessageBuilder {
  fn as_ref(&self) -> &NotificationTypeNewPushMessage { &self.inner }
}



