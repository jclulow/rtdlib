
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents a list of sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Approximate total number of sticker sets found
  total_count: i64,
  /// List of sticker sets
  sets: Vec<StickerSetInfo>,
  
}

impl RObject for StickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerSets" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStickerSetsBuilder {
    let mut inner = StickerSets::default();
    inner.td_name = "stickerSets".to_string();
    RTDStickerSetsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn sets(&self) -> &Vec<StickerSetInfo> { &self.sets }

}

#[doc(hidden)]
pub struct RTDStickerSetsBuilder {
  inner: StickerSets
}

impl RTDStickerSetsBuilder {
  pub fn build(&self) -> StickerSets { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn sets(&mut self, sets: Vec<StickerSetInfo>) -> &mut Self {
    self.inner.sets = sets;
    self
  }

}

impl AsRef<StickerSets> for StickerSets {
  fn as_ref(&self) -> &StickerSets { self }
}

impl AsRef<StickerSets> for RTDStickerSetsBuilder {
  fn as_ref(&self) -> &StickerSets { &self.inner }
}



