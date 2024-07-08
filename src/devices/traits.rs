use crate::media::playback::PlaybackFrame;

use super::{
    errors::{
        CloseError, FindError, InfoError, InitializationError, ListError, OpenError,
        SubmissionError,
    },
    format::{FormatInfo, SupportedFormat},
};

pub trait DeviceProvider: Default {
    fn initialize(&mut self) -> Result<(), InitializationError>;
    fn get_devices(&mut self) -> Result<Vec<impl Device>, ListError>;
    fn get_default_device(&mut self) -> Result<impl Device, FindError>;
    fn get_device_by_uid(&mut self, id: &String) -> Result<impl Device, FindError>;
}

pub trait Device {
    fn open_device(&mut self, format: FormatInfo) -> Result<(), OpenError>;
    fn close_device(&mut self) -> Result<(), CloseError>;
    fn submit_frame(&mut self, frame: PlaybackFrame) -> Result<(), SubmissionError>;

    fn get_supported_formats(&self) -> Result<Vec<SupportedFormat>, InfoError>;
    fn get_default_format(&self) -> Result<FormatInfo, InfoError>;
    fn get_current_format(&self) -> Result<&FormatInfo, InfoError>;
    fn get_name(&self) -> Result<String, InfoError>;
    fn get_uid(&self) -> Result<String, InfoError>;
}
