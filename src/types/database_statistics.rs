
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains database statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatabaseStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Database statistics in an unspecified human-readable format
  statistics: String,
  
}

impl RObject for DatabaseStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "databaseStatistics" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl DatabaseStatistics {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDDatabaseStatisticsBuilder {
    let mut inner = DatabaseStatistics::default();
    inner.td_name = "databaseStatistics".to_string();
    RTDDatabaseStatisticsBuilder { inner }
  }

  pub fn statistics(&self) -> &String { &self.statistics }

}

#[doc(hidden)]
pub struct RTDDatabaseStatisticsBuilder {
  inner: DatabaseStatistics
}

impl RTDDatabaseStatisticsBuilder {
  pub fn build(&self) -> DatabaseStatistics { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn statistics<T: AsRef<str>>(&mut self, statistics: T) -> &mut Self {
    self.inner.statistics = statistics.as_ref().to_string();
    self
  }

}

impl AsRef<DatabaseStatistics> for DatabaseStatistics {
  fn as_ref(&self) -> &DatabaseStatistics { self }
}

impl AsRef<DatabaseStatistics> for RTDDatabaseStatisticsBuilder {
  fn as_ref(&self) -> &DatabaseStatistics { &self.inner }
}



