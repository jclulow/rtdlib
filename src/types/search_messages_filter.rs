
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a filter for message search results
pub trait TDSearchMessagesFilter: Debug + RObject {}

/// Represents a filter for message search results
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SearchMessagesFilter {
  #[doc(hidden)] _Default(()),
  /// Returns all found messages, no filter is applied
  Empty(SearchMessagesFilterEmpty),
  /// Returns only animation messages
  Animation(SearchMessagesFilterAnimation),
  /// Returns only audio messages
  Audio(SearchMessagesFilterAudio),
  /// Returns only document messages
  Document(SearchMessagesFilterDocument),
  /// Returns only photo messages
  Photo(SearchMessagesFilterPhoto),
  /// Returns only video messages
  Video(SearchMessagesFilterVideo),
  /// Returns only voice note messages
  VoiceNote(SearchMessagesFilterVoiceNote),
  /// Returns only photo and video messages
  PhotoAndVideo(SearchMessagesFilterPhotoAndVideo),
  /// Returns only messages containing URLs
  Url(SearchMessagesFilterUrl),
  /// Returns only messages containing chat photos
  ChatPhoto(SearchMessagesFilterChatPhoto),
  /// Returns only call messages
  Call(SearchMessagesFilterCall),
  /// Returns only incoming call messages with missed/declined discard reasons
  MissedCall(SearchMessagesFilterMissedCall),
  /// Returns only video note messages
  VideoNote(SearchMessagesFilterVideoNote),
  /// Returns only voice and video note messages
  VoiceAndVideoNote(SearchMessagesFilterVoiceAndVideoNote),
  /// Returns only messages with mentions of the current user, or messages that are replies to their messages
  Mention(SearchMessagesFilterMention),
  /// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query or by the sending user
  UnreadMention(SearchMessagesFilterUnreadMention),

}

impl Default for SearchMessagesFilter {
  fn default() -> Self { SearchMessagesFilter::_Default(()) }
}

impl<'de> Deserialize<'de> for SearchMessagesFilter {
  fn deserialize<D>(deserializer: D) -> Result<SearchMessagesFilter, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SearchMessagesFilter,
      (searchMessagesFilterEmpty, Empty);
      (searchMessagesFilterAnimation, Animation);
      (searchMessagesFilterAudio, Audio);
      (searchMessagesFilterDocument, Document);
      (searchMessagesFilterPhoto, Photo);
      (searchMessagesFilterVideo, Video);
      (searchMessagesFilterVoiceNote, VoiceNote);
      (searchMessagesFilterPhotoAndVideo, PhotoAndVideo);
      (searchMessagesFilterUrl, Url);
      (searchMessagesFilterChatPhoto, ChatPhoto);
      (searchMessagesFilterCall, Call);
      (searchMessagesFilterMissedCall, MissedCall);
      (searchMessagesFilterVideoNote, VideoNote);
      (searchMessagesFilterVoiceAndVideoNote, VoiceAndVideoNote);
      (searchMessagesFilterMention, Mention);
      (searchMessagesFilterUnreadMention, UnreadMention);

    )(deserializer)
  }
}

impl RObject for SearchMessagesFilter {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      SearchMessagesFilter::Empty(t) => t.td_name(),
      SearchMessagesFilter::Animation(t) => t.td_name(),
      SearchMessagesFilter::Audio(t) => t.td_name(),
      SearchMessagesFilter::Document(t) => t.td_name(),
      SearchMessagesFilter::Photo(t) => t.td_name(),
      SearchMessagesFilter::Video(t) => t.td_name(),
      SearchMessagesFilter::VoiceNote(t) => t.td_name(),
      SearchMessagesFilter::PhotoAndVideo(t) => t.td_name(),
      SearchMessagesFilter::Url(t) => t.td_name(),
      SearchMessagesFilter::ChatPhoto(t) => t.td_name(),
      SearchMessagesFilter::Call(t) => t.td_name(),
      SearchMessagesFilter::MissedCall(t) => t.td_name(),
      SearchMessagesFilter::VideoNote(t) => t.td_name(),
      SearchMessagesFilter::VoiceAndVideoNote(t) => t.td_name(),
      SearchMessagesFilter::Mention(t) => t.td_name(),
      SearchMessagesFilter::UnreadMention(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl SearchMessagesFilter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let SearchMessagesFilter::_Default(_) = self { true } else { false } }

