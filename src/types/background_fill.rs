
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a fill of a background
pub trait TDBackgroundFill: Debug + RObject {}

/// Describes a fill of a background
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BackgroundFill {
  #[doc(hidden)] _Default(()),
  /// Describes a solid fill of a background
  Solid(BackgroundFillSolid),
  /// Describes a gradient fill of a background
  Gradient(BackgroundFillGradient),

}

impl Default for BackgroundFill {
  fn default() -> Self { BackgroundFill::_Default(()) }
}

impl<'de> Deserialize<'de> for BackgroundFill {
  fn deserialize<D>(deserializer: D) -> Result<BackgroundFill, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BackgroundFill,
      (backgroundFillSolid, Solid);
      (backgroundFillGradient, Gradient);

    )(deserializer)
  }
}

impl RObject for BackgroundFill {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      BackgroundFill::Solid(t) => t.td_name(),
      BackgroundFill::Gradient(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl BackgroundFill {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let BackgroundFill::_Default(_) = self { true } else { false } }

  pub fn is_solid(&self) -> bool { if let BackgroundFill::Solid(_) = self { true } else { false } }
  pub fn is_gradient(&self) -> bool { if let BackgroundFill::Gradient(_) = self { true } else { false } }

  pub fn on_solid<F: FnOnce(&BackgroundFillSolid)>(&self, fnc: F) -> &Self { if let BackgroundFill::Solid(t) = self { fnc(t) }; self }
  pub fn on_gradient<F: FnOnce(&BackgroundFillGradient)>(&self, fnc: F) -> &Self { if let BackgroundFill::Gradient(t) = self { fnc(t) }; self }

  pub fn as_solid(&self) -> Option<&BackgroundFillSolid> { if let BackgroundFill::Solid(t) = self { return Some(t) } None }
  pub fn as_gradient(&self) -> Option<&BackgroundFillGradient> { if let BackgroundFill::Gradient(t) = self { return Some(t) } None }



  pub fn solid<T: AsRef<BackgroundFillSolid>>(t: T) -> Self { BackgroundFill::Solid(t.as_ref().clone()) }

  pub fn gradient<T: AsRef<BackgroundFillGradient>>(t: T) -> Self { BackgroundFill::Gradient(t.as_ref().clone()) }

}

impl AsRef<BackgroundFill> for BackgroundFill {
  fn as_ref(&self) -> &BackgroundFill { self }
}







/// Describes a solid fill of a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundFillSolid {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// A color of the background in the RGB24 format
  color: i64,
  
}

impl RObject for BackgroundFillSolid {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundFillSolid" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundFill for BackgroundFillSolid {}



impl BackgroundFillSolid {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundFillSolidBuilder {
    let mut inner = BackgroundFillSolid::default();
    inner.td_name = "backgroundFillSolid".to_string();
    RTDBackgroundFillSolidBuilder { inner }
  }

  pub fn color(&self) -> i64 { self.color }

}

#[doc(hidden)]
pub struct RTDBackgroundFillSolidBuilder {
  inner: BackgroundFillSolid
}

impl RTDBackgroundFillSolidBuilder {
  pub fn build(&self) -> BackgroundFillSolid { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn color(&mut self, color: i64) -> &mut Self {
    self.inner.color = color;
    self
  }

}

impl AsRef<BackgroundFillSolid> for BackgroundFillSolid {
  fn as_ref(&self) -> &BackgroundFillSolid { self }
}

impl AsRef<BackgroundFillSolid> for RTDBackgroundFillSolidBuilder {
  fn as_ref(&self) -> &BackgroundFillSolid { &self.inner }
}







/// Describes a gradient fill of a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundFillGradient {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// A top color of the background in the RGB24 format
  top_color: i64,
  /// A bottom color of the background in the RGB24 format
  bottom_color: i64,
  /// Clockwise rotation angle of the gradient, in degrees; 0-359. Should be always divisible by 45
  rotation_angle: i64,
  
}

impl RObject for BackgroundFillGradient {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundFillGradient" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundFill for BackgroundFillGradient {}



impl BackgroundFillGradient {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundFillGradientBuilder {
    let mut inner = BackgroundFillGradient::default();
    inner.td_name = "backgroundFillGradient".to_string();
    RTDBackgroundFillGradientBuilder { inner }
  }

  pub fn top_color(&self) -> i64 { self.top_color }

  pub fn bottom_color(&self) -> i64 { self.bottom_color }

  pub fn rotation_angle(&self) -> i64 { self.rotation_angle }

}

#[doc(hidden)]
pub struct RTDBackgroundFillGradientBuilder {
  inner: BackgroundFillGradient
}

impl RTDBackgroundFillGradientBuilder {
  pub fn build(&self) -> BackgroundFillGradient { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn top_color(&mut self, top_color: i64) -> &mut Self {
    self.inner.top_color = top_color;
    self
  }

   
  pub fn bottom_color(&mut self, bottom_color: i64) -> &mut Self {
    self.inner.bottom_color = bottom_color;
    self
  }

   
  pub fn rotation_angle(&mut self, rotation_angle: i64) -> &mut Self {
    self.inner.rotation_angle = rotation_angle;
    self
  }

}

impl AsRef<BackgroundFillGradient> for BackgroundFillGradient {
  fn as_ref(&self) -> &BackgroundFillGradient { self }
}

impl AsRef<BackgroundFillGradient> for RTDBackgroundFillGradientBuilder {
  fn as_ref(&self) -> &BackgroundFillGradient { &self.inner }
}



