
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the type of a network
pub trait TDNetworkType: Debug + RObject {}

/// Represents the type of a network
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NetworkType {
  #[doc(hidden)] _Default(()),
  /// The network is not available
  None(NetworkTypeNone),
  /// A mobile network
  Mobile(NetworkTypeMobile),
  /// A mobile roaming network
  MobileRoaming(NetworkTypeMobileRoaming),
  /// A Wi-Fi network
  WiFi(NetworkTypeWiFi),
  /// A different network type (e.g., Ethernet network)
  Other(NetworkTypeOther),

}

impl Default for NetworkType {
  fn default() -> Self { NetworkType::_Default(()) }
}

impl<'de> Deserialize<'de> for NetworkType {
  fn deserialize<D>(deserializer: D) -> Result<NetworkType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      NetworkType,
      (networkTypeNone, None);
      (networkTypeMobile, Mobile);
      (networkTypeMobileRoaming, MobileRoaming);
      (networkTypeWiFi, WiFi);
      (networkTypeOther, Other);

    )(deserializer)
  }
}

impl RObject for NetworkType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      NetworkType::None(t) => t.td_name(),
      NetworkType::Mobile(t) => t.td_name(),
      NetworkType::MobileRoaming(t) => t.td_name(),
      NetworkType::WiFi(t) => t.td_name(),
      NetworkType::Other(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl NetworkType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let NetworkType::_Default(_) = self { true } else { false } }

  pub fn is_none(&self) -> bool { if let NetworkType::None(_) = self { true } else { false } }
  pub fn is_mobile(&self) -> bool { if let NetworkType::Mobile(_) = self { true } else { false } }
  pub fn is_mobile_roaming(&self) -> bool { if let NetworkType::MobileRoaming(_) = self { true } else { false } }
  pub fn is_wi_fi(&self) -> bool { if let NetworkType::WiFi(_) = self { true } else { false } }
  pub fn is_other(&self) -> bool { if let NetworkType::Other(_) = self { true } else { false } }

  pub fn on_none<F: FnOnce(&NetworkTypeNone)>(&self, fnc: F) -> &Self { if let NetworkType::None(t) = self { fnc(t) }; self }
  pub fn on_mobile<F: FnOnce(&NetworkTypeMobile)>(&self, fnc: F) -> &Self { if let NetworkType::Mobile(t) = self { fnc(t) }; self }
  pub fn on_mobile_roaming<F: FnOnce(&NetworkTypeMobileRoaming)>(&self, fnc: F) -> &Self { if let NetworkType::MobileRoaming(t) = self { fnc(t) }; self }
  pub fn on_wi_fi<F: FnOnce(&NetworkTypeWiFi)>(&self, fnc: F) -> &Self { if let NetworkType::WiFi(t) = self { fnc(t) }; self }
  pub fn on_other<F: FnOnce(&NetworkTypeOther)>(&self, fnc: F) -> &Self { if let NetworkType::Other(t) = self { fnc(t) }; self }

  pub fn as_none(&self) -> Option<&NetworkTypeNone> { if let NetworkType::None(t) = self { return Some(t) } None }
  pub fn as_mobile(&self) -> Option<&NetworkTypeMobile> { if let NetworkType::Mobile(t) = self { return Some(t) } None }
  pub fn as_mobile_roaming(&self) -> Option<&NetworkTypeMobileRoaming> { if let NetworkType::MobileRoaming(t) = self { return Some(t) } None }
  pub fn as_wi_fi(&self) -> Option<&NetworkTypeWiFi> { if let NetworkType::WiFi(t) = self { return Some(t) } None }
  pub fn as_other(&self) -> Option<&NetworkTypeOther> { if let NetworkType::Other(t) = self { return Some(t) } None }



  pub fn none<T: AsRef<NetworkTypeNone>>(t: T) -> Self { NetworkType::None(t.as_ref().clone()) }

  pub fn mobile<T: AsRef<NetworkTypeMobile>>(t: T) -> Self { NetworkType::Mobile(t.as_ref().clone()) }

  pub fn mobile_roaming<T: AsRef<NetworkTypeMobileRoaming>>(t: T) -> Self { NetworkType::MobileRoaming(t.as_ref().clone()) }

  pub fn wi_fi<T: AsRef<NetworkTypeWiFi>>(t: T) -> Self { NetworkType::WiFi(t.as_ref().clone()) }

  pub fn other<T: AsRef<NetworkTypeOther>>(t: T) -> Self { NetworkType::Other(t.as_ref().clone()) }

}

impl AsRef<NetworkType> for NetworkType {
  fn as_ref(&self) -> &NetworkType { self }
}







