
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains the content of a message
pub trait TDMessageContent: Debug + RObject {}

/// Contains the content of a message
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MessageContent {
  #[doc(hidden)] _Default(()),
  /// A text message
  MessageText(MessageText),
  /// An animation message (GIF-style).
  MessageAnimation(MessageAnimation),
  /// An audio message
  MessageAudio(MessageAudio),
  /// A document message (general file)
  MessageDocument(MessageDocument),
  /// A photo message
  MessagePhoto(MessagePhoto),
  /// An expired photo message (self-destructed after TTL has elapsed)
  MessageExpiredPhoto(MessageExpiredPhoto),
  /// A sticker message
  MessageSticker(MessageSticker),
  /// A video message
  MessageVideo(MessageVideo),
  /// An expired video message (self-destructed after TTL has elapsed)
  MessageExpiredVideo(MessageExpiredVideo),
  /// A video note message
  MessageVideoNote(MessageVideoNote),
  /// A voice note message
  MessageVoiceNote(MessageVoiceNote),
  /// A message with a location
  MessageLocation(MessageLocation),
  /// A message with information about a venue
  MessageVenue(MessageVenue),
  /// A message with a user contact
  MessageContact(MessageContact),
  /// A message with a game
  MessageGame(MessageGame),
  /// A message with a poll
  MessagePoll(MessagePoll),
  /// A message with an invoice from a bot
  MessageInvoice(MessageInvoice),
  /// A message with information about an ended call
  MessageCall(MessageCall),
  /// A newly created basic group
  MessageBasicGroupChatCreate(MessageBasicGroupChatCreate),
  /// A newly created supergroup or channel
  MessageSupergroupChatCreate(MessageSupergroupChatCreate),
  /// An updated chat title
  MessageChatChangeTitle(MessageChatChangeTitle),
  /// An updated chat photo
  MessageChatChangePhoto(MessageChatChangePhoto),
  /// A deleted chat photo
  MessageChatDeletePhoto(MessageChatDeletePhoto),
  /// New chat members were added
  MessageChatAddMembers(MessageChatAddMembers),
  /// A new member joined the chat by invite link
  MessageChatJoinByLink(MessageChatJoinByLink),
  /// A chat member was deleted
  MessageChatDeleteMember(MessageChatDeleteMember),
  /// A basic group was upgraded to a supergroup and was deactivated as the result
  MessageChatUpgradeTo(MessageChatUpgradeTo),
  /// A supergroup has been created from a basic group
  MessageChatUpgradeFrom(MessageChatUpgradeFrom),
  /// A message has been pinned
  MessagePinMessage(MessagePinMessage),
  /// A screenshot of a message in the chat has been taken
  MessageScreenshotTaken(MessageScreenshotTaken),
  /// The TTL (Time To Live) setting messages in a secret chat has been changed
  MessageChatSetTtl(MessageChatSetTtl),
  /// A non-standard action has happened in the chat
  MessageCustomServiceAction(MessageCustomServiceAction),
  /// A new high score was achieved in a game
  MessageGameScore(MessageGameScore),
  /// A payment has been completed
  MessagePaymentSuccessful(MessagePaymentSuccessful),
  /// A payment has been completed; for bots only
  MessagePaymentSuccessfulBot(MessagePaymentSuccessfulBot),
  /// A contact has registered with Telegram
  MessageContactRegistered(MessageContactRegistered),
  /// The current user has connected a website by logging in using Telegram Login Widget on it
  MessageWebsiteConnected(MessageWebsiteConnected),
  /// Telegram Passport data has been sent
  MessagePassportDataSent(MessagePassportDataSent),
  /// Telegram Passport data has been received; for bots only
  MessagePassportDataReceived(MessagePassportDataReceived),
  /// Message content that is not supported by the client
  MessageUnsupported(MessageUnsupported),

}

impl Default for MessageContent {
  fn default() -> Self { MessageContent::_Default(()) }
}

impl<'de> Deserialize<'de> for MessageContent {
  fn deserialize<D>(deserializer: D) -> Result<MessageContent, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      MessageContent,
      (messageText, MessageText);
      (messageAnimation, MessageAnimation);
      (messageAudio, MessageAudio);
      (messageDocument, MessageDocument);
      (messagePhoto, MessagePhoto);
      (messageExpiredPhoto, MessageExpiredPhoto);
      (messageSticker, MessageSticker);
      (messageVideo, MessageVideo);
      (messageExpiredVideo, MessageExpiredVideo);
      (messageVideoNote, MessageVideoNote);
      (messageVoiceNote, MessageVoiceNote);
      (messageLocation, MessageLocation);
      (messageVenue, MessageVenue);
      (messageContact, MessageContact);
      (messageGame, MessageGame);
      (messagePoll, MessagePoll);
      (messageInvoice, MessageInvoice);
      (messageCall, MessageCall);
      (messageBasicGroupChatCreate, MessageBasicGroupChatCreate);
      (messageSupergroupChatCreate, MessageSupergroupChatCreate);
      (messageChatChangeTitle, MessageChatChangeTitle);
      (messageChatChangePhoto, MessageChatChangePhoto);
      (messageChatDeletePhoto, MessageChatDeletePhoto);
      (messageChatAddMembers, MessageChatAddMembers);
      (messageChatJoinByLink, MessageChatJoinByLink);
      (messageChatDeleteMember, MessageChatDeleteMember);
      (messageChatUpgradeTo, MessageChatUpgradeTo);
      (messageChatUpgradeFrom, MessageChatUpgradeFrom);
      (messagePinMessage, MessagePinMessage);
      (messageScreenshotTaken, MessageScreenshotTaken);
      (messageChatSetTtl, MessageChatSetTtl);
      (messageCustomServiceAction, MessageCustomServiceAction);
      (messageGameScore, MessageGameScore);
      (messagePaymentSuccessful, MessagePaymentSuccessful);
      (messagePaymentSuccessfulBot, MessagePaymentSuccessfulBot);
      (messageContactRegistered, MessageContactRegistered);
      (messageWebsiteConnected, MessageWebsiteConnected);
      (messagePassportDataSent, MessagePassportDataSent);
      (messagePassportDataReceived, MessagePassportDataReceived);
      (messageUnsupported, MessageUnsupported);

    )(deserializer)
  }
}

