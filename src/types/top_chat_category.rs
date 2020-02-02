
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the categories of chats for which a list of frequently used chats can be retrieved
pub trait TDTopChatCategory: Debug + RObject {}

/// Represents the categories of chats for which a list of frequently used chats can be retrieved
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TopChatCategory {
  #[doc(hidden)] _Default(()),
  /// A category containing frequently used private chats with non-bot users
  Users(TopChatCategoryUsers),
  /// A category containing frequently used private chats with bot users
  Bots(TopChatCategoryBots),
  /// A category containing frequently used basic groups and supergroups
  Groups(TopChatCategoryGroups),
  /// A category containing frequently used channels
  Channels(TopChatCategoryChannels),
  /// A category containing frequently used chats with inline bots sorted by their usage in inline mode
  InlineBots(TopChatCategoryInlineBots),
  /// A category containing frequently used chats used for calls
  Calls(TopChatCategoryCalls),
  /// A category containing frequently used chats used to forward messages
  ForwardChats(TopChatCategoryForwardChats),

}

impl Default for TopChatCategory {
  fn default() -> Self { TopChatCategory::_Default(()) }
}

impl<'de> Deserialize<'de> for TopChatCategory {
  fn deserialize<D>(deserializer: D) -> Result<TopChatCategory, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TopChatCategory,
      (topChatCategoryUsers, Users);
      (topChatCategoryBots, Bots);
      (topChatCategoryGroups, Groups);
      (topChatCategoryChannels, Channels);
      (topChatCategoryInlineBots, InlineBots);
      (topChatCategoryCalls, Calls);
      (topChatCategoryForwardChats, ForwardChats);

    )(deserializer)
  }
}

impl RObject for TopChatCategory {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      TopChatCategory::Users(t) => t.td_name(),
      TopChatCategory::Bots(t) => t.td_name(),
      TopChatCategory::Groups(t) => t.td_name(),
      TopChatCategory::Channels(t) => t.td_name(),
      TopChatCategory::InlineBots(t) => t.td_name(),
      TopChatCategory::Calls(t) => t.td_name(),
      TopChatCategory::ForwardChats(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl TopChatCategory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let TopChatCategory::_Default(_) = self { true } else { false } }

  pub fn is_users(&self) -> bool { if let TopChatCategory::Users(_) = self { true } else { false } }
  pub fn is_bots(&self) -> bool { if let TopChatCategory::Bots(_) = self { true } else { false } }
  pub fn is_groups(&self) -> bool { if let TopChatCategory::Groups(_) = self { true } else { false } }
  pub fn is_channels(&self) -> bool { if let TopChatCategory::Channels(_) = self { true } else { false } }
  pub fn is_inline_bots(&self) -> bool { if let TopChatCategory::InlineBots(_) = self { true } else { false } }
  pub fn is_calls(&self) -> bool { if let TopChatCategory::Calls(_) = self { true } else { false } }
  pub fn is_forward_chats(&self) -> bool { if let TopChatCategory::ForwardChats(_) = self { true } else { false } }

  pub fn on_users<F: FnOnce(&TopChatCategoryUsers)>(&self, fnc: F) -> &Self { if let TopChatCategory::Users(t) = self { fnc(t) }; self }
  pub fn on_bots<F: FnOnce(&TopChatCategoryBots)>(&self, fnc: F) -> &Self { if let TopChatCategory::Bots(t) = self { fnc(t) }; self }
  pub fn on_groups<F: FnOnce(&TopChatCategoryGroups)>(&self, fnc: F) -> &Self { if let TopChatCategory::Groups(t) = self { fnc(t) }; self }
  pub fn on_channels<F: FnOnce(&TopChatCategoryChannels)>(&self, fnc: F) -> &Self { if let TopChatCategory::Channels(t) = self { fnc(t) }; self }
  pub fn on_inline_bots<F: FnOnce(&TopChatCategoryInlineBots)>(&self, fnc: F) -> &Self { if let TopChatCategory::InlineBots(t) = self { fnc(t) }; self }
  pub fn on_calls<F: FnOnce(&TopChatCategoryCalls)>(&self, fnc: F) -> &Self { if let TopChatCategory::Calls(t) = self { fnc(t) }; self }
  pub fn on_forward_chats<F: FnOnce(&TopChatCategoryForwardChats)>(&self, fnc: F) -> &Self { if let TopChatCategory::ForwardChats(t) = self { fnc(t) }; self }

