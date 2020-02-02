
use crate::types::*;
use crate::errors::*;




/// Contains full information about a basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BasicGroupFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Contains full information about a basic group
  description: String,
  /// User identifier of the creator of the group; 0 if unknown
  creator_user_id: i64,
  /// Group members
  members: Vec<ChatMember>,
  /// Invite link for this group; available only after it has been generated at least once and only for the group creator
  invite_link: String,
  
}

impl RObject for BasicGroupFullInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "basicGroupFullInfo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl BasicGroupFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBasicGroupFullInfoBuilder {
    let mut inner = BasicGroupFullInfo::default();
    inner.td_name = "basicGroupFullInfo".to_string();
    RTDBasicGroupFullInfoBuilder { inner }
  }

  pub fn description(&self) -> &String { &self.description }

  pub fn creator_user_id(&self) -> i64 { self.creator_user_id }

  pub fn members(&self) -> &Vec<ChatMember> { &self.members }

  pub fn invite_link(&self) -> &String { &self.invite_link }

}

#[doc(hidden)]
pub struct RTDBasicGroupFullInfoBuilder {
  inner: BasicGroupFullInfo
}

impl RTDBasicGroupFullInfoBuilder {
  pub fn build(&self) -> BasicGroupFullInfo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

   
  pub fn creator_user_id(&mut self, creator_user_id: i64) -> &mut Self {
    self.inner.creator_user_id = creator_user_id;
    self
  }

   
  pub fn members(&mut self, members: Vec<ChatMember>) -> &mut Self {
    self.inner.members = members;
    self
  }

   
  pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
    self.inner.invite_link = invite_link.as_ref().to_string();
    self
  }

}

impl AsRef<BasicGroupFullInfo> for BasicGroupFullInfo {
  fn as_ref(&self) -> &BasicGroupFullInfo { self }
}

impl AsRef<BasicGroupFullInfo> for RTDBasicGroupFullInfoBuilder {
  fn as_ref(&self) -> &BasicGroupFullInfo { &self.inner }
}



