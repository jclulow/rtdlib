
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// A simple object containing a string; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// String
  value: String,
  
}

impl RObject for TestString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testString" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestStringBuilder {
    let mut inner = TestString::default();
    inner.td_name = "testString".to_string();
    RTDTestStringBuilder { inner }
  }

  pub fn value(&self) -> &String { &self.value }

}

#[doc(hidden)]
pub struct RTDTestStringBuilder {
  inner: TestString
}

impl RTDTestStringBuilder {
  pub fn build(&self) -> TestString { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }

}

impl AsRef<TestString> for TestString {
  fn as_ref(&self) -> &TestString { self }
}

impl AsRef<TestString> for RTDTestStringBuilder {
  fn as_ref(&self) -> &TestString { &self.inner }
}