impl RObject for MessageContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      MessageContent::MessageText(t) => t.td_name(),
      MessageContent::MessageAnimation(t) => t.td_name(),
      MessageContent::MessageAudio(t) => t.td_name(),
      MessageContent::MessageDocument(t) => t.td_name(),
      MessageContent::MessagePhoto(t) => t.td_name(),
      MessageContent::MessageExpiredPhoto(t) => t.td_name(),
      MessageContent::MessageSticker(t) => t.td_name(),
      MessageContent::MessageVideo(t) => t.td_name(),
      MessageContent::MessageExpiredVideo(t) => t.td_name(),
      MessageContent::MessageVideoNote(t) => t.td_name(),
      MessageContent::MessageVoiceNote(t) => t.td_name(),
      MessageContent::MessageLocation(t) => t.td_name(),
      MessageContent::MessageVenue(t) => t.td_name(),
      MessageContent::MessageContact(t) => t.td_name(),
      MessageContent::MessageGame(t) => t.td_name(),
      MessageContent::MessagePoll(t) => t.td_name(),
      MessageContent::MessageInvoice(t) => t.td_name(),
      MessageContent::MessageCall(t) => t.td_name(),
      MessageContent::MessageBasicGroupChatCreate(t) => t.td_name(),
      MessageContent::MessageSupergroupChatCreate(t) => t.td_name(),
      MessageContent::MessageChatChangeTitle(t) => t.td_name(),
      MessageContent::MessageChatChangePhoto(t) => t.td_name(),
      MessageContent::MessageChatDeletePhoto(t) => t.td_name(),
      MessageContent::MessageChatAddMembers(t) => t.td_name(),
      MessageContent::MessageChatJoinByLink(t) => t.td_name(),
      MessageContent::MessageChatDeleteMember(t) => t.td_name(),
      MessageContent::MessageChatUpgradeTo(t) => t.td_name(),
      MessageContent::MessageChatUpgradeFrom(t) => t.td_name(),
      MessageContent::MessagePinMessage(t) => t.td_name(),
      MessageContent::MessageScreenshotTaken(t) => t.td_name(),
      MessageContent::MessageChatSetTtl(t) => t.td_name(),
      MessageContent::MessageCustomServiceAction(t) => t.td_name(),
      MessageContent::MessageGameScore(t) => t.td_name(),
      MessageContent::MessagePaymentSuccessful(t) => t.td_name(),
      MessageContent::MessagePaymentSuccessfulBot(t) => t.td_name(),
      MessageContent::MessageContactRegistered(t) => t.td_name(),
      MessageContent::MessageWebsiteConnected(t) => t.td_name(),
      MessageContent::MessagePassportDataSent(t) => t.td_name(),
      MessageContent::MessagePassportDataReceived(t) => t.td_name(),
      MessageContent::MessageUnsupported(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl MessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let MessageContent::_Default(_) = self { true } else { false } }

  pub fn is_message_text(&self) -> bool { if let MessageContent::MessageText(_) = self { true } else { false } }
  pub fn is_message_animation(&self) -> bool { if let MessageContent::MessageAnimation(_) = self { true } else { false } }
  pub fn is_message_audio(&self) -> bool { if let MessageContent::MessageAudio(_) = self { true } else { false } }
  pub fn is_message_document(&self) -> bool { if let MessageContent::MessageDocument(_) = self { true } else { false } }
  pub fn is_message_photo(&self) -> bool { if let MessageContent::MessagePhoto(_) = self { true } else { false } }
  pub fn is_message_expired_photo(&self) -> bool { if let MessageContent::MessageExpiredPhoto(_) = self { true } else { false } }
  pub fn is_message_sticker(&self) -> bool { if let MessageContent::MessageSticker(_) = self { true } else { false } }
  pub fn is_message_video(&self) -> bool { if let MessageContent::MessageVideo(_) = self { true } else { false } }
  pub fn is_message_expired_video(&self) -> bool { if let MessageContent::MessageExpiredVideo(_) = self { true } else { false } }
  pub fn is_message_video_note(&self) -> bool { if let MessageContent::MessageVideoNote(_) = self { true } else { false } }
  pub fn is_message_voice_note(&self) -> bool { if let MessageContent::MessageVoiceNote(_) = self { true } else { false } }
  pub fn is_message_location(&self) -> bool { if let MessageContent::MessageLocation(_) = self { true } else { false } }
  pub fn is_message_venue(&self) -> bool { if let MessageContent::MessageVenue(_) = self { true } else { false } }
  pub fn is_message_contact(&self) -> bool { if let MessageContent::MessageContact(_) = self { true } else { false } }
  pub fn is_message_game(&self) -> bool { if let MessageContent::MessageGame(_) = self { true } else { false } }
  pub fn is_message_poll(&self) -> bool { if let MessageContent::MessagePoll(_) = self { true } else { false } }
  pub fn is_message_invoice(&self) -> bool { if let MessageContent::MessageInvoice(_) = self { true } else { false } }
  pub fn is_message_call(&self) -> bool { if let MessageContent::MessageCall(_) = self { true } else { false } }
  pub fn is_message_basic_group_chat_create(&self) -> bool { if let MessageContent::MessageBasicGroupChatCreate(_) = self { true } else { false } }
  pub fn is_message_supergroup_chat_create(&self) -> bool { if let MessageContent::MessageSupergroupChatCreate(_) = self { true } else { false } }
  pub fn is_message_chat_change_title(&self) -> bool { if let MessageContent::MessageChatChangeTitle(_) = self { true } else { false } }
  pub fn is_message_chat_change_photo(&self) -> bool { if let MessageContent::MessageChatChangePhoto(_) = self { true } else { false } }
  pub fn is_message_chat_delete_photo(&self) -> bool { if let MessageContent::MessageChatDeletePhoto(_) = self { true } else { false } }
  pub fn is_message_chat_add_members(&self) -> bool { if let MessageContent::MessageChatAddMembers(_) = self { true } else { false } }
  pub fn is_message_chat_join_by_link(&self) -> bool { if let MessageContent::MessageChatJoinByLink(_) = self { true } else { false } }
  pub fn is_message_chat_delete_member(&self) -> bool { if let MessageContent::MessageChatDeleteMember(_) = self { true } else { false } }
  pub fn is_message_chat_upgrade_to(&self) -> bool { if let MessageContent::MessageChatUpgradeTo(_) = self { true } else { false } }
  pub fn is_message_chat_upgrade_from(&self) -> bool { if let MessageContent::MessageChatUpgradeFrom(_) = self { true } else { false } }
  pub fn is_message_pin_message(&self) -> bool { if let MessageContent::MessagePinMessage(_) = self { true } else { false } }
  pub fn is_message_screenshot_taken(&self) -> bool { if let MessageContent::MessageScreenshotTaken(_) = self { true } else { false } }
  pub fn is_message_chat_set_ttl(&self) -> bool { if let MessageContent::MessageChatSetTtl(_) = self { true } else { false } }
  pub fn is_message_custom_service_action(&self) -> bool { if let MessageContent::MessageCustomServiceAction(_) = self { true } else { false } }
  pub fn is_message_game_score(&self) -> bool { if let MessageContent::MessageGameScore(_) = self { true } else { false } }
  pub fn is_message_payment_successful(&self) -> bool { if let MessageContent::MessagePaymentSuccessful(_) = self { true } else { false } }
  pub fn is_message_payment_successful_bot(&self) -> bool { if let MessageContent::MessagePaymentSuccessfulBot(_) = self { true } else { false } }
  pub fn is_message_contact_registered(&self) -> bool { if let MessageContent::MessageContactRegistered(_) = self { true } else { false } }
  pub fn is_message_website_connected(&self) -> bool { if let MessageContent::MessageWebsiteConnected(_) = self { true } else { false } }
  pub fn is_message_passport_data_sent(&self) -> bool { if let MessageContent::MessagePassportDataSent(_) = self { true } else { false } }
  pub fn is_message_passport_data_received(&self) -> bool { if let MessageContent::MessagePassportDataReceived(_) = self { true } else { false } }
  pub fn is_message_unsupported(&self) -> bool { if let MessageContent::MessageUnsupported(_) = self { true } else { false } }

  pub fn on_message_text<F: FnOnce(&MessageText)>(&self, fnc: F) -> &Self { if let MessageContent::MessageText(t) = self { fnc(t) }; self }
  pub fn on_message_animation<F: FnOnce(&MessageAnimation)>(&self, fnc: F) -> &Self { if let MessageContent::MessageAnimation(t) = self { fnc(t) }; self }
  pub fn on_message_audio<F: FnOnce(&MessageAudio)>(&self, fnc: F) -> &Self { if let MessageContent::MessageAudio(t) = self { fnc(t) }; self }
  pub fn on_message_document<F: FnOnce(&MessageDocument)>(&self, fnc: F) -> &Self { if let MessageContent::MessageDocument(t) = self { fnc(t) }; self }
  pub fn on_message_photo<F: FnOnce(&MessagePhoto)>(&self, fnc: F) -> &Self { if let MessageContent::MessagePhoto(t) = self { fnc(t) }; self }
  pub fn on_message_expired_photo<F: FnOnce(&MessageExpiredPhoto)>(&self, fnc: F) -> &Self { if let MessageContent::MessageExpiredPhoto(t) = self { fnc(t) }; self }
  pub fn on_message_sticker<F: FnOnce(&MessageSticker)>(&self, fnc: F) -> &Self { if let MessageContent::MessageSticker(t) = self { fnc(t) }; self }
  pub fn on_message_video<F: FnOnce(&MessageVideo)>(&self, fnc: F) -> &Self { if let MessageContent::MessageVideo(t) = self { fnc(t) }; self }
  pub fn on_message_expired_video<F: FnOnce(&MessageExpiredVideo)>(&self, fnc: F) -> &Self { if let MessageContent::MessageExpiredVideo(t) = self { fnc(t) }; self }
  pub fn on_message_video_note<F: FnOnce(&MessageVideoNote)>(&self, fnc: F) -> &Self { if let MessageContent::MessageVideoNote(t) = self { fnc(t) }; self }
  pub fn on_message_voice_note<F: FnOnce(&MessageVoiceNote)>(&self, fnc: F) -> &Self { if let MessageContent::MessageVoiceNote(t) = self { fnc(t) }; self }
  pub fn on_message_location<F: FnOnce(&MessageLocation)>(&self, fnc: F) -> &Self { if let MessageContent::MessageLocation(t) = self { fnc(t) }; self }
  pub fn on_message_venue<F: FnOnce(&MessageVenue)>(&self, fnc: F) -> &Self { if let MessageContent::MessageVenue(t) = self { fnc(t) }; self }
  pub fn on_message_contact<F: FnOnce(&MessageContact)>(&self, fnc: F) -> &Self { if let MessageContent::MessageContact(t) = self { fnc(t) }; self }
  pub fn on_message_game<F: FnOnce(&MessageGame)>(&self, fnc: F) -> &Self { if let MessageContent::MessageGame(t) = self { fnc(t) }; self }
  pub fn on_message_poll<F: FnOnce(&MessagePoll)>(&self, fnc: F) -> &Self { if let MessageContent::MessagePoll(t) = self { fnc(t) }; self }
  pub fn on_message_invoice<F: FnOnce(&MessageInvoice)>(&self, fnc: F) -> &Self { if let MessageContent::MessageInvoice(t) = self { fnc(t) }; self }
  pub fn on_message_call<F: FnOnce(&MessageCall)>(&self, fnc: F) -> &Self { if let MessageContent::MessageCall(t) = self { fnc(t) }; self }
  pub fn on_message_basic_group_chat_create<F: FnOnce(&MessageBasicGroupChatCreate)>(&self, fnc: F) -> &Self { if let MessageContent::MessageBasicGroupChatCreate(t) = self { fnc(t) }; self }
  pub fn on_message_supergroup_chat_create<F: FnOnce(&MessageSupergroupChatCreate)>(&self, fnc: F) -> &Self { if let MessageContent::MessageSupergroupChatCreate(t) = self { fnc(t) }; self }
  pub fn on_message_chat_change_title<F: FnOnce(&MessageChatChangeTitle)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatChangeTitle(t) = self { fnc(t) }; self }
  pub fn on_message_chat_change_photo<F: FnOnce(&MessageChatChangePhoto)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatChangePhoto(t) = self { fnc(t) }; self }
  pub fn on_message_chat_delete_photo<F: FnOnce(&MessageChatDeletePhoto)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatDeletePhoto(t) = self { fnc(t) }; self }
  pub fn on_message_chat_add_members<F: FnOnce(&MessageChatAddMembers)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatAddMembers(t) = self { fnc(t) }; self }
  pub fn on_message_chat_join_by_link<F: FnOnce(&MessageChatJoinByLink)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatJoinByLink(t) = self { fnc(t) }; self }
  pub fn on_message_chat_delete_member<F: FnOnce(&MessageChatDeleteMember)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatDeleteMember(t) = self { fnc(t) }; self }
  pub fn on_message_chat_upgrade_to<F: FnOnce(&MessageChatUpgradeTo)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatUpgradeTo(t) = self { fnc(t) }; self }
  pub fn on_message_chat_upgrade_from<F: FnOnce(&MessageChatUpgradeFrom)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatUpgradeFrom(t) = self { fnc(t) }; self }
  pub fn on_message_pin_message<F: FnOnce(&MessagePinMessage)>(&self, fnc: F) -> &Self { if let MessageContent::MessagePinMessage(t) = self { fnc(t) }; self }
  pub fn on_message_screenshot_taken<F: FnOnce(&MessageScreenshotTaken)>(&self, fnc: F) -> &Self { if let MessageContent::MessageScreenshotTaken(t) = self { fnc(t) }; self }
  pub fn on_message_chat_set_ttl<F: FnOnce(&MessageChatSetTtl)>(&self, fnc: F) -> &Self { if let MessageContent::MessageChatSetTtl(t) = self { fnc(t) }; self }
  pub fn on_message_custom_service_action<F: FnOnce(&MessageCustomServiceAction)>(&self, fnc: F) -> &Self { if let MessageContent::MessageCustomServiceAction(t) = self { fnc(t) }; self }
  pub fn on_message_game_score<F: FnOnce(&MessageGameScore)>(&self, fnc: F) -> &Self { if let MessageContent::MessageGameScore(t) = self { fnc(t) }; self }
  pub fn on_message_payment_successful<F: FnOnce(&MessagePaymentSuccessful)>(&self, fnc: F) -> &Self { if let MessageContent::MessagePaymentSuccessful(t) = self { fnc(t) }; self }
  pub fn on_message_payment_successful_bot<F: FnOnce(&MessagePaymentSuccessfulBot)>(&self, fnc: F) -> &Self { if let MessageContent::MessagePaymentSuccessfulBot(t) = self { fnc(t) }; self }
  pub fn on_message_contact_registered<F: FnOnce(&MessageContactRegistered)>(&self, fnc: F) -> &Self { if let MessageContent::MessageContactRegistered(t) = self { fnc(t) }; self }
  pub fn on_message_website_connected<F: FnOnce(&MessageWebsiteConnected)>(&self, fnc: F) -> &Self { if let MessageContent::MessageWebsiteConnected(t) = self { fnc(t) }; self }
  pub fn on_message_passport_data_sent<F: FnOnce(&MessagePassportDataSent)>(&self, fnc: F) -> &Self { if let MessageContent::MessagePassportDataSent(t) = self { fnc(t) }; self }
  pub fn on_message_passport_data_received<F: FnOnce(&MessagePassportDataReceived)>(&self, fnc: F) -> &Self { if let MessageContent::MessagePassportDataReceived(t) = self { fnc(t) }; self }
  pub fn on_message_unsupported<F: FnOnce(&MessageUnsupported)>(&self, fnc: F) -> &Self { if let MessageContent::MessageUnsupported(t) = self { fnc(t) }; self }

  pub fn as_message_text(&self) -> Option<&MessageText> { if let MessageContent::MessageText(t) = self { return Some(t) } None }
  pub fn as_message_animation(&self) -> Option<&MessageAnimation> { if let MessageContent::MessageAnimation(t) = self { return Some(t) } None }
  pub fn as_message_audio(&self) -> Option<&MessageAudio> { if let MessageContent::MessageAudio(t) = self { return Some(t) } None }
  pub fn as_message_document(&self) -> Option<&MessageDocument> { if let MessageContent::MessageDocument(t) = self { return Some(t) } None }
  pub fn as_message_photo(&self) -> Option<&MessagePhoto> { if let MessageContent::MessagePhoto(t) = self { return Some(t) } None }
  pub fn as_message_expired_photo(&self) -> Option<&MessageExpiredPhoto> { if let MessageContent::MessageExpiredPhoto(t) = self { return Some(t) } None }
  pub fn as_message_sticker(&self) -> Option<&MessageSticker> { if let MessageContent::MessageSticker(t) = self { return Some(t) } None }
  pub fn as_message_video(&self) -> Option<&MessageVideo> { if let MessageContent::MessageVideo(t) = self { return Some(t) } None }
  pub fn as_message_expired_video(&self) -> Option<&MessageExpiredVideo> { if let MessageContent::MessageExpiredVideo(t) = self { return Some(t) } None }
  pub fn as_message_video_note(&self) -> Option<&MessageVideoNote> { if let MessageContent::MessageVideoNote(t) = self { return Some(t) } None }
  pub fn as_message_voice_note(&self) -> Option<&MessageVoiceNote> { if let MessageContent::MessageVoiceNote(t) = self { return Some(t) } None }
  pub fn as_message_location(&self) -> Option<&MessageLocation> { if let MessageContent::MessageLocation(t) = self { return Some(t) } None }
  pub fn as_message_venue(&self) -> Option<&MessageVenue> { if let MessageContent::MessageVenue(t) = self { return Some(t) } None }
  pub fn as_message_contact(&self) -> Option<&MessageContact> { if let MessageContent::MessageContact(t) = self { return Some(t) } None }
  pub fn as_message_game(&self) -> Option<&MessageGame> { if let MessageContent::MessageGame(t) = self { return Some(t) } None }
  pub fn as_message_poll(&self) -> Option<&MessagePoll> { if let MessageContent::MessagePoll(t) = self { return Some(t) } None }
  pub fn as_message_invoice(&self) -> Option<&MessageInvoice> { if let MessageContent::MessageInvoice(t) = self { return Some(t) } None }
  pub fn as_message_call(&self) -> Option<&MessageCall> { if let MessageContent::MessageCall(t) = self { return Some(t) } None }
  pub fn as_message_basic_group_chat_create(&self) -> Option<&MessageBasicGroupChatCreate> { if let MessageContent::MessageBasicGroupChatCreate(t) = self { return Some(t) } None }
  pub fn as_message_supergroup_chat_create(&self) -> Option<&MessageSupergroupChatCreate> { if let MessageContent::MessageSupergroupChatCreate(t) = self { return Some(t) } None }
  pub fn as_message_chat_change_title(&self) -> Option<&MessageChatChangeTitle> { if let MessageContent::MessageChatChangeTitle(t) = self { return Some(t) } None }
  pub fn as_message_chat_change_photo(&self) -> Option<&MessageChatChangePhoto> { if let MessageContent::MessageChatChangePhoto(t) = self { return Some(t) } None }
  pub fn as_message_chat_delete_photo(&self) -> Option<&MessageChatDeletePhoto> { if let MessageContent::MessageChatDeletePhoto(t) = self { return Some(t) } None }
  pub fn as_message_chat_add_members(&self) -> Option<&MessageChatAddMembers> { if let MessageContent::MessageChatAddMembers(t) = self { return Some(t) } None }
  pub fn as_message_chat_join_by_link(&self) -> Option<&MessageChatJoinByLink> { if let MessageContent::MessageChatJoinByLink(t) = self { return Some(t) } None }
  pub fn as_message_chat_delete_member(&self) -> Option<&MessageChatDeleteMember> { if let MessageContent::MessageChatDeleteMember(t) = self { return Some(t) } None }
  pub fn as_message_chat_upgrade_to(&self) -> Option<&MessageChatUpgradeTo> { if let MessageContent::MessageChatUpgradeTo(t) = self { return Some(t) } None }
  pub fn as_message_chat_upgrade_from(&self) -> Option<&MessageChatUpgradeFrom> { if let MessageContent::MessageChatUpgradeFrom(t) = self { return Some(t) } None }
  pub fn as_message_pin_message(&self) -> Option<&MessagePinMessage> { if let MessageContent::MessagePinMessage(t) = self { return Some(t) } None }
  pub fn as_message_screenshot_taken(&self) -> Option<&MessageScreenshotTaken> { if let MessageContent::MessageScreenshotTaken(t) = self { return Some(t) } None }
  pub fn as_message_chat_set_ttl(&self) -> Option<&MessageChatSetTtl> { if let MessageContent::MessageChatSetTtl(t) = self { return Some(t) } None }
  pub fn as_message_custom_service_action(&self) -> Option<&MessageCustomServiceAction> { if let MessageContent::MessageCustomServiceAction(t) = self { return Some(t) } None }
  pub fn as_message_game_score(&self) -> Option<&MessageGameScore> { if let MessageContent::MessageGameScore(t) = self { return Some(t) } None }
  pub fn as_message_payment_successful(&self) -> Option<&MessagePaymentSuccessful> { if let MessageContent::MessagePaymentSuccessful(t) = self { return Some(t) } None }
  pub fn as_message_payment_successful_bot(&self) -> Option<&MessagePaymentSuccessfulBot> { if let MessageContent::MessagePaymentSuccessfulBot(t) = self { return Some(t) } None }
  pub fn as_message_contact_registered(&self) -> Option<&MessageContactRegistered> { if let MessageContent::MessageContactRegistered(t) = self { return Some(t) } None }
  pub fn as_message_website_connected(&self) -> Option<&MessageWebsiteConnected> { if let MessageContent::MessageWebsiteConnected(t) = self { return Some(t) } None }
  pub fn as_message_passport_data_sent(&self) -> Option<&MessagePassportDataSent> { if let MessageContent::MessagePassportDataSent(t) = self { return Some(t) } None }
  pub fn as_message_passport_data_received(&self) -> Option<&MessagePassportDataReceived> { if let MessageContent::MessagePassportDataReceived(t) = self { return Some(t) } None }
  pub fn as_message_unsupported(&self) -> Option<&MessageUnsupported> { if let MessageContent::MessageUnsupported(t) = self { return Some(t) } None }



  pub fn message_text<T: AsRef<MessageText>>(t: T) -> Self { MessageContent::MessageText(t.as_ref().clone()) }

  pub fn message_animation<T: AsRef<MessageAnimation>>(t: T) -> Self { MessageContent::MessageAnimation(t.as_ref().clone()) }

  pub fn message_audio<T: AsRef<MessageAudio>>(t: T) -> Self { MessageContent::MessageAudio(t.as_ref().clone()) }

  pub fn message_document<T: AsRef<MessageDocument>>(t: T) -> Self { MessageContent::MessageDocument(t.as_ref().clone()) }

  pub fn message_photo<T: AsRef<MessagePhoto>>(t: T) -> Self { MessageContent::MessagePhoto(t.as_ref().clone()) }

  pub fn message_expired_photo<T: AsRef<MessageExpiredPhoto>>(t: T) -> Self { MessageContent::MessageExpiredPhoto(t.as_ref().clone()) }

  pub fn message_sticker<T: AsRef<MessageSticker>>(t: T) -> Self { MessageContent::MessageSticker(t.as_ref().clone()) }

  pub fn message_video<T: AsRef<MessageVideo>>(t: T) -> Self { MessageContent::MessageVideo(t.as_ref().clone()) }

  pub fn message_expired_video<T: AsRef<MessageExpiredVideo>>(t: T) -> Self { MessageContent::MessageExpiredVideo(t.as_ref().clone()) }

  pub fn message_video_note<T: AsRef<MessageVideoNote>>(t: T) -> Self { MessageContent::MessageVideoNote(t.as_ref().clone()) }

  pub fn message_voice_note<T: AsRef<MessageVoiceNote>>(t: T) -> Self { MessageContent::MessageVoiceNote(t.as_ref().clone()) }

  pub fn message_location<T: AsRef<MessageLocation>>(t: T) -> Self { MessageContent::MessageLocation(t.as_ref().clone()) }

  pub fn message_venue<T: AsRef<MessageVenue>>(t: T) -> Self { MessageContent::MessageVenue(t.as_ref().clone()) }

  pub fn message_contact<T: AsRef<MessageContact>>(t: T) -> Self { MessageContent::MessageContact(t.as_ref().clone()) }

  pub fn message_game<T: AsRef<MessageGame>>(t: T) -> Self { MessageContent::MessageGame(t.as_ref().clone()) }

  pub fn message_poll<T: AsRef<MessagePoll>>(t: T) -> Self { MessageContent::MessagePoll(t.as_ref().clone()) }

  pub fn message_invoice<T: AsRef<MessageInvoice>>(t: T) -> Self { MessageContent::MessageInvoice(t.as_ref().clone()) }

  pub fn message_call<T: AsRef<MessageCall>>(t: T) -> Self { MessageContent::MessageCall(t.as_ref().clone()) }

  pub fn message_basic_group_chat_create<T: AsRef<MessageBasicGroupChatCreate>>(t: T) -> Self { MessageContent::MessageBasicGroupChatCreate(t.as_ref().clone()) }

  pub fn message_supergroup_chat_create<T: AsRef<MessageSupergroupChatCreate>>(t: T) -> Self { MessageContent::MessageSupergroupChatCreate(t.as_ref().clone()) }

  pub fn message_chat_change_title<T: AsRef<MessageChatChangeTitle>>(t: T) -> Self { MessageContent::MessageChatChangeTitle(t.as_ref().clone()) }

  pub fn message_chat_change_photo<T: AsRef<MessageChatChangePhoto>>(t: T) -> Self { MessageContent::MessageChatChangePhoto(t.as_ref().clone()) }

  pub fn message_chat_delete_photo<T: AsRef<MessageChatDeletePhoto>>(t: T) -> Self { MessageContent::MessageChatDeletePhoto(t.as_ref().clone()) }

  pub fn message_chat_add_members<T: AsRef<MessageChatAddMembers>>(t: T) -> Self { MessageContent::MessageChatAddMembers(t.as_ref().clone()) }

  pub fn message_chat_join_by_link<T: AsRef<MessageChatJoinByLink>>(t: T) -> Self { MessageContent::MessageChatJoinByLink(t.as_ref().clone()) }

  pub fn message_chat_delete_member<T: AsRef<MessageChatDeleteMember>>(t: T) -> Self { MessageContent::MessageChatDeleteMember(t.as_ref().clone()) }

  pub fn message_chat_upgrade_to<T: AsRef<MessageChatUpgradeTo>>(t: T) -> Self { MessageContent::MessageChatUpgradeTo(t.as_ref().clone()) }

  pub fn message_chat_upgrade_from<T: AsRef<MessageChatUpgradeFrom>>(t: T) -> Self { MessageContent::MessageChatUpgradeFrom(t.as_ref().clone()) }

  pub fn message_pin_message<T: AsRef<MessagePinMessage>>(t: T) -> Self { MessageContent::MessagePinMessage(t.as_ref().clone()) }

  pub fn message_screenshot_taken<T: AsRef<MessageScreenshotTaken>>(t: T) -> Self { MessageContent::MessageScreenshotTaken(t.as_ref().clone()) }

  pub fn message_chat_set_ttl<T: AsRef<MessageChatSetTtl>>(t: T) -> Self { MessageContent::MessageChatSetTtl(t.as_ref().clone()) }

  pub fn message_custom_service_action<T: AsRef<MessageCustomServiceAction>>(t: T) -> Self { MessageContent::MessageCustomServiceAction(t.as_ref().clone()) }

  pub fn message_game_score<T: AsRef<MessageGameScore>>(t: T) -> Self { MessageContent::MessageGameScore(t.as_ref().clone()) }

  pub fn message_payment_successful<T: AsRef<MessagePaymentSuccessful>>(t: T) -> Self { MessageContent::MessagePaymentSuccessful(t.as_ref().clone()) }

  pub fn message_payment_successful_bot<T: AsRef<MessagePaymentSuccessfulBot>>(t: T) -> Self { MessageContent::MessagePaymentSuccessfulBot(t.as_ref().clone()) }

  pub fn message_contact_registered<T: AsRef<MessageContactRegistered>>(t: T) -> Self { MessageContent::MessageContactRegistered(t.as_ref().clone()) }

  pub fn message_website_connected<T: AsRef<MessageWebsiteConnected>>(t: T) -> Self { MessageContent::MessageWebsiteConnected(t.as_ref().clone()) }

  pub fn message_passport_data_sent<T: AsRef<MessagePassportDataSent>>(t: T) -> Self { MessageContent::MessagePassportDataSent(t.as_ref().clone()) }

  pub fn message_passport_data_received<T: AsRef<MessagePassportDataReceived>>(t: T) -> Self { MessageContent::MessagePassportDataReceived(t.as_ref().clone()) }

  pub fn message_unsupported<T: AsRef<MessageUnsupported>>(t: T) -> Self { MessageContent::MessageUnsupported(t.as_ref().clone()) }

}

