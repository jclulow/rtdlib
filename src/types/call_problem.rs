
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the exact type of a problem with a call
pub trait TDCallProblem: Debug + RObject {}

/// Describes the exact type of a problem with a call
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallProblem {
  #[doc(hidden)] _Default(()),
  /// The user heard their own voice
  Echo(CallProblemEcho),
  /// The user heard background noise
  Noise(CallProblemNoise),
  /// The other side kept disappearing
  Interruptions(CallProblemInterruptions),
  /// The speech was distorted
  DistortedSpeech(CallProblemDistortedSpeech),
  /// The user couldn't hear the other side
  SilentLocal(CallProblemSilentLocal),
  /// The other side couldn't hear the user
  SilentRemote(CallProblemSilentRemote),
  /// The call ended unexpectedly
  Dropped(CallProblemDropped),

}

impl Default for CallProblem {
  fn default() -> Self { CallProblem::_Default(()) }
}

impl<'de> Deserialize<'de> for CallProblem {
  fn deserialize<D>(deserializer: D) -> Result<CallProblem, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      CallProblem,
      (callProblemEcho, Echo);
      (callProblemNoise, Noise);
      (callProblemInterruptions, Interruptions);
      (callProblemDistortedSpeech, DistortedSpeech);
      (callProblemSilentLocal, SilentLocal);
      (callProblemSilentRemote, SilentRemote);
      (callProblemDropped, Dropped);

    )(deserializer)
  }
}

