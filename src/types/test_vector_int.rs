
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// A simple object containing a vector of numbers; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Vector of numbers
  value: Vec<i64>,
  
}

impl RObject for TestVectorInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorInt" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestVectorInt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestVectorIntBuilder {
    let mut inner = TestVectorInt::default();
    inner.td_name = "testVectorInt".to_string();
    RTDTestVectorIntBuilder { inner }
  }

  pub fn value(&self) -> &Vec<i64> { &self.value }

}

#[doc(hidden)]
pub struct RTDTestVectorIntBuilder {
  inner: TestVectorInt
}

impl RTDTestVectorIntBuilder {
  pub fn build(&self) -> TestVectorInt { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn value(&mut self, value: Vec<i64>) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<TestVectorInt> for TestVectorInt {
  fn as_ref(&self) -> &TestVectorInt { self }
}

impl AsRef<TestVectorInt> for RTDTestVectorIntBuilder {
  fn as_ref(&self) -> &TestVectorInt { &self.inner }
}



