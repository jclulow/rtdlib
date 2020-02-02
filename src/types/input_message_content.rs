
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | The content of a message to send
pub trait TDInputMessageContent: Debug + RObject {}

/// The content of a message to send
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
  #[doc(hidden)] _Default(()),
  /// A text message
  InputMessageText(InputMessageText),
  /// An animation message (GIF-style).
  InputMessageAnimation(InputMessageAnimation),
  /// An audio message
  InputMessageAudio(InputMessageAudio),
  /// A document message (general file)
  InputMessageDocument(InputMessageDocument),
  /// A photo message
  InputMessagePhoto(InputMessagePhoto),
  /// A sticker message
  InputMessageSticker(InputMessageSticker),
  /// A video message
  InputMessageVideo(InputMessageVideo),
  /// A video note message
  InputMessageVideoNote(InputMessageVideoNote),
  /// A voice note message
  InputMessageVoiceNote(InputMessageVoiceNote),
  /// A message with a location
  InputMessageLocation(InputMessageLocation),
  /// A message with information about a venue
  InputMessageVenue(InputMessageVenue),
  /// A message containing a user contact
  InputMessageContact(InputMessageContact),
  /// A message with a game; not supported for channels or secret chats
  InputMessageGame(InputMessageGame),
  /// A message with an invoice; can be used only by bots and only in private chats
  InputMessageInvoice(InputMessageInvoice),
  /// A message with a poll. Polls can't be sent to private or secret chats
  InputMessagePoll(InputMessagePoll),
  /// A forwarded message
  InputMessageForwarded(InputMessageForwarded),

}

impl Default for InputMessageContent {
  fn default() -> Self { InputMessageContent::_Default(()) }
}

impl<'de> Deserialize<'de> for InputMessageContent {
  fn deserialize<D>(deserializer: D) -> Result<InputMessageContent, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputMessageContent,
      (inputMessageText, InputMessageText);
      (inputMessageAnimation, InputMessageAnimation);
      (inputMessageAudio, InputMessageAudio);
      (inputMessageDocument, InputMessageDocument);
      (inputMessagePhoto, InputMessagePhoto);
      (inputMessageSticker, InputMessageSticker);
      (inputMessageVideo, InputMessageVideo);
      (inputMessageVideoNote, InputMessageVideoNote);
      (inputMessageVoiceNote, InputMessageVoiceNote);
      (inputMessageLocation, InputMessageLocation);
      (inputMessageVenue, InputMessageVenue);
      (inputMessageContact, InputMessageContact);
      (inputMessageGame, InputMessageGame);
      (inputMessageInvoice, InputMessageInvoice);
      (inputMessagePoll, InputMessagePoll);
      (inputMessageForwarded, InputMessageForwarded);

    )(deserializer)
  }
}

impl RObject for InputMessageContent {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InputMessageContent::InputMessageText(t) => t.td_name(),
      InputMessageContent::InputMessageAnimation(t) => t.td_name(),
      InputMessageContent::InputMessageAudio(t) => t.td_name(),
      InputMessageContent::InputMessageDocument(t) => t.td_name(),
      InputMessageContent::InputMessagePhoto(t) => t.td_name(),
      InputMessageContent::InputMessageSticker(t) => t.td_name(),
      InputMessageContent::InputMessageVideo(t) => t.td_name(),
      InputMessageContent::InputMessageVideoNote(t) => t.td_name(),
      InputMessageContent::InputMessageVoiceNote(t) => t.td_name(),
      InputMessageContent::InputMessageLocation(t) => t.td_name(),
      InputMessageContent::InputMessageVenue(t) => t.td_name(),
      InputMessageContent::InputMessageContact(t) => t.td_name(),
      InputMessageContent::InputMessageGame(t) => t.td_name(),
      InputMessageContent::InputMessageInvoice(t) => t.td_name(),
      InputMessageContent::InputMessagePoll(t) => t.td_name(),
      InputMessageContent::InputMessageForwarded(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InputMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InputMessageContent::_Default(_) = self { true } else { false } }

