use std::ffi::CStr;
use std::ops::{BitAnd, BitOr};

use bitflags;
use derive_more::{From, Into};
use linux_media_sys as media;

use crate::error;
use crate::MediaEntityDesc;
use crate::Version;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum MediaEntityFunctions {
    /// Unknown entity. That generally indicates that a driver didn’t initialize properly the entity, which is a Kernel bug
    Unknown,
    /// Unknown entity. That generally indicates that a driver didn’t initialize properly the entity, which is a Kernel bug
    V4L2SubdevUnknown,
    /// Data streaming input and/or output entity.
    IoV4L,
    /// V4L VBI streaming input or output entity
    IoVBI,
    /// V4L Software Digital Radio (SDR) streaming input or output entity
    IoSWRadio,
    /// DVB Digital TV streaming input or output entity
    IoDTV,
    /// Digital TV demodulator entity.
    DTVDemod,
    /// MPEG Transport stream demux entity. Could be implemented on hardware or in Kernelspace by the Linux DVB subsystem.
    TSDemux,
    /// Digital TV Conditional Access module (CAM) entity
    DTVCondAccess,
    /// Digital TV network ULE/MLE desencapsulation entity. Could be implemented on hardware or in Kernelspace
    DTVNetDecap,
    //// Connector for a Radio Frequency (RF) signal.
    // CONN_RF,
    //// Connector for a S-Video signal.
    // CONN_SVIDEO,
    //// Connector for a RGB composite signal.
    // CONN_COMPOSITE,
    /// Camera video sensor entity.
    CAMSensor,
    /// Flash controller entity.
    Flash,
    /// Lens controller entity.
    Lens,
    /// Analog video decoder, the basic function of the video decoder is to accept analogue video from a wide variety of sources such as broadcast, DVD players, cameras and video cassette recorders, in either NTSC, PAL, SECAM or HD format, separating the stream into its component parts, luminance and chrominance, and output it in some digital video standard, with appropriate timing signals.
    ATVDecoder,
    /// Digital TV, analog TV, radio and/or software radio tuner, with consists on a PLL tuning stage that converts radio frequency (RF) signal into an Intermediate Frequency (IF). Modern tuners have internally IF-PLL decoders for audio and video, but older models have those stages implemented on separate entities.
    Tuner,
    /// IF-PLL video decoder. It receives the IF from a PLL and decodes the analog TV video signal. This is commonly found on some very old analog tuners, like Philips MK3 designs. They all contain a tda9887 (or some software compatible similar chip, like tda9885). Those devices use a different I2C address than the tuner PLL.
    IFVIDDecoder,
    /// IF-PLL sound decoder. It receives the IF from a PLL and decodes the analog TV audio signal. This is commonly found on some very old analog hardware, like Micronas msp3400, Philips tda9840, tda985x, etc. Those devices use a different I2C address than the tuner PLL and should be controlled together with the IF-PLL video decoder.
    IFAUDDecoder,
    /// Audio Capture Function Entity.
    AudioCapture,
    /// Audio Playback Function Entity.
    AudioPlayback,
    /// Audio Mixer Function Entity.
    AudioMixer,
    /// Video composer (blender). An entity capable of video composing must have at least two sink pads and one source pad, and composes input video frames onto output video frames. Composition can be performed using alpha blending, color keying, raster operations (ROP), stitching or any other means.
    ProcVideoComposer,
    /// Video pixel formatter. An entity capable of pixel formatting must have at least one sink pad and one source pad. Read pixel formatters read pixels from memory and perform a subset of unpacking, cropping, color keying, alpha multiplication and pixel encoding conversion. Write pixel formatters perform a subset of dithering, pixel encoding conversion and packing and write pixels to memory.
    ProcVideoPixelFormatter,
    /// Video pixel encoding converter. An entity capable of pixel enconding conversion must have at least one sink pad and one source pad, and convert the encoding of pixels received on its sink pad(s) to a different encoding output on its source pad(s). Pixel encoding conversion includes but isn’t limited to RGB to/from HSV, RGB to/from YUV and CFA (Bayer) to RGB conversions.
    ProcVideoPixelEncConv,
    /// Video look-up table. An entity capable of video lookup table processing must have one sink pad and one source pad. It uses the values of the pixels received on its sink pad to look up entries in internal tables and output them on its source pad. The lookup processing can be performed on all components separately or combine them for multi-dimensional table lookups.
    ProcVideoLUT,
    /// Video scaler. An entity capable of video scaling must have at least one sink pad and one source pad, and scale the video frame(s) received on its sink pad(s) to a different resolution output on its source pad(s). The range of supported scaling ratios is entity-specific and can differ between the horizontal and vertical directions (in particular scaling can be supported in one direction only). Binning and sub-sampling (occasionally also referred to as skipping) are considered as scaling.
    ProcVideoScaler,
    /// Video statistics computation (histogram, 3A, etc.). An entity capable of statistics computation must have one sink pad and one source pad. It computes statistics over the frames received on its sink pad and outputs the statistics data on its source pad.
    ProcVideoStatistics,
    /// Video (MPEG, HEVC, VPx, etc.) encoder. An entity capable of compressing video frames. Must have one sink pad and at least one source pad.
    ProcVideoEncoder,
    /// Video (MPEG, HEVC, VPx, etc.) decoder. An entity capable of decompressing a compressed video stream into uncompressed video frames. Must have one sink pad and at least one source pad.
    ProcVideoDecoder,
    /// Video multiplexer. An entity capable of multiplexing must have at least two sink pads and one source pad, and must pass the video frame(s) received from the active sink pad to the source pad.
    VIDMux,
    /// Video interface bridge. A video interface bridge entity must have at least one sink pad and at least one source pad. It receives video frames on its sink pad from an input video bus of one type (HDMI, eDP, MIPI CSI-2, etc.), and outputs them on its source pad to an output video bus of another type (eDP, MIPI CSI-2, parallel, etc.).
    VIDIFBridge,
    /// Digital video decoder. The basic function of the video decoder is to accept digital video from a wide variety of sources and output it in some digital video standard, with appropriate timing signals.
    DVDecoder,
    /// Digital video encoder. The basic function of the video encoder is to accept digital video from some digital video standard with appropriate timing signals (usually a parallel video bus with sync signals) and output this to a digital video output connector such as HDMI or DisplayPort.
    DVEncoder,
}

