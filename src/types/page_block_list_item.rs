
use crate::types::*;
use crate::errors::*;




/// Describes an item of a list page block
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockListItem {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<String>,
  /// Item label
  label: String,
  /// Item blocks
  page_blocks: Vec<PageBlock>,
  
}

impl RObject for PageBlockListItem {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockListItem" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    self.td_tag.as_deref()
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PageBlockListItem {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockListItemBuilder {
    let mut inner = PageBlockListItem::default();
    inner.td_name = "pageBlockListItem".to_string();
    RTDPageBlockListItemBuilder { inner }
  }

  pub fn label(&self) -> &String { &self.label }

  pub fn page_blocks(&self) -> &Vec<PageBlock> { &self.page_blocks }

}

#[doc(hidden)]
pub struct RTDPageBlockListItemBuilder {
  inner: PageBlockListItem
}

impl RTDPageBlockListItemBuilder {
  pub fn build(&self) -> PageBlockListItem { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(tag.as_ref().to_string());
    self
  }

   
  pub fn label<T: AsRef<str>>(&mut self, label: T) -> &mut Self {
    self.inner.label = label.as_ref().to_string();
    self
  }

   
  pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
    self.inner.page_blocks = page_blocks;
    self
  }

}

impl AsRef<PageBlockListItem> for PageBlockListItem {
  fn as_ref(&self) -> &PageBlockListItem { self }
}

impl AsRef<PageBlockListItem> for RTDPageBlockListItemBuilder {
  fn as_ref(&self) -> &PageBlockListItem { &self.inner }
}



