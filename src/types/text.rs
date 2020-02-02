
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains some text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Text
  text: String,
  
}

impl RObject for Text {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "text" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Text {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextBuilder {
    let mut inner = Text::default();
    inner.td_name = "text".to_string();
    RTDTextBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

}

#[doc(hidden)]
pub struct RTDTextBuilder {
  inner: Text
}

impl RTDTextBuilder {
  pub fn build(&self) -> Text { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

}

impl AsRef<Text> for Text {
  fn as_ref(&self) -> &Text { self }
}

impl AsRef<Text> for RTDTextBuilder {
  fn as_ref(&self) -> &Text { &self.inner }
}