  pub fn is_empty(&self) -> bool { if let SearchMessagesFilter::Empty(_) = self { true } else { false } }
  pub fn is_animation(&self) -> bool { if let SearchMessagesFilter::Animation(_) = self { true } else { false } }
  pub fn is_audio(&self) -> bool { if let SearchMessagesFilter::Audio(_) = self { true } else { false } }
  pub fn is_document(&self) -> bool { if let SearchMessagesFilter::Document(_) = self { true } else { false } }
  pub fn is_photo(&self) -> bool { if let SearchMessagesFilter::Photo(_) = self { true } else { false } }
  pub fn is_video(&self) -> bool { if let SearchMessagesFilter::Video(_) = self { true } else { false } }
  pub fn is_voice_note(&self) -> bool { if let SearchMessagesFilter::VoiceNote(_) = self { true } else { false } }
  pub fn is_photo_and_video(&self) -> bool { if let SearchMessagesFilter::PhotoAndVideo(_) = self { true } else { false } }
  pub fn is_url(&self) -> bool { if let SearchMessagesFilter::Url(_) = self { true } else { false } }
  pub fn is_chat_photo(&self) -> bool { if let SearchMessagesFilter::ChatPhoto(_) = self { true } else { false } }
  pub fn is_call(&self) -> bool { if let SearchMessagesFilter::Call(_) = self { true } else { false } }
  pub fn is_missed_call(&self) -> bool { if let SearchMessagesFilter::MissedCall(_) = self { true } else { false } }
  pub fn is_video_note(&self) -> bool { if let SearchMessagesFilter::VideoNote(_) = self { true } else { false } }
  pub fn is_voice_and_video_note(&self) -> bool { if let SearchMessagesFilter::VoiceAndVideoNote(_) = self { true } else { false } }
  pub fn is_mention(&self) -> bool { if let SearchMessagesFilter::Mention(_) = self { true } else { false } }
  pub fn is_unread_mention(&self) -> bool { if let SearchMessagesFilter::UnreadMention(_) = self { true } else { false } }