impl TryFrom<u32> for MediaEntityFunctions {
    type Error = error::Error;
    fn try_from(v: u32) -> error::Result<Self> {
        use MediaEntityFunctions::*;
        match v {
            media::MEDIA_ENT_F_UNKNOWN => Ok(Unknown),
            media::MEDIA_ENT_F_V4L2_SUBDEV_UNKNOWN => Ok(V4L2SubdevUnknown),
            media::MEDIA_ENT_F_IO_V4L => Ok(IoV4L),
            media::MEDIA_ENT_F_IO_VBI => Ok(IoVBI),
            media::MEDIA_ENT_F_IO_SWRADIO => Ok(IoSWRadio),
            media::MEDIA_ENT_F_IO_DTV => Ok(IoDTV),
            media::MEDIA_ENT_F_DTV_DEMOD => Ok(DTVDemod),
            media::MEDIA_ENT_F_TS_DEMUX => Ok(TSDemux),
            media::MEDIA_ENT_F_DTV_CA => Ok(DTVCondAccess),
            media::MEDIA_ENT_F_DTV_NET_DECAP => Ok(DTVNetDecap),
            // media::MEDIA_ENT_F_CONN_RF => Ok(ConnRF),
            // media::MEDIA_ENT_F_CONN_SVIDEO => Ok(ConnSvideo),
            // media::MEDIA_ENT_F_CONN_COMPOSITE => Ok(ConnComposite),
            media::MEDIA_ENT_F_CAM_SENSOR => Ok(CAMSensor),
            media::MEDIA_ENT_F_FLASH => Ok(Flash),
            media::MEDIA_ENT_F_LENS => Ok(Lens),
            media::MEDIA_ENT_F_ATV_DECODER => Ok(ATVDecoder),
            media::MEDIA_ENT_F_TUNER => Ok(Tuner),
            media::MEDIA_ENT_F_IF_VID_DECODER => Ok(IFVIDDecoder),
            media::MEDIA_ENT_F_IF_AUD_DECODER => Ok(IFAUDDecoder),
            media::MEDIA_ENT_F_AUDIO_CAPTURE => Ok(AudioCapture),
            media::MEDIA_ENT_F_AUDIO_PLAYBACK => Ok(AudioPlayback),
            media::MEDIA_ENT_F_AUDIO_MIXER => Ok(AudioMixer),
            media::MEDIA_ENT_F_PROC_VIDEO_COMPOSER => Ok(ProcVideoComposer),
            media::MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER => Ok(ProcVideoPixelFormatter),
            media::MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV => Ok(ProcVideoPixelEncConv),
            media::MEDIA_ENT_F_PROC_VIDEO_LUT => Ok(ProcVideoLUT),
            media::MEDIA_ENT_F_PROC_VIDEO_SCALER => Ok(ProcVideoScaler),
            media::MEDIA_ENT_F_PROC_VIDEO_STATISTICS => Ok(ProcVideoStatistics),
            media::MEDIA_ENT_F_PROC_VIDEO_ENCODER => Ok(ProcVideoEncoder),
            media::MEDIA_ENT_F_PROC_VIDEO_DECODER => Ok(ProcVideoDecoder),
            media::MEDIA_ENT_F_VID_MUX => Ok(VIDMux),
            media::MEDIA_ENT_F_VID_IF_BRIDGE => Ok(VIDIFBridge),
            media::MEDIA_ENT_F_DV_DECODER => Ok(DVDecoder),
            media::MEDIA_ENT_F_DV_ENCODER => Ok(DVEncoder),
            other => Err(error::Error::EntityFunctionsParseError { from: other }),
        }
    }
}

