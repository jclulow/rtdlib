
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the type of a file
pub trait TDFileType: Debug + RObject {}

/// Represents the type of a file
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FileType {
  #[doc(hidden)] _Default(()),
  /// The data is not a file
  None(FileTypeNone),
  /// The file is an animation
  Animation(FileTypeAnimation),
  /// The file is an audio file
  Audio(FileTypeAudio),
  /// The file is a document
  Document(FileTypeDocument),
  /// The file is a photo
  Photo(FileTypePhoto),
  /// The file is a profile photo
  ProfilePhoto(FileTypeProfilePhoto),
  /// The file was sent to a secret chat (the file type is not known to the server)
  Secret(FileTypeSecret),
  /// The file is a thumbnail of a file from a secret chat
  SecretThumbnail(FileTypeSecretThumbnail),
  /// The file is a file from Secure storage used for storing Telegram Passport files
  Secure(FileTypeSecure),
  /// The file is a sticker
  Sticker(FileTypeSticker),
  /// The file is a thumbnail of another file
  Thumbnail(FileTypeThumbnail),
  /// The file type is not yet known
  Unknown(FileTypeUnknown),
  /// The file is a video
  Video(FileTypeVideo),
  /// The file is a video note
  VideoNote(FileTypeVideoNote),
  /// The file is a voice note
  VoiceNote(FileTypeVoiceNote),
  /// The file is a wallpaper or a background pattern
  Wallpaper(FileTypeWallpaper),

}

impl Default for FileType {
  fn default() -> Self { FileType::_Default(()) }
}

impl<'de> Deserialize<'de> for FileType {
  fn deserialize<D>(deserializer: D) -> Result<FileType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      FileType,
      (fileTypeNone, None);
      (fileTypeAnimation, Animation);
      (fileTypeAudio, Audio);
      (fileTypeDocument, Document);
      (fileTypePhoto, Photo);
      (fileTypeProfilePhoto, ProfilePhoto);
      (fileTypeSecret, Secret);
      (fileTypeSecretThumbnail, SecretThumbnail);
      (fileTypeSecure, Secure);
      (fileTypeSticker, Sticker);
      (fileTypeThumbnail, Thumbnail);
      (fileTypeUnknown, Unknown);
      (fileTypeVideo, Video);
      (fileTypeVideoNote, VideoNote);
      (fileTypeVoiceNote, VoiceNote);
      (fileTypeWallpaper, Wallpaper);

    )(deserializer)
  }
}

