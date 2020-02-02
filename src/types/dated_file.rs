
use crate::types::*;
use crate::errors::*;




/// File with the date it was uploaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatedFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// The file
  file: File,
  /// Point in time (Unix timestamp) when the file was uploaded
  date: i64,
  
}

impl RObject for DatedFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "datedFile" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl DatedFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDatedFileBuilder {
    let mut inner = DatedFile::default();
    inner.td_name = "datedFile".to_string();
    RTDDatedFileBuilder { inner }
  }

  pub fn file(&self) -> &File { &self.file }

  pub fn date(&self) -> i64 { self.date }

}

#[doc(hidden)]
pub struct RTDDatedFileBuilder {
  inner: DatedFile
}

impl RTDDatedFileBuilder {
  pub fn build(&self) -> DatedFile { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
    self.inner.file = file.as_ref().clone();
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

}

impl AsRef<DatedFile> for DatedFile {
  fn as_ref(&self) -> &DatedFile { self }
}

impl AsRef<DatedFile> for RTDDatedFileBuilder {
  fn as_ref(&self) -> &DatedFile { &self.inner }
}



