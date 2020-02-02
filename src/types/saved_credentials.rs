
use crate::types::*;
use crate::errors::*;




/// Contains information about saved card credentials
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedCredentials {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Unique identifier of the saved credentials
  id: String,
  /// Title of the saved credentials
  title: String,
  
}

impl RObject for SavedCredentials {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "savedCredentials" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl SavedCredentials {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSavedCredentialsBuilder {
    let mut inner = SavedCredentials::default();
    inner.td_name = "savedCredentials".to_string();
    RTDSavedCredentialsBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn title(&self) -> &String { &self.title }

}

#[doc(hidden)]
pub struct RTDSavedCredentialsBuilder {
  inner: SavedCredentials
}

impl RTDSavedCredentialsBuilder {
  pub fn build(&self) -> SavedCredentials { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

}

impl AsRef<SavedCredentials> for SavedCredentials {
  fn as_ref(&self) -> &SavedCredentials { self }
}

impl AsRef<SavedCredentials> for RTDSavedCredentialsBuilder {
  fn as_ref(&self) -> &SavedCredentials { &self.inner }
}