  pub fn is_input_message_text(&self) -> bool { if let InputMessageContent::InputMessageText(_) = self { true } else { false } }
  pub fn is_input_message_animation(&self) -> bool { if let InputMessageContent::InputMessageAnimation(_) = self { true } else { false } }
  pub fn is_input_message_audio(&self) -> bool { if let InputMessageContent::InputMessageAudio(_) = self { true } else { false } }
  pub fn is_input_message_document(&self) -> bool { if let InputMessageContent::InputMessageDocument(_) = self { true } else { false } }
  pub fn is_input_message_photo(&self) -> bool { if let InputMessageContent::InputMessagePhoto(_) = self { true } else { false } }
  pub fn is_input_message_sticker(&self) -> bool { if let InputMessageContent::InputMessageSticker(_) = self { true } else { false } }
  pub fn is_input_message_video(&self) -> bool { if let InputMessageContent::InputMessageVideo(_) = self { true } else { false } }
  pub fn is_input_message_video_note(&self) -> bool { if let InputMessageContent::InputMessageVideoNote(_) = self { true } else { false } }
  pub fn is_input_message_voice_note(&self) -> bool { if let InputMessageContent::InputMessageVoiceNote(_) = self { true } else { false } }
  pub fn is_input_message_location(&self) -> bool { if let InputMessageContent::InputMessageLocation(_) = self { true } else { false } }
  pub fn is_input_message_venue(&self) -> bool { if let InputMessageContent::InputMessageVenue(_) = self { true } else { false } }
  pub fn is_input_message_contact(&self) -> bool { if let InputMessageContent::InputMessageContact(_) = self { true } else { false } }
  pub fn is_input_message_game(&self) -> bool { if let InputMessageContent::InputMessageGame(_) = self { true } else { false } }
  pub fn is_input_message_invoice(&self) -> bool { if let InputMessageContent::InputMessageInvoice(_) = self { true } else { false } }
  pub fn is_input_message_poll(&self) -> bool { if let InputMessageContent::InputMessagePoll(_) = self { true } else { false } }
  pub fn is_input_message_forwarded(&self) -> bool { if let InputMessageContent::InputMessageForwarded(_) = self { true } else { false } }

