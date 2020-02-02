
use crate::types::*;
use crate::errors::*;




/// Represents a single button in an inline keyboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Text of the button
  text: String,
  /// Type of the button
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: InlineKeyboardButtonType,
  
}

impl RObject for InlineKeyboardButton {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButton" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl InlineKeyboardButton {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineKeyboardButtonBuilder {
    let mut inner = InlineKeyboardButton::default();
    inner.td_name = "inlineKeyboardButton".to_string();
    RTDInlineKeyboardButtonBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

  pub fn type_(&self) -> &InlineKeyboardButtonType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonBuilder {
  inner: InlineKeyboardButton
}

impl RTDInlineKeyboardButtonBuilder {
  pub fn build(&self) -> InlineKeyboardButton { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

   
  pub fn type_<T: AsRef<InlineKeyboardButtonType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<InlineKeyboardButton> for InlineKeyboardButton {
  fn as_ref(&self) -> &InlineKeyboardButton { self }
}

impl AsRef<InlineKeyboardButton> for RTDInlineKeyboardButtonBuilder {
  fn as_ref(&self) -> &InlineKeyboardButton { &self.inner }
}



