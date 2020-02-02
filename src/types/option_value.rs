
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the value of an option
pub trait TDOptionValue: Debug + RObject {}

/// Represents the value of an option
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum OptionValue {
  #[doc(hidden)] _Default(()),
  /// Represents a boolean option
  Boolean(OptionValueBoolean),
  /// Represents an unknown option or an option which has a default value
  Empty(OptionValueEmpty),
  /// Represents an integer option
  Integer(OptionValueInteger),
  /// Represents a string option
  String(OptionValueString),
  /// Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
  GetOption(GetOption),

}

impl Default for OptionValue {
  fn default() -> Self { OptionValue::_Default(()) }
}

impl<'de> Deserialize<'de> for OptionValue {
  fn deserialize<D>(deserializer: D) -> Result<OptionValue, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      OptionValue,
      (optionValueBoolean, Boolean);
      (optionValueEmpty, Empty);
      (optionValueInteger, Integer);
      (optionValueString, String);
      (getOption, GetOption);

    )(deserializer)
  }
}

impl RObject for OptionValue {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      OptionValue::Boolean(t) => t.td_name(),
      OptionValue::Empty(t) => t.td_name(),
      OptionValue::Integer(t) => t.td_name(),
      OptionValue::String(t) => t.td_name(),
      OptionValue::GetOption(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl OptionValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let OptionValue::_Default(_) = self { true } else { false } }

  pub fn is_boolean(&self) -> bool { if let OptionValue::Boolean(_) = self { true } else { false } }
  pub fn is_empty(&self) -> bool { if let OptionValue::Empty(_) = self { true } else { false } }
  pub fn is_integer(&self) -> bool { if let OptionValue::Integer(_) = self { true } else { false } }
  pub fn is_string(&self) -> bool { if let OptionValue::String(_) = self { true } else { false } }
  pub fn is_get_option(&self) -> bool { if let OptionValue::GetOption(_) = self { true } else { false } }

  pub fn on_boolean<F: FnOnce(&OptionValueBoolean)>(&self, fnc: F) -> &Self { if let OptionValue::Boolean(t) = self { fnc(t) }; self }
  pub fn on_empty<F: FnOnce(&OptionValueEmpty)>(&self, fnc: F) -> &Self { if let OptionValue::Empty(t) = self { fnc(t) }; self }
  pub fn on_integer<F: FnOnce(&OptionValueInteger)>(&self, fnc: F) -> &Self { if let OptionValue::Integer(t) = self { fnc(t) }; self }
  pub fn on_string<F: FnOnce(&OptionValueString)>(&self, fnc: F) -> &Self { if let OptionValue::String(t) = self { fnc(t) }; self }
  pub fn on_get_option<F: FnOnce(&GetOption)>(&self, fnc: F) -> &Self { if let OptionValue::GetOption(t) = self { fnc(t) }; self }

  pub fn as_boolean(&self) -> Option<&OptionValueBoolean> { if let OptionValue::Boolean(t) = self { return Some(t) } None }
  pub fn as_empty(&self) -> Option<&OptionValueEmpty> { if let OptionValue::Empty(t) = self { return Some(t) } None }
  pub fn as_integer(&self) -> Option<&OptionValueInteger> { if let OptionValue::Integer(t) = self { return Some(t) } None }
  pub fn as_string(&self) -> Option<&OptionValueString> { if let OptionValue::String(t) = self { return Some(t) } None }
  pub fn as_get_option(&self) -> Option<&GetOption> { if let OptionValue::GetOption(t) = self { return Some(t) } None }



  pub fn boolean<T: AsRef<OptionValueBoolean>>(t: T) -> Self { OptionValue::Boolean(t.as_ref().clone()) }

  pub fn empty<T: AsRef<OptionValueEmpty>>(t: T) -> Self { OptionValue::Empty(t.as_ref().clone()) }

  pub fn integer<T: AsRef<OptionValueInteger>>(t: T) -> Self { OptionValue::Integer(t.as_ref().clone()) }

  pub fn string<T: AsRef<OptionValueString>>(t: T) -> Self { OptionValue::String(t.as_ref().clone()) }

