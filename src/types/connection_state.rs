
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the current state of the connection to Telegram servers
pub trait TDConnectionState: Debug + RObject {}

/// Describes the current state of the connection to Telegram servers
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ConnectionState {
  #[doc(hidden)] _Default(()),
  /// Currently waiting for the network to become available. Use setNetworkType to change the available network type
  WaitingForNetwork(ConnectionStateWaitingForNetwork),
  /// Currently establishing a connection with a proxy server
  ConnectingToProxy(ConnectionStateConnectingToProxy),
  /// Currently establishing a connection to the Telegram servers
  Connecting(ConnectionStateConnecting),
  /// Downloading data received while the client was offline
  Updating(ConnectionStateUpdating),
  /// There is a working connection to the Telegram servers
  Ready(ConnectionStateReady),

}

impl Default for ConnectionState {
  fn default() -> Self { ConnectionState::_Default(()) }
}

impl<'de> Deserialize<'de> for ConnectionState {
  fn deserialize<D>(deserializer: D) -> Result<ConnectionState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ConnectionState,
      (connectionStateWaitingForNetwork, WaitingForNetwork);
      (connectionStateConnectingToProxy, ConnectingToProxy);
      (connectionStateConnecting, Connecting);
      (connectionStateUpdating, Updating);
      (connectionStateReady, Ready);

    )(deserializer)
  }
}

impl RObject for ConnectionState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ConnectionState::WaitingForNetwork(t) => t.td_name(),
      ConnectionState::ConnectingToProxy(t) => t.td_name(),
      ConnectionState::Connecting(t) => t.td_name(),
      ConnectionState::Updating(t) => t.td_name(),
      ConnectionState::Ready(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ConnectionState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ConnectionState::_Default(_) = self { true } else { false } }

  pub fn is_waiting_for_network(&self) -> bool { if let ConnectionState::WaitingForNetwork(_) = self { true } else { false } }
  pub fn is_connecting_to_proxy(&self) -> bool { if let ConnectionState::ConnectingToProxy(_) = self { true } else { false } }
  pub fn is_connecting(&self) -> bool { if let ConnectionState::Connecting(_) = self { true } else { false } }
  pub fn is_updating(&self) -> bool { if let ConnectionState::Updating(_) = self { true } else { false } }
  pub fn is_ready(&self) -> bool { if let ConnectionState::Ready(_) = self { true } else { false } }

  pub fn on_waiting_for_network<F: FnOnce(&ConnectionStateWaitingForNetwork)>(&self, fnc: F) -> &Self { if let ConnectionState::WaitingForNetwork(t) = self { fnc(t) }; self }
  pub fn on_connecting_to_proxy<F: FnOnce(&ConnectionStateConnectingToProxy)>(&self, fnc: F) -> &Self { if let ConnectionState::ConnectingToProxy(t) = self { fnc(t) }; self }
  pub fn on_connecting<F: FnOnce(&ConnectionStateConnecting)>(&self, fnc: F) -> &Self { if let ConnectionState::Connecting(t) = self { fnc(t) }; self }
  pub fn on_updating<F: FnOnce(&ConnectionStateUpdating)>(&self, fnc: F) -> &Self { if let ConnectionState::Updating(t) = self { fnc(t) }; self }
  pub fn on_ready<F: FnOnce(&ConnectionStateReady)>(&self, fnc: F) -> &Self { if let ConnectionState::Ready(t) = self { fnc(t) }; self }

  pub fn as_waiting_for_network(&self) -> Option<&ConnectionStateWaitingForNetwork> { if let ConnectionState::WaitingForNetwork(t) = self { return Some(t) } None }
  pub fn as_connecting_to_proxy(&self) -> Option<&ConnectionStateConnectingToProxy> { if let ConnectionState::ConnectingToProxy(t) = self { return Some(t) } None }
  pub fn as_connecting(&self) -> Option<&ConnectionStateConnecting> { if let ConnectionState::Connecting(t) = self { return Some(t) } None }
  pub fn as_updating(&self) -> Option<&ConnectionStateUpdating> { if let ConnectionState::Updating(t) = self { return Some(t) } None }
  pub fn as_ready(&self) -> Option<&ConnectionStateReady> { if let ConnectionState::Ready(t) = self { return Some(t) } None }



  pub fn waiting_for_network<T: AsRef<ConnectionStateWaitingForNetwork>>(t: T) -> Self { ConnectionState::WaitingForNetwork(t.as_ref().clone()) }

  pub fn connecting_to_proxy<T: AsRef<ConnectionStateConnectingToProxy>>(t: T) -> Self { ConnectionState::ConnectingToProxy(t.as_ref().clone()) }

  pub fn connecting<T: AsRef<ConnectionStateConnecting>>(t: T) -> Self { ConnectionState::Connecting(t.as_ref().clone()) }

  pub fn updating<T: AsRef<ConnectionStateUpdating>>(t: T) -> Self { ConnectionState::Updating(t.as_ref().clone()) }

  pub fn ready<T: AsRef<ConnectionStateReady>>(t: T) -> Self { ConnectionState::Ready(t.as_ref().clone()) }

}

