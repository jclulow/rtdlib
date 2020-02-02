
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains an HTTP URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The URL
  url: String,
  
}

impl RObject for HttpUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "httpUrl" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl HttpUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDHttpUrlBuilder {
    let mut inner = HttpUrl::default();
    inner.td_name = "httpUrl".to_string();
    RTDHttpUrlBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDHttpUrlBuilder {
  inner: HttpUrl
}

impl RTDHttpUrlBuilder {
  pub fn build(&self) -> HttpUrl { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<HttpUrl> for HttpUrl {
  fn as_ref(&self) -> &HttpUrl { self }
}

impl AsRef<HttpUrl> for RTDHttpUrlBuilder {
  fn as_ref(&self) -> &HttpUrl { &self.inner }
}