  pub fn on_input_message_text<F: FnOnce(&InputMessageText)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageText(t) = self { fnc(t) }; self }
  pub fn on_input_message_animation<F: FnOnce(&InputMessageAnimation)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageAnimation(t) = self { fnc(t) }; self }
  pub fn on_input_message_audio<F: FnOnce(&InputMessageAudio)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageAudio(t) = self { fnc(t) }; self }
  pub fn on_input_message_document<F: FnOnce(&InputMessageDocument)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageDocument(t) = self { fnc(t) }; self }
  pub fn on_input_message_photo<F: FnOnce(&InputMessagePhoto)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessagePhoto(t) = self { fnc(t) }; self }
  pub fn on_input_message_sticker<F: FnOnce(&InputMessageSticker)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageSticker(t) = self { fnc(t) }; self }
  pub fn on_input_message_video<F: FnOnce(&InputMessageVideo)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageVideo(t) = self { fnc(t) }; self }
  pub fn on_input_message_video_note<F: FnOnce(&InputMessageVideoNote)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageVideoNote(t) = self { fnc(t) }; self }
  pub fn on_input_message_voice_note<F: FnOnce(&InputMessageVoiceNote)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageVoiceNote(t) = self { fnc(t) }; self }
  pub fn on_input_message_location<F: FnOnce(&InputMessageLocation)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageLocation(t) = self { fnc(t) }; self }
  pub fn on_input_message_venue<F: FnOnce(&InputMessageVenue)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageVenue(t) = self { fnc(t) }; self }
  pub fn on_input_message_contact<F: FnOnce(&InputMessageContact)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageContact(t) = self { fnc(t) }; self }
  pub fn on_input_message_game<F: FnOnce(&InputMessageGame)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageGame(t) = self { fnc(t) }; self }
  pub fn on_input_message_invoice<F: FnOnce(&InputMessageInvoice)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageInvoice(t) = self { fnc(t) }; self }
  pub fn on_input_message_poll<F: FnOnce(&InputMessagePoll)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessagePoll(t) = self { fnc(t) }; self }
  pub fn on_input_message_forwarded<F: FnOnce(&InputMessageForwarded)>(&self, fnc: F) -> &Self { if let InputMessageContent::InputMessageForwarded(t) = self { fnc(t) }; self }

  pub fn as_input_message_text(&self) -> Option<&InputMessageText> { if let InputMessageContent::InputMessageText(t) = self { return Some(t) } None }
  pub fn as_input_message_animation(&self) -> Option<&InputMessageAnimation> { if let InputMessageContent::InputMessageAnimation(t) = self { return Some(t) } None }
  pub fn as_input_message_audio(&self) -> Option<&InputMessageAudio> { if let InputMessageContent::InputMessageAudio(t) = self { return Some(t) } None }
  pub fn as_input_message_document(&self) -> Option<&InputMessageDocument> { if let InputMessageContent::InputMessageDocument(t) = self { return Some(t) } None }
  pub fn as_input_message_photo(&self) -> Option<&InputMessagePhoto> { if let InputMessageContent::InputMessagePhoto(t) = self { return Some(t) } None }
  pub fn as_input_message_sticker(&self) -> Option<&InputMessageSticker> { if let InputMessageContent::InputMessageSticker(t) = self { return Some(t) } None }
  pub fn as_input_message_video(&self) -> Option<&InputMessageVideo> { if let InputMessageContent::InputMessageVideo(t) = self { return Some(t) } None }
  pub fn as_input_message_video_note(&self) -> Option<&InputMessageVideoNote> { if let InputMessageContent::InputMessageVideoNote(t) = self { return Some(t) } None }
  pub fn as_input_message_voice_note(&self) -> Option<&InputMessageVoiceNote> { if let InputMessageContent::InputMessageVoiceNote(t) = self { return Some(t) } None }
  pub fn as_input_message_location(&self) -> Option<&InputMessageLocation> { if let InputMessageContent::InputMessageLocation(t) = self { return Some(t) } None }
  pub fn as_input_message_venue(&self) -> Option<&InputMessageVenue> { if let InputMessageContent::InputMessageVenue(t) = self { return Some(t) } None }
  pub fn as_input_message_contact(&self) -> Option<&InputMessageContact> { if let InputMessageContent::InputMessageContact(t) = self { return Some(t) } None }
  pub fn as_input_message_game(&self) -> Option<&InputMessageGame> { if let InputMessageContent::InputMessageGame(t) = self { return Some(t) } None }
  pub fn as_input_message_invoice(&self) -> Option<&InputMessageInvoice> { if let InputMessageContent::InputMessageInvoice(t) = self { return Some(t) } None }
  pub fn as_input_message_poll(&self) -> Option<&InputMessagePoll> { if let InputMessageContent::InputMessagePoll(t) = self { return Some(t) } None }
  pub fn as_input_message_forwarded(&self) -> Option<&InputMessageForwarded> { if let InputMessageContent::InputMessageForwarded(t) = self { return Some(t) } None }



  pub fn input_message_text<T: AsRef<InputMessageText>>(t: T) -> Self { InputMessageContent::InputMessageText(t.as_ref().clone()) }

  pub fn input_message_animation<T: AsRef<InputMessageAnimation>>(t: T) -> Self { InputMessageContent::InputMessageAnimation(t.as_ref().clone()) }

  pub fn input_message_audio<T: AsRef<InputMessageAudio>>(t: T) -> Self { InputMessageContent::InputMessageAudio(t.as_ref().clone()) }

  pub fn input_message_document<T: AsRef<InputMessageDocument>>(t: T) -> Self { InputMessageContent::InputMessageDocument(t.as_ref().clone()) }

  pub fn input_message_photo<T: AsRef<InputMessagePhoto>>(t: T) -> Self { InputMessageContent::InputMessagePhoto(t.as_ref().clone()) }

  pub fn input_message_sticker<T: AsRef<InputMessageSticker>>(t: T) -> Self { InputMessageContent::InputMessageSticker(t.as_ref().clone()) }

  pub fn input_message_video<T: AsRef<InputMessageVideo>>(t: T) -> Self { InputMessageContent::InputMessageVideo(t.as_ref().clone()) }

  pub fn input_message_video_note<T: AsRef<InputMessageVideoNote>>(t: T) -> Self { InputMessageContent::InputMessageVideoNote(t.as_ref().clone()) }

  pub fn input_message_voice_note<T: AsRef<InputMessageVoiceNote>>(t: T) -> Self { InputMessageContent::InputMessageVoiceNote(t.as_ref().clone()) }

  pub fn input_message_location<T: AsRef<InputMessageLocation>>(t: T) -> Self { InputMessageContent::InputMessageLocation(t.as_ref().clone()) }

  pub fn input_message_venue<T: AsRef<InputMessageVenue>>(t: T) -> Self { InputMessageContent::InputMessageVenue(t.as_ref().clone()) }

  pub fn input_message_contact<T: AsRef<InputMessageContact>>(t: T) -> Self { InputMessageContent::InputMessageContact(t.as_ref().clone()) }

  pub fn input_message_game<T: AsRef<InputMessageGame>>(t: T) -> Self { InputMessageContent::InputMessageGame(t.as_ref().clone()) }

  pub fn input_message_invoice<T: AsRef<InputMessageInvoice>>(t: T) -> Self { InputMessageContent::InputMessageInvoice(t.as_ref().clone()) }

  pub fn input_message_poll<T: AsRef<InputMessagePoll>>(t: T) -> Self { InputMessageContent::InputMessagePoll(t.as_ref().clone()) }

  pub fn input_message_forwarded<T: AsRef<InputMessageForwarded>>(t: T) -> Self { InputMessageContent::InputMessageForwarded(t.as_ref().clone()) }

}

impl AsRef<InputMessageContent> for InputMessageContent {
  fn as_ref(&self) -> &InputMessageContent { self }
}







/// A text message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Formatted text to be sent; 1-GetOption("message_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities are allowed to be specified manually
  text: FormattedText,
  /// True, if rich web page previews for URLs in the message text should be disabled
  disable_web_page_preview: bool,
  /// True, if a chat message draft should be deleted
  clear_draft: bool,
  
}

impl RObject for InputMessageText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageText" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageText {}



impl InputMessageText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageTextBuilder {
    let mut inner = InputMessageText::default();
    inner.td_name = "inputMessageText".to_string();
    RTDInputMessageTextBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText { &self.text }

  pub fn disable_web_page_preview(&self) -> bool { self.disable_web_page_preview }

  pub fn clear_draft(&self) -> bool { self.clear_draft }

}

#[doc(hidden)]
pub struct RTDInputMessageTextBuilder {
  inner: InputMessageText
}

