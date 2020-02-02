
use crate::types::*;
use crate::errors::*;




/// Contains a list of hashtags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Hashtags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// A list of hashtags
  hashtags: Vec<String>,
  
}

impl RObject for Hashtags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "hashtags" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Hashtags {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDHashtagsBuilder {
    let mut inner = Hashtags::default();
    inner.td_name = "hashtags".to_string();
    RTDHashtagsBuilder { inner }
  }

  pub fn hashtags(&self) -> &Vec<String> { &self.hashtags }

}

#[doc(hidden)]
pub struct RTDHashtagsBuilder {
  inner: Hashtags
}

impl RTDHashtagsBuilder {
  pub fn build(&self) -> Hashtags { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn hashtags(&mut self, hashtags: Vec<String>) -> &mut Self {
    self.inner.hashtags = hashtags;
    self
  }

}

impl AsRef<Hashtags> for Hashtags {
  fn as_ref(&self) -> &Hashtags { self }
}

impl AsRef<Hashtags> for RTDHashtagsBuilder {
  fn as_ref(&self) -> &Hashtags { &self.inner }
}



