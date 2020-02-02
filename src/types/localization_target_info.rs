
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains information about the current localization target
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalizationTargetInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of available language packs for this application
  language_packs: Vec<LanguagePackInfo>,
  
}

impl RObject for LocalizationTargetInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "localizationTargetInfo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LocalizationTargetInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLocalizationTargetInfoBuilder {
    let mut inner = LocalizationTargetInfo::default();
    inner.td_name = "localizationTargetInfo".to_string();
    RTDLocalizationTargetInfoBuilder { inner }
  }

  pub fn language_packs(&self) -> &Vec<LanguagePackInfo> { &self.language_packs }

}

#[doc(hidden)]
pub struct RTDLocalizationTargetInfoBuilder {
  inner: LocalizationTargetInfo
}

impl RTDLocalizationTargetInfoBuilder {
  pub fn build(&self) -> LocalizationTargetInfo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn language_packs(&mut self, language_packs: Vec<LanguagePackInfo>) -> &mut Self {
    self.inner.language_packs = language_packs;
    self
  }

}

impl AsRef<LocalizationTargetInfo> for LocalizationTargetInfo {
  fn as_ref(&self) -> &LocalizationTargetInfo { self }
}

impl AsRef<LocalizationTargetInfo> for RTDLocalizationTargetInfoBuilder {
  fn as_ref(&self) -> &LocalizationTargetInfo { &self.inner }
}



