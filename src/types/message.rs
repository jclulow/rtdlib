
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Describes a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Message {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Message identifier, unique for the chat to which the message belongs
  id: i64,
  /// Identifier of the user who sent the message; 0 if unknown. Currently, it is unknown for channel posts and for channel posts automatically forwarded to discussion group
  sender_user_id: i64,
  /// Chat identifier
  chat_id: i64,
  /// Information about the sending state of the message; may be null
  sending_state: Option<MessageSendingState>,
  /// Information about the scheduling state of the message; may be null
  scheduling_state: Option<MessageSchedulingState>,
  /// True, if the message is outgoing
  is_outgoing: bool,
  /// True, if the message can be edited. For live location and poll messages this fields shows whether editMessageLiveLocation or stopPoll can be used with this message by the client
  can_be_edited: bool,
  /// True, if the message can be forwarded
  can_be_forwarded: bool,
  /// True, if the message can be deleted only for the current user while other users will continue to see it
  can_be_deleted_only_for_self: bool,
  /// True, if the message can be deleted for all users
  can_be_deleted_for_all_users: bool,
  /// True, if the message is a channel post. All messages to channels are channel posts, all other messages are not channel posts
  is_channel_post: bool,
  /// True, if the message contains an unread mention for the current user
  contains_unread_mention: bool,
  /// Point in time (Unix timestamp) when the message was sent
  date: i64,
  /// Point in time (Unix timestamp) when the message was last edited
  edit_date: i64,
  /// Information about the initial message sender; may be null
  forward_info: Option<MessageForwardInfo>,
  /// If non-zero, the identifier of the message this message is replying to; can be the identifier of a deleted message
  reply_to_message_id: i64,
  /// For self-destructing messages, the message's TTL (Time To Live), in seconds; 0 if none. TDLib will send updateDeleteMessages or updateMessageContent once the TTL expires
  ttl: i64,
  /// Time left before the message expires, in seconds
  ttl_expires_in: f32,
  /// If non-zero, the user identifier of the bot through which this message was sent
  via_bot_user_id: i64,
  /// For channel posts, optional author signature
  author_signature: String,
  /// Number of times this message was viewed
  views: i64,
  /// Unique identifier of an album this message belongs to. Only photos and videos can be grouped together in albums
  media_album_id: String,
  /// If non-empty, contains a human-readable description of the reason why access to this message must be restricted
  restriction_reason: String,
  /// Content of the message
  content: MessageContent,
  /// Reply markup for the message; may be null
  reply_markup: Option<ReplyMarkup>,
  
}

impl RObject for Message {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "message" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Message {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageBuilder {
    let mut inner = Message::default();
    inner.td_name = "message".to_string();
    RTDMessageBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn sender_user_id(&self) -> i64 { self.sender_user_id }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn sending_state(&self) -> &Option<MessageSendingState> { &self.sending_state }

  pub fn scheduling_state(&self) -> &Option<MessageSchedulingState> { &self.scheduling_state }

  pub fn is_outgoing(&self) -> bool { self.is_outgoing }

  pub fn can_be_edited(&self) -> bool { self.can_be_edited }

  pub fn can_be_forwarded(&self) -> bool { self.can_be_forwarded }

  pub fn can_be_deleted_only_for_self(&self) -> bool { self.can_be_deleted_only_for_self }

  pub fn can_be_deleted_for_all_users(&self) -> bool { self.can_be_deleted_for_all_users }

  pub fn is_channel_post(&self) -> bool { self.is_channel_post }

  pub fn contains_unread_mention(&self) -> bool { self.contains_unread_mention }

  pub fn date(&self) -> i64 { self.date }

  pub fn edit_date(&self) -> i64 { self.edit_date }

  pub fn forward_info(&self) -> &Option<MessageForwardInfo> { &self.forward_info }

  pub fn reply_to_message_id(&self) -> i64 { self.reply_to_message_id }

  pub fn ttl(&self) -> i64 { self.ttl }

  pub fn ttl_expires_in(&self) -> f32 { self.ttl_expires_in }

  pub fn via_bot_user_id(&self) -> i64 { self.via_bot_user_id }

  pub fn author_signature(&self) -> &String { &self.author_signature }

