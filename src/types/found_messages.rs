
use crate::types::*;
use crate::errors::*;




/// Contains a list of messages found by a search
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundMessages {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// List of messages
  messages: Vec<Message>,
  /// Value to pass as from_search_id to get more results
  next_from_search_id: isize,
  
}

impl RObject for FoundMessages {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "foundMessages" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl FoundMessages {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDFoundMessagesBuilder {
    let mut inner = FoundMessages::default();
    inner.td_name = "foundMessages".to_string();
    RTDFoundMessagesBuilder { inner }
  }

  pub fn messages(&self) -> &Vec<Message> { &self.messages }

  pub fn next_from_search_id(&self) -> isize { self.next_from_search_id }

}

#[doc(hidden)]
pub struct RTDFoundMessagesBuilder {
  inner: FoundMessages
}

impl RTDFoundMessagesBuilder {
  pub fn build(&self) -> FoundMessages { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
    self.inner.messages = messages;
    self
  }

   
  pub fn next_from_search_id(&mut self, next_from_search_id: isize) -> &mut Self {
    self.inner.next_from_search_id = next_from_search_id;
    self
  }

}

impl AsRef<FoundMessages> for FoundMessages {
  fn as_ref(&self) -> &FoundMessages { self }
}

impl AsRef<FoundMessages> for RTDFoundMessagesBuilder {
  fn as_ref(&self) -> &FoundMessages { &self.inner }
}