  pub fn as_users(&self) -> Option<&TopChatCategoryUsers> { if let TopChatCategory::Users(t) = self { return Some(t) } None }
  pub fn as_bots(&self) -> Option<&TopChatCategoryBots> { if let TopChatCategory::Bots(t) = self { return Some(t) } None }
  pub fn as_groups(&self) -> Option<&TopChatCategoryGroups> { if let TopChatCategory::Groups(t) = self { return Some(t) } None }
  pub fn as_channels(&self) -> Option<&TopChatCategoryChannels> { if let TopChatCategory::Channels(t) = self { return Some(t) } None }
  pub fn as_inline_bots(&self) -> Option<&TopChatCategoryInlineBots> { if let TopChatCategory::InlineBots(t) = self { return Some(t) } None }
  pub fn as_calls(&self) -> Option<&TopChatCategoryCalls> { if let TopChatCategory::Calls(t) = self { return Some(t) } None }
  pub fn as_forward_chats(&self) -> Option<&TopChatCategoryForwardChats> { if let TopChatCategory::ForwardChats(t) = self { return Some(t) } None }



  pub fn users<T: AsRef<TopChatCategoryUsers>>(t: T) -> Self { TopChatCategory::Users(t.as_ref().clone()) }

  pub fn bots<T: AsRef<TopChatCategoryBots>>(t: T) -> Self { TopChatCategory::Bots(t.as_ref().clone()) }

  pub fn groups<T: AsRef<TopChatCategoryGroups>>(t: T) -> Self { TopChatCategory::Groups(t.as_ref().clone()) }

  pub fn channels<T: AsRef<TopChatCategoryChannels>>(t: T) -> Self { TopChatCategory::Channels(t.as_ref().clone()) }

  pub fn inline_bots<T: AsRef<TopChatCategoryInlineBots>>(t: T) -> Self { TopChatCategory::InlineBots(t.as_ref().clone()) }

  pub fn calls<T: AsRef<TopChatCategoryCalls>>(t: T) -> Self { TopChatCategory::Calls(t.as_ref().clone()) }

  pub fn forward_chats<T: AsRef<TopChatCategoryForwardChats>>(t: T) -> Self { TopChatCategory::ForwardChats(t.as_ref().clone()) }

}

impl AsRef<TopChatCategory> for TopChatCategory {
  fn as_ref(&self) -> &TopChatCategory { self }
}







/// A category containing frequently used private chats with non-bot users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TopChatCategoryUsers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryUsers" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTopChatCategory for TopChatCategoryUsers {}



