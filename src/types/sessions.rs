
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a list of sessions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sessions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of sessions
  sessions: Vec<Session>,
  
}

impl RObject for Sessions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sessions" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Sessions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDSessionsBuilder {
    let mut inner = Sessions::default();
    inner.td_name = "sessions".to_string();
    RTDSessionsBuilder { inner }
  }

  pub fn sessions(&self) -> &Vec<Session> { &self.sessions }

}

#[doc(hidden)]
pub struct RTDSessionsBuilder {
  inner: Sessions
}

impl RTDSessionsBuilder {
  pub fn build(&self) -> Sessions { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn sessions(&mut self, sessions: Vec<Session>) -> &mut Self {
    self.inner.sessions = sessions;
    self
  }

}

impl AsRef<Sessions> for Sessions {
  fn as_ref(&self) -> &Sessions { self }
}

impl AsRef<Sessions> for RTDSessionsBuilder {
  fn as_ref(&self) -> &Sessions { &self.inner }
}



