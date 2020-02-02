
use crate::types::*;
use crate::errors::*;




/// Contains the response of a request to TON lite server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonLiteServerResponse {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// The response
  response: String,
  
}

impl RObject for TonLiteServerResponse {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tonLiteServerResponse" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TonLiteServerResponse {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTonLiteServerResponseBuilder {
    let mut inner = TonLiteServerResponse::default();
    inner.td_name = "tonLiteServerResponse".to_string();
    RTDTonLiteServerResponseBuilder { inner }
  }

  pub fn response(&self) -> &String { &self.response }

}

#[doc(hidden)]
pub struct RTDTonLiteServerResponseBuilder {
  inner: TonLiteServerResponse
}

impl RTDTonLiteServerResponseBuilder {
  pub fn build(&self) -> TonLiteServerResponse { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn response<T: AsRef<str>>(&mut self, response: T) -> &mut Self {
    self.inner.response = response.as_ref().to_string();
    self
  }

}

impl AsRef<TonLiteServerResponse> for TonLiteServerResponse {
  fn as_ref(&self) -> &TonLiteServerResponse { self }
}

impl AsRef<TonLiteServerResponse> for RTDTonLiteServerResponseBuilder {
  fn as_ref(&self) -> &TonLiteServerResponse { &self.inner }
}



