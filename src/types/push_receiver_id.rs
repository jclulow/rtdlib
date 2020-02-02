
use crate::types::*;
use crate::errors::*;




/// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushReceiverId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// The globally unique identifier of push notification subscription
  id: isize,
  
}

impl RObject for PushReceiverId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushReceiverId" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PushReceiverId {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPushReceiverIdBuilder {
    let mut inner = PushReceiverId::default();
    inner.td_name = "pushReceiverId".to_string();
    RTDPushReceiverIdBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

}

#[doc(hidden)]
pub struct RTDPushReceiverIdBuilder {
  inner: PushReceiverId
}

impl RTDPushReceiverIdBuilder {
  pub fn build(&self) -> PushReceiverId { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

}

impl AsRef<PushReceiverId> for PushReceiverId {
  fn as_ref(&self) -> &PushReceiverId { self }
}

impl AsRef<PushReceiverId> for RTDPushReceiverIdBuilder {
  fn as_ref(&self) -> &PushReceiverId { &self.inner }
}



