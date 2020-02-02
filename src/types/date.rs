
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Represents a date according to the Gregorian calendar
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Date {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Day of the month, 1-31
  day: i64,
  /// Month, 1-12
  month: i64,
  /// Year, 1-9999
  year: i64,
  
}

impl RObject for Date {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "date" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Date {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDateBuilder {
    let mut inner = Date::default();
    inner.td_name = "date".to_string();
    RTDDateBuilder { inner }
  }

  pub fn day(&self) -> i64 { self.day }

  pub fn month(&self) -> i64 { self.month }

  pub fn year(&self) -> i64 { self.year }

}

#[doc(hidden)]
pub struct RTDDateBuilder {
  inner: Date
}

impl RTDDateBuilder {
  pub fn build(&self) -> Date { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn day(&mut self, day: i64) -> &mut Self {
    self.inner.day = day;
    self
  }

   
  pub fn month(&mut self, month: i64) -> &mut Self {
    self.inner.month = month;
    self
  }

   
  pub fn year(&mut self, year: i64) -> &mut Self {
    self.inner.year = year;
    self
  }

}

impl AsRef<Date> for Date {
  fn as_ref(&self) -> &Date { self }
}

impl AsRef<Date> for RTDDateBuilder {
  fn as_ref(&self) -> &Date { &self.inner }
}



