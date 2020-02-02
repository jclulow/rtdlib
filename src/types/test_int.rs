
use crate::types::*;
use crate::errors::*;




/// A simple object containing a number; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Number
  value: i64,
  
}

impl RObject for TestInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testInt" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestInt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestIntBuilder {
    let mut inner = TestInt::default();
    inner.td_name = "testInt".to_string();
    RTDTestIntBuilder { inner }
  }

  pub fn value(&self) -> i64 { self.value }

}

#[doc(hidden)]
pub struct RTDTestIntBuilder {
  inner: TestInt
}

impl RTDTestIntBuilder {
  pub fn build(&self) -> TestInt { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn value(&mut self, value: i64) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<TestInt> for TestInt {
  fn as_ref(&self) -> &TestInt { self }
}

impl AsRef<TestInt> for RTDTestIntBuilder {
  fn as_ref(&self) -> &TestInt { &self.inner }
}



