
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a single result of an inline query
pub trait TDInlineQueryResult: Debug + RObject {}

/// Represents a single result of an inline query
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InlineQueryResult {
  #[doc(hidden)] _Default(()),
  /// Represents a link to an article or web page
  Article(InlineQueryResultArticle),
  /// Represents a user contact
  Contact(InlineQueryResultContact),
  /// Represents a point on the map
  Location(InlineQueryResultLocation),
  /// Represents information about a venue
  Venue(InlineQueryResultVenue),
  /// Represents information about a game
  Game(InlineQueryResultGame),
  /// Represents an animation file
  Animation(InlineQueryResultAnimation),
  /// Represents an audio file
  Audio(InlineQueryResultAudio),
  /// Represents a document
  Document(InlineQueryResultDocument),
  /// Represents a photo
  Photo(InlineQueryResultPhoto),
  /// Represents a sticker
  Sticker(InlineQueryResultSticker),
  /// Represents a video
  Video(InlineQueryResultVideo),
  /// Represents a voice note
  VoiceNote(InlineQueryResultVoiceNote),

}

impl Default for InlineQueryResult {
  fn default() -> Self { InlineQueryResult::_Default(()) }
}

impl<'de> Deserialize<'de> for InlineQueryResult {
  fn deserialize<D>(deserializer: D) -> Result<InlineQueryResult, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InlineQueryResult,
      (inlineQueryResultArticle, Article);
      (inlineQueryResultContact, Contact);
      (inlineQueryResultLocation, Location);
      (inlineQueryResultVenue, Venue);
      (inlineQueryResultGame, Game);
      (inlineQueryResultAnimation, Animation);
      (inlineQueryResultAudio, Audio);
      (inlineQueryResultDocument, Document);
      (inlineQueryResultPhoto, Photo);
      (inlineQueryResultSticker, Sticker);
      (inlineQueryResultVideo, Video);
      (inlineQueryResultVoiceNote, VoiceNote);

    )(deserializer)
  }
}

