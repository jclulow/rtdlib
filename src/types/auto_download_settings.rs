
use crate::types::*;
use crate::errors::*;
use crate::types::_common::Extra;




/// Contains auto-download settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoDownloadSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  td_tag: Option<Extra>,
  /// True, if the auto-download is enabled
  is_auto_download_enabled: bool,
  /// Maximum size of a photo file to be auto-downloaded
  max_photo_file_size: i64,
  /// Maximum size of a video file to be auto-downloaded
  max_video_file_size: i64,
  /// Maximum size of other file types to be auto-downloaded
  max_other_file_size: i64,
  /// Suggested maximum bitrate for uploaded videos
  video_upload_bitrate: i64,
  /// True, if the beginning of videos needs to be preloaded for instant playback
  preload_large_videos: bool,
  /// True, if the next audio track needs to be preloaded while the user is listening to an audio file
  preload_next_audio: bool,
  /// True, if "use less data for calls" option needs to be enabled
  use_less_data_for_calls: bool,
  
}

impl RObject for AutoDownloadSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "autoDownloadSettings" }
  #[doc(hidden)] fn td_tag(&self) -> Option<&str> {
    if self.td_tag.is_none() {
      None
    } else {
      self.td_tag.as_ref().unwrap().tag.as_deref()
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl AutoDownloadSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAutoDownloadSettingsBuilder {
    let mut inner = AutoDownloadSettings::default();
    inner.td_name = "autoDownloadSettings".to_string();
    RTDAutoDownloadSettingsBuilder { inner }
  }

  pub fn is_auto_download_enabled(&self) -> bool { self.is_auto_download_enabled }

  pub fn max_photo_file_size(&self) -> i64 { self.max_photo_file_size }

  pub fn max_video_file_size(&self) -> i64 { self.max_video_file_size }

  pub fn max_other_file_size(&self) -> i64 { self.max_other_file_size }

  pub fn video_upload_bitrate(&self) -> i64 { self.video_upload_bitrate }

  pub fn preload_large_videos(&self) -> bool { self.preload_large_videos }

  pub fn preload_next_audio(&self) -> bool { self.preload_next_audio }

  pub fn use_less_data_for_calls(&self) -> bool { self.use_less_data_for_calls }

}

#[doc(hidden)]
pub struct RTDAutoDownloadSettingsBuilder {
  inner: AutoDownloadSettings
}

impl RTDAutoDownloadSettingsBuilder {
  pub fn build(&self) -> AutoDownloadSettings { self.inner.clone() }
  pub fn td_tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
    self.inner.td_tag = Some(Extra { tag: Some(tag.as_ref().to_string()) });
    self
  }

   
  pub fn is_auto_download_enabled(&mut self, is_auto_download_enabled: bool) -> &mut Self {
    self.inner.is_auto_download_enabled = is_auto_download_enabled;
    self
  }

   
  pub fn max_photo_file_size(&mut self, max_photo_file_size: i64) -> &mut Self {
    self.inner.max_photo_file_size = max_photo_file_size;
    self
  }

   
  pub fn max_video_file_size(&mut self, max_video_file_size: i64) -> &mut Self {
    self.inner.max_video_file_size = max_video_file_size;
    self
  }

   
  pub fn max_other_file_size(&mut self, max_other_file_size: i64) -> &mut Self {
    self.inner.max_other_file_size = max_other_file_size;
    self
  }

   
  pub fn video_upload_bitrate(&mut self, video_upload_bitrate: i64) -> &mut Self {
    self.inner.video_upload_bitrate = video_upload_bitrate;
    self
  }

   
  pub fn preload_large_videos(&mut self, preload_large_videos: bool) -> &mut Self {
    self.inner.preload_large_videos = preload_large_videos;
    self
  }

   
  pub fn preload_next_audio(&mut self, preload_next_audio: bool) -> &mut Self {
    self.inner.preload_next_audio = preload_next_audio;
    self
  }

   
  pub fn use_less_data_for_calls(&mut self, use_less_data_for_calls: bool) -> &mut Self {
    self.inner.use_less_data_for_calls = use_less_data_for_calls;
    self
  }

}

impl AsRef<AutoDownloadSettings> for AutoDownloadSettings {
  fn as_ref(&self) -> &AutoDownloadSettings { self }
}

impl AsRef<AutoDownloadSettings> for RTDAutoDownloadSettingsBuilder {
  fn as_ref(&self) -> &AutoDownloadSettings { &self.inner }
}



