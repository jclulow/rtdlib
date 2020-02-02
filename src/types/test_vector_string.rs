
use crate::types::*;
use crate::errors::*;




/// A simple object containing a vector of strings; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Vector of strings
  value: Vec<String>,
  
}

impl RObject for TestVectorString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorString" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestVectorString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestVectorStringBuilder {
    let mut inner = TestVectorString::default();
    inner.td_name = "testVectorString".to_string();
    RTDTestVectorStringBuilder { inner }
  }

  pub fn value(&self) -> &Vec<String> { &self.value }

}

#[doc(hidden)]
pub struct RTDTestVectorStringBuilder {
  inner: TestVectorString
}

impl RTDTestVectorStringBuilder {
  pub fn build(&self) -> TestVectorString { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn value(&mut self, value: Vec<String>) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<TestVectorString> for TestVectorString {
  fn as_ref(&self) -> &TestVectorString { self }
}

impl AsRef<TestVectorString> for RTDTestVectorStringBuilder {
  fn as_ref(&self) -> &TestVectorString { &self.inner }
}



