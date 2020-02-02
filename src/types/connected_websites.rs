
use crate::types::*;
use crate::errors::*;




/// Contains a list of websites the current user is logged in with Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectedWebsites {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// List of connected websites
  websites: Vec<ConnectedWebsite>,
  
}

impl RObject for ConnectedWebsites {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectedWebsites" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ConnectedWebsites {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectedWebsitesBuilder {
    let mut inner = ConnectedWebsites::default();
    inner.td_name = "connectedWebsites".to_string();
    RTDConnectedWebsitesBuilder { inner }
  }

  pub fn websites(&self) -> &Vec<ConnectedWebsite> { &self.websites }

}

#[doc(hidden)]
pub struct RTDConnectedWebsitesBuilder {
  inner: ConnectedWebsites
}

impl RTDConnectedWebsitesBuilder {
  pub fn build(&self) -> ConnectedWebsites { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn websites(&mut self, websites: Vec<ConnectedWebsite>) -> &mut Self {
    self.inner.websites = websites;
    self
  }

}

impl AsRef<ConnectedWebsites> for ConnectedWebsites {
  fn as_ref(&self) -> &ConnectedWebsites { self }
}

impl AsRef<ConnectedWebsites> for RTDConnectedWebsitesBuilder {
  fn as_ref(&self) -> &ConnectedWebsites { &self.inner }
}



