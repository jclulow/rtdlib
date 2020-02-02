
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains the description of an error in a Telegram Passport element; for bots only
pub trait TDInputPassportElementErrorSource: Debug + RObject {}

/// Contains the description of an error in a Telegram Passport element; for bots only
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputPassportElementErrorSource {
  #[doc(hidden)] _Default(()),
  /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
  Unspecified(InputPassportElementErrorSourceUnspecified),
  /// A data field contains an error. The error is considered resolved when the field's value changes
  DataField(InputPassportElementErrorSourceDataField),
  /// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
  FrontSide(InputPassportElementErrorSourceFrontSide),
  /// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
  ReverseSide(InputPassportElementErrorSourceReverseSide),
  /// The selfie contains an error. The error is considered resolved when the file with the selfie changes
  Selfie(InputPassportElementErrorSourceSelfie),
  /// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
  TranslationFile(InputPassportElementErrorSourceTranslationFile),
  /// The translation of the document contains an error. The error is considered resolved when the list of files changes
  TranslationFiles(InputPassportElementErrorSourceTranslationFiles),
  /// The file contains an error. The error is considered resolved when the file changes
  File(InputPassportElementErrorSourceFile),
  /// The list of attached files contains an error. The error is considered resolved when the file list changes
  Files(InputPassportElementErrorSourceFiles),

}

impl Default for InputPassportElementErrorSource {
  fn default() -> Self { InputPassportElementErrorSource::_Default(()) }
}

impl<'de> Deserialize<'de> for InputPassportElementErrorSource {
  fn deserialize<D>(deserializer: D) -> Result<InputPassportElementErrorSource, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputPassportElementErrorSource,
      (inputPassportElementErrorSourceUnspecified, Unspecified);
      (inputPassportElementErrorSourceDataField, DataField);
      (inputPassportElementErrorSourceFrontSide, FrontSide);
      (inputPassportElementErrorSourceReverseSide, ReverseSide);
      (inputPassportElementErrorSourceSelfie, Selfie);
      (inputPassportElementErrorSourceTranslationFile, TranslationFile);
      (inputPassportElementErrorSourceTranslationFiles, TranslationFiles);
      (inputPassportElementErrorSourceFile, File);
      (inputPassportElementErrorSourceFiles, Files);

    )(deserializer)
  }
}

impl RObject for InputPassportElementErrorSource {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      InputPassportElementErrorSource::Unspecified(t) => t.td_name(),
      InputPassportElementErrorSource::DataField(t) => t.td_name(),
      InputPassportElementErrorSource::FrontSide(t) => t.td_name(),
      InputPassportElementErrorSource::ReverseSide(t) => t.td_name(),
      InputPassportElementErrorSource::Selfie(t) => t.td_name(),
      InputPassportElementErrorSource::TranslationFile(t) => t.td_name(),
      InputPassportElementErrorSource::TranslationFiles(t) => t.td_name(),
      InputPassportElementErrorSource::File(t) => t.td_name(),
      InputPassportElementErrorSource::Files(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl InputPassportElementErrorSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let InputPassportElementErrorSource::_Default(_) = self { true } else { false } }

  pub fn is_unspecified(&self) -> bool { if let InputPassportElementErrorSource::Unspecified(_) = self { true } else { false } }
  pub fn is_data_field(&self) -> bool { if let InputPassportElementErrorSource::DataField(_) = self { true } else { false } }
  pub fn is_front_side(&self) -> bool { if let InputPassportElementErrorSource::FrontSide(_) = self { true } else { false } }
  pub fn is_reverse_side(&self) -> bool { if let InputPassportElementErrorSource::ReverseSide(_) = self { true } else { false } }
  pub fn is_selfie(&self) -> bool { if let InputPassportElementErrorSource::Selfie(_) = self { true } else { false } }
  pub fn is_translation_file(&self) -> bool { if let InputPassportElementErrorSource::TranslationFile(_) = self { true } else { false } }
  pub fn is_translation_files(&self) -> bool { if let InputPassportElementErrorSource::TranslationFiles(_) = self { true } else { false } }
  pub fn is_file(&self) -> bool { if let InputPassportElementErrorSource::File(_) = self { true } else { false } }
  pub fn is_files(&self) -> bool { if let InputPassportElementErrorSource::Files(_) = self { true } else { false } }