impl RObject for InlineQueryResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InlineQueryResult::Article(t) => t.td_name(),
      InlineQueryResult::Contact(t) => t.td_name(),
      InlineQueryResult::Location(t) => t.td_name(),
      InlineQueryResult::Venue(t) => t.td_name(),
      InlineQueryResult::Game(t) => t.td_name(),
      InlineQueryResult::Animation(t) => t.td_name(),
      InlineQueryResult::Audio(t) => t.td_name(),
      InlineQueryResult::Document(t) => t.td_name(),
      InlineQueryResult::Photo(t) => t.td_name(),
      InlineQueryResult::Sticker(t) => t.td_name(),
      InlineQueryResult::Video(t) => t.td_name(),
      InlineQueryResult::VoiceNote(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InlineQueryResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InlineQueryResult::_Default(_) = self { true } else { false } }

  pub fn is_article(&self) -> bool { if let InlineQueryResult::Article(_) = self { true } else { false } }
  pub fn is_contact(&self) -> bool { if let InlineQueryResult::Contact(_) = self { true } else { false } }
  pub fn is_location(&self) -> bool { if let InlineQueryResult::Location(_) = self { true } else { false } }
  pub fn is_venue(&self) -> bool { if let InlineQueryResult::Venue(_) = self { true } else { false } }
  pub fn is_game(&self) -> bool { if let InlineQueryResult::Game(_) = self { true } else { false } }
  pub fn is_animation(&self) -> bool { if let InlineQueryResult::Animation(_) = self { true } else { false } }
  pub fn is_audio(&self) -> bool { if let InlineQueryResult::Audio(_) = self { true } else { false } }
  pub fn is_document(&self) -> bool { if let InlineQueryResult::Document(_) = self { true } else { false } }
  pub fn is_photo(&self) -> bool { if let InlineQueryResult::Photo(_) = self { true } else { false } }
  pub fn is_sticker(&self) -> bool { if let InlineQueryResult::Sticker(_) = self { true } else { false } }
  pub fn is_video(&self) -> bool { if let InlineQueryResult::Video(_) = self { true } else { false } }
  pub fn is_voice_note(&self) -> bool { if let InlineQueryResult::VoiceNote(_) = self { true } else { false } }

  pub fn on_article<F: FnOnce(&InlineQueryResultArticle)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Article(t) = self { fnc(t) }; self }
  pub fn on_contact<F: FnOnce(&InlineQueryResultContact)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Contact(t) = self { fnc(t) }; self }
  pub fn on_location<F: FnOnce(&InlineQueryResultLocation)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Location(t) = self { fnc(t) }; self }
  pub fn on_venue<F: FnOnce(&InlineQueryResultVenue)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Venue(t) = self { fnc(t) }; self }
  pub fn on_game<F: FnOnce(&InlineQueryResultGame)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Game(t) = self { fnc(t) }; self }
  pub fn on_animation<F: FnOnce(&InlineQueryResultAnimation)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Animation(t) = self { fnc(t) }; self }
  pub fn on_audio<F: FnOnce(&InlineQueryResultAudio)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Audio(t) = self { fnc(t) }; self }
  pub fn on_document<F: FnOnce(&InlineQueryResultDocument)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Document(t) = self { fnc(t) }; self }
  pub fn on_photo<F: FnOnce(&InlineQueryResultPhoto)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Photo(t) = self { fnc(t) }; self }
  pub fn on_sticker<F: FnOnce(&InlineQueryResultSticker)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Sticker(t) = self { fnc(t) }; self }
  pub fn on_video<F: FnOnce(&InlineQueryResultVideo)>(&self, fnc: F) -> &Self { if let InlineQueryResult::Video(t) = self { fnc(t) }; self }
  pub fn on_voice_note<F: FnOnce(&InlineQueryResultVoiceNote)>(&self, fnc: F) -> &Self { if let InlineQueryResult::VoiceNote(t) = self { fnc(t) }; self }

  pub fn as_article(&self) -> Option<&InlineQueryResultArticle> { if let InlineQueryResult::Article(t) = self { return Some(t) } None }
  pub fn as_contact(&self) -> Option<&InlineQueryResultContact> { if let InlineQueryResult::Contact(t) = self { return Some(t) } None }
  pub fn as_location(&self) -> Option<&InlineQueryResultLocation> { if let InlineQueryResult::Location(t) = self { return Some(t) } None }
  pub fn as_venue(&self) -> Option<&InlineQueryResultVenue> { if let InlineQueryResult::Venue(t) = self { return Some(t) } None }
  pub fn as_game(&self) -> Option<&InlineQueryResultGame> { if let InlineQueryResult::Game(t) = self { return Some(t) } None }
  pub fn as_animation(&self) -> Option<&InlineQueryResultAnimation> { if let InlineQueryResult::Animation(t) = self { return Some(t) } None }
  pub fn as_audio(&self) -> Option<&InlineQueryResultAudio> { if let InlineQueryResult::Audio(t) = self { return Some(t) } None }
  pub fn as_document(&self) -> Option<&InlineQueryResultDocument> { if let InlineQueryResult::Document(t) = self { return Some(t) } None }
  pub fn as_photo(&self) -> Option<&InlineQueryResultPhoto> { if let InlineQueryResult::Photo(t) = self { return Some(t) } None }
  pub fn as_sticker(&self) -> Option<&InlineQueryResultSticker> { if let InlineQueryResult::Sticker(t) = self { return Some(t) } None }
  pub fn as_video(&self) -> Option<&InlineQueryResultVideo> { if let InlineQueryResult::Video(t) = self { return Some(t) } None }
  pub fn as_voice_note(&self) -> Option<&InlineQueryResultVoiceNote> { if let InlineQueryResult::VoiceNote(t) = self { return Some(t) } None }



  pub fn article<T: AsRef<InlineQueryResultArticle>>(t: T) -> Self { InlineQueryResult::Article(t.as_ref().clone()) }

  pub fn contact<T: AsRef<InlineQueryResultContact>>(t: T) -> Self { InlineQueryResult::Contact(t.as_ref().clone()) }

  pub fn location<T: AsRef<InlineQueryResultLocation>>(t: T) -> Self { InlineQueryResult::Location(t.as_ref().clone()) }

  pub fn venue<T: AsRef<InlineQueryResultVenue>>(t: T) -> Self { InlineQueryResult::Venue(t.as_ref().clone()) }

  pub fn game<T: AsRef<InlineQueryResultGame>>(t: T) -> Self { InlineQueryResult::Game(t.as_ref().clone()) }

  pub fn animation<T: AsRef<InlineQueryResultAnimation>>(t: T) -> Self { InlineQueryResult::Animation(t.as_ref().clone()) }

  pub fn audio<T: AsRef<InlineQueryResultAudio>>(t: T) -> Self { InlineQueryResult::Audio(t.as_ref().clone()) }

  pub fn document<T: AsRef<InlineQueryResultDocument>>(t: T) -> Self { InlineQueryResult::Document(t.as_ref().clone()) }

  pub fn photo<T: AsRef<InlineQueryResultPhoto>>(t: T) -> Self { InlineQueryResult::Photo(t.as_ref().clone()) }

  pub fn sticker<T: AsRef<InlineQueryResultSticker>>(t: T) -> Self { InlineQueryResult::Sticker(t.as_ref().clone()) }

  pub fn video<T: AsRef<InlineQueryResultVideo>>(t: T) -> Self { InlineQueryResult::Video(t.as_ref().clone()) }

  pub fn voice_note<T: AsRef<InlineQueryResultVoiceNote>>(t: T) -> Self { InlineQueryResult::VoiceNote(t.as_ref().clone()) }

}

