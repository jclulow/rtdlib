
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains a caption of an instant view web page block, consisting of a text and a trailing credit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockCaption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// Content of the caption
  text: RichText,
  /// Block credit (like HTML tag <cite>)
  credit: RichText,
  
}

impl RObject for PageBlockCaption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockCaption" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PageBlockCaption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockCaptionBuilder {
    let mut inner = PageBlockCaption::default();
    inner.td_name = "pageBlockCaption".to_string();
    RTDPageBlockCaptionBuilder { inner }
  }

  pub fn text(&self) -> &RichText { &self.text }

  pub fn credit(&self) -> &RichText { &self.credit }

}

#[doc(hidden)]
pub struct RTDPageBlockCaptionBuilder {
  inner: PageBlockCaption
}

impl RTDPageBlockCaptionBuilder {
  pub fn build(&self) -> PageBlockCaption { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn credit<T: AsRef<RichText>>(&mut self, credit: T) -> &mut Self {
    self.inner.credit = credit.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockCaption> for PageBlockCaption {
  fn as_ref(&self) -> &PageBlockCaption { self }
}

impl AsRef<PageBlockCaption> for RTDPageBlockCaptionBuilder {
  fn as_ref(&self) -> &PageBlockCaption { &self.inner }
}