impl AsRef<MessageContent> for MessageContent {
  fn as_ref(&self) -> &MessageContent { self }
}







/// A text message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Text of the message
  text: FormattedText,
  /// A preview of the web page that's mentioned in the text; may be null
  web_page: Option<WebPage>,
  
}

impl RObject for MessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageText" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageText {}



impl MessageText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageTextBuilder {
    let mut inner = MessageText::default();
    inner.td_name = "messageText".to_string();
    RTDMessageTextBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText { &self.text }

  pub fn web_page(&self) -> &Option<WebPage> { &self.web_page }

}

#[doc(hidden)]
pub struct RTDMessageTextBuilder {
  inner: MessageText
}

impl RTDMessageTextBuilder {
  pub fn build(&self) -> MessageText { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn web_page<T: AsRef<WebPage>>(&mut self, web_page: T) -> &mut Self {
    self.inner.web_page = Some(web_page.as_ref().clone());
    self
  }

}

impl AsRef<MessageText> for MessageText {
  fn as_ref(&self) -> &MessageText { self }
}

impl AsRef<MessageText> for RTDMessageTextBuilder {
  fn as_ref(&self) -> &MessageText { &self.inner }
}







/// An animation message (GIF-style).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  animation: Animation,
  /// Animation caption
  caption: FormattedText,
  /// True, if the animation thumbnail must be blurred and the animation must be shown only while tapped
  is_secret: bool,
  
}