impl RTDInputMessageTextBuilder {
  pub fn build(&self) -> InputMessageText { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn disable_web_page_preview(&mut self, disable_web_page_preview: bool) -> &mut Self {
    self.inner.disable_web_page_preview = disable_web_page_preview;
    self
  }

   
  pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
    self.inner.clear_draft = clear_draft;
    self
  }

}

impl AsRef<InputMessageText> for InputMessageText {
  fn as_ref(&self) -> &InputMessageText { self }
}

impl AsRef<InputMessageText> for RTDInputMessageTextBuilder {
  fn as_ref(&self) -> &InputMessageText { &self.inner }
}







/// An animation message (GIF-style).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Animation file to be sent
  animation: InputFile,
  /// Animation thumbnail, if available
  thumbnail: InputThumbnail,
  /// Duration of the animation, in seconds
  duration: i64,
  /// Width of the animation; may be replaced by the server
  width: i64,
  /// Height of the animation; may be replaced by the server
  height: i64,
  /// Animation caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  
}

impl RObject for InputMessageAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageAnimation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageAnimation {}



impl InputMessageAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageAnimationBuilder {
    let mut inner = InputMessageAnimation::default();
    inner.td_name = "inputMessageAnimation".to_string();
    RTDInputMessageAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputFile { &self.animation }

  pub fn thumbnail(&self) -> &InputThumbnail { &self.thumbnail }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDInputMessageAnimationBuilder {
  inner: InputMessageAnimation
}

impl RTDInputMessageAnimationBuilder {
  pub fn build(&self) -> InputMessageAnimation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<InputMessageAnimation> for InputMessageAnimation {
  fn as_ref(&self) -> &InputMessageAnimation { self }
}

impl AsRef<InputMessageAnimation> for RTDInputMessageAnimationBuilder {
  fn as_ref(&self) -> &InputMessageAnimation { &self.inner }
}







/// An audio message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Audio file to be sent
  audio: InputFile,
  /// Thumbnail of the cover for the album, if available
  album_cover_thumbnail: InputThumbnail,
  /// Duration of the audio, in seconds; may be replaced by the server
  duration: i64,
  /// Title of the audio; 0-64 characters; may be replaced by the server
  title: String,
  /// Performer of the audio; 0-64 characters, may be replaced by the server
  performer: String,
  /// Audio caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  
}

impl RObject for InputMessageAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageAudio" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageAudio {}



impl InputMessageAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageAudioBuilder {
    let mut inner = InputMessageAudio::default();
    inner.td_name = "inputMessageAudio".to_string();
    RTDInputMessageAudioBuilder { inner }
  }

  pub fn audio(&self) -> &InputFile { &self.audio }

  pub fn album_cover_thumbnail(&self) -> &InputThumbnail { &self.album_cover_thumbnail }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn title(&self) -> &String { &self.title }

