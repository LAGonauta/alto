#[nolink]
#[link_args="-framework OpenAL"]
#[cfg(target_os = "macos")]
extern mod linkhack {}

#[nolink]
#[link_args="-lopenal"]
#[cfg(target_os = "linux")]
extern mod linkhack {}

use libc::*;

type ALboolean  = c_char;
type ALchar     = c_char;
type ALbyte     = c_char;
type ALubyte    = c_uchar;
type ALshort    = c_short;
type ALushort   = c_ushort;
type ALint      = c_int;
type ALuint     = c_uint;
type ALsizei    = c_int;
type ALenum     = c_int;
type ALfloat    = c_float;
type ALdouble   = c_double;
type ALvoid     = c_void;

pub const AL_NONE                           : ALenum = 0;
pub const AL_FALSE                          : ALenum = 0;
pub const AL_TRUE                           : ALenum = 1;
pub const AL_SOURCE_RELATIVE                : ALenum = 0x202;
pub const AL_CONE_INNER_ANGLE               : ALenum = 0x1001;
pub const AL_CONE_OUTER_ANGLE               : ALenum = 0x1002;
pub const AL_PITCH                          : ALenum = 0x1003;
pub const AL_POSITION                       : ALenum = 0x1004;
pub const AL_DIRECTION                      : ALenum = 0x1005;
pub const AL_VELOCITY                       : ALenum = 0x1006;
pub const AL_LOOPING                        : ALenum = 0x1007;
pub const AL_BUFFER                         : ALenum = 0x1009;
pub const AL_GAIN                           : ALenum = 0x100A;
pub const AL_MIN_GAIN                       : ALenum = 0x100D;
pub const AL_MAX_GAIN                       : ALenum = 0x100E;
pub const AL_ORIENTATION                    : ALenum = 0x100F;
pub const AL_SOURCE_STATE                   : ALenum = 0x1010;
pub const AL_INITIAL                        : ALenum = 0x1011;
pub const AL_PLAYING                        : ALenum = 0x1012;
pub const AL_PAUSED                         : ALenum = 0x1013;
pub const AL_STOPPED                        : ALenum = 0x1014;
pub const AL_BUFFERS_QUEUED                 : ALenum = 0x1015;
pub const AL_BUFFERS_PROCESSED              : ALenum = 0x1016;
pub const AL_SEC_OFFSET                     : ALenum = 0x1024;
pub const AL_SAMPLE_OFFSET                  : ALenum = 0x1025;
pub const AL_BYTE_OFFSET                    : ALenum = 0x1026;
pub const AL_SOURCE_TYPE                    : ALenum = 0x1027;
pub const AL_STATIC                         : ALenum = 0x1028;
pub const AL_STREAMING                      : ALenum = 0x1029;
pub const AL_UNDETERMINED                   : ALenum = 0x1030;
pub const AL_FORMAT_MONO8                   : ALenum = 0x1100;
pub const AL_FORMAT_MONO16                  : ALenum = 0x1101;
pub const AL_FORMAT_STEREO8                 : ALenum = 0x1102;
pub const AL_FORMAT_STEREO16                : ALenum = 0x1103;
pub const AL_REFERENCE_DISTANCE             : ALenum = 0x1020;
pub const AL_ROLLOFF_FACTOR                 : ALenum = 0x1021;
pub const AL_CONE_OUTER_GAIN                : ALenum = 0x1022;
pub const AL_MAX_DISTANCE                   : ALenum = 0x1023;
pub const AL_FREQUENCY                      : ALenum = 0x2001;
pub const AL_BITS                           : ALenum = 0x2002;
pub const AL_CHANNELS                       : ALenum = 0x2003;
pub const AL_SIZE                           : ALenum = 0x2004;
pub const AL_UNUSED                         : ALenum = 0x2010;
pub const AL_PENDING                        : ALenum = 0x2011;
pub const AL_PROCESSED                      : ALenum = 0x2012;
pub const AL_NO_ERROR                       : ALenum = AL_FALSE;
pub const AL_INVALID_NAME                   : ALenum = 0xA001;
pub const AL_INVALID_ENUM                   : ALenum = 0xA002;
pub const AL_INVALID_VALUE                  : ALenum = 0xA003;
pub const AL_INVALID_OPERATION              : ALenum = 0xA004;
pub const AL_OUT_OF_MEMORY                  : ALenum = 0xA005;
pub const AL_VENDOR                         : ALenum = 0xB001;
pub const AL_VERSION                        : ALenum = 0xB002;
pub const AL_RENDERER                       : ALenum = 0xB003;
pub const AL_EXTENSIONS                     : ALenum = 0xB004;
pub const AL_DOPPLER_FACTOR                 : ALenum = 0xC000;
pub const AL_DOPPLER_VELOCITY               : ALenum = 0xC001;
pub const AL_SPEED_OF_SOUND                 : ALenum = 0xC003;
pub const AL_DISTANCE_MODEL                 : ALenum = 0xD000;
pub const AL_INVERSE_DISTANCE               : ALenum = 0xD001;
pub const AL_INVERSE_DISTANCE_CLAMPED       : ALenum = 0xD002;
pub const AL_LINEAR_DISTANCE                : ALenum = 0xD003;
pub const AL_LINEAR_DISTANCE_CLAMPED        : ALenum = 0xD004;
pub const AL_EXPONENT_DISTANCE              : ALenum = 0xD005;
pub const AL_EXPONENT_DISTANCE_CLAMPED      : ALenum = 0xD006;