impl RObject for MessageAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageAnimation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageAnimation {}



impl MessageAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageAnimationBuilder {
    let mut inner = MessageAnimation::default();
    inner.td_name = "messageAnimation".to_string();
    RTDMessageAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &Animation { &self.animation }

  pub fn caption(&self) -> &FormattedText { &self.caption }

  pub fn is_secret(&self) -> bool { self.is_secret }

}

#[doc(hidden)]
pub struct RTDMessageAnimationBuilder {
  inner: MessageAnimation
}

impl RTDMessageAnimationBuilder {
  pub fn build(&self) -> MessageAnimation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
    self.inner.is_secret = is_secret;
    self
  }

}

impl AsRef<MessageAnimation> for MessageAnimation {
  fn as_ref(&self) -> &MessageAnimation { self }
}

impl AsRef<MessageAnimation> for RTDMessageAnimationBuilder {
  fn as_ref(&self) -> &MessageAnimation { &self.inner }
}







/// An audio message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  audio: Audio,
  /// Audio caption
  caption: FormattedText,
  
}

impl RObject for MessageAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageAudio" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageAudio {}



impl MessageAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageAudioBuilder {
    let mut inner = MessageAudio::default();
    inner.td_name = "messageAudio".to_string();
    RTDMessageAudioBuilder { inner }
  }

  pub fn audio(&self) -> &Audio { &self.audio }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDMessageAudioBuilder {
  inner: MessageAudio
}

impl RTDMessageAudioBuilder {
  pub fn build(&self) -> MessageAudio { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<MessageAudio> for MessageAudio {
  fn as_ref(&self) -> &MessageAudio { self }
}

impl AsRef<MessageAudio> for RTDMessageAudioBuilder {
  fn as_ref(&self) -> &MessageAudio { &self.inner }
}







/// A document message (general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  document: Document,
  /// Document caption
  caption: FormattedText,
  
}

impl RObject for MessageDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageDocument" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageDocument {}



impl MessageDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageDocumentBuilder {
    let mut inner = MessageDocument::default();
    inner.td_name = "messageDocument".to_string();
    RTDMessageDocumentBuilder { inner }
  }

  pub fn document(&self) -> &Document { &self.document }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDMessageDocumentBuilder {
  inner: MessageDocument
}

impl RTDMessageDocumentBuilder {
  pub fn build(&self) -> MessageDocument { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<MessageDocument> for MessageDocument {
  fn as_ref(&self) -> &MessageDocument { self }
}

impl AsRef<MessageDocument> for RTDMessageDocumentBuilder {
  fn as_ref(&self) -> &MessageDocument { &self.inner }
}







/// A photo message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  photo: Photo,
  /// Photo caption
  caption: FormattedText,
  /// True, if the photo must be blurred and must be shown only while tapped
  is_secret: bool,
  
}

impl RObject for MessagePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessagePhoto {}



impl MessagePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePhotoBuilder {
    let mut inner = MessagePhoto::default();
    inner.td_name = "messagePhoto".to_string();
    RTDMessagePhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Photo { &self.photo }

  pub fn caption(&self) -> &FormattedText { &self.caption }

  pub fn is_secret(&self) -> bool { self.is_secret }

}

#[doc(hidden)]
pub struct RTDMessagePhotoBuilder {
  inner: MessagePhoto
}

impl RTDMessagePhotoBuilder {
  pub fn build(&self) -> MessagePhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
    self.inner.is_secret = is_secret;
    self
  }

}

impl AsRef<MessagePhoto> for MessagePhoto {
  fn as_ref(&self) -> &MessagePhoto { self }
}

impl AsRef<MessagePhoto> for RTDMessagePhotoBuilder {
  fn as_ref(&self) -> &MessagePhoto { &self.inner }
}







/// An expired photo message (self-destructed after TTL has elapsed)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageExpiredPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageExpiredPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageExpiredPhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageExpiredPhoto {}



impl MessageExpiredPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageExpiredPhotoBuilder {
    let mut inner = MessageExpiredPhoto::default();
    inner.td_name = "messageExpiredPhoto".to_string();
    RTDMessageExpiredPhotoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageExpiredPhotoBuilder {
  inner: MessageExpiredPhoto
}

impl RTDMessageExpiredPhotoBuilder {
  pub fn build(&self) -> MessageExpiredPhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageExpiredPhoto> for MessageExpiredPhoto {
  fn as_ref(&self) -> &MessageExpiredPhoto { self }
}

impl AsRef<MessageExpiredPhoto> for RTDMessageExpiredPhotoBuilder {
  fn as_ref(&self) -> &MessageExpiredPhoto { &self.inner }
}







/// A sticker message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  sticker: Sticker,
  
}

impl RObject for MessageSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSticker" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageSticker {}



impl MessageSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageStickerBuilder {
    let mut inner = MessageSticker::default();
    inner.td_name = "messageSticker".to_string();
    RTDMessageStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &Sticker { &self.sticker }

}

#[doc(hidden)]
pub struct RTDMessageStickerBuilder {
  inner: MessageSticker
}

impl RTDMessageStickerBuilder {
  pub fn build(&self) -> MessageSticker { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<MessageSticker> for MessageSticker {
  fn as_ref(&self) -> &MessageSticker { self }
}

impl AsRef<MessageSticker> for RTDMessageStickerBuilder {
  fn as_ref(&self) -> &MessageSticker { &self.inner }
}







/// A video message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  video: Video,
  /// Video caption
  caption: FormattedText,
  /// True, if the video thumbnail must be blurred and the video must be shown only while tapped
  is_secret: bool,
  
}

impl RObject for MessageVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVideo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageVideo {}



impl MessageVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageVideoBuilder {
    let mut inner = MessageVideo::default();
    inner.td_name = "messageVideo".to_string();
    RTDMessageVideoBuilder { inner }
  }

