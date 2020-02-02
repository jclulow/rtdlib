
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a list of text entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntities {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of text entities
  entities: Vec<TextEntity>,
  
}

impl RObject for TextEntities {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntities" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TextEntities {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntitiesBuilder {
    let mut inner = TextEntities::default();
    inner.td_name = "textEntities".to_string();
    RTDTextEntitiesBuilder { inner }
  }

  pub fn entities(&self) -> &Vec<TextEntity> { &self.entities }

}

#[doc(hidden)]
pub struct RTDTextEntitiesBuilder {
  inner: TextEntities
}

impl RTDTextEntitiesBuilder {
  pub fn build(&self) -> TextEntities { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn entities(&mut self, entities: Vec<TextEntity>) -> &mut Self {
    self.inner.entities = entities;
    self
  }

}

impl AsRef<TextEntities> for TextEntities {
  fn as_ref(&self) -> &TextEntities { self }
}

impl AsRef<TextEntities> for RTDTextEntitiesBuilder {
  fn as_ref(&self) -> &TextEntities { &self.inner }
}



