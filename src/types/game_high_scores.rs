
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a list of game high scores
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameHighScores {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// A list of game high scores
  scores: Vec<GameHighScore>,
  
}

impl RObject for GameHighScores {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "gameHighScores" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl GameHighScores {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDGameHighScoresBuilder {
    let mut inner = GameHighScores::default();
    inner.td_name = "gameHighScores".to_string();
    RTDGameHighScoresBuilder { inner }
  }

  pub fn scores(&self) -> &Vec<GameHighScore> { &self.scores }

}

#[doc(hidden)]
pub struct RTDGameHighScoresBuilder {
  inner: GameHighScores
}

impl RTDGameHighScoresBuilder {
  pub fn build(&self) -> GameHighScores { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn scores(&mut self, scores: Vec<GameHighScore>) -> &mut Self {
    self.inner.scores = scores;
    self
  }

}

impl AsRef<GameHighScores> for GameHighScores {
  fn as_ref(&self) -> &GameHighScores { self }
}

impl AsRef<GameHighScores> for RTDGameHighScoresBuilder {
  fn as_ref(&self) -> &GameHighScores { &self.inner }
}



