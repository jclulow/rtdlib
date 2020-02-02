
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidatedOrderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Temporary identifier of the order information
  order_info_id: String,
  /// Available shipping options
  shipping_options: Vec<ShippingOption>,
  
}

impl RObject for ValidatedOrderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "validatedOrderInfo" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ValidatedOrderInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDValidatedOrderInfoBuilder {
    let mut inner = ValidatedOrderInfo::default();
    inner.td_name = "validatedOrderInfo".to_string();
    RTDValidatedOrderInfoBuilder { inner }
  }

  pub fn order_info_id(&self) -> &String { &self.order_info_id }

  pub fn shipping_options(&self) -> &Vec<ShippingOption> { &self.shipping_options }

}

#[doc(hidden)]
pub struct RTDValidatedOrderInfoBuilder {
  inner: ValidatedOrderInfo
}

impl RTDValidatedOrderInfoBuilder {
  pub fn build(&self) -> ValidatedOrderInfo { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn order_info_id<T: AsRef<str>>(&mut self, order_info_id: T) -> &mut Self {
    self.inner.order_info_id = order_info_id.as_ref().to_string();
    self
  }

   
  pub fn shipping_options(&mut self, shipping_options: Vec<ShippingOption>) -> &mut Self {
    self.inner.shipping_options = shipping_options;
    self
  }

}

impl AsRef<ValidatedOrderInfo> for ValidatedOrderInfo {
  fn as_ref(&self) -> &ValidatedOrderInfo { self }
}

impl AsRef<ValidatedOrderInfo> for RTDValidatedOrderInfoBuilder {
  fn as_ref(&self) -> &ValidatedOrderInfo { &self.inner }
}



