
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains information about the origin of a forwarded message
pub trait TDMessageForwardOrigin: Debug + RObject {}

/// Contains information about the origin of a forwarded message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageForwardOrigin {
  #[doc(hidden)] _Default(()),
  /// The message was originally written by a known user
  User(MessageForwardOriginUser),
  /// The message was originally written by a user, which is hidden by their privacy settings
  HiddenUser(MessageForwardOriginHiddenUser),
  /// The message was originally a post in a channel
  Channel(MessageForwardOriginChannel),

}

impl Default for MessageForwardOrigin {
  fn default() -> Self { MessageForwardOrigin::_Default(()) }
}

impl<'de> Deserialize<'de> for MessageForwardOrigin {
  fn deserialize<D>(deserializer: D) -> Result<MessageForwardOrigin, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageForwardOrigin,
      (messageForwardOriginUser, User);
      (messageForwardOriginHiddenUser, HiddenUser);
      (messageForwardOriginChannel, Channel);

    )(deserializer)
  }
}

impl RObject for MessageForwardOrigin {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      MessageForwardOrigin::User(t) => t.td_name(),
      MessageForwardOrigin::HiddenUser(t) => t.td_name(),
      MessageForwardOrigin::Channel(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl MessageForwardOrigin {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let MessageForwardOrigin::_Default(_) = self { true } else { false } }

  pub fn is_user(&self) -> bool { if let MessageForwardOrigin::User(_) = self { true } else { false } }
  pub fn is_hidden_user(&self) -> bool { if let MessageForwardOrigin::HiddenUser(_) = self { true } else { false } }
  pub fn is_channel(&self) -> bool { if let MessageForwardOrigin::Channel(_) = self { true } else { false } }

  pub fn on_user<F: FnOnce(&MessageForwardOriginUser)>(&self, fnc: F) -> &Self { if let MessageForwardOrigin::User(t) = self { fnc(t) }; self }
  pub fn on_hidden_user<F: FnOnce(&MessageForwardOriginHiddenUser)>(&self, fnc: F) -> &Self { if let MessageForwardOrigin::HiddenUser(t) = self { fnc(t) }; self }
  pub fn on_channel<F: FnOnce(&MessageForwardOriginChannel)>(&self, fnc: F) -> &Self { if let MessageForwardOrigin::Channel(t) = self { fnc(t) }; self }

  pub fn as_user(&self) -> Option<&MessageForwardOriginUser> { if let MessageForwardOrigin::User(t) = self { return Some(t) } None }
  pub fn as_hidden_user(&self) -> Option<&MessageForwardOriginHiddenUser> { if let MessageForwardOrigin::HiddenUser(t) = self { return Some(t) } None }
  pub fn as_channel(&self) -> Option<&MessageForwardOriginChannel> { if let MessageForwardOrigin::Channel(t) = self { return Some(t) } None }



  pub fn user<T: AsRef<MessageForwardOriginUser>>(t: T) -> Self { MessageForwardOrigin::User(t.as_ref().clone()) }

  pub fn hidden_user<T: AsRef<MessageForwardOriginHiddenUser>>(t: T) -> Self { MessageForwardOrigin::HiddenUser(t.as_ref().clone()) }

  pub fn channel<T: AsRef<MessageForwardOriginChannel>>(t: T) -> Self { MessageForwardOrigin::Channel(t.as_ref().clone()) }

}

impl AsRef<MessageForwardOrigin> for MessageForwardOrigin {
  fn as_ref(&self) -> &MessageForwardOrigin { self }
}







/// The message was originally written by a known user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardOriginUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Identifier of the user that originally sent the message
  sender_user_id: i64,
  
}

impl RObject for MessageForwardOriginUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardOriginUser" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageForwardOrigin for MessageForwardOriginUser {}



impl MessageForwardOriginUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageForwardOriginUserBuilder {
    let mut inner = MessageForwardOriginUser::default();
    inner.td_name = "messageForwardOriginUser".to_string();
    RTDMessageForwardOriginUserBuilder { inner }
  }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

}

#[doc(hidden)]
pub struct RTDMessageForwardOriginUserBuilder {
  inner: MessageForwardOriginUser
}

impl RTDMessageForwardOriginUserBuilder {
  pub fn build(&self) -> MessageForwardOriginUser { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

}

impl AsRef<MessageForwardOriginUser> for MessageForwardOriginUser {
  fn as_ref(&self) -> &MessageForwardOriginUser { self }
}

impl AsRef<MessageForwardOriginUser> for RTDMessageForwardOriginUserBuilder {
  fn as_ref(&self) -> &MessageForwardOriginUser { &self.inner }
}







/// The message was originally written by a user, which is hidden by their privacy settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardOriginHiddenUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Name of the sender
  sender_name: String,
  
}

impl RObject for MessageForwardOriginHiddenUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardOriginHiddenUser" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageForwardOrigin for MessageForwardOriginHiddenUser {}



impl MessageForwardOriginHiddenUser {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageForwardOriginHiddenUserBuilder {
    let mut inner = MessageForwardOriginHiddenUser::default();
    inner.td_name = "messageForwardOriginHiddenUser".to_string();
    RTDMessageForwardOriginHiddenUserBuilder { inner }
  }

  pub fn sender_name(&self) -> &String { &self.sender_name }

}

#[doc(hidden)]
pub struct RTDMessageForwardOriginHiddenUserBuilder {
  inner: MessageForwardOriginHiddenUser
}

impl RTDMessageForwardOriginHiddenUserBuilder {
  pub fn build(&self) -> MessageForwardOriginHiddenUser { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
    self.inner.sender_name = sender_name.as_ref().to_string();
    self
  }

}

impl AsRef<MessageForwardOriginHiddenUser> for MessageForwardOriginHiddenUser {
  fn as_ref(&self) -> &MessageForwardOriginHiddenUser { self }
}

impl AsRef<MessageForwardOriginHiddenUser> for RTDMessageForwardOriginHiddenUserBuilder {
  fn as_ref(&self) -> &MessageForwardOriginHiddenUser { &self.inner }
}







/// The message was originally a post in a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageForwardOriginChannel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Identifier of the chat from which the message was originally forwarded
  chat_id: i64,
  /// Message identifier of the original message; 0 if unknown
  message_id: i64,
  /// Original post author signature
  author_signature: String,
  
}

impl RObject for MessageForwardOriginChannel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardOriginChannel" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageForwardOrigin for MessageForwardOriginChannel {}



impl MessageForwardOriginChannel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageForwardOriginChannelBuilder {
    let mut inner = MessageForwardOriginChannel::default();
    inner.td_name = "messageForwardOriginChannel".to_string();
    RTDMessageForwardOriginChannelBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn author_signature(&self) -> &String { &self.author_signature }

}

#[doc(hidden)]
pub struct RTDMessageForwardOriginChannelBuilder {
  inner: MessageForwardOriginChannel
}

impl RTDMessageForwardOriginChannelBuilder {
  pub fn build(&self) -> MessageForwardOriginChannel { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
    self.inner.author_signature = author_signature.as_ref().to_string();
    self
  }

}

impl AsRef<MessageForwardOriginChannel> for MessageForwardOriginChannel {
  fn as_ref(&self) -> &MessageForwardOriginChannel { self }
}

impl AsRef<MessageForwardOriginChannel> for RTDMessageForwardOriginChannelBuilder {
  fn as_ref(&self) -> &MessageForwardOriginChannel { &self.inner }
}