  pub fn video(&self) -> &Video { &self.video }

  pub fn caption(&self) -> &FormattedText { &self.caption }

  pub fn is_secret(&self) -> bool { self.is_secret }

}

#[doc(hidden)]
pub struct RTDMessageVideoBuilder {
  inner: MessageVideo
}

impl RTDMessageVideoBuilder {
  pub fn build(&self) -> MessageVideo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
    self.inner.is_secret = is_secret;
    self
  }

}

impl AsRef<MessageVideo> for MessageVideo {
  fn as_ref(&self) -> &MessageVideo { self }
}

impl AsRef<MessageVideo> for RTDMessageVideoBuilder {
  fn as_ref(&self) -> &MessageVideo { &self.inner }
}







/// An expired video message (self-destructed after TTL has elapsed)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageExpiredVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageExpiredVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageExpiredVideo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageExpiredVideo {}



impl MessageExpiredVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageExpiredVideoBuilder {
    let mut inner = MessageExpiredVideo::default();
    inner.td_name = "messageExpiredVideo".to_string();
    RTDMessageExpiredVideoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageExpiredVideoBuilder {
  inner: MessageExpiredVideo
}

impl RTDMessageExpiredVideoBuilder {
  pub fn build(&self) -> MessageExpiredVideo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageExpiredVideo> for MessageExpiredVideo {
  fn as_ref(&self) -> &MessageExpiredVideo { self }
}

impl AsRef<MessageExpiredVideo> for RTDMessageExpiredVideoBuilder {
  fn as_ref(&self) -> &MessageExpiredVideo { &self.inner }
}







/// A video note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  video_note: VideoNote,
  /// True, if at least one of the recipients has viewed the video note
  is_viewed: bool,
  /// True, if the video note thumbnail must be blurred and the video note must be shown only while tapped
  is_secret: bool,
  
}

impl RObject for MessageVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVideoNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageVideoNote {}



impl MessageVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageVideoNoteBuilder {
    let mut inner = MessageVideoNote::default();
    inner.td_name = "messageVideoNote".to_string();
    RTDMessageVideoNoteBuilder { inner }
  }

  pub fn video_note(&self) -> &VideoNote { &self.video_note }

  pub fn is_viewed(&self) -> bool { self.is_viewed }

  pub fn is_secret(&self) -> bool { self.is_secret }

}

#[doc(hidden)]
pub struct RTDMessageVideoNoteBuilder {
  inner: MessageVideoNote
}

impl RTDMessageVideoNoteBuilder {
  pub fn build(&self) -> MessageVideoNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn video_note<T: AsRef<VideoNote>>(&mut self, video_note: T) -> &mut Self {
    self.inner.video_note = video_note.as_ref().clone();
    self
  }

   
  pub fn is_viewed(&mut self, is_viewed: bool) -> &mut Self {
    self.inner.is_viewed = is_viewed;
    self
  }

   
  pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
    self.inner.is_secret = is_secret;
    self
  }

}

impl AsRef<MessageVideoNote> for MessageVideoNote {
  fn as_ref(&self) -> &MessageVideoNote { self }
}

impl AsRef<MessageVideoNote> for RTDMessageVideoNoteBuilder {
  fn as_ref(&self) -> &MessageVideoNote { &self.inner }
}







/// A voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  voice_note: VoiceNote,
  /// Voice note caption
  caption: FormattedText,
  /// True, if at least one of the recipients has listened to the voice note
  is_listened: bool,
  
}

impl RObject for MessageVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVoiceNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageVoiceNote {}



impl MessageVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageVoiceNoteBuilder {
    let mut inner = MessageVoiceNote::default();
    inner.td_name = "messageVoiceNote".to_string();
    RTDMessageVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &VoiceNote { &self.voice_note }

  pub fn caption(&self) -> &FormattedText { &self.caption }

  pub fn is_listened(&self) -> bool { self.is_listened }

}

#[doc(hidden)]
pub struct RTDMessageVoiceNoteBuilder {
  inner: MessageVoiceNote
}

impl RTDMessageVoiceNoteBuilder {
  pub fn build(&self) -> MessageVoiceNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = voice_note.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn is_listened(&mut self, is_listened: bool) -> &mut Self {
    self.inner.is_listened = is_listened;
    self
  }

}

impl AsRef<MessageVoiceNote> for MessageVoiceNote {
  fn as_ref(&self) -> &MessageVoiceNote { self }
}

impl AsRef<MessageVoiceNote> for RTDMessageVoiceNoteBuilder {
  fn as_ref(&self) -> &MessageVoiceNote { &self.inner }
}







/// A message with a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  location: Location,
  /// Time relative to the message sent date until which the location can be updated, in seconds
  live_period: i64,
  /// Left time for which the location can be updated, in seconds. updateMessageContent is not sent when this field changes
  expires_in: i64,
  
}

impl RObject for MessageLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageLocation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageLocation {}



impl MessageLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageLocationBuilder {
    let mut inner = MessageLocation::default();
    inner.td_name = "messageLocation".to_string();
    RTDMessageLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location { &self.location }

  pub fn live_period(&self) -> i64 { self.live_period }

  pub fn expires_in(&self) -> i64 { self.expires_in }

}

#[doc(hidden)]
pub struct RTDMessageLocationBuilder {
  inner: MessageLocation
}

impl RTDMessageLocationBuilder {
  pub fn build(&self) -> MessageLocation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

   
  pub fn live_period(&mut self, live_period: i64) -> &mut Self {
    self.inner.live_period = live_period;
    self
  }

   
  pub fn expires_in(&mut self, expires_in: i64) -> &mut Self {
    self.inner.expires_in = expires_in;
    self
  }

}

impl AsRef<MessageLocation> for MessageLocation {
  fn as_ref(&self) -> &MessageLocation { self }
}

impl AsRef<MessageLocation> for RTDMessageLocationBuilder {
  fn as_ref(&self) -> &MessageLocation { &self.inner }
}







/// A message with information about a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  venue: Venue,
  
}

impl RObject for MessageVenue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVenue" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageVenue {}



impl MessageVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageVenueBuilder {
    let mut inner = MessageVenue::default();
    inner.td_name = "messageVenue".to_string();
    RTDMessageVenueBuilder { inner }
  }

  pub fn venue(&self) -> &Venue { &self.venue }

}

#[doc(hidden)]
pub struct RTDMessageVenueBuilder {
  inner: MessageVenue
}

impl RTDMessageVenueBuilder {
  pub fn build(&self) -> MessageVenue { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
    self.inner.venue = venue.as_ref().clone();
    self
  }

}

impl AsRef<MessageVenue> for MessageVenue {
  fn as_ref(&self) -> &MessageVenue { self }
}

impl AsRef<MessageVenue> for RTDMessageVenueBuilder {
  fn as_ref(&self) -> &MessageVenue { &self.inner }
}







/// A message with a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message content
  contact: Contact,
  
}

impl RObject for MessageContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageContact" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageContact {}



impl MessageContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageContactBuilder {
    let mut inner = MessageContact::default();
    inner.td_name = "messageContact".to_string();
    RTDMessageContactBuilder { inner }
  }

  pub fn contact(&self) -> &Contact { &self.contact }

}

#[doc(hidden)]
pub struct RTDMessageContactBuilder {
  inner: MessageContact
}

impl RTDMessageContactBuilder {
  pub fn build(&self) -> MessageContact { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
    self.inner.contact = contact.as_ref().clone();
    self
  }

}

impl AsRef<MessageContact> for MessageContact {
  fn as_ref(&self) -> &MessageContact { self }
}

impl AsRef<MessageContact> for RTDMessageContactBuilder {
  fn as_ref(&self) -> &MessageContact { &self.inner }
}







/// A message with a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Game
  game: Game,
  
}

impl RObject for MessageGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageGame" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageGame {}



impl MessageGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageGameBuilder {
    let mut inner = MessageGame::default();
    inner.td_name = "messageGame".to_string();
    RTDMessageGameBuilder { inner }
  }

  pub fn game(&self) -> &Game { &self.game }

}

#[doc(hidden)]
pub struct RTDMessageGameBuilder {
  inner: MessageGame
}

impl RTDMessageGameBuilder {
  pub fn build(&self) -> MessageGame { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn game<T: AsRef<Game>>(&mut self, game: T) -> &mut Self {
    self.inner.game = game.as_ref().clone();
    self
  }

}

impl AsRef<MessageGame> for MessageGame {
  fn as_ref(&self) -> &MessageGame { self }
}

impl AsRef<MessageGame> for RTDMessageGameBuilder {
  fn as_ref(&self) -> &MessageGame { &self.inner }
}







/// A message with a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Poll
  poll: Poll,
  
}

impl RObject for MessagePoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePoll" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessagePoll {}



impl MessagePoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePollBuilder {
    let mut inner = MessagePoll::default();
    inner.td_name = "messagePoll".to_string();
    RTDMessagePollBuilder { inner }
  }

  pub fn poll(&self) -> &Poll { &self.poll }

}

#[doc(hidden)]
pub struct RTDMessagePollBuilder {
  inner: MessagePoll
}

impl RTDMessagePollBuilder {
  pub fn build(&self) -> MessagePoll { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn poll<T: AsRef<Poll>>(&mut self, poll: T) -> &mut Self {
    self.inner.poll = poll.as_ref().clone();
    self
  }

}

impl AsRef<MessagePoll> for MessagePoll {
  fn as_ref(&self) -> &MessagePoll { self }
}

impl AsRef<MessagePoll> for RTDMessagePollBuilder {
  fn as_ref(&self) -> &MessagePoll { &self.inner }
}







/// A message with an invoice from a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Product title
  title: String,
  /// A message with an invoice from a bot
  description: String,
  /// Product photo; may be null
  photo: Option<Photo>,
  /// Currency for the product price
  currency: String,
  /// Product total price in the minimal quantity of the currency
  total_amount: i64,
  /// Unique invoice bot start_parameter. To share an invoice use the URL https://t.me/{bot_username}?start={start_parameter}
  start_parameter: String,
  /// True, if the invoice is a test invoice
  is_test: bool,
  /// True, if the shipping address should be specified
  need_shipping_address: bool,
  /// The identifier of the message with the receipt, after the product has been purchased
  receipt_message_id: i64,
  
}