  pub fn performer(&self) -> &String { &self.performer }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDInputMessageAudioBuilder {
  inner: InputMessageAudio
}

impl RTDInputMessageAudioBuilder {
  pub fn build(&self) -> InputMessageAudio { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn audio<T: AsRef<InputFile>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }

   
  pub fn album_cover_thumbnail<T: AsRef<InputThumbnail>>(&mut self, album_cover_thumbnail: T) -> &mut Self {
    self.inner.album_cover_thumbnail = album_cover_thumbnail.as_ref().clone();
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn performer<T: AsRef<str>>(&mut self, performer: T) -> &mut Self {
    self.inner.performer = performer.as_ref().to_string();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<InputMessageAudio> for InputMessageAudio {
  fn as_ref(&self) -> &InputMessageAudio { self }
}

impl AsRef<InputMessageAudio> for RTDInputMessageAudioBuilder {
  fn as_ref(&self) -> &InputMessageAudio { &self.inner }
}







/// A document message (general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Document to be sent
  document: InputFile,
  /// Document thumbnail, if available
  thumbnail: InputThumbnail,
  /// Document caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  
}

impl RObject for InputMessageDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageDocument" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageDocument {}



impl InputMessageDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageDocumentBuilder {
    let mut inner = InputMessageDocument::default();
    inner.td_name = "inputMessageDocument".to_string();
    RTDInputMessageDocumentBuilder { inner }
  }

  pub fn document(&self) -> &InputFile { &self.document }

  pub fn thumbnail(&self) -> &InputThumbnail { &self.thumbnail }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDInputMessageDocumentBuilder {
  inner: InputMessageDocument
}

impl RTDInputMessageDocumentBuilder {
  pub fn build(&self) -> InputMessageDocument { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn document<T: AsRef<InputFile>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<InputMessageDocument> for InputMessageDocument {
  fn as_ref(&self) -> &InputMessageDocument { self }
}

impl AsRef<InputMessageDocument> for RTDInputMessageDocumentBuilder {
  fn as_ref(&self) -> &InputMessageDocument { &self.inner }
}







/// A photo message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessagePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Photo to send
  photo: InputFile,
  /// Photo thumbnail to be sent, this is sent to the other party in secret chats only
  thumbnail: InputThumbnail,
  /// File identifiers of the stickers added to the photo, if applicable
  added_sticker_file_ids: Vec<i64>,
  /// Photo width
  width: i64,
  /// Photo height
  height: i64,
  /// Photo caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  /// Photo TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats
  ttl: i64,
  
}

impl RObject for InputMessagePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessagePhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessagePhoto {}



impl InputMessagePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessagePhotoBuilder {
    let mut inner = InputMessagePhoto::default();
    inner.td_name = "inputMessagePhoto".to_string();
    RTDInputMessagePhotoBuilder { inner }
  }

  pub fn photo(&self) -> &InputFile { &self.photo }

  pub fn thumbnail(&self) -> &InputThumbnail { &self.thumbnail }

  pub fn added_sticker_file_ids(&self) -> &Vec<i64> { &self.added_sticker_file_ids }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn caption(&self) -> &FormattedText { &self.caption }

  pub fn ttl(&self) -> i64 { self.ttl }

}

#[doc(hidden)]
pub struct RTDInputMessagePhotoBuilder {
  inner: InputMessagePhoto
}

impl RTDInputMessagePhotoBuilder {
  pub fn build(&self) -> InputMessagePhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

   
  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i64>) -> &mut Self {
    self.inner.added_sticker_file_ids = added_sticker_file_ids;
    self
  }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn ttl(&mut self, ttl: i64) -> &mut Self {
    self.inner.ttl = ttl;
    self
  }

}

impl AsRef<InputMessagePhoto> for InputMessagePhoto {
  fn as_ref(&self) -> &InputMessagePhoto { self }
}

impl AsRef<InputMessagePhoto> for RTDInputMessagePhotoBuilder {
  fn as_ref(&self) -> &InputMessagePhoto { &self.inner }
}







/// A sticker message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Sticker to be sent
  sticker: InputFile,
  /// Sticker thumbnail, if available
  thumbnail: InputThumbnail,
  /// Sticker width
  width: i64,
  /// Sticker height
  height: i64,
  
}

impl RObject for InputMessageSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageSticker" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageSticker {}



impl InputMessageSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageStickerBuilder {
    let mut inner = InputMessageSticker::default();
    inner.td_name = "inputMessageSticker".to_string();
    RTDInputMessageStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile { &self.sticker }

  pub fn thumbnail(&self) -> &InputThumbnail { &self.thumbnail }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

}

#[doc(hidden)]
pub struct RTDInputMessageStickerBuilder {
  inner: InputMessageSticker
}

impl RTDInputMessageStickerBuilder {
  pub fn build(&self) -> InputMessageSticker { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

}

impl AsRef<InputMessageSticker> for InputMessageSticker {
  fn as_ref(&self) -> &InputMessageSticker { self }
}

impl AsRef<InputMessageSticker> for RTDInputMessageStickerBuilder {
  fn as_ref(&self) -> &InputMessageSticker { &self.inner }
}







/// A video message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Video to be sent
  video: InputFile,
  /// Video thumbnail, if available
  thumbnail: InputThumbnail,
  /// File identifiers of the stickers added to the video, if applicable
  added_sticker_file_ids: Vec<i64>,
  /// Duration of the video, in seconds
  duration: i64,
  /// Video width
  width: i64,
  /// Video height
  height: i64,
  /// True, if the video should be tried to be streamed
  supports_streaming: bool,
  /// Video caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  /// Video TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats
  ttl: i64,
  
}

impl RObject for InputMessageVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageVideo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageVideo {}



impl InputMessageVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageVideoBuilder {
    let mut inner = InputMessageVideo::default();
    inner.td_name = "inputMessageVideo".to_string();
    RTDInputMessageVideoBuilder { inner }
  }

  pub fn video(&self) -> &InputFile { &self.video }

  pub fn thumbnail(&self) -> &InputThumbnail { &self.thumbnail }

  pub fn added_sticker_file_ids(&self) -> &Vec<i64> { &self.added_sticker_file_ids }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn supports_streaming(&self) -> bool { self.supports_streaming }

  pub fn caption(&self) -> &FormattedText { &self.caption }

  pub fn ttl(&self) -> i64 { self.ttl }

}

#[doc(hidden)]
pub struct RTDInputMessageVideoBuilder {
  inner: InputMessageVideo
}

impl RTDInputMessageVideoBuilder {
  pub fn build(&self) -> InputMessageVideo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn video<T: AsRef<InputFile>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

   
  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i64>) -> &mut Self {
    self.inner.added_sticker_file_ids = added_sticker_file_ids;
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

   
  pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
    self.inner.supports_streaming = supports_streaming;
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

   
  pub fn ttl(&mut self, ttl: i64) -> &mut Self {
    self.inner.ttl = ttl;
    self
  }

}

impl AsRef<InputMessageVideo> for InputMessageVideo {
  fn as_ref(&self) -> &InputMessageVideo { self }
}

impl AsRef<InputMessageVideo> for RTDInputMessageVideoBuilder {
  fn as_ref(&self) -> &InputMessageVideo { &self.inner }
}