  pub fn on_unspecified<F: FnOnce(&InputPassportElementErrorSourceUnspecified)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::Unspecified(t) = self { fnc(t) }; self }
  pub fn on_data_field<F: FnOnce(&InputPassportElementErrorSourceDataField)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::DataField(t) = self { fnc(t) }; self }
  pub fn on_front_side<F: FnOnce(&InputPassportElementErrorSourceFrontSide)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::FrontSide(t) = self { fnc(t) }; self }
  pub fn on_reverse_side<F: FnOnce(&InputPassportElementErrorSourceReverseSide)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::ReverseSide(t) = self { fnc(t) }; self }
  pub fn on_selfie<F: FnOnce(&InputPassportElementErrorSourceSelfie)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::Selfie(t) = self { fnc(t) }; self }
  pub fn on_translation_file<F: FnOnce(&InputPassportElementErrorSourceTranslationFile)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::TranslationFile(t) = self { fnc(t) }; self }
  pub fn on_translation_files<F: FnOnce(&InputPassportElementErrorSourceTranslationFiles)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::TranslationFiles(t) = self { fnc(t) }; self }
  pub fn on_file<F: FnOnce(&InputPassportElementErrorSourceFile)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::File(t) = self { fnc(t) }; self }
  pub fn on_files<F: FnOnce(&InputPassportElementErrorSourceFiles)>(&self, fnc: F) -> &Self { if let InputPassportElementErrorSource::Files(t) = self { fnc(t) }; self }

  pub fn as_unspecified(&self) -> Option<&InputPassportElementErrorSourceUnspecified> { if let InputPassportElementErrorSource::Unspecified(t) = self { return Some(t) } None }
  pub fn as_data_field(&self) -> Option<&InputPassportElementErrorSourceDataField> { if let InputPassportElementErrorSource::DataField(t) = self { return Some(t) } None }
  pub fn as_front_side(&self) -> Option<&InputPassportElementErrorSourceFrontSide> { if let InputPassportElementErrorSource::FrontSide(t) = self { return Some(t) } None }
  pub fn as_reverse_side(&self) -> Option<&InputPassportElementErrorSourceReverseSide> { if let InputPassportElementErrorSource::ReverseSide(t) = self { return Some(t) } None }
  pub fn as_selfie(&self) -> Option<&InputPassportElementErrorSourceSelfie> { if let InputPassportElementErrorSource::Selfie(t) = self { return Some(t) } None }
  pub fn as_translation_file(&self) -> Option<&InputPassportElementErrorSourceTranslationFile> { if let InputPassportElementErrorSource::TranslationFile(t) = self { return Some(t) } None }
  pub fn as_translation_files(&self) -> Option<&InputPassportElementErrorSourceTranslationFiles> { if let InputPassportElementErrorSource::TranslationFiles(t) = self { return Some(t) } None }
  pub fn as_file(&self) -> Option<&InputPassportElementErrorSourceFile> { if let InputPassportElementErrorSource::File(t) = self { return Some(t) } None }
  pub fn as_files(&self) -> Option<&InputPassportElementErrorSourceFiles> { if let InputPassportElementErrorSource::Files(t) = self { return Some(t) } None }



  pub fn unspecified<T: AsRef<InputPassportElementErrorSourceUnspecified>>(t: T) -> Self { InputPassportElementErrorSource::Unspecified(t.as_ref().clone()) }

  pub fn data_field<T: AsRef<InputPassportElementErrorSourceDataField>>(t: T) -> Self { InputPassportElementErrorSource::DataField(t.as_ref().clone()) }

  pub fn front_side<T: AsRef<InputPassportElementErrorSourceFrontSide>>(t: T) -> Self { InputPassportElementErrorSource::FrontSide(t.as_ref().clone()) }

  pub fn reverse_side<T: AsRef<InputPassportElementErrorSourceReverseSide>>(t: T) -> Self { InputPassportElementErrorSource::ReverseSide(t.as_ref().clone()) }

  pub fn selfie<T: AsRef<InputPassportElementErrorSourceSelfie>>(t: T) -> Self { InputPassportElementErrorSource::Selfie(t.as_ref().clone()) }

  pub fn translation_file<T: AsRef<InputPassportElementErrorSourceTranslationFile>>(t: T) -> Self { InputPassportElementErrorSource::TranslationFile(t.as_ref().clone()) }

  pub fn translation_files<T: AsRef<InputPassportElementErrorSourceTranslationFiles>>(t: T) -> Self { InputPassportElementErrorSource::TranslationFiles(t.as_ref().clone()) }

  pub fn file<T: AsRef<InputPassportElementErrorSourceFile>>(t: T) -> Self { InputPassportElementErrorSource::File(t.as_ref().clone()) }

  pub fn files<T: AsRef<InputPassportElementErrorSourceFiles>>(t: T) -> Self { InputPassportElementErrorSource::Files(t.as_ref().clone()) }

}

