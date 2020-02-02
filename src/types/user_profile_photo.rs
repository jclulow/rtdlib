
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains full information about a user profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Unique user profile photo identifier
  id: isize,
  /// Point in time (Unix timestamp) when the photo has been added
  added_date: i64,
  /// Available variants of the user photo, in different sizes
  sizes: Vec<PhotoSize>,
  
}

impl RObject for UserProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userProfilePhoto" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl UserProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserProfilePhotoBuilder {
    let mut inner = UserProfilePhoto::default();
    inner.td_name = "userProfilePhoto".to_string();
    RTDUserProfilePhotoBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn added_date(&self) -> i64 { self.added_date }

  pub fn sizes(&self) -> &Vec<PhotoSize> { &self.sizes }

}

#[doc(hidden)]
pub struct RTDUserProfilePhotoBuilder {
  inner: UserProfilePhoto
}

impl RTDUserProfilePhotoBuilder {
  pub fn build(&self) -> UserProfilePhoto { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn added_date(&mut self, added_date: i64) -> &mut Self {
    self.inner.added_date = added_date;
    self
  }

   
  pub fn sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self {
    self.inner.sizes = sizes;
    self
  }

}

impl AsRef<UserProfilePhoto> for UserProfilePhoto {
  fn as_ref(&self) -> &UserProfilePhoto { self }
}

impl AsRef<UserProfilePhoto> for RTDUserProfilePhotoBuilder {
  fn as_ref(&self) -> &UserProfilePhoto { &self.inner }
}



