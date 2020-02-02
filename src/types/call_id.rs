
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains the call identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Call identifier
  id: i64,
  
}

impl RObject for CallId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callId" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl CallId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallIdBuilder {
    let mut inner = CallId::default();
    inner.td_name = "callId".to_string();
    RTDCallIdBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

}

#[doc(hidden)]
pub struct RTDCallIdBuilder {
  inner: CallId
}

impl RTDCallIdBuilder {
  pub fn build(&self) -> CallId { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

}

impl AsRef<CallId> for CallId {
  fn as_ref(&self) -> &CallId { self }
}

impl AsRef<CallId> for RTDCallIdBuilder {
  fn as_ref(&self) -> &CallId { &self.inner }
}