impl AsRef<ConnectionState> for ConnectionState {
  fn as_ref(&self) -> &ConnectionState { self }
}







/// Currently waiting for the network to become available. Use setNetworkType to change the available network type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateWaitingForNetwork {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for ConnectionStateWaitingForNetwork {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateWaitingForNetwork" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDConnectionState for ConnectionStateWaitingForNetwork {}



impl ConnectionStateWaitingForNetwork {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectionStateWaitingForNetworkBuilder {
    let mut inner = ConnectionStateWaitingForNetwork::default();
    inner.td_name = "connectionStateWaitingForNetwork".to_string();
    RTDConnectionStateWaitingForNetworkBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDConnectionStateWaitingForNetworkBuilder {
  inner: ConnectionStateWaitingForNetwork
}

impl RTDConnectionStateWaitingForNetworkBuilder {
  pub fn build(&self) -> ConnectionStateWaitingForNetwork { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<ConnectionStateWaitingForNetwork> for ConnectionStateWaitingForNetwork {
  fn as_ref(&self) -> &ConnectionStateWaitingForNetwork { self }
}

impl AsRef<ConnectionStateWaitingForNetwork> for RTDConnectionStateWaitingForNetworkBuilder {
  fn as_ref(&self) -> &ConnectionStateWaitingForNetwork { &self.inner }
}







/// Currently establishing a connection with a proxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateConnectingToProxy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for ConnectionStateConnectingToProxy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateConnectingToProxy" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDConnectionState for ConnectionStateConnectingToProxy {}



impl ConnectionStateConnectingToProxy {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectionStateConnectingToProxyBuilder {
    let mut inner = ConnectionStateConnectingToProxy::default();
    inner.td_name = "connectionStateConnectingToProxy".to_string();
    RTDConnectionStateConnectingToProxyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDConnectionStateConnectingToProxyBuilder {
  inner: ConnectionStateConnectingToProxy
}

impl RTDConnectionStateConnectingToProxyBuilder {
  pub fn build(&self) -> ConnectionStateConnectingToProxy { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<ConnectionStateConnectingToProxy> for ConnectionStateConnectingToProxy {
  fn as_ref(&self) -> &ConnectionStateConnectingToProxy { self }
}

impl AsRef<ConnectionStateConnectingToProxy> for RTDConnectionStateConnectingToProxyBuilder {
  fn as_ref(&self) -> &ConnectionStateConnectingToProxy { &self.inner }
}







/// Currently establishing a connection to the Telegram servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateConnecting {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for ConnectionStateConnecting {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateConnecting" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDConnectionState for ConnectionStateConnecting {}



impl ConnectionStateConnecting {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectionStateConnectingBuilder {
    let mut inner = ConnectionStateConnecting::default();
    inner.td_name = "connectionStateConnecting".to_string();
    RTDConnectionStateConnectingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDConnectionStateConnectingBuilder {
  inner: ConnectionStateConnecting
}

impl RTDConnectionStateConnectingBuilder {
  pub fn build(&self) -> ConnectionStateConnecting { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<ConnectionStateConnecting> for ConnectionStateConnecting {
  fn as_ref(&self) -> &ConnectionStateConnecting { self }
}

impl AsRef<ConnectionStateConnecting> for RTDConnectionStateConnectingBuilder {
  fn as_ref(&self) -> &ConnectionStateConnecting { &self.inner }
}







/// Downloading data received while the client was offline
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateUpdating {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for ConnectionStateUpdating {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateUpdating" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDConnectionState for ConnectionStateUpdating {}



impl ConnectionStateUpdating {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectionStateUpdatingBuilder {
    let mut inner = ConnectionStateUpdating::default();
    inner.td_name = "connectionStateUpdating".to_string();
    RTDConnectionStateUpdatingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDConnectionStateUpdatingBuilder {
  inner: ConnectionStateUpdating
}

impl RTDConnectionStateUpdatingBuilder {
  pub fn build(&self) -> ConnectionStateUpdating { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<ConnectionStateUpdating> for ConnectionStateUpdating {
  fn as_ref(&self) -> &ConnectionStateUpdating { self }
}

impl AsRef<ConnectionStateUpdating> for RTDConnectionStateUpdatingBuilder {
  fn as_ref(&self) -> &ConnectionStateUpdating { &self.inner }
}







/// There is a working connection to the Telegram servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for ConnectionStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "connectionStateReady" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDConnectionState for ConnectionStateReady {}



impl ConnectionStateReady {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDConnectionStateReadyBuilder {
    let mut inner = ConnectionStateReady::default();
    inner.td_name = "connectionStateReady".to_string();
    RTDConnectionStateReadyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDConnectionStateReadyBuilder {
  inner: ConnectionStateReady
}

impl RTDConnectionStateReadyBuilder {
  pub fn build(&self) -> ConnectionStateReady { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<ConnectionStateReady> for ConnectionStateReady {
  fn as_ref(&self) -> &ConnectionStateReady { self }
}

impl AsRef<ConnectionStateReady> for RTDConnectionStateReadyBuilder {
  fn as_ref(&self) -> &ConnectionStateReady { &self.inner }
}



