
use crate::types::*;
use crate::errors::*;




/// Contains a list of backgrounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Backgrounds {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// A list of backgrounds
  backgrounds: Vec<Background>,
  
}

impl RObject for Backgrounds {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgrounds" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Backgrounds {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundsBuilder {
    let mut inner = Backgrounds::default();
    inner.td_name = "backgrounds".to_string();
    RTDBackgroundsBuilder { inner }
  }

  pub fn backgrounds(&self) -> &Vec<Background> { &self.backgrounds }

}

#[doc(hidden)]
pub struct RTDBackgroundsBuilder {
  inner: Backgrounds
}

impl RTDBackgroundsBuilder {
  pub fn build(&self) -> Backgrounds { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn backgrounds(&mut self, backgrounds: Vec<Background>) -> &mut Self {
    self.inner.backgrounds = backgrounds;
    self
  }

}

impl AsRef<Backgrounds> for Backgrounds {
  fn as_ref(&self) -> &Backgrounds { self }
}

impl AsRef<Backgrounds> for RTDBackgroundsBuilder {
  fn as_ref(&self) -> &Backgrounds { &self.inner }
}



