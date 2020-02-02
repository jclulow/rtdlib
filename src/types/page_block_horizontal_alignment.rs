
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a horizontal alignment of a table cell content
pub trait TDPageBlockHorizontalAlignment: Debug + RObject {}

/// Describes a horizontal alignment of a table cell content
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageBlockHorizontalAlignment {
  #[doc(hidden)] _Default(()),
  /// The content should be left-aligned
  Left(PageBlockHorizontalAlignmentLeft),
  /// The content should be center-aligned
  Center(PageBlockHorizontalAlignmentCenter),
  /// The content should be right-aligned
  Right(PageBlockHorizontalAlignmentRight),

}

impl Default for PageBlockHorizontalAlignment {
  fn default() -> Self { PageBlockHorizontalAlignment::_Default(()) }
}

impl<'de> Deserialize<'de> for PageBlockHorizontalAlignment {
  fn deserialize<D>(deserializer: D) -> Result<PageBlockHorizontalAlignment, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PageBlockHorizontalAlignment,
      (pageBlockHorizontalAlignmentLeft, Left);
      (pageBlockHorizontalAlignmentCenter, Center);
      (pageBlockHorizontalAlignmentRight, Right);

    )(deserializer)
  }
}

impl RObject for PageBlockHorizontalAlignment {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PageBlockHorizontalAlignment::Left(t) => t.td_name(),
      PageBlockHorizontalAlignment::Center(t) => t.td_name(),
      PageBlockHorizontalAlignment::Right(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PageBlockHorizontalAlignment {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PageBlockHorizontalAlignment::_Default(_) = self { true } else { false } }

  pub fn is_left(&self) -> bool { if let PageBlockHorizontalAlignment::Left(_) = self { true } else { false } }
  pub fn is_center(&self) -> bool { if let PageBlockHorizontalAlignment::Center(_) = self { true } else { false } }
  pub fn is_right(&self) -> bool { if let PageBlockHorizontalAlignment::Right(_) = self { true } else { false } }

  pub fn on_left<F: FnOnce(&PageBlockHorizontalAlignmentLeft)>(&self, fnc: F) -> &Self { if let PageBlockHorizontalAlignment::Left(t) = self { fnc(t) }; self }
  pub fn on_center<F: FnOnce(&PageBlockHorizontalAlignmentCenter)>(&self, fnc: F) -> &Self { if let PageBlockHorizontalAlignment::Center(t) = self { fnc(t) }; self }
  pub fn on_right<F: FnOnce(&PageBlockHorizontalAlignmentRight)>(&self, fnc: F) -> &Self { if let PageBlockHorizontalAlignment::Right(t) = self { fnc(t) }; self }

  pub fn as_left(&self) -> Option<&PageBlockHorizontalAlignmentLeft> { if let PageBlockHorizontalAlignment::Left(t) = self { return Some(t) } None }
  pub fn as_center(&self) -> Option<&PageBlockHorizontalAlignmentCenter> { if let PageBlockHorizontalAlignment::Center(t) = self { return Some(t) } None }
  pub fn as_right(&self) -> Option<&PageBlockHorizontalAlignmentRight> { if let PageBlockHorizontalAlignment::Right(t) = self { return Some(t) } None }



  pub fn left<T: AsRef<PageBlockHorizontalAlignmentLeft>>(t: T) -> Self { PageBlockHorizontalAlignment::Left(t.as_ref().clone()) }

  pub fn center<T: AsRef<PageBlockHorizontalAlignmentCenter>>(t: T) -> Self { PageBlockHorizontalAlignment::Center(t.as_ref().clone()) }

  pub fn right<T: AsRef<PageBlockHorizontalAlignmentRight>>(t: T) -> Self { PageBlockHorizontalAlignment::Right(t.as_ref().clone()) }

}

impl AsRef<PageBlockHorizontalAlignment> for PageBlockHorizontalAlignment {
  fn as_ref(&self) -> &PageBlockHorizontalAlignment { self }
}







/// The content should be left-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for PageBlockHorizontalAlignmentLeft {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHorizontalAlignmentLeft" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentLeft {}



impl PageBlockHorizontalAlignmentLeft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockHorizontalAlignmentLeftBuilder {
    let mut inner = PageBlockHorizontalAlignmentLeft::default();
    inner.td_name = "pageBlockHorizontalAlignmentLeft".to_string();
    RTDPageBlockHorizontalAlignmentLeftBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPageBlockHorizontalAlignmentLeftBuilder {
  inner: PageBlockHorizontalAlignmentLeft
}

impl RTDPageBlockHorizontalAlignmentLeftBuilder {
  pub fn build(&self) -> PageBlockHorizontalAlignmentLeft { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<PageBlockHorizontalAlignmentLeft> for PageBlockHorizontalAlignmentLeft {
  fn as_ref(&self) -> &PageBlockHorizontalAlignmentLeft { self }
}

impl AsRef<PageBlockHorizontalAlignmentLeft> for RTDPageBlockHorizontalAlignmentLeftBuilder {
  fn as_ref(&self) -> &PageBlockHorizontalAlignmentLeft { &self.inner }
}







/// The content should be center-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentCenter {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for PageBlockHorizontalAlignmentCenter {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHorizontalAlignmentCenter" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentCenter {}



impl PageBlockHorizontalAlignmentCenter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockHorizontalAlignmentCenterBuilder {
    let mut inner = PageBlockHorizontalAlignmentCenter::default();
    inner.td_name = "pageBlockHorizontalAlignmentCenter".to_string();
    RTDPageBlockHorizontalAlignmentCenterBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPageBlockHorizontalAlignmentCenterBuilder {
  inner: PageBlockHorizontalAlignmentCenter
}

impl RTDPageBlockHorizontalAlignmentCenterBuilder {
  pub fn build(&self) -> PageBlockHorizontalAlignmentCenter { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<PageBlockHorizontalAlignmentCenter> for PageBlockHorizontalAlignmentCenter {
  fn as_ref(&self) -> &PageBlockHorizontalAlignmentCenter { self }
}

impl AsRef<PageBlockHorizontalAlignmentCenter> for RTDPageBlockHorizontalAlignmentCenterBuilder {
  fn as_ref(&self) -> &PageBlockHorizontalAlignmentCenter { &self.inner }
}







/// The content should be right-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentRight {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for PageBlockHorizontalAlignmentRight {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockHorizontalAlignmentRight" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentRight {}



impl PageBlockHorizontalAlignmentRight {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockHorizontalAlignmentRightBuilder {
    let mut inner = PageBlockHorizontalAlignmentRight::default();
    inner.td_name = "pageBlockHorizontalAlignmentRight".to_string();
    RTDPageBlockHorizontalAlignmentRightBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPageBlockHorizontalAlignmentRightBuilder {
  inner: PageBlockHorizontalAlignmentRight
}

impl RTDPageBlockHorizontalAlignmentRightBuilder {
  pub fn build(&self) -> PageBlockHorizontalAlignmentRight { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<PageBlockHorizontalAlignmentRight> for PageBlockHorizontalAlignmentRight {
  fn as_ref(&self) -> &PageBlockHorizontalAlignmentRight { self }
}

impl AsRef<PageBlockHorizontalAlignmentRight> for RTDPageBlockHorizontalAlignmentRightBuilder {
  fn as_ref(&self) -> &PageBlockHorizontalAlignmentRight { &self.inner }
}



