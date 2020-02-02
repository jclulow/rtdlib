
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents a list of chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of chat identifiers
  chat_ids: Vec<i64>,
  
}

impl RObject for Chats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chats" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Chats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatsBuilder {
    let mut inner = Chats::default();
    inner.td_name = "chats".to_string();
    RTDChatsBuilder { inner }
  }

  pub fn chat_ids(&self) -> &Vec<i64> { &self.chat_ids }

}

#[doc(hidden)]
pub struct RTDChatsBuilder {
  inner: Chats
}

impl RTDChatsBuilder {
  pub fn build(&self) -> Chats { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.chat_ids = chat_ids;
    self
  }

}

impl AsRef<Chats> for Chats {
  fn as_ref(&self) -> &Chats { self }
}

impl AsRef<Chats> for RTDChatsBuilder {
  fn as_ref(&self) -> &Chats { &self.inner }
}