  pub fn views(&self) -> i64 { self.views }

  pub fn media_album_id(&self) -> &String { &self.media_album_id }

  pub fn restriction_reason(&self) -> &String { &self.restriction_reason }

  pub fn content(&self) -> &MessageContent { &self.content }

  pub fn reply_markup(&self) -> &Option<ReplyMarkup> { &self.reply_markup }

}

#[doc(hidden)]
pub struct RTDMessageBuilder {
  inner: Message
}

impl RTDMessageBuilder {
  pub fn build(&self) -> Message { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn sender_user_id(&mut self, sender_user_id: i64) -> &mut Self {
    self.inner.sender_user_id = sender_user_id;
    self
  }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn sending_state<T: AsRef<MessageSendingState>>(&mut self, sending_state: T) -> &mut Self {
    self.inner.sending_state = Some(sending_state.as_ref().clone());
    self
  }

   
  pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(&mut self, scheduling_state: T) -> &mut Self {
    self.inner.scheduling_state = Some(scheduling_state.as_ref().clone());
    self
  }

   
  pub fn is_outgoing(&mut self, is_outgoing: bool) -> &mut Self {
    self.inner.is_outgoing = is_outgoing;
    self
  }

   
  pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    self.inner.can_be_edited = can_be_edited;
    self
  }

   
  pub fn can_be_forwarded(&mut self, can_be_forwarded: bool) -> &mut Self {
    self.inner.can_be_forwarded = can_be_forwarded;
    self
  }

   
  pub fn can_be_deleted_only_for_self(&mut self, can_be_deleted_only_for_self: bool) -> &mut Self {
    self.inner.can_be_deleted_only_for_self = can_be_deleted_only_for_self;
    self
  }

   
  pub fn can_be_deleted_for_all_users(&mut self, can_be_deleted_for_all_users: bool) -> &mut Self {
    self.inner.can_be_deleted_for_all_users = can_be_deleted_for_all_users;
    self
  }

   
  pub fn is_channel_post(&mut self, is_channel_post: bool) -> &mut Self {
    self.inner.is_channel_post = is_channel_post;
    self
  }

   
  pub fn contains_unread_mention(&mut self, contains_unread_mention: bool) -> &mut Self {
    self.inner.contains_unread_mention = contains_unread_mention;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn edit_date(&mut self, edit_date: i64) -> &mut Self {
    self.inner.edit_date = edit_date;
    self
  }

   
  pub fn forward_info<T: AsRef<MessageForwardInfo>>(&mut self, forward_info: T) -> &mut Self {
    self.inner.forward_info = Some(forward_info.as_ref().clone());
    self
  }

   
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.reply_to_message_id = reply_to_message_id;
    self
  }

   
  pub fn ttl(&mut self, ttl: i64) -> &mut Self {
    self.inner.ttl = ttl;
    self
  }

   
  pub fn ttl_expires_in(&mut self, ttl_expires_in: f32) -> &mut Self {
    self.inner.ttl_expires_in = ttl_expires_in;
    self
  }

   
  pub fn via_bot_user_id(&mut self, via_bot_user_id: i64) -> &mut Self {
    self.inner.via_bot_user_id = via_bot_user_id;
    self
  }

   
  pub fn author_signature<T: AsRef<str>>(&mut self, author_signature: T) -> &mut Self {
    self.inner.author_signature = author_signature.as_ref().to_string();
    self
  }

   
  pub fn views(&mut self, views: i64) -> &mut Self {
    self.inner.views = views;
    self
  }

   
  pub fn media_album_id<T: AsRef<str>>(&mut self, media_album_id: T) -> &mut Self {
    self.inner.media_album_id = media_album_id.as_ref().to_string();
    self
  }

   
  pub fn restriction_reason<T: AsRef<str>>(&mut self, restriction_reason: T) -> &mut Self {
    self.inner.restriction_reason = restriction_reason.as_ref().to_string();
    self
  }

   
  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

   
  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = Some(reply_markup.as_ref().clone());
    self
  }

}

impl AsRef<Message> for Message {
  fn as_ref(&self) -> &Message { self }
}

impl AsRef<Message> for RTDMessageBuilder {
  fn as_ref(&self) -> &Message { &self.inner }
}



