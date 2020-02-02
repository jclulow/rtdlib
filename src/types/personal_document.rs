
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// A personal document, containing some information about a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of files containing the pages of the document
  files: Vec<DatedFile>,
  /// List of files containing a certified English translation of the document
  translation: Vec<DatedFile>,
  
}

impl RObject for PersonalDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "personalDocument" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PersonalDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPersonalDocumentBuilder {
    let mut inner = PersonalDocument::default();
    inner.td_name = "personalDocument".to_string();
    RTDPersonalDocumentBuilder { inner }
  }

  pub fn files(&self) -> &Vec<DatedFile> { &self.files }

  pub fn translation(&self) -> &Vec<DatedFile> { &self.translation }

}

#[doc(hidden)]
pub struct RTDPersonalDocumentBuilder {
  inner: PersonalDocument
}

impl RTDPersonalDocumentBuilder {
  pub fn build(&self) -> PersonalDocument { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn files(&mut self, files: Vec<DatedFile>) -> &mut Self {
    self.inner.files = files;
    self
  }

   
  pub fn translation(&mut self, translation: Vec<DatedFile>) -> &mut Self {
    self.inner.translation = translation;
    self
  }

}

impl AsRef<PersonalDocument> for PersonalDocument {
  fn as_ref(&self) -> &PersonalDocument { self }
}

impl AsRef<PersonalDocument> for RTDPersonalDocumentBuilder {
  fn as_ref(&self) -> &PersonalDocument { &self.inner }
}