impl RObject for FileType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      FileType::None(t) => t.td_name(),
      FileType::Animation(t) => t.td_name(),
      FileType::Audio(t) => t.td_name(),
      FileType::Document(t) => t.td_name(),
      FileType::Photo(t) => t.td_name(),
      FileType::ProfilePhoto(t) => t.td_name(),
      FileType::Secret(t) => t.td_name(),
      FileType::SecretThumbnail(t) => t.td_name(),
      FileType::Secure(t) => t.td_name(),
      FileType::Sticker(t) => t.td_name(),
      FileType::Thumbnail(t) => t.td_name(),
      FileType::Unknown(t) => t.td_name(),
      FileType::Video(t) => t.td_name(),
      FileType::VideoNote(t) => t.td_name(),
      FileType::VoiceNote(t) => t.td_name(),
      FileType::Wallpaper(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl FileType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let FileType::_Default(_) = self { true } else { false } }

  pub fn is_none(&self) -> bool { if let FileType::None(_) = self { true } else { false } }
  pub fn is_animation(&self) -> bool { if let FileType::Animation(_) = self { true } else { false } }
  pub fn is_audio(&self) -> bool { if let FileType::Audio(_) = self { true } else { false } }
  pub fn is_document(&self) -> bool { if let FileType::Document(_) = self { true } else { false } }
  pub fn is_photo(&self) -> bool { if let FileType::Photo(_) = self { true } else { false } }
  pub fn is_profile_photo(&self) -> bool { if let FileType::ProfilePhoto(_) = self { true } else { false } }
  pub fn is_secret(&self) -> bool { if let FileType::Secret(_) = self { true } else { false } }
  pub fn is_secret_thumbnail(&self) -> bool { if let FileType::SecretThumbnail(_) = self { true } else { false } }
  pub fn is_secure(&self) -> bool { if let FileType::Secure(_) = self { true } else { false } }
  pub fn is_sticker(&self) -> bool { if let FileType::Sticker(_) = self { true } else { false } }
  pub fn is_thumbnail(&self) -> bool { if let FileType::Thumbnail(_) = self { true } else { false } }
  pub fn is_unknown(&self) -> bool { if let FileType::Unknown(_) = self { true } else { false } }
  pub fn is_video(&self) -> bool { if let FileType::Video(_) = self { true } else { false } }
  pub fn is_video_note(&self) -> bool { if let FileType::VideoNote(_) = self { true } else { false } }
  pub fn is_voice_note(&self) -> bool { if let FileType::VoiceNote(_) = self { true } else { false } }
  pub fn is_wallpaper(&self) -> bool { if let FileType::Wallpaper(_) = self { true } else { false } }

  pub fn on_none<F: FnOnce(&FileTypeNone)>(&self, fnc: F) -> &Self { if let FileType::None(t) = self { fnc(t) }; self }
  pub fn on_animation<F: FnOnce(&FileTypeAnimation)>(&self, fnc: F) -> &Self { if let FileType::Animation(t) = self { fnc(t) }; self }
  pub fn on_audio<F: FnOnce(&FileTypeAudio)>(&self, fnc: F) -> &Self { if let FileType::Audio(t) = self { fnc(t) }; self }
  pub fn on_document<F: FnOnce(&FileTypeDocument)>(&self, fnc: F) -> &Self { if let FileType::Document(t) = self { fnc(t) }; self }
  pub fn on_photo<F: FnOnce(&FileTypePhoto)>(&self, fnc: F) -> &Self { if let FileType::Photo(t) = self { fnc(t) }; self }
  pub fn on_profile_photo<F: FnOnce(&FileTypeProfilePhoto)>(&self, fnc: F) -> &Self { if let FileType::ProfilePhoto(t) = self { fnc(t) }; self }
  pub fn on_secret<F: FnOnce(&FileTypeSecret)>(&self, fnc: F) -> &Self { if let FileType::Secret(t) = self { fnc(t) }; self }
  pub fn on_secret_thumbnail<F: FnOnce(&FileTypeSecretThumbnail)>(&self, fnc: F) -> &Self { if let FileType::SecretThumbnail(t) = self { fnc(t) }; self }
  pub fn on_secure<F: FnOnce(&FileTypeSecure)>(&self, fnc: F) -> &Self { if let FileType::Secure(t) = self { fnc(t) }; self }
  pub fn on_sticker<F: FnOnce(&FileTypeSticker)>(&self, fnc: F) -> &Self { if let FileType::Sticker(t) = self { fnc(t) }; self }
  pub fn on_thumbnail<F: FnOnce(&FileTypeThumbnail)>(&self, fnc: F) -> &Self { if let FileType::Thumbnail(t) = self { fnc(t) }; self }
  pub fn on_unknown<F: FnOnce(&FileTypeUnknown)>(&self, fnc: F) -> &Self { if let FileType::Unknown(t) = self { fnc(t) }; self }
  pub fn on_video<F: FnOnce(&FileTypeVideo)>(&self, fnc: F) -> &Self { if let FileType::Video(t) = self { fnc(t) }; self }
  pub fn on_video_note<F: FnOnce(&FileTypeVideoNote)>(&self, fnc: F) -> &Self { if let FileType::VideoNote(t) = self { fnc(t) }; self }
  pub fn on_voice_note<F: FnOnce(&FileTypeVoiceNote)>(&self, fnc: F) -> &Self { if let FileType::VoiceNote(t) = self { fnc(t) }; self }
  pub fn on_wallpaper<F: FnOnce(&FileTypeWallpaper)>(&self, fnc: F) -> &Self { if let FileType::Wallpaper(t) = self { fnc(t) }; self }

  pub fn as_none(&self) -> Option<&FileTypeNone> { if let FileType::None(t) = self { return Some(t) } None }
  pub fn as_animation(&self) -> Option<&FileTypeAnimation> { if let FileType::Animation(t) = self { return Some(t) } None }
  pub fn as_audio(&self) -> Option<&FileTypeAudio> { if let FileType::Audio(t) = self { return Some(t) } None }
  pub fn as_document(&self) -> Option<&FileTypeDocument> { if let FileType::Document(t) = self { return Some(t) } None }
  pub fn as_photo(&self) -> Option<&FileTypePhoto> { if let FileType::Photo(t) = self { return Some(t) } None }
  pub fn as_profile_photo(&self) -> Option<&FileTypeProfilePhoto> { if let FileType::ProfilePhoto(t) = self { return Some(t) } None }
  pub fn as_secret(&self) -> Option<&FileTypeSecret> { if let FileType::Secret(t) = self { return Some(t) } None }
  pub fn as_secret_thumbnail(&self) -> Option<&FileTypeSecretThumbnail> { if let FileType::SecretThumbnail(t) = self { return Some(t) } None }
  pub fn as_secure(&self) -> Option<&FileTypeSecure> { if let FileType::Secure(t) = self { return Some(t) } None }
  pub fn as_sticker(&self) -> Option<&FileTypeSticker> { if let FileType::Sticker(t) = self { return Some(t) } None }
  pub fn as_thumbnail(&self) -> Option<&FileTypeThumbnail> { if let FileType::Thumbnail(t) = self { return Some(t) } None }
  pub fn as_unknown(&self) -> Option<&FileTypeUnknown> { if let FileType::Unknown(t) = self { return Some(t) } None }
  pub fn as_video(&self) -> Option<&FileTypeVideo> { if let FileType::Video(t) = self { return Some(t) } None }
  pub fn as_video_note(&self) -> Option<&FileTypeVideoNote> { if let FileType::VideoNote(t) = self { return Some(t) } None }
  pub fn as_voice_note(&self) -> Option<&FileTypeVoiceNote> { if let FileType::VoiceNote(t) = self { return Some(t) } None }
  pub fn as_wallpaper(&self) -> Option<&FileTypeWallpaper> { if let FileType::Wallpaper(t) = self { return Some(t) } None }



  pub fn none<T: AsRef<FileTypeNone>>(t: T) -> Self { FileType::None(t.as_ref().clone()) }

  pub fn animation<T: AsRef<FileTypeAnimation>>(t: T) -> Self { FileType::Animation(t.as_ref().clone()) }

  pub fn audio<T: AsRef<FileTypeAudio>>(t: T) -> Self { FileType::Audio(t.as_ref().clone()) }

  pub fn document<T: AsRef<FileTypeDocument>>(t: T) -> Self { FileType::Document(t.as_ref().clone()) }

  pub fn photo<T: AsRef<FileTypePhoto>>(t: T) -> Self { FileType::Photo(t.as_ref().clone()) }

  pub fn profile_photo<T: AsRef<FileTypeProfilePhoto>>(t: T) -> Self { FileType::ProfilePhoto(t.as_ref().clone()) }

  pub fn secret<T: AsRef<FileTypeSecret>>(t: T) -> Self { FileType::Secret(t.as_ref().clone()) }

  pub fn secret_thumbnail<T: AsRef<FileTypeSecretThumbnail>>(t: T) -> Self { FileType::SecretThumbnail(t.as_ref().clone()) }

  pub fn secure<T: AsRef<FileTypeSecure>>(t: T) -> Self { FileType::Secure(t.as_ref().clone()) }

  pub fn sticker<T: AsRef<FileTypeSticker>>(t: T) -> Self { FileType::Sticker(t.as_ref().clone()) }

  pub fn thumbnail<T: AsRef<FileTypeThumbnail>>(t: T) -> Self { FileType::Thumbnail(t.as_ref().clone()) }

  pub fn unknown<T: AsRef<FileTypeUnknown>>(t: T) -> Self { FileType::Unknown(t.as_ref().clone()) }

  pub fn video<T: AsRef<FileTypeVideo>>(t: T) -> Self { FileType::Video(t.as_ref().clone()) }

  pub fn video_note<T: AsRef<FileTypeVideoNote>>(t: T) -> Self { FileType::VideoNote(t.as_ref().clone()) }

  pub fn voice_note<T: AsRef<FileTypeVoiceNote>>(t: T) -> Self { FileType::VoiceNote(t.as_ref().clone()) }

  pub fn wallpaper<T: AsRef<FileTypeWallpaper>>(t: T) -> Self { FileType::Wallpaper(t.as_ref().clone()) }

}

