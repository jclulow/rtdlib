
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents a list of proxy servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Proxies {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// List of proxy servers
  proxies: Vec<Proxy>,
  
}

impl RObject for Proxies {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "proxies" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Proxies {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDProxiesBuilder {
    let mut inner = Proxies::default();
    inner.td_name = "proxies".to_string();
    RTDProxiesBuilder { inner }
  }

  pub fn proxies(&self) -> &Vec<Proxy> { &self.proxies }

}

#[doc(hidden)]
pub struct RTDProxiesBuilder {
  inner: Proxies
}

impl RTDProxiesBuilder {
  pub fn build(&self) -> Proxies { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn proxies(&mut self, proxies: Vec<Proxy>) -> &mut Self {
    self.inner.proxies = proxies;
    self
  }

}

impl AsRef<Proxies> for Proxies {
  fn as_ref(&self) -> &Proxies { self }
}

impl AsRef<Proxies> for RTDProxiesBuilder {
  fn as_ref(&self) -> &Proxies { &self.inner }
}



