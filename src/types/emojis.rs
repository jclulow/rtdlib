
use crate::types::*;
use crate::errors::*;




/// Represents a list of emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Emojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// List of emojis
  emojis: Vec<String>,
  
}

impl RObject for Emojis {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "emojis" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Emojis {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDEmojisBuilder {
    let mut inner = Emojis::default();
    inner.td_name = "emojis".to_string();
    RTDEmojisBuilder { inner }
  }

  pub fn emojis(&self) -> &Vec<String> { &self.emojis }

}

#[doc(hidden)]
pub struct RTDEmojisBuilder {
  inner: Emojis
}

impl RTDEmojisBuilder {
  pub fn build(&self) -> Emojis { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }

}

impl AsRef<Emojis> for Emojis {
  fn as_ref(&self) -> &Emojis { self }
}

impl AsRef<Emojis> for RTDEmojisBuilder {
  fn as_ref(&self) -> &Emojis { &self.inner }
}



