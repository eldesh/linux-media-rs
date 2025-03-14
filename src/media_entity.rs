use std::ffi::CStr;

use derive_more::{Display, From, Into};
use linux_media_sys as media;

use crate::error;
use crate::MediaVersion;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum MediaEntityFunctions {
    /// Unknown entity. That generally indicates that a driver didn’t initialize properly the entity, which is a Kernel bug
    MEDIA_ENT_F_UNKNOWN,
    /// Data streaming input and/or output entity.
    MEDIA_ENT_F_IO_V4L,
    /// V4L VBI streaming input or output entity
    MEDIA_ENT_F_IO_VBI,
    /// V4L Software Digital Radio (SDR) streaming input or output entity
    MEDIA_ENT_F_IO_SWRADIO,
    /// DVB Digital TV streaming input or output entity
    MEDIA_ENT_F_IO_DTV,
    /// Digital TV demodulator entity.
    MEDIA_ENT_F_DTV_DEMOD,
    /// MPEG Transport stream demux entity. Could be implemented on hardware or in Kernelspace by the Linux DVB subsystem.
    MEDIA_ENT_F_TS_DEMUX,
    /// Digital TV Conditional Access module (CAM) entity
    MEDIA_ENT_F_DTV_CA,
    /// Digital TV network ULE/MLE desencapsulation entity. Could be implemented on hardware or in Kernelspace
    MEDIA_ENT_F_DTV_NET_DECAP,
    //// Connector for a Radio Frequency (RF) signal.
    // MEDIA_ENT_F_CONN_RF,
    //// Connector for a S-Video signal.
    // MEDIA_ENT_F_CONN_SVIDEO,
    //// Connector for a RGB composite signal.
    // MEDIA_ENT_F_CONN_COMPOSITE,
    /// Camera video sensor entity.
    MEDIA_ENT_F_CAM_SENSOR,
    /// Flash controller entity.
    MEDIA_ENT_F_FLASH,
    /// Lens controller entity.
    MEDIA_ENT_F_LENS,
    /// Analog video decoder, the basic function of the video decoder is to accept analogue video from a wide variety of sources such as broadcast, DVD players, cameras and video cassette recorders, in either NTSC, PAL, SECAM or HD format, separating the stream into its component parts, luminance and chrominance, and output it in some digital video standard, with appropriate timing signals.
    MEDIA_ENT_F_ATV_DECODER,
    /// Digital TV, analog TV, radio and/or software radio tuner, with consists on a PLL tuning stage that converts radio frequency (RF) signal into an Intermediate Frequency (IF). Modern tuners have internally IF-PLL decoders for audio and video, but older models have those stages implemented on separate entities.
    MEDIA_ENT_F_TUNER,
    /// IF-PLL video decoder. It receives the IF from a PLL and decodes the analog TV video signal. This is commonly found on some very old analog tuners, like Philips MK3 designs. They all contain a tda9887 (or some software compatible similar chip, like tda9885). Those devices use a different I2C address than the tuner PLL.
    MEDIA_ENT_F_IF_VID_DECODER,
    /// IF-PLL sound decoder. It receives the IF from a PLL and decodes the analog TV audio signal. This is commonly found on some very old analog hardware, like Micronas msp3400, Philips tda9840, tda985x, etc. Those devices use a different I2C address than the tuner PLL and should be controlled together with the IF-PLL video decoder.
    MEDIA_ENT_F_IF_AUD_DECODER,
    /// Audio Capture Function Entity.
    MEDIA_ENT_F_AUDIO_CAPTURE,
    /// Audio Playback Function Entity.
    MEDIA_ENT_F_AUDIO_PLAYBACK,
    /// Audio Mixer Function Entity.
    MEDIA_ENT_F_AUDIO_MIXER,
    /// Video composer (blender). An entity capable of video composing must have at least two sink pads and one source pad, and composes input video frames onto output video frames. Composition can be performed using alpha blending, color keying, raster operations (ROP), stitching or any other means.
    MEDIA_ENT_F_PROC_VIDEO_COMPOSER,
    /// Video pixel formatter. An entity capable of pixel formatting must have at least one sink pad and one source pad. Read pixel formatters read pixels from memory and perform a subset of unpacking, cropping, color keying, alpha multiplication and pixel encoding conversion. Write pixel formatters perform a subset of dithering, pixel encoding conversion and packing and write pixels to memory.
    MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER,
    /// Video pixel encoding converter. An entity capable of pixel enconding conversion must have at least one sink pad and one source pad, and convert the encoding of pixels received on its sink pad(s) to a different encoding output on its source pad(s). Pixel encoding conversion includes but isn’t limited to RGB to/from HSV, RGB to/from YUV and CFA (Bayer) to RGB conversions.
    MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV,
    /// Video look-up table. An entity capable of video lookup table processing must have one sink pad and one source pad. It uses the values of the pixels received on its sink pad to look up entries in internal tables and output them on its source pad. The lookup processing can be performed on all components separately or combine them for multi-dimensional table lookups.
    MEDIA_ENT_F_PROC_VIDEO_LUT,
    /// Video scaler. An entity capable of video scaling must have at least one sink pad and one source pad, and scale the video frame(s) received on its sink pad(s) to a different resolution output on its source pad(s). The range of supported scaling ratios is entity-specific and can differ between the horizontal and vertical directions (in particular scaling can be supported in one direction only). Binning and sub-sampling (occasionally also referred to as skipping) are considered as scaling.
    MEDIA_ENT_F_PROC_VIDEO_SCALER,
    /// Video statistics computation (histogram, 3A, etc.). An entity capable of statistics computation must have one sink pad and one source pad. It computes statistics over the frames received on its sink pad and outputs the statistics data on its source pad.
    MEDIA_ENT_F_PROC_VIDEO_STATISTICS,
    /// Video (MPEG, HEVC, VPx, etc.) encoder. An entity capable of compressing video frames. Must have one sink pad and at least one source pad.
    MEDIA_ENT_F_PROC_VIDEO_ENCODER,
    /// Video (MPEG, HEVC, VPx, etc.) decoder. An entity capable of decompressing a compressed video stream into uncompressed video frames. Must have one sink pad and at least one source pad.
    MEDIA_ENT_F_PROC_VIDEO_DECODER,
    /// Video multiplexer. An entity capable of multiplexing must have at least two sink pads and one source pad, and must pass the video frame(s) received from the active sink pad to the source pad.
    MEDIA_ENT_F_VID_MUX,
    /// Video interface bridge. A video interface bridge entity must have at least one sink pad and at least one source pad. It receives video frames on its sink pad from an input video bus of one type (HDMI, eDP, MIPI CSI-2, etc.), and outputs them on its source pad to an output video bus of another type (eDP, MIPI CSI-2, parallel, etc.).
    MEDIA_ENT_F_VID_IF_BRIDGE,
    /// Digital video decoder. The basic function of the video decoder is to accept digital video from a wide variety of sources and output it in some digital video standard, with appropriate timing signals.
    MEDIA_ENT_F_DV_DECODER,
    /// Digital video encoder. The basic function of the video encoder is to accept digital video from some digital video standard with appropriate timing signals (usually a parallel video bus with sync signals) and output this to a digital video output connector such as HDMI or DisplayPort.
    MEDIA_ENT_F_DV_ENCODER,
}

