
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents a list of chat administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// A list of chat administrators
  administrators: Vec<ChatAdministrator>,
  
}

impl RObject for ChatAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatAdministrators" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatAdministrators {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatAdministratorsBuilder {
    let mut inner = ChatAdministrators::default();
    inner.td_name = "chatAdministrators".to_string();
    RTDChatAdministratorsBuilder { inner }
  }

  pub fn administrators(&self) -> &Vec<ChatAdministrator> { &self.administrators }

}

#[doc(hidden)]
pub struct RTDChatAdministratorsBuilder {
  inner: ChatAdministrators
}

impl RTDChatAdministratorsBuilder {
  pub fn build(&self) -> ChatAdministrators { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn administrators(&mut self, administrators: Vec<ChatAdministrator>) -> &mut Self {
    self.inner.administrators = administrators;
    self
  }

}

impl AsRef<ChatAdministrators> for ChatAdministrators {
  fn as_ref(&self) -> &ChatAdministrators { self }
}

impl AsRef<ChatAdministrators> for RTDChatAdministratorsBuilder {
  fn as_ref(&self) -> &ChatAdministrators { &self.inner }
}