impl RObject for MessageInvoice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageInvoice" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageInvoice {}



impl MessageInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageInvoiceBuilder {
    let mut inner = MessageInvoice::default();
    inner.td_name = "messageInvoice".to_string();
    RTDMessageInvoiceBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

  pub fn photo(&self) -> &Option<Photo> { &self.photo }

  pub fn currency(&self) -> &String { &self.currency }

  pub fn total_amount(&self) -> i64 { self.total_amount }

  pub fn start_parameter(&self) -> &String { &self.start_parameter }

  pub fn is_test(&self) -> bool { self.is_test }

  pub fn need_shipping_address(&self) -> bool { self.need_shipping_address }

  pub fn receipt_message_id(&self) -> i64 { self.receipt_message_id }

}

#[doc(hidden)]
pub struct RTDMessageInvoiceBuilder {
  inner: MessageInvoice
}

impl RTDMessageInvoiceBuilder {
  pub fn build(&self) -> MessageInvoice { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

   
  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

   
  pub fn total_amount(&mut self, total_amount: i64) -> &mut Self {
    self.inner.total_amount = total_amount;
    self
  }

   
  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

   
  pub fn is_test(&mut self, is_test: bool) -> &mut Self {
    self.inner.is_test = is_test;
    self
  }

   
  pub fn need_shipping_address(&mut self, need_shipping_address: bool) -> &mut Self {
    self.inner.need_shipping_address = need_shipping_address;
    self
  }

   
  pub fn receipt_message_id(&mut self, receipt_message_id: i64) -> &mut Self {
    self.inner.receipt_message_id = receipt_message_id;
    self
  }

}

impl AsRef<MessageInvoice> for MessageInvoice {
  fn as_ref(&self) -> &MessageInvoice { self }
}

impl AsRef<MessageInvoice> for RTDMessageInvoiceBuilder {
  fn as_ref(&self) -> &MessageInvoice { &self.inner }
}







/// A message with information about an ended call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Reason why the call was discarded
  discard_reason: CallDiscardReason,
  /// Call duration, in seconds
  duration: i64,
  
}

impl RObject for MessageCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageCall" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageCall {}



impl MessageCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageCallBuilder {
    let mut inner = MessageCall::default();
    inner.td_name = "messageCall".to_string();
    RTDMessageCallBuilder { inner }
  }

  pub fn discard_reason(&self) -> &CallDiscardReason { &self.discard_reason }

  pub fn duration(&self) -> i64 { self.duration }

}

#[doc(hidden)]
pub struct RTDMessageCallBuilder {
  inner: MessageCall
}

impl RTDMessageCallBuilder {
  pub fn build(&self) -> MessageCall { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn discard_reason<T: AsRef<CallDiscardReason>>(&mut self, discard_reason: T) -> &mut Self {
    self.inner.discard_reason = discard_reason.as_ref().clone();
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

}

impl AsRef<MessageCall> for MessageCall {
  fn as_ref(&self) -> &MessageCall { self }
}

impl AsRef<MessageCall> for RTDMessageCallBuilder {
  fn as_ref(&self) -> &MessageCall { &self.inner }
}







/// A newly created basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageBasicGroupChatCreate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Title of the basic group
  title: String,
  /// User identifiers of members in the basic group
  member_user_ids: Vec<i64>,
  
}

impl RObject for MessageBasicGroupChatCreate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageBasicGroupChatCreate" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageBasicGroupChatCreate {}



impl MessageBasicGroupChatCreate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageBasicGroupChatCreateBuilder {
    let mut inner = MessageBasicGroupChatCreate::default();
    inner.td_name = "messageBasicGroupChatCreate".to_string();
    RTDMessageBasicGroupChatCreateBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

  pub fn member_user_ids(&self) -> &Vec<i64> { &self.member_user_ids }

}

#[doc(hidden)]
pub struct RTDMessageBasicGroupChatCreateBuilder {
  inner: MessageBasicGroupChatCreate
}

impl RTDMessageBasicGroupChatCreateBuilder {
  pub fn build(&self) -> MessageBasicGroupChatCreate { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn member_user_ids(&mut self, member_user_ids: Vec<i64>) -> &mut Self {
    self.inner.member_user_ids = member_user_ids;
    self
  }

}

impl AsRef<MessageBasicGroupChatCreate> for MessageBasicGroupChatCreate {
  fn as_ref(&self) -> &MessageBasicGroupChatCreate { self }
}

impl AsRef<MessageBasicGroupChatCreate> for RTDMessageBasicGroupChatCreateBuilder {
  fn as_ref(&self) -> &MessageBasicGroupChatCreate { &self.inner }
}







/// A newly created supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSupergroupChatCreate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Title of the supergroup or channel
  title: String,
  
}

impl RObject for MessageSupergroupChatCreate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSupergroupChatCreate" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageSupergroupChatCreate {}



impl MessageSupergroupChatCreate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSupergroupChatCreateBuilder {
    let mut inner = MessageSupergroupChatCreate::default();
    inner.td_name = "messageSupergroupChatCreate".to_string();
    RTDMessageSupergroupChatCreateBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDMessageSupergroupChatCreateBuilder {
  inner: MessageSupergroupChatCreate
}

impl RTDMessageSupergroupChatCreateBuilder {
  pub fn build(&self) -> MessageSupergroupChatCreate { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<MessageSupergroupChatCreate> for MessageSupergroupChatCreate {
  fn as_ref(&self) -> &MessageSupergroupChatCreate { self }
}

impl AsRef<MessageSupergroupChatCreate> for RTDMessageSupergroupChatCreateBuilder {
  fn as_ref(&self) -> &MessageSupergroupChatCreate { &self.inner }
}







/// An updated chat title
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatChangeTitle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// New chat title
  title: String,
  
}

impl RObject for MessageChatChangeTitle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatChangeTitle" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatChangeTitle {}



impl MessageChatChangeTitle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatChangeTitleBuilder {
    let mut inner = MessageChatChangeTitle::default();
    inner.td_name = "messageChatChangeTitle".to_string();
    RTDMessageChatChangeTitleBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDMessageChatChangeTitleBuilder {
  inner: MessageChatChangeTitle
}

impl RTDMessageChatChangeTitleBuilder {
  pub fn build(&self) -> MessageChatChangeTitle { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<MessageChatChangeTitle> for MessageChatChangeTitle {
  fn as_ref(&self) -> &MessageChatChangeTitle { self }
}

impl AsRef<MessageChatChangeTitle> for RTDMessageChatChangeTitleBuilder {
  fn as_ref(&self) -> &MessageChatChangeTitle { &self.inner }
}







/// An updated chat photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatChangePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// New chat photo
  photo: Photo,
  
}

impl RObject for MessageChatChangePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatChangePhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatChangePhoto {}



impl MessageChatChangePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatChangePhotoBuilder {
    let mut inner = MessageChatChangePhoto::default();
    inner.td_name = "messageChatChangePhoto".to_string();
    RTDMessageChatChangePhotoBuilder { inner }
  }

  pub fn photo(&self) -> &Photo { &self.photo }

}

#[doc(hidden)]
pub struct RTDMessageChatChangePhotoBuilder {
  inner: MessageChatChangePhoto
}

impl RTDMessageChatChangePhotoBuilder {
  pub fn build(&self) -> MessageChatChangePhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

}

impl AsRef<MessageChatChangePhoto> for MessageChatChangePhoto {
  fn as_ref(&self) -> &MessageChatChangePhoto { self }
}

impl AsRef<MessageChatChangePhoto> for RTDMessageChatChangePhotoBuilder {
  fn as_ref(&self) -> &MessageChatChangePhoto { &self.inner }
}







/// A deleted chat photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatDeletePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageChatDeletePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatDeletePhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatDeletePhoto {}



impl MessageChatDeletePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatDeletePhotoBuilder {
    let mut inner = MessageChatDeletePhoto::default();
    inner.td_name = "messageChatDeletePhoto".to_string();
    RTDMessageChatDeletePhotoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageChatDeletePhotoBuilder {
  inner: MessageChatDeletePhoto
}

impl RTDMessageChatDeletePhotoBuilder {
  pub fn build(&self) -> MessageChatDeletePhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageChatDeletePhoto> for MessageChatDeletePhoto {
  fn as_ref(&self) -> &MessageChatDeletePhoto { self }
}

impl AsRef<MessageChatDeletePhoto> for RTDMessageChatDeletePhotoBuilder {
  fn as_ref(&self) -> &MessageChatDeletePhoto { &self.inner }
}







/// New chat members were added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatAddMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// User identifiers of the new members
  member_user_ids: Vec<i64>,
  
}

impl RObject for MessageChatAddMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatAddMembers" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatAddMembers {}



impl MessageChatAddMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatAddMembersBuilder {
    let mut inner = MessageChatAddMembers::default();
    inner.td_name = "messageChatAddMembers".to_string();
    RTDMessageChatAddMembersBuilder { inner }
  }