pub extern {
    pub fn alEnable(capability: ALenum);
    pub fn alDisable(capability: ALenum); 
    pub fn alIsEnabled(capability: ALenum) -> ALboolean;
    pub fn alGetString(param: ALenum) -> *ALchar;
    pub fn alGetBooleanv(param: ALenum, data: *ALboolean);
    pub fn alGetIntegerv(param: ALenum, data: *ALint);
    pub fn alGetFloatv(param: ALenum, data: *ALfloat);
    pub fn alGetDoublev(param: ALenum, data: *ALdouble);
    pub fn alGetBoolean(param: ALenum) -> ALboolean;
    pub fn alGetInteger(param: ALenum) -> ALint;
    pub fn alGetFloat(param: ALenum) -> ALfloat;
    pub fn alGetDouble(param: ALenum) -> ALdouble;
    pub fn alGetError() -> ALenum;
    pub fn alIsExtensionPresent(extname: *ALchar) -> ALboolean;
    pub fn alGetProcAddress(fname: *ALchar) -> *c_void;
    pub fn alGetEnumValue(ename: *ALchar) -> ALenum;

    pub fn alListenerf(param: ALenum, value: ALfloat);
    pub fn alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
    pub fn alListenerfv(param: ALenum, values: *ALfloat); 
    pub fn alListeneri(param: ALenum, value: ALint);
    pub fn alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint);
    pub fn alListeneriv(param: ALenum, values: *ALint);
    pub fn alGetListenerf(param: ALenum, value: *ALfloat);
    pub fn alGetListener3f(param: ALenum, value1: *ALfloat, value2: *ALfloat, value3: *ALfloat);
    pub fn alGetListenerfv(param: ALenum, values: *ALfloat);
    pub fn alGetListeneri(param: ALenum, value: *ALint);
    pub fn alGetListener3i(param: ALenum, value1: *ALint, value2: *ALint, value3: *ALint);
    pub fn alGetListeneriv(param: ALenum, values: *ALint);
    pub fn alGenSources(n: ALsizei, sources: *ALuint); 
    pub fn alDeleteSources(n: ALsizei, sources: ALuint);
    pub fn alIsSource(sid: ALuint) -> ALboolean;
    pub fn alSourcef(sid: ALuint, param: ALenum, value: ALfloat); 
    pub fn alSource3f(sid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
    pub fn alSourcefv(sid: ALuint, param: ALenum, values: *ALfloat); 
    pub fn alSourcei(sid: ALuint, param: ALenum, value: ALint); 
    pub fn alSource3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
    pub fn alSourceiv(sid: ALuint, param: ALenum, values: *ALint);
    pub fn alGetSourcef(sid: ALuint, param: ALenum, value: *ALfloat);
    pub fn alGetSource3f(sid: ALuint, param: ALenum, value1: *ALfloat, value2: *ALfloat, value3: *ALfloat);
    pub fn alGetSourcefv(sid: ALuint, param: ALenum, values: *ALfloat);
    pub fn alGetSourcei(sid: ALuint,  param: ALenum, value: *ALint);
    pub fn alGetSource3i(sid: ALuint, param: ALenum, value1: *ALint, value2: *ALint, value3: *ALint);
    pub fn alGetSourceiv(sid: ALuint,  param: ALenum, values: *ALint);
    pub fn alSourcePlayv(ns: ALsizei, sids: *ALuint);
    pub fn alSourceStopv(ns: ALsizei, sids: *ALuint);
    pub fn alSourceRewindv(ns: ALsizei, sids: *ALuint);
    pub fn alSourcePausev(ns: ALsizei, sids: *ALuint);
    pub fn alSourcePlay(sid: ALuint);
    pub fn alSourceStop(sid: ALuint);
    pub fn alSourceRewind(sid: ALuint);
    pub fn alSourcePause(sid: ALuint);
    pub fn alSourceQueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *ALuint);
    pub fn alSourceUnqueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *ALuint);
    pub fn alGenBuffers(n: ALsizei, buffers: *ALuint);
    pub fn alDeleteBuffers(n: ALsizei, buffers: *ALuint);
    pub fn alIsBuffer(bid: ALuint) -> ALboolean;
    pub fn alBufferData(bid: ALuint, format: ALenum, data: *ALvoid, size: ALsizei, freq: ALsizei);
    pub fn alBufferf(bid: ALuint, param: ALenum, value: ALfloat);
    pub fn alBuffer3f(bid: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
    pub fn alBufferfv(bid: ALuint, param: ALenum, values: *ALfloat);
    pub fn alBufferi(bid: ALuint, param: ALenum, value: ALint);
    pub fn alBuffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
    pub fn alBufferiv(bid: ALuint, param: ALenum, values: *ALint);
    pub fn alGetBufferf(bid: ALuint, param: ALenum, value: *ALfloat);
    pub fn alGetBuffer3f(bid: ALuint, param: ALenum, value1: *ALfloat, value2: *ALfloat, value3: *ALfloat);
    pub fn alGetBufferfv(bid: ALuint, param: ALenum, values: *ALfloat);
    pub fn alGetBufferi(bid: ALuint, param: ALenum, value: *ALint);
    pub fn alGetBuffer3i(bid: ALuint, param: ALenum, value1: *ALint, value2: *ALint, value3: *ALint);
    pub fn alGetBufferiv(bid: ALuint, param: ALenum, values: *ALint);
    pub fn alDopplerFactor(value: ALfloat);
    pub fn alDopplerVelocity(value: ALfloat);
    pub fn alSpeedOfSound(value: ALfloat);
}