
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Provides information about a bot and its supported commands
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Provides information about a bot and its supported commands
  description: String,
  /// A list of commands supported by the bot
  commands: Vec<BotCommand>,
  
}

impl RObject for BotInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botInfo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl BotInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotInfoBuilder {
    let mut inner = BotInfo::default();
    inner.td_name = "botInfo".to_string();
    RTDBotInfoBuilder { inner }
  }

  pub fn description(&self) -> &String { &self.description }

  pub fn commands(&self) -> &Vec<BotCommand> { &self.commands }

}

#[doc(hidden)]
pub struct RTDBotInfoBuilder {
  inner: BotInfo
}

impl RTDBotInfoBuilder {
  pub fn build(&self) -> BotInfo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
    self.inner.description = description.as_ref().to_string();
    self
  }

   
  pub fn commands(&mut self, commands: Vec<BotCommand>) -> &mut Self {
    self.inner.commands = commands;
    self
  }

}

impl AsRef<BotInfo> for BotInfo {
  fn as_ref(&self) -> &BotInfo { self }
}

impl AsRef<BotInfo> for RTDBotInfoBuilder {
  fn as_ref(&self) -> &BotInfo { &self.inner }
}



