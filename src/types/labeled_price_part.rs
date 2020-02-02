
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Portion of the price of a product (e.g., "delivery cost", "tax amount")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LabeledPricePart {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Label for this portion of the product price
  label: String,
  /// Currency amount in minimal quantity of the currency
  amount: i64,
  
}

impl RObject for LabeledPricePart {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "labeledPricePart" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LabeledPricePart {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLabeledPricePartBuilder {
    let mut inner = LabeledPricePart::default();
    inner.td_name = "labeledPricePart".to_string();
    RTDLabeledPricePartBuilder { inner }
  }

  pub fn label(&self) -> &String { &self.label }

  pub fn amount(&self) -> i64 { self.amount }

}

#[doc(hidden)]
pub struct RTDLabeledPricePartBuilder {
  inner: LabeledPricePart
}

impl RTDLabeledPricePartBuilder {
  pub fn build(&self) -> LabeledPricePart { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn label<T: AsRef<str>>(&mut self, label: T) -> &mut Self {
    self.inner.label = label.as_ref().to_string();
    self
  }

   
  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.amount = amount;
    self
  }

}

impl AsRef<LabeledPricePart> for LabeledPricePart {
  fn as_ref(&self) -> &LabeledPricePart { self }
}

impl AsRef<LabeledPricePart> for RTDLabeledPricePartBuilder {
  fn as_ref(&self) -> &LabeledPricePart { &self.inner }
}