  pub fn member_user_ids(&self) -> &Vec<i64> { &self.member_user_ids }

}

#[doc(hidden)]
pub struct RTDMessageChatAddMembersBuilder {
  inner: MessageChatAddMembers
}

impl RTDMessageChatAddMembersBuilder {
  pub fn build(&self) -> MessageChatAddMembers { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn member_user_ids(&mut self, member_user_ids: Vec<i64>) -> &mut Self {
    self.inner.member_user_ids = member_user_ids;
    self
  }

}

impl AsRef<MessageChatAddMembers> for MessageChatAddMembers {
  fn as_ref(&self) -> &MessageChatAddMembers { self }
}

impl AsRef<MessageChatAddMembers> for RTDMessageChatAddMembersBuilder {
  fn as_ref(&self) -> &MessageChatAddMembers { &self.inner }
}







/// A new member joined the chat by invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatJoinByLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageChatJoinByLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatJoinByLink" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatJoinByLink {}



impl MessageChatJoinByLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatJoinByLinkBuilder {
    let mut inner = MessageChatJoinByLink::default();
    inner.td_name = "messageChatJoinByLink".to_string();
    RTDMessageChatJoinByLinkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageChatJoinByLinkBuilder {
  inner: MessageChatJoinByLink
}

impl RTDMessageChatJoinByLinkBuilder {
  pub fn build(&self) -> MessageChatJoinByLink { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageChatJoinByLink> for MessageChatJoinByLink {
  fn as_ref(&self) -> &MessageChatJoinByLink { self }
}

impl AsRef<MessageChatJoinByLink> for RTDMessageChatJoinByLinkBuilder {
  fn as_ref(&self) -> &MessageChatJoinByLink { &self.inner }
}







/// A chat member was deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatDeleteMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// User identifier of the deleted chat member
  user_id: i64,
  
}

impl RObject for MessageChatDeleteMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatDeleteMember" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatDeleteMember {}



impl MessageChatDeleteMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatDeleteMemberBuilder {
    let mut inner = MessageChatDeleteMember::default();
    inner.td_name = "messageChatDeleteMember".to_string();
    RTDMessageChatDeleteMemberBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDMessageChatDeleteMemberBuilder {
  inner: MessageChatDeleteMember
}

impl RTDMessageChatDeleteMemberBuilder {
  pub fn build(&self) -> MessageChatDeleteMember { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<MessageChatDeleteMember> for MessageChatDeleteMember {
  fn as_ref(&self) -> &MessageChatDeleteMember { self }
}

impl AsRef<MessageChatDeleteMember> for RTDMessageChatDeleteMemberBuilder {
  fn as_ref(&self) -> &MessageChatDeleteMember { &self.inner }
}







/// A basic group was upgraded to a supergroup and was deactivated as the result
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatUpgradeTo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Identifier of the supergroup to which the basic group was upgraded
  supergroup_id: i64,
  
}

impl RObject for MessageChatUpgradeTo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatUpgradeTo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatUpgradeTo {}



impl MessageChatUpgradeTo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatUpgradeToBuilder {
    let mut inner = MessageChatUpgradeTo::default();
    inner.td_name = "messageChatUpgradeTo".to_string();
    RTDMessageChatUpgradeToBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

}

#[doc(hidden)]
pub struct RTDMessageChatUpgradeToBuilder {
  inner: MessageChatUpgradeTo
}

impl RTDMessageChatUpgradeToBuilder {
  pub fn build(&self) -> MessageChatUpgradeTo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

}

impl AsRef<MessageChatUpgradeTo> for MessageChatUpgradeTo {
  fn as_ref(&self) -> &MessageChatUpgradeTo { self }
}

impl AsRef<MessageChatUpgradeTo> for RTDMessageChatUpgradeToBuilder {
  fn as_ref(&self) -> &MessageChatUpgradeTo { &self.inner }
}







/// A supergroup has been created from a basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatUpgradeFrom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Title of the newly created supergroup
  title: String,
  /// The identifier of the original basic group
  basic_group_id: i64,
  
}

impl RObject for MessageChatUpgradeFrom {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatUpgradeFrom" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatUpgradeFrom {}



impl MessageChatUpgradeFrom {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatUpgradeFromBuilder {
    let mut inner = MessageChatUpgradeFrom::default();
    inner.td_name = "messageChatUpgradeFrom".to_string();
    RTDMessageChatUpgradeFromBuilder { inner }
  }

  pub fn title(&self) -> &String { &self.title }

  pub fn basic_group_id(&self) -> i64 { self.basic_group_id }

}

#[doc(hidden)]
pub struct RTDMessageChatUpgradeFromBuilder {
  inner: MessageChatUpgradeFrom
}

impl RTDMessageChatUpgradeFromBuilder {
  pub fn build(&self) -> MessageChatUpgradeFrom { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
    self.inner.basic_group_id = basic_group_id;
    self
  }

}

impl AsRef<MessageChatUpgradeFrom> for MessageChatUpgradeFrom {
  fn as_ref(&self) -> &MessageChatUpgradeFrom { self }
}

impl AsRef<MessageChatUpgradeFrom> for RTDMessageChatUpgradeFromBuilder {
  fn as_ref(&self) -> &MessageChatUpgradeFrom { &self.inner }
}







/// A message has been pinned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePinMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Identifier of the pinned message, can be an identifier of a deleted message or 0
  message_id: i64,
  
}

impl RObject for MessagePinMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePinMessage" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessagePinMessage {}



impl MessagePinMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePinMessageBuilder {
    let mut inner = MessagePinMessage::default();
    inner.td_name = "messagePinMessage".to_string();
    RTDMessagePinMessageBuilder { inner }
  }

