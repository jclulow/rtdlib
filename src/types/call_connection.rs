
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Describes the address of UDP reflectors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallConnection {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Reflector identifier
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")] id: i64,
  /// IPv4 reflector address
  ip: String,
  /// IPv6 reflector address
  ipv6: String,
  /// Reflector port number
  port: i64,
  /// Connection peer tag
  peer_tag: String,
  
}

impl RObject for CallConnection {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callConnection" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl CallConnection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallConnectionBuilder {
    let mut inner = CallConnection::default();
    inner.td_name = "callConnection".to_string();
    RTDCallConnectionBuilder { inner }
  }

  pub fn id(&self) -> i64 { self.id }

  pub fn ip(&self) -> &String { &self.ip }

  pub fn ipv6(&self) -> &String { &self.ipv6 }

  pub fn port(&self) -> i64 { self.port }

  pub fn peer_tag(&self) -> &String { &self.peer_tag }

}

#[doc(hidden)]
pub struct RTDCallConnectionBuilder {
  inner: CallConnection
}

impl RTDCallConnectionBuilder {
  pub fn build(&self) -> CallConnection { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn ip<T: AsRef<str>>(&mut self, ip: T) -> &mut Self {
    self.inner.ip = ip.as_ref().to_string();
    self
  }

   
  pub fn ipv6<T: AsRef<str>>(&mut self, ipv6: T) -> &mut Self {
    self.inner.ipv6 = ipv6.as_ref().to_string();
    self
  }

   
  pub fn port(&mut self, port: i64) -> &mut Self {
    self.inner.port = port;
    self
  }

   
  pub fn peer_tag<T: AsRef<str>>(&mut self, peer_tag: T) -> &mut Self {
    self.inner.peer_tag = peer_tag.as_ref().to_string();
    self
  }

}

impl AsRef<CallConnection> for CallConnection {
  fn as_ref(&self) -> &CallConnection { self }
}

impl AsRef<CallConnection> for RTDCallConnectionBuilder {
  fn as_ref(&self) -> &CallConnection { &self.inner }
}



