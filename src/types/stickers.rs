
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents a list of stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of stickers
  stickers: Vec<Sticker>,
  
}

impl RObject for Stickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickers" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Stickers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStickersBuilder {
    let mut inner = Stickers::default();
    inner.td_name = "stickers".to_string();
    RTDStickersBuilder { inner }
  }

  pub fn stickers(&self) -> &Vec<Sticker> { &self.stickers }

}

#[doc(hidden)]
pub struct RTDStickersBuilder {
  inner: Stickers
}

impl RTDStickersBuilder {
  pub fn build(&self) -> Stickers { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self {
    self.inner.stickers = stickers;
    self
  }

}

impl AsRef<Stickers> for Stickers {
  fn as_ref(&self) -> &Stickers { self }
}

impl AsRef<Stickers> for RTDStickersBuilder {
  fn as_ref(&self) -> &Stickers { &self.inner }
}