impl AsRef<FileType> for FileType {
  fn as_ref(&self) -> &FileType { self }
}







/// The data is not a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeNone {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeNone" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeNone {}



impl FileTypeNone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeNoneBuilder {
    let mut inner = FileTypeNone::default();
    inner.td_name = "fileTypeNone".to_string();
    RTDFileTypeNoneBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeNoneBuilder {
  inner: FileTypeNone
}

impl RTDFileTypeNoneBuilder {
  pub fn build(&self) -> FileTypeNone { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeNone> for FileTypeNone {
  fn as_ref(&self) -> &FileTypeNone { self }
}

impl AsRef<FileTypeNone> for RTDFileTypeNoneBuilder {
  fn as_ref(&self) -> &FileTypeNone { &self.inner }
}







/// The file is an animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeAnimation" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeAnimation {}



impl FileTypeAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeAnimationBuilder {
    let mut inner = FileTypeAnimation::default();
    inner.td_name = "fileTypeAnimation".to_string();
    RTDFileTypeAnimationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeAnimationBuilder {
  inner: FileTypeAnimation
}

impl RTDFileTypeAnimationBuilder {
  pub fn build(&self) -> FileTypeAnimation { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeAnimation> for FileTypeAnimation {
  fn as_ref(&self) -> &FileTypeAnimation { self }
}

impl AsRef<FileTypeAnimation> for RTDFileTypeAnimationBuilder {
  fn as_ref(&self) -> &FileTypeAnimation { &self.inner }
}







/// The file is an audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeAudio" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeAudio {}



impl FileTypeAudio {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeAudioBuilder {
    let mut inner = FileTypeAudio::default();
    inner.td_name = "fileTypeAudio".to_string();
    RTDFileTypeAudioBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeAudioBuilder {
  inner: FileTypeAudio
}

impl RTDFileTypeAudioBuilder {
  pub fn build(&self) -> FileTypeAudio { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeAudio> for FileTypeAudio {
  fn as_ref(&self) -> &FileTypeAudio { self }
}

impl AsRef<FileTypeAudio> for RTDFileTypeAudioBuilder {
  fn as_ref(&self) -> &FileTypeAudio { &self.inner }
}







/// The file is a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeDocument" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeDocument {}



impl FileTypeDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeDocumentBuilder {
    let mut inner = FileTypeDocument::default();
    inner.td_name = "fileTypeDocument".to_string();
    RTDFileTypeDocumentBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeDocumentBuilder {
  inner: FileTypeDocument
}

impl RTDFileTypeDocumentBuilder {
  pub fn build(&self) -> FileTypeDocument { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeDocument> for FileTypeDocument {
  fn as_ref(&self) -> &FileTypeDocument { self }
}

impl AsRef<FileTypeDocument> for RTDFileTypeDocumentBuilder {
  fn as_ref(&self) -> &FileTypeDocument { &self.inner }
}







/// The file is a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypePhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypePhoto {}



impl FileTypePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypePhotoBuilder {
    let mut inner = FileTypePhoto::default();
    inner.td_name = "fileTypePhoto".to_string();
    RTDFileTypePhotoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypePhotoBuilder {
  inner: FileTypePhoto
}

impl RTDFileTypePhotoBuilder {
  pub fn build(&self) -> FileTypePhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypePhoto> for FileTypePhoto {
  fn as_ref(&self) -> &FileTypePhoto { self }
}

impl AsRef<FileTypePhoto> for RTDFileTypePhotoBuilder {
  fn as_ref(&self) -> &FileTypePhoto { &self.inner }
}







/// The file is a profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeProfilePhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeProfilePhoto {}



impl FileTypeProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeProfilePhotoBuilder {
    let mut inner = FileTypeProfilePhoto::default();
    inner.td_name = "fileTypeProfilePhoto".to_string();
    RTDFileTypeProfilePhotoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeProfilePhotoBuilder {
  inner: FileTypeProfilePhoto
}

impl RTDFileTypeProfilePhotoBuilder {
  pub fn build(&self) -> FileTypeProfilePhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeProfilePhoto> for FileTypeProfilePhoto {
  fn as_ref(&self) -> &FileTypeProfilePhoto { self }
}

impl AsRef<FileTypeProfilePhoto> for RTDFileTypeProfilePhotoBuilder {
  fn as_ref(&self) -> &FileTypeProfilePhoto { &self.inner }
}







/// The file was sent to a secret chat (the file type is not known to the server)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecret {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeSecret {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSecret" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeSecret {}



impl FileTypeSecret {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeSecretBuilder {
    let mut inner = FileTypeSecret::default();
    inner.td_name = "fileTypeSecret".to_string();
    RTDFileTypeSecretBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeSecretBuilder {
  inner: FileTypeSecret
}

impl RTDFileTypeSecretBuilder {
  pub fn build(&self) -> FileTypeSecret { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeSecret> for FileTypeSecret {
  fn as_ref(&self) -> &FileTypeSecret { self }
}

impl AsRef<FileTypeSecret> for RTDFileTypeSecretBuilder {
  fn as_ref(&self) -> &FileTypeSecret { &self.inner }
}







/// The file is a thumbnail of a file from a secret chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecretThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeSecretThumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSecretThumbnail" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeSecretThumbnail {}



impl FileTypeSecretThumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeSecretThumbnailBuilder {
    let mut inner = FileTypeSecretThumbnail::default();
    inner.td_name = "fileTypeSecretThumbnail".to_string();
    RTDFileTypeSecretThumbnailBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeSecretThumbnailBuilder {
  inner: FileTypeSecretThumbnail
}

impl RTDFileTypeSecretThumbnailBuilder {
  pub fn build(&self) -> FileTypeSecretThumbnail { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeSecretThumbnail> for FileTypeSecretThumbnail {
  fn as_ref(&self) -> &FileTypeSecretThumbnail { self }
}

impl AsRef<FileTypeSecretThumbnail> for RTDFileTypeSecretThumbnailBuilder {
  fn as_ref(&self) -> &FileTypeSecretThumbnail { &self.inner }
}







/// The file is a file from Secure storage used for storing Telegram Passport files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecure {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeSecure {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSecure" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeSecure {}



impl FileTypeSecure {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeSecureBuilder {
    let mut inner = FileTypeSecure::default();
    inner.td_name = "fileTypeSecure".to_string();
    RTDFileTypeSecureBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeSecureBuilder {
  inner: FileTypeSecure
}

impl RTDFileTypeSecureBuilder {
  pub fn build(&self) -> FileTypeSecure { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeSecure> for FileTypeSecure {
  fn as_ref(&self) -> &FileTypeSecure { self }
}

impl AsRef<FileTypeSecure> for RTDFileTypeSecureBuilder {
  fn as_ref(&self) -> &FileTypeSecure { &self.inner }
}







/// The file is a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeSticker" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeSticker {}



impl FileTypeSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeStickerBuilder {
    let mut inner = FileTypeSticker::default();
    inner.td_name = "fileTypeSticker".to_string();
    RTDFileTypeStickerBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeStickerBuilder {
  inner: FileTypeSticker
}

impl RTDFileTypeStickerBuilder {
  pub fn build(&self) -> FileTypeSticker { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeSticker> for FileTypeSticker {
  fn as_ref(&self) -> &FileTypeSticker { self }
}

impl AsRef<FileTypeSticker> for RTDFileTypeStickerBuilder {
  fn as_ref(&self) -> &FileTypeSticker { &self.inner }
}







/// The file is a thumbnail of another file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeThumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeThumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeThumbnail" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeThumbnail {}



impl FileTypeThumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeThumbnailBuilder {
    let mut inner = FileTypeThumbnail::default();
    inner.td_name = "fileTypeThumbnail".to_string();
    RTDFileTypeThumbnailBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeThumbnailBuilder {
  inner: FileTypeThumbnail
}

impl RTDFileTypeThumbnailBuilder {
  pub fn build(&self) -> FileTypeThumbnail { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeThumbnail> for FileTypeThumbnail {
  fn as_ref(&self) -> &FileTypeThumbnail { self }
}

impl AsRef<FileTypeThumbnail> for RTDFileTypeThumbnailBuilder {
  fn as_ref(&self) -> &FileTypeThumbnail { &self.inner }
}







/// The file type is not yet known
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeUnknown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeUnknown {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeUnknown" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeUnknown {}



impl FileTypeUnknown {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeUnknownBuilder {
    let mut inner = FileTypeUnknown::default();
    inner.td_name = "fileTypeUnknown".to_string();
    RTDFileTypeUnknownBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeUnknownBuilder {
  inner: FileTypeUnknown
}

impl RTDFileTypeUnknownBuilder {
  pub fn build(&self) -> FileTypeUnknown { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeUnknown> for FileTypeUnknown {
  fn as_ref(&self) -> &FileTypeUnknown { self }
}

impl AsRef<FileTypeUnknown> for RTDFileTypeUnknownBuilder {
  fn as_ref(&self) -> &FileTypeUnknown { &self.inner }
}







/// The file is a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeVideo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeVideo {}



impl FileTypeVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeVideoBuilder {
    let mut inner = FileTypeVideo::default();
    inner.td_name = "fileTypeVideo".to_string();
    RTDFileTypeVideoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeVideoBuilder {
  inner: FileTypeVideo
}

impl RTDFileTypeVideoBuilder {
  pub fn build(&self) -> FileTypeVideo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeVideo> for FileTypeVideo {
  fn as_ref(&self) -> &FileTypeVideo { self }
}

impl AsRef<FileTypeVideo> for RTDFileTypeVideoBuilder {
  fn as_ref(&self) -> &FileTypeVideo { &self.inner }
}







/// The file is a video note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeVideoNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeVideoNote {}



impl FileTypeVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeVideoNoteBuilder {
    let mut inner = FileTypeVideoNote::default();
    inner.td_name = "fileTypeVideoNote".to_string();
    RTDFileTypeVideoNoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeVideoNoteBuilder {
  inner: FileTypeVideoNote
}

impl RTDFileTypeVideoNoteBuilder {
  pub fn build(&self) -> FileTypeVideoNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeVideoNote> for FileTypeVideoNote {
  fn as_ref(&self) -> &FileTypeVideoNote { self }
}

impl AsRef<FileTypeVideoNote> for RTDFileTypeVideoNoteBuilder {
  fn as_ref(&self) -> &FileTypeVideoNote { &self.inner }
}







/// The file is a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeVoiceNote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeVoiceNote {}



impl FileTypeVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeVoiceNoteBuilder {
    let mut inner = FileTypeVoiceNote::default();
    inner.td_name = "fileTypeVoiceNote".to_string();
    RTDFileTypeVoiceNoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeVoiceNoteBuilder {
  inner: FileTypeVoiceNote
}

impl RTDFileTypeVoiceNoteBuilder {
  pub fn build(&self) -> FileTypeVoiceNote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeVoiceNote> for FileTypeVoiceNote {
  fn as_ref(&self) -> &FileTypeVoiceNote { self }
}

impl AsRef<FileTypeVoiceNote> for RTDFileTypeVoiceNoteBuilder {
  fn as_ref(&self) -> &FileTypeVoiceNote { &self.inner }
}







/// The file is a wallpaper or a background pattern
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeWallpaper {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for FileTypeWallpaper {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "fileTypeWallpaper" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDFileType for FileTypeWallpaper {}



impl FileTypeWallpaper {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFileTypeWallpaperBuilder {
    let mut inner = FileTypeWallpaper::default();
    inner.td_name = "fileTypeWallpaper".to_string();
    RTDFileTypeWallpaperBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDFileTypeWallpaperBuilder {
  inner: FileTypeWallpaper
}

impl RTDFileTypeWallpaperBuilder {
  pub fn build(&self) -> FileTypeWallpaper { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<FileTypeWallpaper> for FileTypeWallpaper {
  fn as_ref(&self) -> &FileTypeWallpaper { self }
}

impl AsRef<FileTypeWallpaper> for RTDFileTypeWallpaperBuilder {
  fn as_ref(&self) -> &FileTypeWallpaper { &self.inner }
}