impl TryFrom<u32> for MediaEntityFunctions {
    type Error = error::Error;
    fn try_from(v: u32) -> error::Result<Self> {
        use MediaEntityFunctions::*;
        match v {
            media::MEDIA_ENT_F_UNKNOWN => Ok(MEDIA_ENT_F_UNKNOWN),
            media::MEDIA_ENT_F_IO_V4L => Ok(MEDIA_ENT_F_IO_V4L),
            media::MEDIA_ENT_F_IO_VBI => Ok(MEDIA_ENT_F_IO_VBI),
            media::MEDIA_ENT_F_IO_SWRADIO => Ok(MEDIA_ENT_F_IO_SWRADIO),
            media::MEDIA_ENT_F_IO_DTV => Ok(MEDIA_ENT_F_IO_DTV),
            media::MEDIA_ENT_F_DTV_DEMOD => Ok(MEDIA_ENT_F_DTV_DEMOD),
            media::MEDIA_ENT_F_TS_DEMUX => Ok(MEDIA_ENT_F_TS_DEMUX),
            media::MEDIA_ENT_F_DTV_CA => Ok(MEDIA_ENT_F_DTV_CA),
            media::MEDIA_ENT_F_DTV_NET_DECAP => Ok(MEDIA_ENT_F_DTV_NET_DECAP),
            // media::MEDIA_ENT_F_CONN_RF => Ok(MEDIA_ENT_F_CONN_RF),
            // media::MEDIA_ENT_F_CONN_SVIDEO => Ok(MEDIA_ENT_F_CONN_SVIDEO),
            // media::MEDIA_ENT_F_CONN_COMPOSITE => Ok(MEDIA_ENT_F_CONN_COMPOSITE),
            media::MEDIA_ENT_F_CAM_SENSOR => Ok(MEDIA_ENT_F_CAM_SENSOR),
            media::MEDIA_ENT_F_FLASH => Ok(MEDIA_ENT_F_FLASH),
            media::MEDIA_ENT_F_LENS => Ok(MEDIA_ENT_F_LENS),
            media::MEDIA_ENT_F_ATV_DECODER => Ok(MEDIA_ENT_F_ATV_DECODER),
            media::MEDIA_ENT_F_TUNER => Ok(MEDIA_ENT_F_TUNER),
            media::MEDIA_ENT_F_IF_VID_DECODER => Ok(MEDIA_ENT_F_IF_VID_DECODER),
            media::MEDIA_ENT_F_IF_AUD_DECODER => Ok(MEDIA_ENT_F_IF_AUD_DECODER),
            media::MEDIA_ENT_F_AUDIO_CAPTURE => Ok(MEDIA_ENT_F_AUDIO_CAPTURE),
            media::MEDIA_ENT_F_AUDIO_PLAYBACK => Ok(MEDIA_ENT_F_AUDIO_PLAYBACK),
            media::MEDIA_ENT_F_AUDIO_MIXER => Ok(MEDIA_ENT_F_AUDIO_MIXER),
            media::MEDIA_ENT_F_PROC_VIDEO_COMPOSER => Ok(MEDIA_ENT_F_PROC_VIDEO_COMPOSER),
            media::MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER => {
                Ok(MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER)
            }
            media::MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV => {
                Ok(MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV)
            }
            media::MEDIA_ENT_F_PROC_VIDEO_LUT => Ok(MEDIA_ENT_F_PROC_VIDEO_LUT),
            media::MEDIA_ENT_F_PROC_VIDEO_SCALER => Ok(MEDIA_ENT_F_PROC_VIDEO_SCALER),
            media::MEDIA_ENT_F_PROC_VIDEO_STATISTICS => Ok(MEDIA_ENT_F_PROC_VIDEO_STATISTICS),
            media::MEDIA_ENT_F_PROC_VIDEO_ENCODER => Ok(MEDIA_ENT_F_PROC_VIDEO_ENCODER),
            media::MEDIA_ENT_F_PROC_VIDEO_DECODER => Ok(MEDIA_ENT_F_PROC_VIDEO_DECODER),
            media::MEDIA_ENT_F_VID_MUX => Ok(MEDIA_ENT_F_VID_MUX),
            media::MEDIA_ENT_F_VID_IF_BRIDGE => Ok(MEDIA_ENT_F_VID_IF_BRIDGE),
            media::MEDIA_ENT_F_DV_DECODER => Ok(MEDIA_ENT_F_DV_DECODER),
            media::MEDIA_ENT_F_DV_ENCODER => Ok(MEDIA_ENT_F_DV_ENCODER),
            other => Err(error::Error::EntityFunctionsParseError { from: other }),
        }
    }
}