  pub fn message_id(&self) -> i64 { self.message_id }

}

#[doc(hidden)]
pub struct RTDMessagePinMessageBuilder {
  inner: MessagePinMessage
}

impl RTDMessagePinMessageBuilder {
  pub fn build(&self) -> MessagePinMessage { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

}

impl AsRef<MessagePinMessage> for MessagePinMessage {
  fn as_ref(&self) -> &MessagePinMessage { self }
}

impl AsRef<MessagePinMessage> for RTDMessagePinMessageBuilder {
  fn as_ref(&self) -> &MessagePinMessage { &self.inner }
}







/// A screenshot of a message in the chat has been taken
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageScreenshotTaken {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageScreenshotTaken {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageScreenshotTaken" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageScreenshotTaken {}



impl MessageScreenshotTaken {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageScreenshotTakenBuilder {
    let mut inner = MessageScreenshotTaken::default();
    inner.td_name = "messageScreenshotTaken".to_string();
    RTDMessageScreenshotTakenBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageScreenshotTakenBuilder {
  inner: MessageScreenshotTaken
}

impl RTDMessageScreenshotTakenBuilder {
  pub fn build(&self) -> MessageScreenshotTaken { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageScreenshotTaken> for MessageScreenshotTaken {
  fn as_ref(&self) -> &MessageScreenshotTaken { self }
}

impl AsRef<MessageScreenshotTaken> for RTDMessageScreenshotTakenBuilder {
  fn as_ref(&self) -> &MessageScreenshotTaken { &self.inner }
}







/// The TTL (Time To Live) setting messages in a secret chat has been changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageChatSetTtl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// New TTL
  ttl: i64,
  
}

impl RObject for MessageChatSetTtl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatSetTtl" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageChatSetTtl {}



impl MessageChatSetTtl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageChatSetTtlBuilder {
    let mut inner = MessageChatSetTtl::default();
    inner.td_name = "messageChatSetTtl".to_string();
    RTDMessageChatSetTtlBuilder { inner }
  }

  pub fn ttl(&self) -> i64 { self.ttl }

}

#[doc(hidden)]
pub struct RTDMessageChatSetTtlBuilder {
  inner: MessageChatSetTtl
}

impl RTDMessageChatSetTtlBuilder {
  pub fn build(&self) -> MessageChatSetTtl { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn ttl(&mut self, ttl: i64) -> &mut Self {
    self.inner.ttl = ttl;
    self
  }

}

impl AsRef<MessageChatSetTtl> for MessageChatSetTtl {
  fn as_ref(&self) -> &MessageChatSetTtl { self }
}

impl AsRef<MessageChatSetTtl> for RTDMessageChatSetTtlBuilder {
  fn as_ref(&self) -> &MessageChatSetTtl { &self.inner }
}







/// A non-standard action has happened in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCustomServiceAction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Message text to be shown in the chat
  text: String,
  
}

impl RObject for MessageCustomServiceAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageCustomServiceAction" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageCustomServiceAction {}



impl MessageCustomServiceAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageCustomServiceActionBuilder {
    let mut inner = MessageCustomServiceAction::default();
    inner.td_name = "messageCustomServiceAction".to_string();
    RTDMessageCustomServiceActionBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

}

#[doc(hidden)]
pub struct RTDMessageCustomServiceActionBuilder {
  inner: MessageCustomServiceAction
}

impl RTDMessageCustomServiceActionBuilder {
  pub fn build(&self) -> MessageCustomServiceAction { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

}

impl AsRef<MessageCustomServiceAction> for MessageCustomServiceAction {
  fn as_ref(&self) -> &MessageCustomServiceAction { self }
}

impl AsRef<MessageCustomServiceAction> for RTDMessageCustomServiceActionBuilder {
  fn as_ref(&self) -> &MessageCustomServiceAction { &self.inner }
}







/// A new high score was achieved in a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageGameScore {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Identifier of the message with the game, can be an identifier of a deleted message
  game_message_id: i64,
  /// Identifier of the game; may be different from the games presented in the message with the game
  game_id: isize,
  /// New score
  score: i64,
  
}

impl RObject for MessageGameScore {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageGameScore" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageGameScore {}



impl MessageGameScore {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageGameScoreBuilder {
    let mut inner = MessageGameScore::default();
    inner.td_name = "messageGameScore".to_string();
    RTDMessageGameScoreBuilder { inner }
  }

  pub fn game_message_id(&self) -> i64 { self.game_message_id }

  pub fn game_id(&self) -> isize { self.game_id }

  pub fn score(&self) -> i64 { self.score }

}

#[doc(hidden)]
pub struct RTDMessageGameScoreBuilder {
  inner: MessageGameScore
}

impl RTDMessageGameScoreBuilder {
  pub fn build(&self) -> MessageGameScore { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn game_message_id(&mut self, game_message_id: i64) -> &mut Self {
    self.inner.game_message_id = game_message_id;
    self
  }

   
  pub fn game_id(&mut self, game_id: isize) -> &mut Self {
    self.inner.game_id = game_id;
    self
  }

   
  pub fn score(&mut self, score: i64) -> &mut Self {
    self.inner.score = score;
    self
  }

}

impl AsRef<MessageGameScore> for MessageGameScore {
  fn as_ref(&self) -> &MessageGameScore { self }
}

impl AsRef<MessageGameScore> for RTDMessageGameScoreBuilder {
  fn as_ref(&self) -> &MessageGameScore { &self.inner }
}







/// A payment has been completed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePaymentSuccessful {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Identifier of the message with the corresponding invoice; can be an identifier of a deleted message
  invoice_message_id: i64,
  /// Currency for the price of the product
  currency: String,
  /// Total price for the product, in the minimal quantity of the currency
  total_amount: i64,
  
}

impl RObject for MessagePaymentSuccessful {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePaymentSuccessful" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessagePaymentSuccessful {}



impl MessagePaymentSuccessful {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePaymentSuccessfulBuilder {
    let mut inner = MessagePaymentSuccessful::default();
    inner.td_name = "messagePaymentSuccessful".to_string();
    RTDMessagePaymentSuccessfulBuilder { inner }
  }

  pub fn invoice_message_id(&self) -> i64 { self.invoice_message_id }

  pub fn currency(&self) -> &String { &self.currency }

  pub fn total_amount(&self) -> i64 { self.total_amount }

}

#[doc(hidden)]
pub struct RTDMessagePaymentSuccessfulBuilder {
  inner: MessagePaymentSuccessful
}

impl RTDMessagePaymentSuccessfulBuilder {
  pub fn build(&self) -> MessagePaymentSuccessful { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn invoice_message_id(&mut self, invoice_message_id: i64) -> &mut Self {
    self.inner.invoice_message_id = invoice_message_id;
    self
  }

   
  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

   
  pub fn total_amount(&mut self, total_amount: i64) -> &mut Self {
    self.inner.total_amount = total_amount;
    self
  }

}

impl AsRef<MessagePaymentSuccessful> for MessagePaymentSuccessful {
  fn as_ref(&self) -> &MessagePaymentSuccessful { self }
}

impl AsRef<MessagePaymentSuccessful> for RTDMessagePaymentSuccessfulBuilder {
  fn as_ref(&self) -> &MessagePaymentSuccessful { &self.inner }
}







/// A payment has been completed; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePaymentSuccessfulBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Identifier of the message with the corresponding invoice; can be an identifier of a deleted message
  invoice_message_id: i64,
  /// Currency for price of the product
  currency: String,
  /// Total price for the product, in the minimal quantity of the currency
  total_amount: i64,
  /// Invoice payload
  invoice_payload: String,
  /// Identifier of the shipping option chosen by the user; may be empty if not applicable
  shipping_option_id: String,
  /// Information about the order; may be null
  order_info: Option<OrderInfo>,
  /// Telegram payment identifier
  telegram_payment_charge_id: String,
  /// Provider payment identifier
  provider_payment_charge_id: String,
  
}

impl RObject for MessagePaymentSuccessfulBot {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePaymentSuccessfulBot" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessagePaymentSuccessfulBot {}



impl MessagePaymentSuccessfulBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePaymentSuccessfulBotBuilder {
    let mut inner = MessagePaymentSuccessfulBot::default();
    inner.td_name = "messagePaymentSuccessfulBot".to_string();
    RTDMessagePaymentSuccessfulBotBuilder { inner }
  }

  pub fn invoice_message_id(&self) -> i64 { self.invoice_message_id }

  pub fn currency(&self) -> &String { &self.currency }

  pub fn total_amount(&self) -> i64 { self.total_amount }

  pub fn invoice_payload(&self) -> &String { &self.invoice_payload }

  pub fn shipping_option_id(&self) -> &String { &self.shipping_option_id }

  pub fn order_info(&self) -> &Option<OrderInfo> { &self.order_info }

  pub fn telegram_payment_charge_id(&self) -> &String { &self.telegram_payment_charge_id }

  pub fn provider_payment_charge_id(&self) -> &String { &self.provider_payment_charge_id }

}

#[doc(hidden)]
pub struct RTDMessagePaymentSuccessfulBotBuilder {
  inner: MessagePaymentSuccessfulBot
}

impl RTDMessagePaymentSuccessfulBotBuilder {
  pub fn build(&self) -> MessagePaymentSuccessfulBot { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn invoice_message_id(&mut self, invoice_message_id: i64) -> &mut Self {
    self.inner.invoice_message_id = invoice_message_id;
    self
  }

   
  pub fn currency<T: AsRef<str>>(&mut self, currency: T) -> &mut Self {
    self.inner.currency = currency.as_ref().to_string();
    self
  }

   
  pub fn total_amount(&mut self, total_amount: i64) -> &mut Self {
    self.inner.total_amount = total_amount;
    self
  }

   
  pub fn invoice_payload<T: AsRef<str>>(&mut self, invoice_payload: T) -> &mut Self {
    self.inner.invoice_payload = invoice_payload.as_ref().to_string();
    self
  }

   
  pub fn shipping_option_id<T: AsRef<str>>(&mut self, shipping_option_id: T) -> &mut Self {
    self.inner.shipping_option_id = shipping_option_id.as_ref().to_string();
    self
  }

   
  pub fn order_info<T: AsRef<OrderInfo>>(&mut self, order_info: T) -> &mut Self {
    self.inner.order_info = Some(order_info.as_ref().clone());
    self
  }

   
  pub fn telegram_payment_charge_id<T: AsRef<str>>(&mut self, telegram_payment_charge_id: T) -> &mut Self {
    self.inner.telegram_payment_charge_id = telegram_payment_charge_id.as_ref().to_string();
    self
  }

   
  pub fn provider_payment_charge_id<T: AsRef<str>>(&mut self, provider_payment_charge_id: T) -> &mut Self {
    self.inner.provider_payment_charge_id = provider_payment_charge_id.as_ref().to_string();
    self
  }

}

impl AsRef<MessagePaymentSuccessfulBot> for MessagePaymentSuccessfulBot {
  fn as_ref(&self) -> &MessagePaymentSuccessfulBot { self }
}

impl AsRef<MessagePaymentSuccessfulBot> for RTDMessagePaymentSuccessfulBotBuilder {
  fn as_ref(&self) -> &MessagePaymentSuccessfulBot { &self.inner }
}







/// A contact has registered with Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageContactRegistered {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageContactRegistered {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageContactRegistered" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageContactRegistered {}



impl MessageContactRegistered {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageContactRegisteredBuilder {
    let mut inner = MessageContactRegistered::default();
    inner.td_name = "messageContactRegistered".to_string();
    RTDMessageContactRegisteredBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageContactRegisteredBuilder {
  inner: MessageContactRegistered
}

impl RTDMessageContactRegisteredBuilder {
  pub fn build(&self) -> MessageContactRegistered { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageContactRegistered> for MessageContactRegistered {
  fn as_ref(&self) -> &MessageContactRegistered { self }
}

impl AsRef<MessageContactRegistered> for RTDMessageContactRegisteredBuilder {
  fn as_ref(&self) -> &MessageContactRegistered { &self.inner }
}







/// The current user has connected a website by logging in using Telegram Login Widget on it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageWebsiteConnected {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Domain name of the connected website
  domain_name: String,
  
}

impl RObject for MessageWebsiteConnected {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageWebsiteConnected" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageWebsiteConnected {}



impl MessageWebsiteConnected {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageWebsiteConnectedBuilder {
    let mut inner = MessageWebsiteConnected::default();
    inner.td_name = "messageWebsiteConnected".to_string();
    RTDMessageWebsiteConnectedBuilder { inner }
  }

  pub fn domain_name(&self) -> &String { &self.domain_name }

}

#[doc(hidden)]
pub struct RTDMessageWebsiteConnectedBuilder {
  inner: MessageWebsiteConnected
}

impl RTDMessageWebsiteConnectedBuilder {
  pub fn build(&self) -> MessageWebsiteConnected { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn domain_name<T: AsRef<str>>(&mut self, domain_name: T) -> &mut Self {
    self.inner.domain_name = domain_name.as_ref().to_string();
    self
  }

}

impl AsRef<MessageWebsiteConnected> for MessageWebsiteConnected {
  fn as_ref(&self) -> &MessageWebsiteConnected { self }
}

impl AsRef<MessageWebsiteConnected> for RTDMessageWebsiteConnectedBuilder {
  fn as_ref(&self) -> &MessageWebsiteConnected { &self.inner }
}







/// Telegram Passport data has been sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePassportDataSent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// List of Telegram Passport element types sent
  types: Vec<PassportElementType>,
  
}

impl RObject for MessagePassportDataSent {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePassportDataSent" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessagePassportDataSent {}



impl MessagePassportDataSent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePassportDataSentBuilder {
    let mut inner = MessagePassportDataSent::default();
    inner.td_name = "messagePassportDataSent".to_string();
    RTDMessagePassportDataSentBuilder { inner }
  }

  pub fn types(&self) -> &Vec<PassportElementType> { &self.types }

}

#[doc(hidden)]
pub struct RTDMessagePassportDataSentBuilder {
  inner: MessagePassportDataSent
}

impl RTDMessagePassportDataSentBuilder {
  pub fn build(&self) -> MessagePassportDataSent { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn types(&mut self, types: Vec<PassportElementType>) -> &mut Self {
    self.inner.types = types;
    self
  }

}

impl AsRef<MessagePassportDataSent> for MessagePassportDataSent {
  fn as_ref(&self) -> &MessagePassportDataSent { self }
}

impl AsRef<MessagePassportDataSent> for RTDMessagePassportDataSentBuilder {
  fn as_ref(&self) -> &MessagePassportDataSent { &self.inner }
}







/// Telegram Passport data has been received; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePassportDataReceived {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// List of received Telegram Passport elements
  elements: Vec<EncryptedPassportElement>,
  /// Encrypted data credentials
  credentials: EncryptedCredentials,
  
}

impl RObject for MessagePassportDataReceived {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messagePassportDataReceived" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessagePassportDataReceived {}



impl MessagePassportDataReceived {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessagePassportDataReceivedBuilder {
    let mut inner = MessagePassportDataReceived::default();
    inner.td_name = "messagePassportDataReceived".to_string();
    RTDMessagePassportDataReceivedBuilder { inner }
  }

  pub fn elements(&self) -> &Vec<EncryptedPassportElement> { &self.elements }

  pub fn credentials(&self) -> &EncryptedCredentials { &self.credentials }

}

#[doc(hidden)]
pub struct RTDMessagePassportDataReceivedBuilder {
  inner: MessagePassportDataReceived
}

impl RTDMessagePassportDataReceivedBuilder {
  pub fn build(&self) -> MessagePassportDataReceived { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn elements(&mut self, elements: Vec<EncryptedPassportElement>) -> &mut Self {
    self.inner.elements = elements;
    self
  }

   
  pub fn credentials<T: AsRef<EncryptedCredentials>>(&mut self, credentials: T) -> &mut Self {
    self.inner.credentials = credentials.as_ref().clone();
    self
  }

}

impl AsRef<MessagePassportDataReceived> for MessagePassportDataReceived {
  fn as_ref(&self) -> &MessagePassportDataReceived { self }
}

impl AsRef<MessagePassportDataReceived> for RTDMessagePassportDataReceivedBuilder {
  fn as_ref(&self) -> &MessagePassportDataReceived { &self.inner }
}







/// Message content that is not supported by the client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageUnsupported {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for MessageUnsupported {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageUnsupported" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDMessageContent for MessageUnsupported {}



impl MessageUnsupported {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageUnsupportedBuilder {
    let mut inner = MessageUnsupported::default();
    inner.td_name = "messageUnsupported".to_string();
    RTDMessageUnsupportedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDMessageUnsupportedBuilder {
  inner: MessageUnsupported
}

impl RTDMessageUnsupportedBuilder {
  pub fn build(&self) -> MessageUnsupported { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<MessageUnsupported> for MessageUnsupported {
  fn as_ref(&self) -> &MessageUnsupported { self }
}

impl AsRef<MessageUnsupported> for RTDMessageUnsupportedBuilder {
  fn as_ref(&self) -> &MessageUnsupported { &self.inner }
}



