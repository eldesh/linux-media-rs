use linux_media_sys as media;

use crate::error;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum MediaInterfaceType {
    /// Device node interface for the Digital TV frontend
    /// typically, /dev/dvb/adapter?/frontend?
    DigitalTVFrontEnd,
    /// Device node interface for the Digital TV demux
    /// typically, /dev/dvb/adapter?/demux?
    DigitalTVDemux,
    /// Device node interface for the Digital TV DVR
    /// typically, /dev/dvb/adapter?/dvr?
    DigitalTVDVR,
    /// Device node interface for the Digital TV Conditional Access
    /// typically, /dev/dvb/adapter?/ca?
    DigitalTVConditionalAccess,
    /// Device node interface for the Digital TV network control
    /// typically, /dev/dvb/adapter?/net?
    DigitalTVNetworkControl,
    /// Device node interface for video (V4L)
    /// typically, /dev/video?
    V4LVideo,
    /// Device node interface for VBI (V4L)
    /// typically, /dev/vbi?
    V4LVBI,
    /// Device node interface for radio (V4L)
    /// typically, /dev/radio?
    V4LRadio,
    /// Device node interface for a V4L subdevice
    /// typically, /dev/v4l-subdev?
    V4LSubdev,
    /// Device node interface for Software Defined Radio (V4L)
    /// typically, /dev/swradio?
    V4LSoftwareDefinedRadio,
    /// Device node interface for Touch device (V4L)
    /// typically, /dev/v4l-touch?
    V4LTouchDevice,
    /// Device node interface for ALSA PCM Capture
    /// typically, /dev/snd/pcmC?D?c
    ALSAPCMCapture,
    /// Device node interface for ALSA PCM Playback
    /// typically, /dev/snd/pcmC?D?p
    ALSAPCMPlayback,
    /// Device node interface for ALSA Control
    /// typically, /dev/snd/controlC?
    ALSAControl,
    /// Device node interface for ALSA Compress
    /// typically, /dev/snd/compr?
    ALSACompress,
    /// Device node interface for ALSA Raw MIDI
    /// typically, /dev/snd/midi?
    ALSARawMIDI,
    /// Device node interface for ALSA Hardware Dependent
    /// typically, /dev/snd/hwC?D?
    ALSAHardwareDependent,
    /// Device node interface for ALSA Sequencer
    /// typically, /dev/snd/seq
    ALSASequencer,
    /// Device node interface for ALSA Timer
    /// typically, /dev/snd/timer
    ALSATimer,
}

impl Into<u32> for MediaInterfaceType {
    fn into(self: Self) -> u32 {
        use MediaInterfaceType::*;
        match self {
            DigitalTVFrontEnd => media::MEDIA_INTF_T_DVB_FE,
            DigitalTVDemux => media::MEDIA_INTF_T_DVB_DEMUX,
            DigitalTVDVR => media::MEDIA_INTF_T_DVB_DVR,
            DigitalTVConditionalAccess => media::MEDIA_INTF_T_DVB_CA,
            DigitalTVNetworkControl => media::MEDIA_INTF_T_DVB_NET,
            V4LVideo => media::MEDIA_INTF_T_V4L_VIDEO,
            V4LVBI => media::MEDIA_INTF_T_V4L_VBI,
            V4LRadio => media::MEDIA_INTF_T_V4L_RADIO,
            V4LSubdev => media::MEDIA_INTF_T_V4L_SUBDEV,
            V4LSoftwareDefinedRadio => media::MEDIA_INTF_T_V4L_SWRADIO,
            V4LTouchDevice => media::MEDIA_INTF_T_V4L_TOUCH,
            ALSAPCMCapture => media::MEDIA_INTF_T_ALSA_PCM_CAPTURE,
            ALSAPCMPlayback => media::MEDIA_INTF_T_ALSA_PCM_PLAYBACK,
            ALSAControl => media::MEDIA_INTF_T_ALSA_CONTROL,
            ALSACompress => media::MEDIA_INTF_T_ALSA_COMPRESS,
            ALSARawMIDI => media::MEDIA_INTF_T_ALSA_RAWMIDI,
            ALSAHardwareDependent => media::MEDIA_INTF_T_ALSA_HWDEP,
            ALSASequencer => media::MEDIA_INTF_T_ALSA_SEQUENCER,
            ALSATimer => media::MEDIA_INTF_T_ALSA_TIMER,
        }
    }
}

impl TryFrom<u32> for MediaInterfaceType {
    type Error = error::Error;
    fn try_from(v: u32) -> std::result::Result<Self, Self::Error> {
        use MediaInterfaceType::*;
        match v {
            media::MEDIA_INTF_T_DVB_FE => Ok(DigitalTVFrontEnd),
            media::MEDIA_INTF_T_DVB_DEMUX => Ok(DigitalTVDemux),
            media::MEDIA_INTF_T_DVB_DVR => Ok(DigitalTVDVR),
            media::MEDIA_INTF_T_DVB_CA => Ok(DigitalTVConditionalAccess),
            media::MEDIA_INTF_T_DVB_NET => Ok(DigitalTVNetworkControl),
            media::MEDIA_INTF_T_V4L_VIDEO => Ok(V4LVideo),
            media::MEDIA_INTF_T_V4L_VBI => Ok(V4LVBI),
            media::MEDIA_INTF_T_V4L_RADIO => Ok(V4LRadio),
            media::MEDIA_INTF_T_V4L_SUBDEV => Ok(V4LSubdev),
            media::MEDIA_INTF_T_V4L_SWRADIO => Ok(V4LSoftwareDefinedRadio),
            media::MEDIA_INTF_T_V4L_TOUCH => Ok(V4LTouchDevice),
            media::MEDIA_INTF_T_ALSA_PCM_CAPTURE => Ok(ALSAPCMCapture),
            media::MEDIA_INTF_T_ALSA_PCM_PLAYBACK => Ok(ALSAPCMPlayback),
            media::MEDIA_INTF_T_ALSA_CONTROL => Ok(ALSAControl),
            media::MEDIA_INTF_T_ALSA_COMPRESS => Ok(ALSACompress),
            media::MEDIA_INTF_T_ALSA_RAWMIDI => Ok(ALSARawMIDI),
            media::MEDIA_INTF_T_ALSA_HWDEP => Ok(ALSAHardwareDependent),
            media::MEDIA_INTF_T_ALSA_SEQUENCER => Ok(ALSASequencer),
            media::MEDIA_INTF_T_ALSA_TIMER => Ok(ALSATimer),
            _ => Err(error::Error::InterfaceTypeParseError { from: v }),
        }
    }
}