impl RObject for CallProblem {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      CallProblem::Echo(t) => t.td_name(),
      CallProblem::Noise(t) => t.td_name(),
      CallProblem::Interruptions(t) => t.td_name(),
      CallProblem::DistortedSpeech(t) => t.td_name(),
      CallProblem::SilentLocal(t) => t.td_name(),
      CallProblem::SilentRemote(t) => t.td_name(),
      CallProblem::Dropped(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl CallProblem {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let CallProblem::_Default(_) = self { true } else { false } }

  pub fn is_echo(&self) -> bool { if let CallProblem::Echo(_) = self { true } else { false } }
  pub fn is_noise(&self) -> bool { if let CallProblem::Noise(_) = self { true } else { false } }
  pub fn is_interruptions(&self) -> bool { if let CallProblem::Interruptions(_) = self { true } else { false } }
  pub fn is_distorted_speech(&self) -> bool { if let CallProblem::DistortedSpeech(_) = self { true } else { false } }
  pub fn is_silent_local(&self) -> bool { if let CallProblem::SilentLocal(_) = self { true } else { false } }
  pub fn is_silent_remote(&self) -> bool { if let CallProblem::SilentRemote(_) = self { true } else { false } }
  pub fn is_dropped(&self) -> bool { if let CallProblem::Dropped(_) = self { true } else { false } }

  pub fn on_echo<F: FnOnce(&CallProblemEcho)>(&self, fnc: F) -> &Self { if let CallProblem::Echo(t) = self { fnc(t) }; self }
  pub fn on_noise<F: FnOnce(&CallProblemNoise)>(&self, fnc: F) -> &Self { if let CallProblem::Noise(t) = self { fnc(t) }; self }
  pub fn on_interruptions<F: FnOnce(&CallProblemInterruptions)>(&self, fnc: F) -> &Self { if let CallProblem::Interruptions(t) = self { fnc(t) }; self }
  pub fn on_distorted_speech<F: FnOnce(&CallProblemDistortedSpeech)>(&self, fnc: F) -> &Self { if let CallProblem::DistortedSpeech(t) = self { fnc(t) }; self }
  pub fn on_silent_local<F: FnOnce(&CallProblemSilentLocal)>(&self, fnc: F) -> &Self { if let CallProblem::SilentLocal(t) = self { fnc(t) }; self }
  pub fn on_silent_remote<F: FnOnce(&CallProblemSilentRemote)>(&self, fnc: F) -> &Self { if let CallProblem::SilentRemote(t) = self { fnc(t) }; self }
  pub fn on_dropped<F: FnOnce(&CallProblemDropped)>(&self, fnc: F) -> &Self { if let CallProblem::Dropped(t) = self { fnc(t) }; self }

  pub fn as_echo(&self) -> Option<&CallProblemEcho> { if let CallProblem::Echo(t) = self { return Some(t) } None }
  pub fn as_noise(&self) -> Option<&CallProblemNoise> { if let CallProblem::Noise(t) = self { return Some(t) } None }
  pub fn as_interruptions(&self) -> Option<&CallProblemInterruptions> { if let CallProblem::Interruptions(t) = self { return Some(t) } None }
  pub fn as_distorted_speech(&self) -> Option<&CallProblemDistortedSpeech> { if let CallProblem::DistortedSpeech(t) = self { return Some(t) } None }
  pub fn as_silent_local(&self) -> Option<&CallProblemSilentLocal> { if let CallProblem::SilentLocal(t) = self { return Some(t) } None }
  pub fn as_silent_remote(&self) -> Option<&CallProblemSilentRemote> { if let CallProblem::SilentRemote(t) = self { return Some(t) } None }
  pub fn as_dropped(&self) -> Option<&CallProblemDropped> { if let CallProblem::Dropped(t) = self { return Some(t) } None }



  pub fn echo<T: AsRef<CallProblemEcho>>(t: T) -> Self { CallProblem::Echo(t.as_ref().clone()) }

  pub fn noise<T: AsRef<CallProblemNoise>>(t: T) -> Self { CallProblem::Noise(t.as_ref().clone()) }

  pub fn interruptions<T: AsRef<CallProblemInterruptions>>(t: T) -> Self { CallProblem::Interruptions(t.as_ref().clone()) }

  pub fn distorted_speech<T: AsRef<CallProblemDistortedSpeech>>(t: T) -> Self { CallProblem::DistortedSpeech(t.as_ref().clone()) }

  pub fn silent_local<T: AsRef<CallProblemSilentLocal>>(t: T) -> Self { CallProblem::SilentLocal(t.as_ref().clone()) }

  pub fn silent_remote<T: AsRef<CallProblemSilentRemote>>(t: T) -> Self { CallProblem::SilentRemote(t.as_ref().clone()) }

  pub fn dropped<T: AsRef<CallProblemDropped>>(t: T) -> Self { CallProblem::Dropped(t.as_ref().clone()) }

}

impl AsRef<CallProblem> for CallProblem {
  fn as_ref(&self) -> &CallProblem { self }
}







/// The user heard their own voice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemEcho {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for CallProblemEcho {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProblemEcho" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallProblem for CallProblemEcho {}



impl CallProblemEcho {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProblemEchoBuilder {
    let mut inner = CallProblemEcho::default();
    inner.td_name = "callProblemEcho".to_string();
    RTDCallProblemEchoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCallProblemEchoBuilder {
  inner: CallProblemEcho
}

impl RTDCallProblemEchoBuilder {
  pub fn build(&self) -> CallProblemEcho { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<CallProblemEcho> for CallProblemEcho {
  fn as_ref(&self) -> &CallProblemEcho { self }
}

impl AsRef<CallProblemEcho> for RTDCallProblemEchoBuilder {
  fn as_ref(&self) -> &CallProblemEcho { &self.inner }
}







/// The user heard background noise
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemNoise {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for CallProblemNoise {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProblemNoise" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallProblem for CallProblemNoise {}



impl CallProblemNoise {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProblemNoiseBuilder {
    let mut inner = CallProblemNoise::default();
    inner.td_name = "callProblemNoise".to_string();
    RTDCallProblemNoiseBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCallProblemNoiseBuilder {
  inner: CallProblemNoise
}

impl RTDCallProblemNoiseBuilder {
  pub fn build(&self) -> CallProblemNoise { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<CallProblemNoise> for CallProblemNoise {
  fn as_ref(&self) -> &CallProblemNoise { self }
}

impl AsRef<CallProblemNoise> for RTDCallProblemNoiseBuilder {
  fn as_ref(&self) -> &CallProblemNoise { &self.inner }
}







/// The other side kept disappearing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemInterruptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for CallProblemInterruptions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProblemInterruptions" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallProblem for CallProblemInterruptions {}



impl CallProblemInterruptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProblemInterruptionsBuilder {
    let mut inner = CallProblemInterruptions::default();
    inner.td_name = "callProblemInterruptions".to_string();
    RTDCallProblemInterruptionsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCallProblemInterruptionsBuilder {
  inner: CallProblemInterruptions
}

impl RTDCallProblemInterruptionsBuilder {
  pub fn build(&self) -> CallProblemInterruptions { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<CallProblemInterruptions> for CallProblemInterruptions {
  fn as_ref(&self) -> &CallProblemInterruptions { self }
}

impl AsRef<CallProblemInterruptions> for RTDCallProblemInterruptionsBuilder {
  fn as_ref(&self) -> &CallProblemInterruptions { &self.inner }
}







/// The speech was distorted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemDistortedSpeech {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for CallProblemDistortedSpeech {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProblemDistortedSpeech" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallProblem for CallProblemDistortedSpeech {}



impl CallProblemDistortedSpeech {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProblemDistortedSpeechBuilder {
    let mut inner = CallProblemDistortedSpeech::default();
    inner.td_name = "callProblemDistortedSpeech".to_string();
    RTDCallProblemDistortedSpeechBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCallProblemDistortedSpeechBuilder {
  inner: CallProblemDistortedSpeech
}

impl RTDCallProblemDistortedSpeechBuilder {
  pub fn build(&self) -> CallProblemDistortedSpeech { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<CallProblemDistortedSpeech> for CallProblemDistortedSpeech {
  fn as_ref(&self) -> &CallProblemDistortedSpeech { self }
}

impl AsRef<CallProblemDistortedSpeech> for RTDCallProblemDistortedSpeechBuilder {
  fn as_ref(&self) -> &CallProblemDistortedSpeech { &self.inner }
}







/// The user couldn't hear the other side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemSilentLocal {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for CallProblemSilentLocal {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProblemSilentLocal" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallProblem for CallProblemSilentLocal {}



impl CallProblemSilentLocal {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProblemSilentLocalBuilder {
    let mut inner = CallProblemSilentLocal::default();
    inner.td_name = "callProblemSilentLocal".to_string();
    RTDCallProblemSilentLocalBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCallProblemSilentLocalBuilder {
  inner: CallProblemSilentLocal
}

impl RTDCallProblemSilentLocalBuilder {
  pub fn build(&self) -> CallProblemSilentLocal { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<CallProblemSilentLocal> for CallProblemSilentLocal {
  fn as_ref(&self) -> &CallProblemSilentLocal { self }
}

impl AsRef<CallProblemSilentLocal> for RTDCallProblemSilentLocalBuilder {
  fn as_ref(&self) -> &CallProblemSilentLocal { &self.inner }
}







/// The other side couldn't hear the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemSilentRemote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for CallProblemSilentRemote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProblemSilentRemote" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallProblem for CallProblemSilentRemote {}



impl CallProblemSilentRemote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProblemSilentRemoteBuilder {
    let mut inner = CallProblemSilentRemote::default();
    inner.td_name = "callProblemSilentRemote".to_string();
    RTDCallProblemSilentRemoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCallProblemSilentRemoteBuilder {
  inner: CallProblemSilentRemote
}

impl RTDCallProblemSilentRemoteBuilder {
  pub fn build(&self) -> CallProblemSilentRemote { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<CallProblemSilentRemote> for CallProblemSilentRemote {
  fn as_ref(&self) -> &CallProblemSilentRemote { self }
}

impl AsRef<CallProblemSilentRemote> for RTDCallProblemSilentRemoteBuilder {
  fn as_ref(&self) -> &CallProblemSilentRemote { &self.inner }
}







/// The call ended unexpectedly
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemDropped {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for CallProblemDropped {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProblemDropped" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDCallProblem for CallProblemDropped {}



impl CallProblemDropped {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProblemDroppedBuilder {
    let mut inner = CallProblemDropped::default();
    inner.td_name = "callProblemDropped".to_string();
    RTDCallProblemDroppedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDCallProblemDroppedBuilder {
  inner: CallProblemDropped
}

impl RTDCallProblemDroppedBuilder {
  pub fn build(&self) -> CallProblemDropped { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<CallProblemDropped> for CallProblemDropped {
  fn as_ref(&self) -> &CallProblemDropped { self }
}

impl AsRef<CallProblemDropped> for RTDCallProblemDroppedBuilder {
  fn as_ref(&self) -> &CallProblemDropped { &self.inner }
}