/// A video note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Video note to be sent
  video_note: InputFile,
  /// Video thumbnail, if available
  thumbnail: InputThumbnail,
  /// Duration of the video, in seconds
  duration: i64,
  /// Video width and height; must be positive and not greater than 640
  length: i64,
  
}

impl RObject for InputMessageVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageVideoNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageVideoNote {}



impl InputMessageVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageVideoNoteBuilder {
    let mut inner = InputMessageVideoNote::default();
    inner.td_name = "inputMessageVideoNote".to_string();
    RTDInputMessageVideoNoteBuilder { inner }
  }

  pub fn video_note(&self) -> &InputFile { &self.video_note }

  pub fn thumbnail(&self) -> &InputThumbnail { &self.thumbnail }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn length(&self) -> i64 { self.length }

}

#[doc(hidden)]
pub struct RTDInputMessageVideoNoteBuilder {
  inner: InputMessageVideoNote
}

impl RTDInputMessageVideoNoteBuilder {
  pub fn build(&self) -> InputMessageVideoNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn video_note<T: AsRef<InputFile>>(&mut self, video_note: T) -> &mut Self {
    self.inner.video_note = video_note.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

}

impl AsRef<InputMessageVideoNote> for InputMessageVideoNote {
  fn as_ref(&self) -> &InputMessageVideoNote { self }
}

impl AsRef<InputMessageVideoNote> for RTDInputMessageVideoNoteBuilder {
  fn as_ref(&self) -> &InputMessageVideoNote { &self.inner }
}







/// A voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Voice note to be sent
  voice_note: InputFile,
  /// Duration of the voice note, in seconds
  duration: i64,
  /// Waveform representation of the voice note, in 5-bit format
  waveform: String,
  /// Voice note caption; 0-GetOption("message_caption_length_max") characters
  caption: FormattedText,
  
}

impl RObject for InputMessageVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageVoiceNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageVoiceNote {}



impl InputMessageVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageVoiceNoteBuilder {
    let mut inner = InputMessageVoiceNote::default();
    inner.td_name = "inputMessageVoiceNote".to_string();
    RTDInputMessageVoiceNoteBuilder { inner }
  }

  pub fn voice_note(&self) -> &InputFile { &self.voice_note }

  pub fn duration(&self) -> i64 { self.duration }

  pub fn waveform(&self) -> &String { &self.waveform }

  pub fn caption(&self) -> &FormattedText { &self.caption }

}

#[doc(hidden)]
pub struct RTDInputMessageVoiceNoteBuilder {
  inner: InputMessageVoiceNote
}

impl RTDInputMessageVoiceNoteBuilder {
  pub fn build(&self) -> InputMessageVoiceNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn voice_note<T: AsRef<InputFile>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = voice_note.as_ref().clone();
    self
  }

   
  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

   
  pub fn waveform<T: AsRef<str>>(&mut self, waveform: T) -> &mut Self {
    self.inner.waveform = waveform.as_ref().to_string();
    self
  }

   
  pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
    self.inner.caption = caption.as_ref().clone();
    self
  }

}

impl AsRef<InputMessageVoiceNote> for InputMessageVoiceNote {
  fn as_ref(&self) -> &InputMessageVoiceNote { self }
}

impl AsRef<InputMessageVoiceNote> for RTDInputMessageVoiceNoteBuilder {
  fn as_ref(&self) -> &InputMessageVoiceNote { &self.inner }
}







/// A message with a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Location to be sent
  location: Location,
  /// Period for which the location can be updated, in seconds; should bebetween 60 and 86400 for a live location and 0 otherwise
  live_period: i64,
  
}

impl RObject for InputMessageLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageLocation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageLocation {}



impl InputMessageLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageLocationBuilder {
    let mut inner = InputMessageLocation::default();
    inner.td_name = "inputMessageLocation".to_string();
    RTDInputMessageLocationBuilder { inner }
  }

  pub fn location(&self) -> &Location { &self.location }

  pub fn live_period(&self) -> i64 { self.live_period }

}

#[doc(hidden)]
pub struct RTDInputMessageLocationBuilder {
  inner: InputMessageLocation
}

impl RTDInputMessageLocationBuilder {
  pub fn build(&self) -> InputMessageLocation { self.inner.clone() }
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

}

impl AsRef<InputMessageLocation> for InputMessageLocation {
  fn as_ref(&self) -> &InputMessageLocation { self }
}

impl AsRef<InputMessageLocation> for RTDInputMessageLocationBuilder {
  fn as_ref(&self) -> &InputMessageLocation { &self.inner }
}







/// A message with information about a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Venue to send
  venue: Venue,
  
}

impl RObject for InputMessageVenue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageVenue" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageVenue {}



impl InputMessageVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageVenueBuilder {
    let mut inner = InputMessageVenue::default();
    inner.td_name = "inputMessageVenue".to_string();
    RTDInputMessageVenueBuilder { inner }
  }

  pub fn venue(&self) -> &Venue { &self.venue }

}