impl AsRef<InlineQueryResult> for InlineQueryResult {
  fn as_ref(&self) -> &InlineQueryResult { self }
}







/// Represents a link to an article or web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// URL of the result, if it exists
  url: String,
  /// True, if the URL must be not shown
  hide_url: bool,
  /// Title of the result
  title: String,
  /// Represents a link to an article or web page
  description: String,
  /// Result thumbnail; may be null
  thumbnail: Option<PhotoSize>,
  
}

impl RObject for InlineQueryResultArticle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultArticle" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultArticle {}



impl InlineQueryResultArticle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultArticleBuilder {
    let mut inner = InlineQueryResultArticle::default();
    inner.td_name = "inlineQueryResultArticle".to_string();
    RTDInlineQueryResultArticleBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn url(&self) -> &String { &self.url }

  pub fn hide_url(&self) -> bool { self.hide_url }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultArticleBuilder {
  inner: InlineQueryResultArticle
}

impl RTDInlineQueryResultArticleBuilder {
  pub fn build(&self) -> InlineQueryResultArticle { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn hide_url(&mut self, hide_url: bool) -> &mut Self {
    self.inner.hide_url = hide_url;
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

   
  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

}

impl AsRef<InlineQueryResultArticle> for InlineQueryResultArticle {
  fn as_ref(&self) -> &InlineQueryResultArticle { self }
}

impl AsRef<InlineQueryResultArticle> for RTDInlineQueryResultArticleBuilder {
  fn as_ref(&self) -> &InlineQueryResultArticle { &self.inner }
}







/// Represents a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// A user contact
  contact: Contact,
  /// Result thumbnail; may be null
  thumbnail: Option<PhotoSize>,
  
}

impl RObject for InlineQueryResultContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultContact" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultContact {}



impl InlineQueryResultContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultContactBuilder {
    let mut inner = InlineQueryResultContact::default();
    inner.td_name = "inlineQueryResultContact".to_string();
    RTDInlineQueryResultContactBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn contact(&self) -> &Contact { &self.contact }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultContactBuilder {
  inner: InlineQueryResultContact
}

impl RTDInlineQueryResultContactBuilder {
  pub fn build(&self) -> InlineQueryResultContact { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
    self.inner.contact = contact.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

}

impl AsRef<InlineQueryResultContact> for InlineQueryResultContact {
  fn as_ref(&self) -> &InlineQueryResultContact { self }
}

impl AsRef<InlineQueryResultContact> for RTDInlineQueryResultContactBuilder {
  fn as_ref(&self) -> &InlineQueryResultContact { &self.inner }
}







/// Represents a point on the map
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Location result
  location: Location,
  /// Title of the result
  title: String,
  /// Result thumbnail; may be null
  thumbnail: Option<PhotoSize>,
  
}

impl RObject for InlineQueryResultLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultLocation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultLocation {}



impl InlineQueryResultLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultLocationBuilder {
    let mut inner = InlineQueryResultLocation::default();
    inner.td_name = "inlineQueryResultLocation".to_string();
    RTDInlineQueryResultLocationBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn location(&self) -> &Location { &self.location }

