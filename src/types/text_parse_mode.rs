
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the way the text should be parsed for TextEntities
pub trait TDTextParseMode: Debug + RObject {}

/// Describes the way the text should be parsed for TextEntities
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TextParseMode {
  #[doc(hidden)] _Default(()),
  /// The text should be parsed in markdown-style
  Markdown(TextParseModeMarkdown),
  /// The text should be parsed in HTML-style
  HTML(TextParseModeHTML),

}

impl Default for TextParseMode {
  fn default() -> Self { TextParseMode::_Default(()) }
}

impl<'de> Deserialize<'de> for TextParseMode {
  fn deserialize<D>(deserializer: D) -> Result<TextParseMode, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TextParseMode,
      (textParseModeMarkdown, Markdown);
      (textParseModeHTML, HTML);

    )(deserializer)
  }
}

impl RObject for TextParseMode {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      TextParseMode::Markdown(t) => t.td_name(),
      TextParseMode::HTML(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> { None }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl TextParseMode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let TextParseMode::_Default(_) = self { true } else { false } }

  pub fn is_markdown(&self) -> bool { if let TextParseMode::Markdown(_) = self { true } else { false } }
  pub fn is_h_t_m_l(&self) -> bool { if let TextParseMode::HTML(_) = self { true } else { false } }

  pub fn on_markdown<F: FnOnce(&TextParseModeMarkdown)>(&self, fnc: F) -> &Self { if let TextParseMode::Markdown(t) = self { fnc(t) }; self }
  pub fn on_h_t_m_l<F: FnOnce(&TextParseModeHTML)>(&self, fnc: F) -> &Self { if let TextParseMode::HTML(t) = self { fnc(t) }; self }

  pub fn as_markdown(&self) -> Option<&TextParseModeMarkdown> { if let TextParseMode::Markdown(t) = self { return Some(t) } None }
  pub fn as_h_t_m_l(&self) -> Option<&TextParseModeHTML> { if let TextParseMode::HTML(t) = self { return Some(t) } None }



  pub fn markdown<T: AsRef<TextParseModeMarkdown>>(t: T) -> Self { TextParseMode::Markdown(t.as_ref().clone()) }

  pub fn h_t_m_l<T: AsRef<TextParseModeHTML>>(t: T) -> Self { TextParseMode::HTML(t.as_ref().clone()) }

}

impl AsRef<TextParseMode> for TextParseMode {
  fn as_ref(&self) -> &TextParseMode { self }
}







/// The text should be parsed in markdown-style
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextParseModeMarkdown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Version of the parser: 0 or 1  Bot API Markdown parse mode, 2  Bot API MarkdownV2 parse mode
  version: i64,
  
}

impl RObject for TextParseModeMarkdown {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textParseModeMarkdown" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextParseMode for TextParseModeMarkdown {}



impl TextParseModeMarkdown {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextParseModeMarkdownBuilder {
    let mut inner = TextParseModeMarkdown::default();
    inner.td_name = "textParseModeMarkdown".to_string();
    RTDTextParseModeMarkdownBuilder { inner }
  }

  pub fn version(&self) -> i64 { self.version }

}

#[doc(hidden)]
pub struct RTDTextParseModeMarkdownBuilder {
  inner: TextParseModeMarkdown
}

impl RTDTextParseModeMarkdownBuilder {
  pub fn build(&self) -> TextParseModeMarkdown { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn version(&mut self, version: i64) -> &mut Self {
    self.inner.version = version;
    self
  }

}

impl AsRef<TextParseModeMarkdown> for TextParseModeMarkdown {
  fn as_ref(&self) -> &TextParseModeMarkdown { self }
}

impl AsRef<TextParseModeMarkdown> for RTDTextParseModeMarkdownBuilder {
  fn as_ref(&self) -> &TextParseModeMarkdown { &self.inner }
}







/// The text should be parsed in HTML-style
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextParseModeHTML {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  
}

impl RObject for TextParseModeHTML {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textParseModeHTML" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextParseMode for TextParseModeHTML {}



impl TextParseModeHTML {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextParseModeHTMLBuilder {
    let mut inner = TextParseModeHTML::default();
    inner.td_name = "textParseModeHTML".to_string();
    RTDTextParseModeHTMLBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextParseModeHTMLBuilder {
  inner: TextParseModeHTML
}

impl RTDTextParseModeHTMLBuilder {
  pub fn build(&self) -> TextParseModeHTML { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

}

impl AsRef<TextParseModeHTML> for TextParseModeHTML {
  fn as_ref(&self) -> &TextParseModeHTML { self }
}

impl AsRef<TextParseModeHTML> for RTDTextParseModeHTMLBuilder {
  fn as_ref(&self) -> &TextParseModeHTML { &self.inner }
}