#[doc(hidden)]
pub struct RTDInputMessageVenueBuilder {
  inner: InputMessageVenue
}

impl RTDInputMessageVenueBuilder {
  pub fn build(&self) -> InputMessageVenue { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
    self.inner.venue = venue.as_ref().clone();
    self
  }

}

impl AsRef<InputMessageVenue> for InputMessageVenue {
  fn as_ref(&self) -> &InputMessageVenue { self }
}

impl AsRef<InputMessageVenue> for RTDInputMessageVenueBuilder {
  fn as_ref(&self) -> &InputMessageVenue { &self.inner }
}







/// A message containing a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Contact to send
  contact: Contact,
  
}

impl RObject for InputMessageContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageContact" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageContact {}



impl InputMessageContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageContactBuilder {
    let mut inner = InputMessageContact::default();
    inner.td_name = "inputMessageContact".to_string();
    RTDInputMessageContactBuilder { inner }
  }

  pub fn contact(&self) -> &Contact { &self.contact }

}

#[doc(hidden)]
pub struct RTDInputMessageContactBuilder {
  inner: InputMessageContact
}

impl RTDInputMessageContactBuilder {
  pub fn build(&self) -> InputMessageContact { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
    self.inner.contact = contact.as_ref().clone();
    self
  }

}

impl AsRef<InputMessageContact> for InputMessageContact {
  fn as_ref(&self) -> &InputMessageContact { self }
}

impl AsRef<InputMessageContact> for RTDInputMessageContactBuilder {
  fn as_ref(&self) -> &InputMessageContact { &self.inner }
}







/// A message with a game; not supported for channels or secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// User identifier of the bot that owns the game
  bot_user_id: i64,
  /// Short name of the game
  game_short_name: String,
  
}

impl RObject for InputMessageGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageGame" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageGame {}



impl InputMessageGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageGameBuilder {
    let mut inner = InputMessageGame::default();
    inner.td_name = "inputMessageGame".to_string();
    RTDInputMessageGameBuilder { inner }
  }

  pub fn bot_user_id(&self) -> i64 { self.bot_user_id }

  pub fn game_short_name(&self) -> &String { &self.game_short_name }

}

#[doc(hidden)]
pub struct RTDInputMessageGameBuilder {
  inner: InputMessageGame
}

impl RTDInputMessageGameBuilder {
  pub fn build(&self) -> InputMessageGame { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
    self.inner.bot_user_id = bot_user_id;
    self
  }

   
  pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
    self.inner.game_short_name = game_short_name.as_ref().to_string();
    self
  }

}

impl AsRef<InputMessageGame> for InputMessageGame {
  fn as_ref(&self) -> &InputMessageGame { self }
}

impl AsRef<InputMessageGame> for RTDInputMessageGameBuilder {
  fn as_ref(&self) -> &InputMessageGame { &self.inner }
}







/// A message with an invoice; can be used only by bots and only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageInvoice {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Invoice
  invoice: Invoice,
  /// Product title; 1-32 characters
  title: String,
  /// A message with an invoice; can be used only by bots and only in private chats
  description: String,
  /// Product photo URL; optional
  photo_url: String,
  /// Product photo size
  photo_size: i64,
  /// Product photo width
  photo_width: i64,
  /// Product photo height
  photo_height: i64,
  /// The invoice payload
  payload: String,
  /// Payment provider token
  provider_token: String,
  /// JSON-encoded data about the invoice, which will be shared with the payment provider
  provider_data: String,
  /// Unique invoice bot start_parameter for the generation of this invoice
  start_parameter: String,
  
}

impl RObject for InputMessageInvoice {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageInvoice" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageInvoice {}



impl InputMessageInvoice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageInvoiceBuilder {
    let mut inner = InputMessageInvoice::default();
    inner.td_name = "inputMessageInvoice".to_string();
    RTDInputMessageInvoiceBuilder { inner }
  }

  pub fn invoice(&self) -> &Invoice { &self.invoice }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

  pub fn photo_url(&self) -> &String { &self.photo_url }

  pub fn photo_size(&self) -> i64 { self.photo_size }

  pub fn photo_width(&self) -> i64 { self.photo_width }

  pub fn photo_height(&self) -> i64 { self.photo_height }

  pub fn payload(&self) -> &String { &self.payload }

  pub fn provider_token(&self) -> &String { &self.provider_token }

  pub fn provider_data(&self) -> &String { &self.provider_data }

  pub fn start_parameter(&self) -> &String { &self.start_parameter }

}

#[doc(hidden)]
pub struct RTDInputMessageInvoiceBuilder {
  inner: InputMessageInvoice
}