  pub fn on_empty<F: FnOnce(&SearchMessagesFilterEmpty)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Empty(t) = self { fnc(t) }; self }
  pub fn on_animation<F: FnOnce(&SearchMessagesFilterAnimation)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Animation(t) = self { fnc(t) }; self }
  pub fn on_audio<F: FnOnce(&SearchMessagesFilterAudio)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Audio(t) = self { fnc(t) }; self }
  pub fn on_document<F: FnOnce(&SearchMessagesFilterDocument)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Document(t) = self { fnc(t) }; self }
  pub fn on_photo<F: FnOnce(&SearchMessagesFilterPhoto)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Photo(t) = self { fnc(t) }; self }
  pub fn on_video<F: FnOnce(&SearchMessagesFilterVideo)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Video(t) = self { fnc(t) }; self }
  pub fn on_voice_note<F: FnOnce(&SearchMessagesFilterVoiceNote)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::VoiceNote(t) = self { fnc(t) }; self }
  pub fn on_photo_and_video<F: FnOnce(&SearchMessagesFilterPhotoAndVideo)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::PhotoAndVideo(t) = self { fnc(t) }; self }
  pub fn on_url<F: FnOnce(&SearchMessagesFilterUrl)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Url(t) = self { fnc(t) }; self }
  pub fn on_chat_photo<F: FnOnce(&SearchMessagesFilterChatPhoto)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::ChatPhoto(t) = self { fnc(t) }; self }
  pub fn on_call<F: FnOnce(&SearchMessagesFilterCall)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Call(t) = self { fnc(t) }; self }
  pub fn on_missed_call<F: FnOnce(&SearchMessagesFilterMissedCall)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::MissedCall(t) = self { fnc(t) }; self }
  pub fn on_video_note<F: FnOnce(&SearchMessagesFilterVideoNote)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::VideoNote(t) = self { fnc(t) }; self }
  pub fn on_voice_and_video_note<F: FnOnce(&SearchMessagesFilterVoiceAndVideoNote)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::VoiceAndVideoNote(t) = self { fnc(t) }; self }
  pub fn on_mention<F: FnOnce(&SearchMessagesFilterMention)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::Mention(t) = self { fnc(t) }; self }
  pub fn on_unread_mention<F: FnOnce(&SearchMessagesFilterUnreadMention)>(&self, fnc: F) -> &Self { if let SearchMessagesFilter::UnreadMention(t) = self { fnc(t) }; self }

  pub fn as_empty(&self) -> Option<&SearchMessagesFilterEmpty> { if let SearchMessagesFilter::Empty(t) = self { return Some(t) } None }
  pub fn as_animation(&self) -> Option<&SearchMessagesFilterAnimation> { if let SearchMessagesFilter::Animation(t) = self { return Some(t) } None }
  pub fn as_audio(&self) -> Option<&SearchMessagesFilterAudio> { if let SearchMessagesFilter::Audio(t) = self { return Some(t) } None }
  pub fn as_document(&self) -> Option<&SearchMessagesFilterDocument> { if let SearchMessagesFilter::Document(t) = self { return Some(t) } None }
  pub fn as_photo(&self) -> Option<&SearchMessagesFilterPhoto> { if let SearchMessagesFilter::Photo(t) = self { return Some(t) } None }
  pub fn as_video(&self) -> Option<&SearchMessagesFilterVideo> { if let SearchMessagesFilter::Video(t) = self { return Some(t) } None }
  pub fn as_voice_note(&self) -> Option<&SearchMessagesFilterVoiceNote> { if let SearchMessagesFilter::VoiceNote(t) = self { return Some(t) } None }
  pub fn as_photo_and_video(&self) -> Option<&SearchMessagesFilterPhotoAndVideo> { if let SearchMessagesFilter::PhotoAndVideo(t) = self { return Some(t) } None }
  pub fn as_url(&self) -> Option<&SearchMessagesFilterUrl> { if let SearchMessagesFilter::Url(t) = self { return Some(t) } None }
  pub fn as_chat_photo(&self) -> Option<&SearchMessagesFilterChatPhoto> { if let SearchMessagesFilter::ChatPhoto(t) = self { return Some(t) } None }
  pub fn as_call(&self) -> Option<&SearchMessagesFilterCall> { if let SearchMessagesFilter::Call(t) = self { return Some(t) } None }
  pub fn as_missed_call(&self) -> Option<&SearchMessagesFilterMissedCall> { if let SearchMessagesFilter::MissedCall(t) = self { return Some(t) } None }
  pub fn as_video_note(&self) -> Option<&SearchMessagesFilterVideoNote> { if let SearchMessagesFilter::VideoNote(t) = self { return Some(t) } None }
  pub fn as_voice_and_video_note(&self) -> Option<&SearchMessagesFilterVoiceAndVideoNote> { if let SearchMessagesFilter::VoiceAndVideoNote(t) = self { return Some(t) } None }
  pub fn as_mention(&self) -> Option<&SearchMessagesFilterMention> { if let SearchMessagesFilter::Mention(t) = self { return Some(t) } None }
  pub fn as_unread_mention(&self) -> Option<&SearchMessagesFilterUnreadMention> { if let SearchMessagesFilter::UnreadMention(t) = self { return Some(t) } None }



  pub fn empty<T: AsRef<SearchMessagesFilterEmpty>>(t: T) -> Self { SearchMessagesFilter::Empty(t.as_ref().clone()) }

  pub fn animation<T: AsRef<SearchMessagesFilterAnimation>>(t: T) -> Self { SearchMessagesFilter::Animation(t.as_ref().clone()) }

  pub fn audio<T: AsRef<SearchMessagesFilterAudio>>(t: T) -> Self { SearchMessagesFilter::Audio(t.as_ref().clone()) }

  pub fn document<T: AsRef<SearchMessagesFilterDocument>>(t: T) -> Self { SearchMessagesFilter::Document(t.as_ref().clone()) }

  pub fn photo<T: AsRef<SearchMessagesFilterPhoto>>(t: T) -> Self { SearchMessagesFilter::Photo(t.as_ref().clone()) }

  pub fn video<T: AsRef<SearchMessagesFilterVideo>>(t: T) -> Self { SearchMessagesFilter::Video(t.as_ref().clone()) }

  pub fn voice_note<T: AsRef<SearchMessagesFilterVoiceNote>>(t: T) -> Self { SearchMessagesFilter::VoiceNote(t.as_ref().clone()) }

  pub fn photo_and_video<T: AsRef<SearchMessagesFilterPhotoAndVideo>>(t: T) -> Self { SearchMessagesFilter::PhotoAndVideo(t.as_ref().clone()) }

  pub fn url<T: AsRef<SearchMessagesFilterUrl>>(t: T) -> Self { SearchMessagesFilter::Url(t.as_ref().clone()) }

  pub fn chat_photo<T: AsRef<SearchMessagesFilterChatPhoto>>(t: T) -> Self { SearchMessagesFilter::ChatPhoto(t.as_ref().clone()) }

  pub fn call<T: AsRef<SearchMessagesFilterCall>>(t: T) -> Self { SearchMessagesFilter::Call(t.as_ref().clone()) }

  pub fn missed_call<T: AsRef<SearchMessagesFilterMissedCall>>(t: T) -> Self { SearchMessagesFilter::MissedCall(t.as_ref().clone()) }

  pub fn video_note<T: AsRef<SearchMessagesFilterVideoNote>>(t: T) -> Self { SearchMessagesFilter::VideoNote(t.as_ref().clone()) }

  pub fn voice_and_video_note<T: AsRef<SearchMessagesFilterVoiceAndVideoNote>>(t: T) -> Self { SearchMessagesFilter::VoiceAndVideoNote(t.as_ref().clone()) }

  pub fn mention<T: AsRef<SearchMessagesFilterMention>>(t: T) -> Self { SearchMessagesFilter::Mention(t.as_ref().clone()) }

  pub fn unread_mention<T: AsRef<SearchMessagesFilterUnreadMention>>(t: T) -> Self { SearchMessagesFilter::UnreadMention(t.as_ref().clone()) }

}

