
use crate::types::*;
use crate::errors::*;




/// A simple object containing a vector of objects that hold a number; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorIntObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Vector of objects
  value: Vec<TestInt>,
  
}

impl RObject for TestVectorIntObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorIntObject" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestVectorIntObject {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestVectorIntObjectBuilder {
    let mut inner = TestVectorIntObject::default();
    inner.td_name = "testVectorIntObject".to_string();
    RTDTestVectorIntObjectBuilder { inner }
  }

  pub fn value(&self) -> &Vec<TestInt> { &self.value }

}

#[doc(hidden)]
pub struct RTDTestVectorIntObjectBuilder {
  inner: TestVectorIntObject
}

impl RTDTestVectorIntObjectBuilder {
  pub fn build(&self) -> TestVectorIntObject { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn value(&mut self, value: Vec<TestInt>) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<TestVectorIntObject> for TestVectorIntObject {
  fn as_ref(&self) -> &TestVectorIntObject { self }
}

impl AsRef<TestVectorIntObject> for RTDTestVectorIntObjectBuilder {
  fn as_ref(&self) -> &TestVectorIntObject { &self.inner }
}