impl TopChatCategoryUsers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTopChatCategoryUsersBuilder {
    let mut inner = TopChatCategoryUsers::default();
    inner.td_name = "topChatCategoryUsers".to_string();
    RTDTopChatCategoryUsersBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTopChatCategoryUsersBuilder {
  inner: TopChatCategoryUsers
}

impl RTDTopChatCategoryUsersBuilder {
  pub fn build(&self) -> TopChatCategoryUsers { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TopChatCategoryUsers> for TopChatCategoryUsers {
  fn as_ref(&self) -> &TopChatCategoryUsers { self }
}

impl AsRef<TopChatCategoryUsers> for RTDTopChatCategoryUsersBuilder {
  fn as_ref(&self) -> &TopChatCategoryUsers { &self.inner }
}







/// A category containing frequently used private chats with bot users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TopChatCategoryBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryBots" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTopChatCategory for TopChatCategoryBots {}



impl TopChatCategoryBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTopChatCategoryBotsBuilder {
    let mut inner = TopChatCategoryBots::default();
    inner.td_name = "topChatCategoryBots".to_string();
    RTDTopChatCategoryBotsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTopChatCategoryBotsBuilder {
  inner: TopChatCategoryBots
}

impl RTDTopChatCategoryBotsBuilder {
  pub fn build(&self) -> TopChatCategoryBots { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TopChatCategoryBots> for TopChatCategoryBots {
  fn as_ref(&self) -> &TopChatCategoryBots { self }
}

impl AsRef<TopChatCategoryBots> for RTDTopChatCategoryBotsBuilder {
  fn as_ref(&self) -> &TopChatCategoryBots { &self.inner }
}







/// A category containing frequently used basic groups and supergroups
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryGroups {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TopChatCategoryGroups {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryGroups" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTopChatCategory for TopChatCategoryGroups {}



impl TopChatCategoryGroups {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTopChatCategoryGroupsBuilder {
    let mut inner = TopChatCategoryGroups::default();
    inner.td_name = "topChatCategoryGroups".to_string();
    RTDTopChatCategoryGroupsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTopChatCategoryGroupsBuilder {
  inner: TopChatCategoryGroups
}

impl RTDTopChatCategoryGroupsBuilder {
  pub fn build(&self) -> TopChatCategoryGroups { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TopChatCategoryGroups> for TopChatCategoryGroups {
  fn as_ref(&self) -> &TopChatCategoryGroups { self }
}

impl AsRef<TopChatCategoryGroups> for RTDTopChatCategoryGroupsBuilder {
  fn as_ref(&self) -> &TopChatCategoryGroups { &self.inner }
}







/// A category containing frequently used channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryChannels {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TopChatCategoryChannels {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryChannels" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTopChatCategory for TopChatCategoryChannels {}



impl TopChatCategoryChannels {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTopChatCategoryChannelsBuilder {
    let mut inner = TopChatCategoryChannels::default();
    inner.td_name = "topChatCategoryChannels".to_string();
    RTDTopChatCategoryChannelsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTopChatCategoryChannelsBuilder {
  inner: TopChatCategoryChannels
}

impl RTDTopChatCategoryChannelsBuilder {
  pub fn build(&self) -> TopChatCategoryChannels { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TopChatCategoryChannels> for TopChatCategoryChannels {
  fn as_ref(&self) -> &TopChatCategoryChannels { self }
}

impl AsRef<TopChatCategoryChannels> for RTDTopChatCategoryChannelsBuilder {
  fn as_ref(&self) -> &TopChatCategoryChannels { &self.inner }
}







/// A category containing frequently used chats with inline bots sorted by their usage in inline mode
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryInlineBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TopChatCategoryInlineBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryInlineBots" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTopChatCategory for TopChatCategoryInlineBots {}



impl TopChatCategoryInlineBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTopChatCategoryInlineBotsBuilder {
    let mut inner = TopChatCategoryInlineBots::default();
    inner.td_name = "topChatCategoryInlineBots".to_string();
    RTDTopChatCategoryInlineBotsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTopChatCategoryInlineBotsBuilder {
  inner: TopChatCategoryInlineBots
}

impl RTDTopChatCategoryInlineBotsBuilder {
  pub fn build(&self) -> TopChatCategoryInlineBots { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TopChatCategoryInlineBots> for TopChatCategoryInlineBots {
  fn as_ref(&self) -> &TopChatCategoryInlineBots { self }
}

impl AsRef<TopChatCategoryInlineBots> for RTDTopChatCategoryInlineBotsBuilder {
  fn as_ref(&self) -> &TopChatCategoryInlineBots { &self.inner }
}







/// A category containing frequently used chats used for calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryCalls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TopChatCategoryCalls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryCalls" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTopChatCategory for TopChatCategoryCalls {}



impl TopChatCategoryCalls {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTopChatCategoryCallsBuilder {
    let mut inner = TopChatCategoryCalls::default();
    inner.td_name = "topChatCategoryCalls".to_string();
    RTDTopChatCategoryCallsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTopChatCategoryCallsBuilder {
  inner: TopChatCategoryCalls
}

impl RTDTopChatCategoryCallsBuilder {
  pub fn build(&self) -> TopChatCategoryCalls { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TopChatCategoryCalls> for TopChatCategoryCalls {
  fn as_ref(&self) -> &TopChatCategoryCalls { self }
}

impl AsRef<TopChatCategoryCalls> for RTDTopChatCategoryCallsBuilder {
  fn as_ref(&self) -> &TopChatCategoryCalls { &self.inner }
}







/// A category containing frequently used chats used to forward messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TopChatCategoryForwardChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TopChatCategoryForwardChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryForwardChats" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTopChatCategory for TopChatCategoryForwardChats {}



impl TopChatCategoryForwardChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTopChatCategoryForwardChatsBuilder {
    let mut inner = TopChatCategoryForwardChats::default();
    inner.td_name = "topChatCategoryForwardChats".to_string();
    RTDTopChatCategoryForwardChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTopChatCategoryForwardChatsBuilder {
  inner: TopChatCategoryForwardChats
}

impl RTDTopChatCategoryForwardChatsBuilder {
  pub fn build(&self) -> TopChatCategoryForwardChats { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TopChatCategoryForwardChats> for TopChatCategoryForwardChats {
  fn as_ref(&self) -> &TopChatCategoryForwardChats { self }
}

impl AsRef<TopChatCategoryForwardChats> for RTDTopChatCategoryForwardChatsBuilder {
  fn as_ref(&self) -> &TopChatCategoryForwardChats { &self.inner }
}