impl AsRef<InputPassportElementErrorSource> for InputPassportElementErrorSource {
  fn as_ref(&self) -> &InputPassportElementErrorSource { self }
}







/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceUnspecified {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hash of the entire element
  element_hash: String,
  
}

impl RObject for InputPassportElementErrorSourceUnspecified {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceUnspecified" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceUnspecified {}



impl InputPassportElementErrorSourceUnspecified {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceUnspecifiedBuilder {
    let mut inner = InputPassportElementErrorSourceUnspecified::default();
    inner.td_name = "inputPassportElementErrorSourceUnspecified".to_string();
    RTDInputPassportElementErrorSourceUnspecifiedBuilder { inner }
  }

  pub fn element_hash(&self) -> &String { &self.element_hash }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceUnspecifiedBuilder {
  inner: InputPassportElementErrorSourceUnspecified
}

impl RTDInputPassportElementErrorSourceUnspecifiedBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceUnspecified { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn element_hash<T: AsRef<str>>(&mut self, element_hash: T) -> &mut Self {
    self.inner.element_hash = element_hash.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementErrorSourceUnspecified> for InputPassportElementErrorSourceUnspecified {
  fn as_ref(&self) -> &InputPassportElementErrorSourceUnspecified { self }
}

impl AsRef<InputPassportElementErrorSourceUnspecified> for RTDInputPassportElementErrorSourceUnspecifiedBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceUnspecified { &self.inner }
}







/// A data field contains an error. The error is considered resolved when the field's value changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceDataField {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Field name
  field_name: String,
  /// Current data hash
  data_hash: String,
  
}

impl RObject for InputPassportElementErrorSourceDataField {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceDataField" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceDataField {}



impl InputPassportElementErrorSourceDataField {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceDataFieldBuilder {
    let mut inner = InputPassportElementErrorSourceDataField::default();
    inner.td_name = "inputPassportElementErrorSourceDataField".to_string();
    RTDInputPassportElementErrorSourceDataFieldBuilder { inner }
  }

  pub fn field_name(&self) -> &String { &self.field_name }

  pub fn data_hash(&self) -> &String { &self.data_hash }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceDataFieldBuilder {
  inner: InputPassportElementErrorSourceDataField
}

impl RTDInputPassportElementErrorSourceDataFieldBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceDataField { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn field_name<T: AsRef<str>>(&mut self, field_name: T) -> &mut Self {
    self.inner.field_name = field_name.as_ref().to_string();
    self
  }

   
  pub fn data_hash<T: AsRef<str>>(&mut self, data_hash: T) -> &mut Self {
    self.inner.data_hash = data_hash.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementErrorSourceDataField> for InputPassportElementErrorSourceDataField {
  fn as_ref(&self) -> &InputPassportElementErrorSourceDataField { self }
}

impl AsRef<InputPassportElementErrorSourceDataField> for RTDInputPassportElementErrorSourceDataFieldBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceDataField { &self.inner }
}







/// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFrontSide {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hash of the file containing the front side
  file_hash: String,
  
}

impl RObject for InputPassportElementErrorSourceFrontSide {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceFrontSide" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceFrontSide {}



impl InputPassportElementErrorSourceFrontSide {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceFrontSideBuilder {
    let mut inner = InputPassportElementErrorSourceFrontSide::default();
    inner.td_name = "inputPassportElementErrorSourceFrontSide".to_string();
    RTDInputPassportElementErrorSourceFrontSideBuilder { inner }
  }