/// Media entity flags
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Display)]
pub enum MediaEntityFlags {
    /// Default entity for its type. Used to discover the default audio, VBI and video devices, the default camera sensor, etc.
    #[display("MEDIA_ENT_FL_DEFAULT")]
    MEDIA_ENT_FL_DEFAULT,
    /// The entity represents a connector.
    #[display("MEDIA_ENT_FL_CONNECTOR")]
    MEDIA_ENT_FL_CONNECTOR,
}

impl TryFrom<u32> for MediaEntityFlags {
    type Error = error::Error;
    fn try_from(v: u32) -> error::Result<Self> {
        use MediaEntityFlags::*;
        match v {
            media::MEDIA_ENT_FL_DEFAULT => Ok(MEDIA_ENT_FL_DEFAULT),
            media::MEDIA_ENT_FL_CONNECTOR => Ok(MEDIA_ENT_FL_CONNECTOR),
            other => Err(error::Error::EntityFlagsParseError { from: other }),
        }
    }
}

impl Default for MediaEntityFlags {
    fn default() -> Self {
        MediaEntityFlags::MEDIA_ENT_FL_DEFAULT
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, From, Into)]
pub struct EntityId(u32);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaEntity {
    id: EntityId,
    name: String,
    function: MediaEntityFunctions,
    /// Entity flags, see Media entity flags for details. Only valid if MEDIA_V2_ENTITY_HAS_FLAGS(media_version) returns true. The media_version is defined in struct media_device_info and can be retrieved using ioctl MEDIA_IOC_DEVICE_INFO.
    flags: Option<MediaEntityFlags>,
}

impl MediaEntity {
    pub fn new(
        ver: MediaVersion,
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

    pub fn has_flags(version: MediaVersion) -> bool {
        media::MEDIA_V2_ENTITY_HAS_FLAGS(Into::<u32>::into(version).into())
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}

impl From<media::media_v2_entity> for MediaEntity {
    fn from(entity: media::media_v2_entity) -> Self {
        let id = EntityId::from(entity.id);
        let name = CStr::from_bytes_until_nul(&entity.name)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let function: MediaEntityFunctions = entity.function.try_into().unwrap();
        let flags: Option<MediaEntityFlags> = entity.flags.try_into().ok();
        Self {
            id,
            name,
            function,
            flags,
        }
    }
}
