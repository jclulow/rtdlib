
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Describes a location on planet Earth
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Latitude of the location in degrees; as defined by the sender
  latitude: f32,
  /// Longitude of the location, in degrees; as defined by the sender
  longitude: f32,
  
}

impl RObject for Location {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "location" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Location {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLocationBuilder {
    let mut inner = Location::default();
    inner.td_name = "location".to_string();
    RTDLocationBuilder { inner }
  }

  pub fn latitude(&self) -> f32 { self.latitude }

  pub fn longitude(&self) -> f32 { self.longitude }

}

#[doc(hidden)]
pub struct RTDLocationBuilder {
  inner: Location
}

impl RTDLocationBuilder {
  pub fn build(&self) -> Location { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn latitude(&mut self, latitude: f32) -> &mut Self {
    self.inner.latitude = latitude;
    self
  }

   
  pub fn longitude(&mut self, longitude: f32) -> &mut Self {
    self.inner.longitude = longitude;
    self
  }

}

impl AsRef<Location> for Location {
  fn as_ref(&self) -> &Location { self }
}

impl AsRef<Location> for RTDLocationBuilder {
  fn as_ref(&self) -> &Location { &self.inner }
}



