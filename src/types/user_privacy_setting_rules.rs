
use crate::types::*;
use crate::errors::*;




/// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRules {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// A list of rules
  rules: Vec<UserPrivacySettingRule>,
  
}

impl RObject for UserPrivacySettingRules {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userPrivacySettingRules" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl UserPrivacySettingRules {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserPrivacySettingRulesBuilder {
    let mut inner = UserPrivacySettingRules::default();
    inner.td_name = "userPrivacySettingRules".to_string();
    RTDUserPrivacySettingRulesBuilder { inner }
  }

  pub fn rules(&self) -> &Vec<UserPrivacySettingRule> { &self.rules }

}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRulesBuilder {
  inner: UserPrivacySettingRules
}

impl RTDUserPrivacySettingRulesBuilder {
  pub fn build(&self) -> UserPrivacySettingRules { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn rules(&mut self, rules: Vec<UserPrivacySettingRule>) -> &mut Self {
    self.inner.rules = rules;
    self
  }

}

impl AsRef<UserPrivacySettingRules> for UserPrivacySettingRules {
  fn as_ref(&self) -> &UserPrivacySettingRules { self }
}

impl AsRef<UserPrivacySettingRules> for RTDUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &UserPrivacySettingRules { &self.inner }
}