  pub fn file_hash(&self) -> &String { &self.file_hash }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceFrontSideBuilder {
  inner: InputPassportElementErrorSourceFrontSide
}

impl RTDInputPassportElementErrorSourceFrontSideBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceFrontSide { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
    self.inner.file_hash = file_hash.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementErrorSourceFrontSide> for InputPassportElementErrorSourceFrontSide {
  fn as_ref(&self) -> &InputPassportElementErrorSourceFrontSide { self }
}

impl AsRef<InputPassportElementErrorSourceFrontSide> for RTDInputPassportElementErrorSourceFrontSideBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceFrontSide { &self.inner }
}







/// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceReverseSide {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hash of the file containing the reverse side
  file_hash: String,
  
}

impl RObject for InputPassportElementErrorSourceReverseSide {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceReverseSide" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceReverseSide {}



impl InputPassportElementErrorSourceReverseSide {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceReverseSideBuilder {
    let mut inner = InputPassportElementErrorSourceReverseSide::default();
    inner.td_name = "inputPassportElementErrorSourceReverseSide".to_string();
    RTDInputPassportElementErrorSourceReverseSideBuilder { inner }
  }

  pub fn file_hash(&self) -> &String { &self.file_hash }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceReverseSideBuilder {
  inner: InputPassportElementErrorSourceReverseSide
}

impl RTDInputPassportElementErrorSourceReverseSideBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceReverseSide { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
    self.inner.file_hash = file_hash.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementErrorSourceReverseSide> for InputPassportElementErrorSourceReverseSide {
  fn as_ref(&self) -> &InputPassportElementErrorSourceReverseSide { self }
}

impl AsRef<InputPassportElementErrorSourceReverseSide> for RTDInputPassportElementErrorSourceReverseSideBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceReverseSide { &self.inner }
}







/// The selfie contains an error. The error is considered resolved when the file with the selfie changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceSelfie {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hash of the file containing the selfie
  file_hash: String,
  
}

impl RObject for InputPassportElementErrorSourceSelfie {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceSelfie" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceSelfie {}



impl InputPassportElementErrorSourceSelfie {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceSelfieBuilder {
    let mut inner = InputPassportElementErrorSourceSelfie::default();
    inner.td_name = "inputPassportElementErrorSourceSelfie".to_string();
    RTDInputPassportElementErrorSourceSelfieBuilder { inner }
  }

  pub fn file_hash(&self) -> &String { &self.file_hash }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceSelfieBuilder {
  inner: InputPassportElementErrorSourceSelfie
}

impl RTDInputPassportElementErrorSourceSelfieBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceSelfie { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
    self.inner.file_hash = file_hash.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementErrorSourceSelfie> for InputPassportElementErrorSourceSelfie {
  fn as_ref(&self) -> &InputPassportElementErrorSourceSelfie { self }
}

impl AsRef<InputPassportElementErrorSourceSelfie> for RTDInputPassportElementErrorSourceSelfieBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceSelfie { &self.inner }
}







/// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceTranslationFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hash of the file containing the translation
  file_hash: String,
  
}

impl RObject for InputPassportElementErrorSourceTranslationFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceTranslationFile" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceTranslationFile {}



impl InputPassportElementErrorSourceTranslationFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceTranslationFileBuilder {
    let mut inner = InputPassportElementErrorSourceTranslationFile::default();
    inner.td_name = "inputPassportElementErrorSourceTranslationFile".to_string();
    RTDInputPassportElementErrorSourceTranslationFileBuilder { inner }
  }

