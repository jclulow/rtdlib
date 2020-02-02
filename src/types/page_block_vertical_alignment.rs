
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a Vertical alignment of a table cell content
pub trait TDPageBlockVerticalAlignment: Debug + RObject {}

/// Describes a Vertical alignment of a table cell content
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageBlockVerticalAlignment {
  #[doc(hidden)] _Default(()),
  /// The content should be top-aligned
  Top(PageBlockVerticalAlignmentTop),
  /// The content should be middle-aligned
  Middle(PageBlockVerticalAlignmentMiddle),
  /// The content should be bottom-aligned
  Bottom(PageBlockVerticalAlignmentBottom),

}

impl Default for PageBlockVerticalAlignment {
  fn default() -> Self { PageBlockVerticalAlignment::_Default(()) }
}

impl<'de> Deserialize<'de> for PageBlockVerticalAlignment {
  fn deserialize<D>(deserializer: D) -> Result<PageBlockVerticalAlignment, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PageBlockVerticalAlignment,
      (pageBlockVerticalAlignmentTop, Top);
      (pageBlockVerticalAlignmentMiddle, Middle);
      (pageBlockVerticalAlignmentBottom, Bottom);

    )(deserializer)
  }
}

impl RObject for PageBlockVerticalAlignment {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      PageBlockVerticalAlignment::Top(t) => t.td_name(),
      PageBlockVerticalAlignment::Middle(t) => t.td_name(),
      PageBlockVerticalAlignment::Bottom(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl PageBlockVerticalAlignment {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let PageBlockVerticalAlignment::_Default(_) = self { true } else { false } }

  pub fn is_top(&self) -> bool { if let PageBlockVerticalAlignment::Top(_) = self { true } else { false } }
  pub fn is_middle(&self) -> bool { if let PageBlockVerticalAlignment::Middle(_) = self { true } else { false } }
  pub fn is_bottom(&self) -> bool { if let PageBlockVerticalAlignment::Bottom(_) = self { true } else { false } }

  pub fn on_top<F: FnOnce(&PageBlockVerticalAlignmentTop)>(&self, fnc: F) -> &Self { if let PageBlockVerticalAlignment::Top(t) = self { fnc(t) }; self }
  pub fn on_middle<F: FnOnce(&PageBlockVerticalAlignmentMiddle)>(&self, fnc: F) -> &Self { if let PageBlockVerticalAlignment::Middle(t) = self { fnc(t) }; self }
  pub fn on_bottom<F: FnOnce(&PageBlockVerticalAlignmentBottom)>(&self, fnc: F) -> &Self { if let PageBlockVerticalAlignment::Bottom(t) = self { fnc(t) }; self }

  pub fn as_top(&self) -> Option<&PageBlockVerticalAlignmentTop> { if let PageBlockVerticalAlignment::Top(t) = self { return Some(t) } None }
  pub fn as_middle(&self) -> Option<&PageBlockVerticalAlignmentMiddle> { if let PageBlockVerticalAlignment::Middle(t) = self { return Some(t) } None }
  pub fn as_bottom(&self) -> Option<&PageBlockVerticalAlignmentBottom> { if let PageBlockVerticalAlignment::Bottom(t) = self { return Some(t) } None }



  pub fn top<T: AsRef<PageBlockVerticalAlignmentTop>>(t: T) -> Self { PageBlockVerticalAlignment::Top(t.as_ref().clone()) }

  pub fn middle<T: AsRef<PageBlockVerticalAlignmentMiddle>>(t: T) -> Self { PageBlockVerticalAlignment::Middle(t.as_ref().clone()) }

  pub fn bottom<T: AsRef<PageBlockVerticalAlignmentBottom>>(t: T) -> Self { PageBlockVerticalAlignment::Bottom(t.as_ref().clone()) }

}

impl AsRef<PageBlockVerticalAlignment> for PageBlockVerticalAlignment {
  fn as_ref(&self) -> &PageBlockVerticalAlignment { self }
}







/// The content should be top-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentTop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for PageBlockVerticalAlignmentTop {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVerticalAlignmentTop" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentTop {}



impl PageBlockVerticalAlignmentTop {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockVerticalAlignmentTopBuilder {
    let mut inner = PageBlockVerticalAlignmentTop::default();
    inner.td_name = "pageBlockVerticalAlignmentTop".to_string();
    RTDPageBlockVerticalAlignmentTopBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPageBlockVerticalAlignmentTopBuilder {
  inner: PageBlockVerticalAlignmentTop
}

impl RTDPageBlockVerticalAlignmentTopBuilder {
  pub fn build(&self) -> PageBlockVerticalAlignmentTop { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<PageBlockVerticalAlignmentTop> for PageBlockVerticalAlignmentTop {
  fn as_ref(&self) -> &PageBlockVerticalAlignmentTop { self }
}

impl AsRef<PageBlockVerticalAlignmentTop> for RTDPageBlockVerticalAlignmentTopBuilder {
  fn as_ref(&self) -> &PageBlockVerticalAlignmentTop { &self.inner }
}







/// The content should be middle-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentMiddle {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for PageBlockVerticalAlignmentMiddle {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVerticalAlignmentMiddle" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentMiddle {}



impl PageBlockVerticalAlignmentMiddle {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockVerticalAlignmentMiddleBuilder {
    let mut inner = PageBlockVerticalAlignmentMiddle::default();
    inner.td_name = "pageBlockVerticalAlignmentMiddle".to_string();
    RTDPageBlockVerticalAlignmentMiddleBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPageBlockVerticalAlignmentMiddleBuilder {
  inner: PageBlockVerticalAlignmentMiddle
}

impl RTDPageBlockVerticalAlignmentMiddleBuilder {
  pub fn build(&self) -> PageBlockVerticalAlignmentMiddle { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<PageBlockVerticalAlignmentMiddle> for PageBlockVerticalAlignmentMiddle {
  fn as_ref(&self) -> &PageBlockVerticalAlignmentMiddle { self }
}

impl AsRef<PageBlockVerticalAlignmentMiddle> for RTDPageBlockVerticalAlignmentMiddleBuilder {
  fn as_ref(&self) -> &PageBlockVerticalAlignmentMiddle { &self.inner }
}







/// The content should be bottom-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentBottom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for PageBlockVerticalAlignmentBottom {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockVerticalAlignmentBottom" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentBottom {}



impl PageBlockVerticalAlignmentBottom {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockVerticalAlignmentBottomBuilder {
    let mut inner = PageBlockVerticalAlignmentBottom::default();
    inner.td_name = "pageBlockVerticalAlignmentBottom".to_string();
    RTDPageBlockVerticalAlignmentBottomBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDPageBlockVerticalAlignmentBottomBuilder {
  inner: PageBlockVerticalAlignmentBottom
}

impl RTDPageBlockVerticalAlignmentBottomBuilder {
  pub fn build(&self) -> PageBlockVerticalAlignmentBottom { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<PageBlockVerticalAlignmentBottom> for PageBlockVerticalAlignmentBottom {
  fn as_ref(&self) -> &PageBlockVerticalAlignmentBottom { self }
}

impl AsRef<PageBlockVerticalAlignmentBottom> for RTDPageBlockVerticalAlignmentBottomBuilder {
  fn as_ref(&self) -> &PageBlockVerticalAlignmentBottom { &self.inner }
}