  pub fn title(&self) -> &String { &self.title }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultLocationBuilder {
  inner: InlineQueryResultLocation
}

impl RTDInlineQueryResultLocationBuilder {
  pub fn build(&self) -> InlineQueryResultLocation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
    self.inner.location = location.as_ref().clone();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

   
  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

}

impl AsRef<InlineQueryResultLocation> for InlineQueryResultLocation {
  fn as_ref(&self) -> &InlineQueryResultLocation { self }
}

impl AsRef<InlineQueryResultLocation> for RTDInlineQueryResultLocationBuilder {
  fn as_ref(&self) -> &InlineQueryResultLocation { &self.inner }
}







/// Represents information about a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Venue result
  venue: Venue,
  /// Result thumbnail; may be null
  thumbnail: Option<PhotoSize>,
  
}

impl RObject for InlineQueryResultVenue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultVenue" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultVenue {}



impl InlineQueryResultVenue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultVenueBuilder {
    let mut inner = InlineQueryResultVenue::default();
    inner.td_name = "inlineQueryResultVenue".to_string();
    RTDInlineQueryResultVenueBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn venue(&self) -> &Venue { &self.venue }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultVenueBuilder {
  inner: InlineQueryResultVenue
}

impl RTDInlineQueryResultVenueBuilder {
  pub fn build(&self) -> InlineQueryResultVenue { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
    self.inner.venue = venue.as_ref().clone();
    self
  }

   
  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

}

impl AsRef<InlineQueryResultVenue> for InlineQueryResultVenue {
  fn as_ref(&self) -> &InlineQueryResultVenue { self }
}

impl AsRef<InlineQueryResultVenue> for RTDInlineQueryResultVenueBuilder {
  fn as_ref(&self) -> &InlineQueryResultVenue { &self.inner }
}







/// Represents information about a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Game result
  game: Game,
  
}

impl RObject for InlineQueryResultGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultGame" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultGame {}



impl InlineQueryResultGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultGameBuilder {
    let mut inner = InlineQueryResultGame::default();
    inner.td_name = "inlineQueryResultGame".to_string();
    RTDInlineQueryResultGameBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn game(&self) -> &Game { &self.game }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultGameBuilder {
  inner: InlineQueryResultGame
}

impl RTDInlineQueryResultGameBuilder {
  pub fn build(&self) -> InlineQueryResultGame { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn game<T: AsRef<Game>>(&mut self, game: T) -> &mut Self {
    self.inner.game = game.as_ref().clone();
    self
  }

}

impl AsRef<InlineQueryResultGame> for InlineQueryResultGame {
  fn as_ref(&self) -> &InlineQueryResultGame { self }
}

impl AsRef<InlineQueryResultGame> for RTDInlineQueryResultGameBuilder {
  fn as_ref(&self) -> &InlineQueryResultGame { &self.inner }
}







/// Represents an animation file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Animation file
  animation: Animation,
  /// Animation title
  title: String,
  
}

impl RObject for InlineQueryResultAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultAnimation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultAnimation {}



impl InlineQueryResultAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultAnimationBuilder {
    let mut inner = InlineQueryResultAnimation::default();
    inner.td_name = "inlineQueryResultAnimation".to_string();
    RTDInlineQueryResultAnimationBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn animation(&self) -> &Animation { &self.animation }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultAnimationBuilder {
  inner: InlineQueryResultAnimation
}

impl RTDInlineQueryResultAnimationBuilder {
  pub fn build(&self) -> InlineQueryResultAnimation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<InlineQueryResultAnimation> for InlineQueryResultAnimation {
  fn as_ref(&self) -> &InlineQueryResultAnimation { self }
}

impl AsRef<InlineQueryResultAnimation> for RTDInlineQueryResultAnimationBuilder {
  fn as_ref(&self) -> &InlineQueryResultAnimation { &self.inner }
}







/// Represents an audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Audio file
  audio: Audio,
  
}

impl RObject for InlineQueryResultAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultAudio" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultAudio {}



impl InlineQueryResultAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultAudioBuilder {
    let mut inner = InlineQueryResultAudio::default();
    inner.td_name = "inlineQueryResultAudio".to_string();
    RTDInlineQueryResultAudioBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn audio(&self) -> &Audio { &self.audio }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultAudioBuilder {
  inner: InlineQueryResultAudio
}

impl RTDInlineQueryResultAudioBuilder {
  pub fn build(&self) -> InlineQueryResultAudio { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
    self.inner.audio = audio.as_ref().clone();
    self
  }

}

impl AsRef<InlineQueryResultAudio> for InlineQueryResultAudio {
  fn as_ref(&self) -> &InlineQueryResultAudio { self }
}

impl AsRef<InlineQueryResultAudio> for RTDInlineQueryResultAudioBuilder {
  fn as_ref(&self) -> &InlineQueryResultAudio { &self.inner }
}







/// Represents a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Document
  document: Document,
  /// Document title
  title: String,
  /// Represents a document
  description: String,
  
}

impl RObject for InlineQueryResultDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultDocument" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultDocument {}



impl InlineQueryResultDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultDocumentBuilder {
    let mut inner = InlineQueryResultDocument::default();
    inner.td_name = "inlineQueryResultDocument".to_string();
    RTDInlineQueryResultDocumentBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn document(&self) -> &Document { &self.document }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultDocumentBuilder {
  inner: InlineQueryResultDocument
}

impl RTDInlineQueryResultDocumentBuilder {
  pub fn build(&self) -> InlineQueryResultDocument { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = document.as_ref().clone();
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

}

impl AsRef<InlineQueryResultDocument> for InlineQueryResultDocument {
  fn as_ref(&self) -> &InlineQueryResultDocument { self }
}

impl AsRef<InlineQueryResultDocument> for RTDInlineQueryResultDocumentBuilder {
  fn as_ref(&self) -> &InlineQueryResultDocument { &self.inner }
}







/// Represents a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Photo
  photo: Photo,
  /// Title of the result, if known
  title: String,
  /// Represents a photo
  description: String,
  
}

impl RObject for InlineQueryResultPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultPhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultPhoto {}



impl InlineQueryResultPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultPhotoBuilder {
    let mut inner = InlineQueryResultPhoto::default();
    inner.td_name = "inlineQueryResultPhoto".to_string();
    RTDInlineQueryResultPhotoBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn photo(&self) -> &Photo { &self.photo }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultPhotoBuilder {
  inner: InlineQueryResultPhoto
}

impl RTDInlineQueryResultPhotoBuilder {
  pub fn build(&self) -> InlineQueryResultPhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
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

}

impl AsRef<InlineQueryResultPhoto> for InlineQueryResultPhoto {
  fn as_ref(&self) -> &InlineQueryResultPhoto { self }
}

impl AsRef<InlineQueryResultPhoto> for RTDInlineQueryResultPhotoBuilder {
  fn as_ref(&self) -> &InlineQueryResultPhoto { &self.inner }
}







/// Represents a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Sticker
  sticker: Sticker,
  
}

impl RObject for InlineQueryResultSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultSticker" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultSticker {}



impl InlineQueryResultSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultStickerBuilder {
    let mut inner = InlineQueryResultSticker::default();
    inner.td_name = "inlineQueryResultSticker".to_string();
    RTDInlineQueryResultStickerBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn sticker(&self) -> &Sticker { &self.sticker }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultStickerBuilder {
  inner: InlineQueryResultSticker
}

impl RTDInlineQueryResultStickerBuilder {
  pub fn build(&self) -> InlineQueryResultSticker { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<InlineQueryResultSticker> for InlineQueryResultSticker {
  fn as_ref(&self) -> &InlineQueryResultSticker { self }
}

impl AsRef<InlineQueryResultSticker> for RTDInlineQueryResultStickerBuilder {
  fn as_ref(&self) -> &InlineQueryResultSticker { &self.inner }
}







/// Represents a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Video
  video: Video,
  /// Title of the video
  title: String,
  /// Represents a video
  description: String,
  
}

impl RObject for InlineQueryResultVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultVideo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultVideo {}



impl InlineQueryResultVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultVideoBuilder {
    let mut inner = InlineQueryResultVideo::default();
    inner.td_name = "inlineQueryResultVideo".to_string();
    RTDInlineQueryResultVideoBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn video(&self) -> &Video { &self.video }

