
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains part of the list of user photos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfilePhotos {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Total number of user profile photos
  total_count: i64,
  /// A list of photos
  photos: Vec<UserProfilePhoto>,
  
}

impl RObject for UserProfilePhotos {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userProfilePhotos" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl UserProfilePhotos {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserProfilePhotosBuilder {
    let mut inner = UserProfilePhotos::default();
    inner.td_name = "userProfilePhotos".to_string();
    RTDUserProfilePhotosBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn photos(&self) -> &Vec<UserProfilePhoto> { &self.photos }

}

#[doc(hidden)]
pub struct RTDUserProfilePhotosBuilder {
  inner: UserProfilePhotos
}

impl RTDUserProfilePhotosBuilder {
  pub fn build(&self) -> UserProfilePhotos { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn photos(&mut self, photos: Vec<UserProfilePhoto>) -> &mut Self {
    self.inner.photos = photos;
    self
  }

}

impl AsRef<UserProfilePhotos> for UserProfilePhotos {
  fn as_ref(&self) -> &UserProfilePhotos { self }
}

impl AsRef<UserProfilePhotos> for RTDUserProfilePhotosBuilder {
  fn as_ref(&self) -> &UserProfilePhotos { &self.inner }
}