impl AsRef<SearchMessagesFilter> for SearchMessagesFilter {
  fn as_ref(&self) -> &SearchMessagesFilter { self }
}







/// Returns all found messages, no filter is applied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterEmpty" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterEmpty {}



impl SearchMessagesFilterEmpty {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterEmptyBuilder {
    let mut inner = SearchMessagesFilterEmpty::default();
    inner.td_name = "searchMessagesFilterEmpty".to_string();
    RTDSearchMessagesFilterEmptyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterEmptyBuilder {
  inner: SearchMessagesFilterEmpty
}

impl RTDSearchMessagesFilterEmptyBuilder {
  pub fn build(&self) -> SearchMessagesFilterEmpty { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterEmpty> for SearchMessagesFilterEmpty {
  fn as_ref(&self) -> &SearchMessagesFilterEmpty { self }
}

impl AsRef<SearchMessagesFilterEmpty> for RTDSearchMessagesFilterEmptyBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterEmpty { &self.inner }
}







/// Returns only animation messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterAnimation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterAnimation {}



impl SearchMessagesFilterAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterAnimationBuilder {
    let mut inner = SearchMessagesFilterAnimation::default();
    inner.td_name = "searchMessagesFilterAnimation".to_string();
    RTDSearchMessagesFilterAnimationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterAnimationBuilder {
  inner: SearchMessagesFilterAnimation
}

impl RTDSearchMessagesFilterAnimationBuilder {
  pub fn build(&self) -> SearchMessagesFilterAnimation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterAnimation> for SearchMessagesFilterAnimation {
  fn as_ref(&self) -> &SearchMessagesFilterAnimation { self }
}

impl AsRef<SearchMessagesFilterAnimation> for RTDSearchMessagesFilterAnimationBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterAnimation { &self.inner }
}







/// Returns only audio messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterAudio" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterAudio {}



impl SearchMessagesFilterAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterAudioBuilder {
    let mut inner = SearchMessagesFilterAudio::default();
    inner.td_name = "searchMessagesFilterAudio".to_string();
    RTDSearchMessagesFilterAudioBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterAudioBuilder {
  inner: SearchMessagesFilterAudio
}

impl RTDSearchMessagesFilterAudioBuilder {
  pub fn build(&self) -> SearchMessagesFilterAudio { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterAudio> for SearchMessagesFilterAudio {
  fn as_ref(&self) -> &SearchMessagesFilterAudio { self }
}

impl AsRef<SearchMessagesFilterAudio> for RTDSearchMessagesFilterAudioBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterAudio { &self.inner }
}







/// Returns only document messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterDocument" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterDocument {}



impl SearchMessagesFilterDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterDocumentBuilder {
    let mut inner = SearchMessagesFilterDocument::default();
    inner.td_name = "searchMessagesFilterDocument".to_string();
    RTDSearchMessagesFilterDocumentBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterDocumentBuilder {
  inner: SearchMessagesFilterDocument
}

impl RTDSearchMessagesFilterDocumentBuilder {
  pub fn build(&self) -> SearchMessagesFilterDocument { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterDocument> for SearchMessagesFilterDocument {
  fn as_ref(&self) -> &SearchMessagesFilterDocument { self }
}

impl AsRef<SearchMessagesFilterDocument> for RTDSearchMessagesFilterDocumentBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterDocument { &self.inner }
}







/// Returns only photo messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterPhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterPhoto {}



impl SearchMessagesFilterPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterPhotoBuilder {
    let mut inner = SearchMessagesFilterPhoto::default();
    inner.td_name = "searchMessagesFilterPhoto".to_string();
    RTDSearchMessagesFilterPhotoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterPhotoBuilder {
  inner: SearchMessagesFilterPhoto
}

impl RTDSearchMessagesFilterPhotoBuilder {
  pub fn build(&self) -> SearchMessagesFilterPhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterPhoto> for SearchMessagesFilterPhoto {
  fn as_ref(&self) -> &SearchMessagesFilterPhoto { self }
}

impl AsRef<SearchMessagesFilterPhoto> for RTDSearchMessagesFilterPhotoBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterPhoto { &self.inner }
}







/// Returns only video messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterVideo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterVideo {}



impl SearchMessagesFilterVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterVideoBuilder {
    let mut inner = SearchMessagesFilterVideo::default();
    inner.td_name = "searchMessagesFilterVideo".to_string();
    RTDSearchMessagesFilterVideoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVideoBuilder {
  inner: SearchMessagesFilterVideo
}

impl RTDSearchMessagesFilterVideoBuilder {
  pub fn build(&self) -> SearchMessagesFilterVideo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterVideo> for SearchMessagesFilterVideo {
  fn as_ref(&self) -> &SearchMessagesFilterVideo { self }
}

impl AsRef<SearchMessagesFilterVideo> for RTDSearchMessagesFilterVideoBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterVideo { &self.inner }
}







/// Returns only voice note messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterVoiceNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterVoiceNote {}



impl SearchMessagesFilterVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterVoiceNoteBuilder {
    let mut inner = SearchMessagesFilterVoiceNote::default();
    inner.td_name = "searchMessagesFilterVoiceNote".to_string();
    RTDSearchMessagesFilterVoiceNoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVoiceNoteBuilder {
  inner: SearchMessagesFilterVoiceNote
}

impl RTDSearchMessagesFilterVoiceNoteBuilder {
  pub fn build(&self) -> SearchMessagesFilterVoiceNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterVoiceNote> for SearchMessagesFilterVoiceNote {
  fn as_ref(&self) -> &SearchMessagesFilterVoiceNote { self }
}

impl AsRef<SearchMessagesFilterVoiceNote> for RTDSearchMessagesFilterVoiceNoteBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterVoiceNote { &self.inner }
}







/// Returns only photo and video messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterPhotoAndVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterPhotoAndVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterPhotoAndVideo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterPhotoAndVideo {}



impl SearchMessagesFilterPhotoAndVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterPhotoAndVideoBuilder {
    let mut inner = SearchMessagesFilterPhotoAndVideo::default();
    inner.td_name = "searchMessagesFilterPhotoAndVideo".to_string();
    RTDSearchMessagesFilterPhotoAndVideoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterPhotoAndVideoBuilder {
  inner: SearchMessagesFilterPhotoAndVideo
}

impl RTDSearchMessagesFilterPhotoAndVideoBuilder {
  pub fn build(&self) -> SearchMessagesFilterPhotoAndVideo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterPhotoAndVideo> for SearchMessagesFilterPhotoAndVideo {
  fn as_ref(&self) -> &SearchMessagesFilterPhotoAndVideo { self }
}

impl AsRef<SearchMessagesFilterPhotoAndVideo> for RTDSearchMessagesFilterPhotoAndVideoBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterPhotoAndVideo { &self.inner }
}







/// Returns only messages containing URLs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterUrl" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterUrl {}



impl SearchMessagesFilterUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterUrlBuilder {
    let mut inner = SearchMessagesFilterUrl::default();
    inner.td_name = "searchMessagesFilterUrl".to_string();
    RTDSearchMessagesFilterUrlBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterUrlBuilder {
  inner: SearchMessagesFilterUrl
}

impl RTDSearchMessagesFilterUrlBuilder {
  pub fn build(&self) -> SearchMessagesFilterUrl { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterUrl> for SearchMessagesFilterUrl {
  fn as_ref(&self) -> &SearchMessagesFilterUrl { self }
}

impl AsRef<SearchMessagesFilterUrl> for RTDSearchMessagesFilterUrlBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterUrl { &self.inner }
}







/// Returns only messages containing chat photos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterChatPhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterChatPhoto {}



impl SearchMessagesFilterChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterChatPhotoBuilder {
    let mut inner = SearchMessagesFilterChatPhoto::default();
    inner.td_name = "searchMessagesFilterChatPhoto".to_string();
    RTDSearchMessagesFilterChatPhotoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterChatPhotoBuilder {
  inner: SearchMessagesFilterChatPhoto
}

impl RTDSearchMessagesFilterChatPhotoBuilder {
  pub fn build(&self) -> SearchMessagesFilterChatPhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterChatPhoto> for SearchMessagesFilterChatPhoto {
  fn as_ref(&self) -> &SearchMessagesFilterChatPhoto { self }
}

impl AsRef<SearchMessagesFilterChatPhoto> for RTDSearchMessagesFilterChatPhotoBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterChatPhoto { &self.inner }
}







/// Returns only call messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterCall" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterCall {}



impl SearchMessagesFilterCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterCallBuilder {
    let mut inner = SearchMessagesFilterCall::default();
    inner.td_name = "searchMessagesFilterCall".to_string();
    RTDSearchMessagesFilterCallBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterCallBuilder {
  inner: SearchMessagesFilterCall
}

impl RTDSearchMessagesFilterCallBuilder {
  pub fn build(&self) -> SearchMessagesFilterCall { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterCall> for SearchMessagesFilterCall {
  fn as_ref(&self) -> &SearchMessagesFilterCall { self }
}

impl AsRef<SearchMessagesFilterCall> for RTDSearchMessagesFilterCallBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterCall { &self.inner }
}







/// Returns only incoming call messages with missed/declined discard reasons
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterMissedCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterMissedCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterMissedCall" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterMissedCall {}



impl SearchMessagesFilterMissedCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterMissedCallBuilder {
    let mut inner = SearchMessagesFilterMissedCall::default();
    inner.td_name = "searchMessagesFilterMissedCall".to_string();
    RTDSearchMessagesFilterMissedCallBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterMissedCallBuilder {
  inner: SearchMessagesFilterMissedCall
}

impl RTDSearchMessagesFilterMissedCallBuilder {
  pub fn build(&self) -> SearchMessagesFilterMissedCall { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterMissedCall> for SearchMessagesFilterMissedCall {
  fn as_ref(&self) -> &SearchMessagesFilterMissedCall { self }
}

impl AsRef<SearchMessagesFilterMissedCall> for RTDSearchMessagesFilterMissedCallBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterMissedCall { &self.inner }
}







/// Returns only video note messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterVideoNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterVideoNote {}



impl SearchMessagesFilterVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterVideoNoteBuilder {
    let mut inner = SearchMessagesFilterVideoNote::default();
    inner.td_name = "searchMessagesFilterVideoNote".to_string();
    RTDSearchMessagesFilterVideoNoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVideoNoteBuilder {
  inner: SearchMessagesFilterVideoNote
}

impl RTDSearchMessagesFilterVideoNoteBuilder {
  pub fn build(&self) -> SearchMessagesFilterVideoNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterVideoNote> for SearchMessagesFilterVideoNote {
  fn as_ref(&self) -> &SearchMessagesFilterVideoNote { self }
}

impl AsRef<SearchMessagesFilterVideoNote> for RTDSearchMessagesFilterVideoNoteBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterVideoNote { &self.inner }
}







/// Returns only voice and video note messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVoiceAndVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterVoiceAndVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterVoiceAndVideoNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterVoiceAndVideoNote {}



impl SearchMessagesFilterVoiceAndVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterVoiceAndVideoNoteBuilder {
    let mut inner = SearchMessagesFilterVoiceAndVideoNote::default();
    inner.td_name = "searchMessagesFilterVoiceAndVideoNote".to_string();
    RTDSearchMessagesFilterVoiceAndVideoNoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVoiceAndVideoNoteBuilder {
  inner: SearchMessagesFilterVoiceAndVideoNote
}

impl RTDSearchMessagesFilterVoiceAndVideoNoteBuilder {
  pub fn build(&self) -> SearchMessagesFilterVoiceAndVideoNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterVoiceAndVideoNote> for SearchMessagesFilterVoiceAndVideoNote {
  fn as_ref(&self) -> &SearchMessagesFilterVoiceAndVideoNote { self }
}

impl AsRef<SearchMessagesFilterVoiceAndVideoNote> for RTDSearchMessagesFilterVoiceAndVideoNoteBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterVoiceAndVideoNote { &self.inner }
}







/// Returns only messages with mentions of the current user, or messages that are replies to their messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterMention {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterMention {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterMention" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterMention {}



impl SearchMessagesFilterMention {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterMentionBuilder {
    let mut inner = SearchMessagesFilterMention::default();
    inner.td_name = "searchMessagesFilterMention".to_string();
    RTDSearchMessagesFilterMentionBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterMentionBuilder {
  inner: SearchMessagesFilterMention
}

impl RTDSearchMessagesFilterMentionBuilder {
  pub fn build(&self) -> SearchMessagesFilterMention { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterMention> for SearchMessagesFilterMention {
  fn as_ref(&self) -> &SearchMessagesFilterMention { self }
}

impl AsRef<SearchMessagesFilterMention> for RTDSearchMessagesFilterMentionBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterMention { &self.inner }
}







/// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query or by the sending user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterUnreadMention {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for SearchMessagesFilterUnreadMention {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterUnreadMention" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDSearchMessagesFilter for SearchMessagesFilterUnreadMention {}



impl SearchMessagesFilterUnreadMention {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSearchMessagesFilterUnreadMentionBuilder {
    let mut inner = SearchMessagesFilterUnreadMention::default();
    inner.td_name = "searchMessagesFilterUnreadMention".to_string();
    RTDSearchMessagesFilterUnreadMentionBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterUnreadMentionBuilder {
  inner: SearchMessagesFilterUnreadMention
}

impl RTDSearchMessagesFilterUnreadMentionBuilder {
  pub fn build(&self) -> SearchMessagesFilterUnreadMention { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<SearchMessagesFilterUnreadMention> for SearchMessagesFilterUnreadMention {
  fn as_ref(&self) -> &SearchMessagesFilterUnreadMention { self }
}

impl AsRef<SearchMessagesFilterUnreadMention> for RTDSearchMessagesFilterUnreadMentionBuilder {
  fn as_ref(&self) -> &SearchMessagesFilterUnreadMention { &self.inner }
}