  pub fn get_option<T: AsRef<GetOption>>(t: T) -> Self { OptionValue::GetOption(t.as_ref().clone()) }

}

impl AsRef<OptionValue> for OptionValue {
  fn as_ref(&self) -> &OptionValue { self }
}







/// Represents a boolean option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueBoolean {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// The value of the option
  value: bool,
  
}

impl RObject for OptionValueBoolean {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueBoolean" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDOptionValue for OptionValueBoolean {}



impl OptionValueBoolean {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOptionValueBooleanBuilder {
    let mut inner = OptionValueBoolean::default();
    inner.td_name = "optionValueBoolean".to_string();
    RTDOptionValueBooleanBuilder { inner }
  }

  pub fn value(&self) -> bool { self.value }

}

#[doc(hidden)]
pub struct RTDOptionValueBooleanBuilder {
  inner: OptionValueBoolean
}

impl RTDOptionValueBooleanBuilder {
  pub fn build(&self) -> OptionValueBoolean { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn value(&mut self, value: bool) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<OptionValueBoolean> for OptionValueBoolean {
  fn as_ref(&self) -> &OptionValueBoolean { self }
}

impl AsRef<OptionValueBoolean> for RTDOptionValueBooleanBuilder {
  fn as_ref(&self) -> &OptionValueBoolean { &self.inner }
}







/// Represents an unknown option or an option which has a default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  
}

impl RObject for OptionValueEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueEmpty" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDOptionValue for OptionValueEmpty {}



impl OptionValueEmpty {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOptionValueEmptyBuilder {
    let mut inner = OptionValueEmpty::default();
    inner.td_name = "optionValueEmpty".to_string();
    RTDOptionValueEmptyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDOptionValueEmptyBuilder {
  inner: OptionValueEmpty
}

impl RTDOptionValueEmptyBuilder {
  pub fn build(&self) -> OptionValueEmpty { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

}

impl AsRef<OptionValueEmpty> for OptionValueEmpty {
  fn as_ref(&self) -> &OptionValueEmpty { self }
}

impl AsRef<OptionValueEmpty> for RTDOptionValueEmptyBuilder {
  fn as_ref(&self) -> &OptionValueEmpty { &self.inner }
}







/// Represents an integer option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueInteger {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// The value of the option
  value: i64,
  
}

impl RObject for OptionValueInteger {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueInteger" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDOptionValue for OptionValueInteger {}



impl OptionValueInteger {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOptionValueIntegerBuilder {
    let mut inner = OptionValueInteger::default();
    inner.td_name = "optionValueInteger".to_string();
    RTDOptionValueIntegerBuilder { inner }
  }

  pub fn value(&self) -> i64 { self.value }

}

#[doc(hidden)]
pub struct RTDOptionValueIntegerBuilder {
  inner: OptionValueInteger
}

impl RTDOptionValueIntegerBuilder {
  pub fn build(&self) -> OptionValueInteger { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn value(&mut self, value: i64) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<OptionValueInteger> for OptionValueInteger {
  fn as_ref(&self) -> &OptionValueInteger { self }
}

impl AsRef<OptionValueInteger> for RTDOptionValueIntegerBuilder {
  fn as_ref(&self) -> &OptionValueInteger { &self.inner }
}







/// Represents a string option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptionValueString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// The value of the option
  value: String,
  
}

impl RObject for OptionValueString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "optionValueString" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDOptionValue for OptionValueString {}



impl OptionValueString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDOptionValueStringBuilder {
    let mut inner = OptionValueString::default();
    inner.td_name = "optionValueString".to_string();
    RTDOptionValueStringBuilder { inner }
  }

  pub fn value(&self) -> &String { &self.value }

}

#[doc(hidden)]
pub struct RTDOptionValueStringBuilder {
  inner: OptionValueString
}

impl RTDOptionValueStringBuilder {
  pub fn build(&self) -> OptionValueString { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }

}

impl AsRef<OptionValueString> for OptionValueString {
  fn as_ref(&self) -> &OptionValueString { self }
}

impl AsRef<OptionValueString> for RTDOptionValueStringBuilder {
  fn as_ref(&self) -> &OptionValueString { &self.inner }
}



