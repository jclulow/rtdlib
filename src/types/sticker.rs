
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Describes a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// The identifier of the sticker set to which the sticker belongs; 0 if none
  set_id: String,
  /// Sticker width; as defined by the sender
  width: i64,
  /// Sticker height; as defined by the sender
  height: i64,
  /// Emoji corresponding to the sticker
  emoji: String,
  /// True, if the sticker is an animated sticker in TGS format
  is_animated: bool,
  /// True, if the sticker is a mask
  is_mask: bool,
  /// Position where the mask should be placed; may be null
  mask_position: Option<MaskPosition>,
  /// Sticker thumbnail in WEBP or JPEG format; may be null
  thumbnail: Option<PhotoSize>,
  /// File containing the sticker
  sticker: File,
  
}

impl RObject for Sticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "sticker" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Sticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStickerBuilder {
    let mut inner = Sticker::default();
    inner.td_name = "sticker".to_string();
    RTDStickerBuilder { inner }
  }

  pub fn set_id(&self) -> &String { &self.set_id }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn emoji(&self) -> &String { &self.emoji }

  pub fn is_animated(&self) -> bool { self.is_animated }

  pub fn is_mask(&self) -> bool { self.is_mask }

  pub fn mask_position(&self) -> &Option<MaskPosition> { &self.mask_position }

  pub fn thumbnail(&self) -> &Option<PhotoSize> { &self.thumbnail }

  pub fn sticker(&self) -> &File { &self.sticker }

}

#[doc(hidden)]
pub struct RTDStickerBuilder {
  inner: Sticker
}

impl RTDStickerBuilder {
  pub fn build(&self) -> Sticker { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn set_id<T: AsRef<str>>(&mut self, set_id: T) -> &mut Self {
    self.inner.set_id = set_id.as_ref().to_string();
    self
  }

   
  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

   
  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }

   
  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

   
  pub fn is_animated(&mut self, is_animated: bool) -> &mut Self {
    self.inner.is_animated = is_animated;
    self
  }

   
  pub fn is_mask(&mut self, is_mask: bool) -> &mut Self {
    self.inner.is_mask = is_mask;
    self
  }

   
  pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
    self.inner.mask_position = Some(mask_position.as_ref().clone());
    self
  }

   
  pub fn thumbnail<T: AsRef<PhotoSize>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

   
  pub fn sticker<T: AsRef<File>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

}

impl AsRef<Sticker> for Sticker {
  fn as_ref(&self) -> &Sticker { self }
}

impl AsRef<Sticker> for RTDStickerBuilder {
  fn as_ref(&self) -> &Sticker { &self.inner }
}