bitflags::bitflags! {
    /// Media entity flags
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    pub struct MediaEntityFlags: u32 {
        /// Default entity for its type. Used to discover the default audio, VBI and video devices, the default camera sensor, etc.
        const Default = media::MEDIA_ENT_FL_DEFAULT;
        /// The entity represents a connector.
        const Connector = media::MEDIA_ENT_FL_CONNECTOR;
    }
}

impl TryFrom<u32> for MediaEntityFlags {
    type Error = error::Error;
    fn try_from(v: u32) -> error::Result<Self> {
        MediaEntityFlags::from_bits(v)
            .ok_or_else(|| error::Error::EntityFlagsParseError { from: v })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, From, Into)]
pub struct EntityId(u32);

/// for or'ing with linux_media_sys::MEDIA_ENT_ID_FLAG_NEXT.
impl BitOr for EntityId {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        EntityId(self.0 | rhs.0)
    }
}

/// for clearing linux_media_sys::MEDIA_ENT_ID_FLAG_NEXT.
impl BitAnd for EntityId {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        EntityId(self.0 & rhs.0)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaEntity {
    id: EntityId,
    name: String,
    function: MediaEntityFunctions,
    /// media entity flags.
    /// Only `Some` if `has_flags` return true.
    flags: Option<MediaEntityFlags>,
}

impl MediaEntity {
    fn new(
        id: EntityId,
        name: &str,
        function: MediaEntityFunctions,
        flags: Option<MediaEntityFlags>,
    ) -> Self {
        Self {
            id,
            name: name.to_owned(),
            function,
            flags,
        }
    }

    pub fn has_flags(version: Version) -> bool {
        media::MEDIA_V2_ENTITY_HAS_FLAGS(<Version as Into<u32>>::into(version).into())
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn from_raw_entity(version: Version, entity: media::media_v2_entity) -> Self {
        let id = EntityId::from(entity.id);
        let name = CStr::from_bytes_until_nul(&entity.name)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let function: MediaEntityFunctions = entity.function.try_into().unwrap();
        let flags: Option<MediaEntityFlags> = if Self::has_flags(version) {
            Some(entity.flags.try_into().unwrap())
        } else {
            None
        };
        Self {
            id,
            name,
            function,
            flags,
        }
    }

    pub fn from_desc(version: Version, desc: MediaEntityDesc) -> Self {
        Self {
            id: desc.id,
            name: desc.name,
            function: desc.r#type,
            flags: if Self::has_flags(version) {
                Some(desc.flags)
            } else {
                None
            },
        }
    }
}
