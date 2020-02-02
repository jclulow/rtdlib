
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a TDLib internal log verbosity level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogVerbosityLevel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Log verbosity level
  verbosity_level: i64,
  
}

impl RObject for LogVerbosityLevel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logVerbosityLevel" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LogVerbosityLevel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLogVerbosityLevelBuilder {
    let mut inner = LogVerbosityLevel::default();
    inner.td_name = "logVerbosityLevel".to_string();
    RTDLogVerbosityLevelBuilder { inner }
  }

  pub fn verbosity_level(&self) -> i64 { self.verbosity_level }

}

#[doc(hidden)]
pub struct RTDLogVerbosityLevelBuilder {
  inner: LogVerbosityLevel
}

impl RTDLogVerbosityLevelBuilder {
  pub fn build(&self) -> LogVerbosityLevel { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn verbosity_level(&mut self, verbosity_level: i64) -> &mut Self {
    self.inner.verbosity_level = verbosity_level;
    self
  }

}

impl AsRef<LogVerbosityLevel> for LogVerbosityLevel {
  fn as_ref(&self) -> &LogVerbosityLevel { self }
}

impl AsRef<LogVerbosityLevel> for RTDLogVerbosityLevelBuilder {
  fn as_ref(&self) -> &LogVerbosityLevel { &self.inner }
}



