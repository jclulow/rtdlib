
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains the salt to be used with locally stored password to access a local TON-based wallet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletPasswordSalt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The salt
  salt: String,
  
}

impl RObject for TonWalletPasswordSalt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tonWalletPasswordSalt" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TonWalletPasswordSalt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTonWalletPasswordSaltBuilder {
    let mut inner = TonWalletPasswordSalt::default();
    inner.td_name = "tonWalletPasswordSalt".to_string();
    RTDTonWalletPasswordSaltBuilder { inner }
  }

  pub fn salt(&self) -> &String { &self.salt }

}

#[doc(hidden)]
pub struct RTDTonWalletPasswordSaltBuilder {
  inner: TonWalletPasswordSalt
}

impl RTDTonWalletPasswordSaltBuilder {
  pub fn build(&self) -> TonWalletPasswordSalt { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn salt<T: AsRef<str>>(&mut self, salt: T) -> &mut Self {
    self.inner.salt = salt.as_ref().to_string();
    self
  }

}

impl AsRef<TonWalletPasswordSalt> for TonWalletPasswordSalt {
  fn as_ref(&self) -> &TonWalletPasswordSalt { self }
}

impl AsRef<TonWalletPasswordSalt> for RTDTonWalletPasswordSaltBuilder {
  fn as_ref(&self) -> &TonWalletPasswordSalt { &self.inner }
}