  pub fn title(&self) -> &String { &self.title }

  pub fn description(&self) -> &String { &self.description }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultVideoBuilder {
  inner: InlineQueryResultVideo
}

impl RTDInlineQueryResultVideoBuilder {
  pub fn build(&self) -> InlineQueryResultVideo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
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

}

impl AsRef<InlineQueryResultVideo> for InlineQueryResultVideo {
  fn as_ref(&self) -> &InlineQueryResultVideo { self }
}

impl AsRef<InlineQueryResultVideo> for RTDInlineQueryResultVideoBuilder {
  fn as_ref(&self) -> &InlineQueryResultVideo { &self.inner }
}







/// Represents a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique identifier of the query result
  id: String,
  /// Voice note
  voice_note: VoiceNote,
  /// Title of the voice note
  title: String,
  
}

impl RObject for InlineQueryResultVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultVoiceNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInlineQueryResult for InlineQueryResultVoiceNote {}



impl InlineQueryResultVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultVoiceNoteBuilder {
    let mut inner = InlineQueryResultVoiceNote::default();
    inner.td_name = "inlineQueryResultVoiceNote".to_string();
    RTDInlineQueryResultVoiceNoteBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn voice_note(&self) -> &VoiceNote { &self.voice_note }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultVoiceNoteBuilder {
  inner: InlineQueryResultVoiceNote
}

impl RTDInlineQueryResultVoiceNoteBuilder {
  pub fn build(&self) -> InlineQueryResultVoiceNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
    self.inner.voice_note = voice_note.as_ref().clone();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<InlineQueryResultVoiceNote> for InlineQueryResultVoiceNote {
  fn as_ref(&self) -> &InlineQueryResultVoiceNote { self }
}

impl AsRef<InlineQueryResultVoiceNote> for RTDInlineQueryResultVoiceNoteBuilder {
  fn as_ref(&self) -> &InlineQueryResultVoiceNote { &self.inner }
}



