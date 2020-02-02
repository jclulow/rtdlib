
use crate::types::*;
use crate::errors::*;




/// Contains a counter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Count {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Count
  count: i64,
  
}

impl RObject for Count {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "count" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Count {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCountBuilder {
    let mut inner = Count::default();
    inner.td_name = "count".to_string();
    RTDCountBuilder { inner }
  }

  pub fn count(&self) -> i64 { self.count }

}

#[doc(hidden)]
pub struct RTDCountBuilder {
  inner: Count
}

impl RTDCountBuilder {
  pub fn build(&self) -> Count { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn count(&mut self, count: i64) -> &mut Self {
    self.inner.count = count;
    self
  }

}

impl AsRef<Count> for Count {
  fn as_ref(&self) -> &Count { self }
}

impl AsRef<Count> for RTDCountBuilder {
  fn as_ref(&self) -> &Count { &self.inner }
}



