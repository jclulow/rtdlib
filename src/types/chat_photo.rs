
use crate::types::*;
use crate::errors::*;




/// Describes the photo of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// A small (160x160) chat photo. The file can be downloaded only before the photo is changed
  small: File,
  /// A big (640x640) chat photo. The file can be downloaded only before the photo is changed
  big: File,
  
}

impl RObject for ChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatPhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatPhotoBuilder {
    let mut inner = ChatPhoto::default();
    inner.td_name = "chatPhoto".to_string();
    RTDChatPhotoBuilder { inner }
  }

  pub fn small(&self) -> &File { &self.small }

  pub fn big(&self) -> &File { &self.big }

}

#[doc(hidden)]
pub struct RTDChatPhotoBuilder {
  inner: ChatPhoto
}

impl RTDChatPhotoBuilder {
  pub fn build(&self) -> ChatPhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn small<T: AsRef<File>>(&mut self, small: T) -> &mut Self {
    self.inner.small = small.as_ref().clone();
    self
  }

   
  pub fn big<T: AsRef<File>>(&mut self, big: T) -> &mut Self {
    self.inner.big = big.as_ref().clone();
    self
  }

}

impl AsRef<ChatPhoto> for ChatPhoto {
  fn as_ref(&self) -> &ChatPhoto { self }
}

impl AsRef<ChatPhoto> for RTDChatPhotoBuilder {
  fn as_ref(&self) -> &ChatPhoto { &self.inner }
}