impl RTDInputMessageInvoiceBuilder {
  pub fn build(&self) -> InputMessageInvoice { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
    self.inner.invoice = invoice.as_ref().clone();
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

   
  pub fn photo_url<T: AsRef<str>>(&mut self, photo_url: T) -> &mut Self {
    self.inner.photo_url = photo_url.as_ref().to_string();
    self
  }

   
  pub fn photo_size(&mut self, photo_size: i64) -> &mut Self {
    self.inner.photo_size = photo_size;
    self
  }

   
  pub fn photo_width(&mut self, photo_width: i64) -> &mut Self {
    self.inner.photo_width = photo_width;
    self
  }

   
  pub fn photo_height(&mut self, photo_height: i64) -> &mut Self {
    self.inner.photo_height = photo_height;
    self
  }

   
  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

   
  pub fn provider_token<T: AsRef<str>>(&mut self, provider_token: T) -> &mut Self {
    self.inner.provider_token = provider_token.as_ref().to_string();
    self
  }

   
  pub fn provider_data<T: AsRef<str>>(&mut self, provider_data: T) -> &mut Self {
    self.inner.provider_data = provider_data.as_ref().to_string();
    self
  }

   
  pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
    self.inner.start_parameter = start_parameter.as_ref().to_string();
    self
  }

}

impl AsRef<InputMessageInvoice> for InputMessageInvoice {
  fn as_ref(&self) -> &InputMessageInvoice { self }
}

impl AsRef<InputMessageInvoice> for RTDInputMessageInvoiceBuilder {
  fn as_ref(&self) -> &InputMessageInvoice { &self.inner }
}







/// A message with a poll. Polls can't be sent to private or secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessagePoll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Poll question, 1-255 characters
  question: String,
  /// List of poll answer options, 2-10 strings 1-100 characters each
  options: Vec<String>,
  
}

impl RObject for InputMessagePoll {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessagePoll" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessagePoll {}



impl InputMessagePoll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessagePollBuilder {
    let mut inner = InputMessagePoll::default();
    inner.td_name = "inputMessagePoll".to_string();
    RTDInputMessagePollBuilder { inner }
  }

  pub fn question(&self) -> &String { &self.question }

  pub fn options(&self) -> &Vec<String> { &self.options }

}

#[doc(hidden)]
pub struct RTDInputMessagePollBuilder {
  inner: InputMessagePoll
}

impl RTDInputMessagePollBuilder {
  pub fn build(&self) -> InputMessagePoll { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn question<T: AsRef<str>>(&mut self, question: T) -> &mut Self {
    self.inner.question = question.as_ref().to_string();
    self
  }

   
  pub fn options(&mut self, options: Vec<String>) -> &mut Self {
    self.inner.options = options;
    self
  }

}

impl AsRef<InputMessagePoll> for InputMessagePoll {
  fn as_ref(&self) -> &InputMessagePoll { self }
}

impl AsRef<InputMessagePoll> for RTDInputMessagePollBuilder {
  fn as_ref(&self) -> &InputMessagePoll { &self.inner }
}







/// A forwarded message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageForwarded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Identifier for the chat this forwarded message came from
  from_chat_id: i64,
  /// Identifier of the message to forward
  message_id: i64,
  /// True, if a game message should be shared within a launched game; applies only to game messages
  in_game_share: bool,
  /// True, if content of the message needs to be copied without a link to the original message. Always true if the message is forwarded to a secret chat
  send_copy: bool,
  /// True, if media caption of the message copy needs to be removed. Ignored if send_copy is false
  remove_caption: bool,
  
}

impl RObject for InputMessageForwarded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageForwarded" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputMessageContent for InputMessageForwarded {}



impl InputMessageForwarded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputMessageForwardedBuilder {
    let mut inner = InputMessageForwarded::default();
    inner.td_name = "inputMessageForwarded".to_string();
    RTDInputMessageForwardedBuilder { inner }
  }

  pub fn from_chat_id(&self) -> i64 { self.from_chat_id }

  pub fn message_id(&self) -> i64 { self.message_id }

  pub fn in_game_share(&self) -> bool { self.in_game_share }

  pub fn send_copy(&self) -> bool { self.send_copy }

  pub fn remove_caption(&self) -> bool { self.remove_caption }

}

#[doc(hidden)]
pub struct RTDInputMessageForwardedBuilder {
  inner: InputMessageForwarded
}

impl RTDInputMessageForwardedBuilder {
  pub fn build(&self) -> InputMessageForwarded { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
    self.inner.from_chat_id = from_chat_id;
    self
  }

   
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.message_id = message_id;
    self
  }

   
  pub fn in_game_share(&mut self, in_game_share: bool) -> &mut Self {
    self.inner.in_game_share = in_game_share;
    self
  }

   
  pub fn send_copy(&mut self, send_copy: bool) -> &mut Self {
    self.inner.send_copy = send_copy;
    self
  }

   
  pub fn remove_caption(&mut self, remove_caption: bool) -> &mut Self {
    self.inner.remove_caption = remove_caption;
    self
  }

}

impl AsRef<InputMessageForwarded> for InputMessageForwarded {
  fn as_ref(&self) -> &InputMessageForwarded { self }
}

impl AsRef<InputMessageForwarded> for RTDInputMessageForwardedBuilder {
  fn as_ref(&self) -> &InputMessageForwarded { &self.inner }
}