/// The network is not available
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeNone {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for NetworkTypeNone {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeNone" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNetworkType for NetworkTypeNone {}



impl NetworkTypeNone {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkTypeNoneBuilder {
    let mut inner = NetworkTypeNone::default();
    inner.td_name = "networkTypeNone".to_string();
    RTDNetworkTypeNoneBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNetworkTypeNoneBuilder {
  inner: NetworkTypeNone
}

impl RTDNetworkTypeNoneBuilder {
  pub fn build(&self) -> NetworkTypeNone { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<NetworkTypeNone> for NetworkTypeNone {
  fn as_ref(&self) -> &NetworkTypeNone { self }
}

impl AsRef<NetworkTypeNone> for RTDNetworkTypeNoneBuilder {
  fn as_ref(&self) -> &NetworkTypeNone { &self.inner }
}







/// A mobile network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeMobile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for NetworkTypeMobile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeMobile" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNetworkType for NetworkTypeMobile {}



impl NetworkTypeMobile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkTypeMobileBuilder {
    let mut inner = NetworkTypeMobile::default();
    inner.td_name = "networkTypeMobile".to_string();
    RTDNetworkTypeMobileBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNetworkTypeMobileBuilder {
  inner: NetworkTypeMobile
}

impl RTDNetworkTypeMobileBuilder {
  pub fn build(&self) -> NetworkTypeMobile { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<NetworkTypeMobile> for NetworkTypeMobile {
  fn as_ref(&self) -> &NetworkTypeMobile { self }
}

impl AsRef<NetworkTypeMobile> for RTDNetworkTypeMobileBuilder {
  fn as_ref(&self) -> &NetworkTypeMobile { &self.inner }
}







/// A mobile roaming network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeMobileRoaming {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for NetworkTypeMobileRoaming {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeMobileRoaming" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNetworkType for NetworkTypeMobileRoaming {}



impl NetworkTypeMobileRoaming {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkTypeMobileRoamingBuilder {
    let mut inner = NetworkTypeMobileRoaming::default();
    inner.td_name = "networkTypeMobileRoaming".to_string();
    RTDNetworkTypeMobileRoamingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNetworkTypeMobileRoamingBuilder {
  inner: NetworkTypeMobileRoaming
}

impl RTDNetworkTypeMobileRoamingBuilder {
  pub fn build(&self) -> NetworkTypeMobileRoaming { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<NetworkTypeMobileRoaming> for NetworkTypeMobileRoaming {
  fn as_ref(&self) -> &NetworkTypeMobileRoaming { self }
}

impl AsRef<NetworkTypeMobileRoaming> for RTDNetworkTypeMobileRoamingBuilder {
  fn as_ref(&self) -> &NetworkTypeMobileRoaming { &self.inner }
}







/// A Wi-Fi network
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeWiFi {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for NetworkTypeWiFi {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeWiFi" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNetworkType for NetworkTypeWiFi {}



impl NetworkTypeWiFi {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkTypeWiFiBuilder {
    let mut inner = NetworkTypeWiFi::default();
    inner.td_name = "networkTypeWiFi".to_string();
    RTDNetworkTypeWiFiBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNetworkTypeWiFiBuilder {
  inner: NetworkTypeWiFi
}

impl RTDNetworkTypeWiFiBuilder {
  pub fn build(&self) -> NetworkTypeWiFi { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<NetworkTypeWiFi> for NetworkTypeWiFi {
  fn as_ref(&self) -> &NetworkTypeWiFi { self }
}

impl AsRef<NetworkTypeWiFi> for RTDNetworkTypeWiFiBuilder {
  fn as_ref(&self) -> &NetworkTypeWiFi { &self.inner }
}







/// A different network type (e.g., Ethernet network)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkTypeOther {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for NetworkTypeOther {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkTypeOther" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNetworkType for NetworkTypeOther {}



impl NetworkTypeOther {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkTypeOtherBuilder {
    let mut inner = NetworkTypeOther::default();
    inner.td_name = "networkTypeOther".to_string();
    RTDNetworkTypeOtherBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDNetworkTypeOtherBuilder {
  inner: NetworkTypeOther
}

impl RTDNetworkTypeOtherBuilder {
  pub fn build(&self) -> NetworkTypeOther { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<NetworkTypeOther> for NetworkTypeOther {
  fn as_ref(&self) -> &NetworkTypeOther { self }
}

impl AsRef<NetworkTypeOther> for RTDNetworkTypeOtherBuilder {
  fn as_ref(&self) -> &NetworkTypeOther { &self.inner }
}



