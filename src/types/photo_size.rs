
use crate::types::*;
use crate::errors::*;




/// Photo description
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhotoSize {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Thumbnail type (see https://core.telegram.org/constructor/photoSize)
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: String,
  /// Information about the photo file
  photo: File,
  /// Photo width
  width: i64,
  /// Photo height
  height: i64,
  
}

impl RObject for PhotoSize {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "photoSize" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PhotoSize {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPhotoSizeBuilder {
    let mut inner = PhotoSize::default();
    inner.td_name = "photoSize".to_string();
    RTDPhotoSizeBuilder { inner }
  }

  pub fn type_(&self) -> &String { &self.type_ }

  pub fn photo(&self) -> &File { &self.photo }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

}

#[doc(hidden)]
pub struct RTDPhotoSizeBuilder {
  inner: PhotoSize
}

impl RTDPhotoSizeBuilder {
  pub fn build(&self) -> PhotoSize { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().to_string();
    self
  }

   
  pub fn photo<T: AsRef<File>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
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

impl AsRef<PhotoSize> for PhotoSize {
  fn as_ref(&self) -> &PhotoSize { self }
}

impl AsRef<PhotoSize> for RTDPhotoSizeBuilder {
  fn as_ref(&self) -> &PhotoSize { &self.inner }
}