  pub fn file_hash(&self) -> &String { &self.file_hash }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceTranslationFileBuilder {
  inner: InputPassportElementErrorSourceTranslationFile
}

impl RTDInputPassportElementErrorSourceTranslationFileBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceTranslationFile { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
    self.inner.file_hash = file_hash.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementErrorSourceTranslationFile> for InputPassportElementErrorSourceTranslationFile {
  fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFile { self }
}

impl AsRef<InputPassportElementErrorSourceTranslationFile> for RTDInputPassportElementErrorSourceTranslationFileBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFile { &self.inner }
}







/// The translation of the document contains an error. The error is considered resolved when the list of files changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceTranslationFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hashes of all files with the translation
  file_hashes: Vec<String>,
  
}

impl RObject for InputPassportElementErrorSourceTranslationFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceTranslationFiles" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceTranslationFiles {}



impl InputPassportElementErrorSourceTranslationFiles {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceTranslationFilesBuilder {
    let mut inner = InputPassportElementErrorSourceTranslationFiles::default();
    inner.td_name = "inputPassportElementErrorSourceTranslationFiles".to_string();
    RTDInputPassportElementErrorSourceTranslationFilesBuilder { inner }
  }

  pub fn file_hashes(&self) -> &Vec<String> { &self.file_hashes }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceTranslationFilesBuilder {
  inner: InputPassportElementErrorSourceTranslationFiles
}

impl RTDInputPassportElementErrorSourceTranslationFilesBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceTranslationFiles { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file_hashes(&mut self, file_hashes: Vec<String>) -> &mut Self {
    self.inner.file_hashes = file_hashes;
    self
  }

}

impl AsRef<InputPassportElementErrorSourceTranslationFiles> for InputPassportElementErrorSourceTranslationFiles {
  fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFiles { self }
}

impl AsRef<InputPassportElementErrorSourceTranslationFiles> for RTDInputPassportElementErrorSourceTranslationFilesBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFiles { &self.inner }
}







/// The file contains an error. The error is considered resolved when the file changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hash of the file which has the error
  file_hash: String,
  
}

impl RObject for InputPassportElementErrorSourceFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceFile" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceFile {}



impl InputPassportElementErrorSourceFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceFileBuilder {
    let mut inner = InputPassportElementErrorSourceFile::default();
    inner.td_name = "inputPassportElementErrorSourceFile".to_string();
    RTDInputPassportElementErrorSourceFileBuilder { inner }
  }

  pub fn file_hash(&self) -> &String { &self.file_hash }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceFileBuilder {
  inner: InputPassportElementErrorSourceFile
}

impl RTDInputPassportElementErrorSourceFileBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceFile { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
    self.inner.file_hash = file_hash.as_ref().to_string();
    self
  }

}

impl AsRef<InputPassportElementErrorSourceFile> for InputPassportElementErrorSourceFile {
  fn as_ref(&self) -> &InputPassportElementErrorSourceFile { self }
}

impl AsRef<InputPassportElementErrorSourceFile> for RTDInputPassportElementErrorSourceFileBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceFile { &self.inner }
}







/// The list of attached files contains an error. The error is considered resolved when the file list changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFiles {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Current hashes of all attached files
  file_hashes: Vec<String>,
  
}

impl RObject for InputPassportElementErrorSourceFiles {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputPassportElementErrorSourceFiles" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceFiles {}



impl InputPassportElementErrorSourceFiles {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInputPassportElementErrorSourceFilesBuilder {
    let mut inner = InputPassportElementErrorSourceFiles::default();
    inner.td_name = "inputPassportElementErrorSourceFiles".to_string();
    RTDInputPassportElementErrorSourceFilesBuilder { inner }
  }

  pub fn file_hashes(&self) -> &Vec<String> { &self.file_hashes }

}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceFilesBuilder {
  inner: InputPassportElementErrorSourceFiles
}

impl RTDInputPassportElementErrorSourceFilesBuilder {
  pub fn build(&self) -> InputPassportElementErrorSourceFiles { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn file_hashes(&mut self, file_hashes: Vec<String>) -> &mut Self {
    self.inner.file_hashes = file_hashes;
    self
  }

}

impl AsRef<InputPassportElementErrorSourceFiles> for InputPassportElementErrorSourceFiles {
  fn as_ref(&self) -> &InputPassportElementErrorSourceFiles { self }
}

impl AsRef<InputPassportElementErrorSourceFiles> for RTDInputPassportElementErrorSourceFilesBuilder {
  fn as_ref(&self) -> &InputPassportElementErrorSourceFiles { &self.inner }
}



