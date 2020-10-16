/* automatically generated by rust-bindgen */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const OPUS_OK: ::std::os::raw::c_int = 0;
pub const OPUS_BAD_ARG: ::std::os::raw::c_int = -1;
pub const OPUS_BUFFER_TOO_SMALL: ::std::os::raw::c_int = -2;
pub const OPUS_INTERNAL_ERROR: ::std::os::raw::c_int = -3;
pub const OPUS_INVALID_PACKET: ::std::os::raw::c_int = -4;
pub const OPUS_UNIMPLEMENTED: ::std::os::raw::c_int = -5;
pub const OPUS_INVALID_STATE: ::std::os::raw::c_int = -6;
pub const OPUS_ALLOC_FAIL: ::std::os::raw::c_int = -7;
pub const OPUS_SET_APPLICATION_REQUEST: ::std::os::raw::c_int = 4000;
pub const OPUS_GET_APPLICATION_REQUEST: ::std::os::raw::c_int = 4001;
pub const OPUS_SET_BITRATE_REQUEST: ::std::os::raw::c_int = 4002;
pub const OPUS_GET_BITRATE_REQUEST: ::std::os::raw::c_int = 4003;
pub const OPUS_SET_MAX_BANDWIDTH_REQUEST: ::std::os::raw::c_int = 4004;
pub const OPUS_GET_MAX_BANDWIDTH_REQUEST: ::std::os::raw::c_int = 4005;
pub const OPUS_SET_VBR_REQUEST: ::std::os::raw::c_int = 4006;
pub const OPUS_GET_VBR_REQUEST: ::std::os::raw::c_int = 4007;
pub const OPUS_SET_BANDWIDTH_REQUEST: ::std::os::raw::c_int = 4008;
pub const OPUS_GET_BANDWIDTH_REQUEST: ::std::os::raw::c_int = 4009;
pub const OPUS_SET_COMPLEXITY_REQUEST: ::std::os::raw::c_int = 4010;
pub const OPUS_GET_COMPLEXITY_REQUEST: ::std::os::raw::c_int = 4011;
pub const OPUS_SET_INBAND_FEC_REQUEST: ::std::os::raw::c_int = 4012;
pub const OPUS_GET_INBAND_FEC_REQUEST: ::std::os::raw::c_int = 4013;
pub const OPUS_SET_PACKET_LOSS_PERC_REQUEST: ::std::os::raw::c_int = 4014;
pub const OPUS_GET_PACKET_LOSS_PERC_REQUEST: ::std::os::raw::c_int = 4015;
pub const OPUS_SET_DTX_REQUEST: ::std::os::raw::c_int = 4016;
pub const OPUS_GET_DTX_REQUEST: ::std::os::raw::c_int = 4017;
pub const OPUS_SET_VBR_CONSTRAINT_REQUEST: ::std::os::raw::c_int = 4020;
pub const OPUS_GET_VBR_CONSTRAINT_REQUEST: ::std::os::raw::c_int = 4021;
pub const OPUS_SET_FORCE_CHANNELS_REQUEST: ::std::os::raw::c_int = 4022;
pub const OPUS_GET_FORCE_CHANNELS_REQUEST: ::std::os::raw::c_int = 4023;
pub const OPUS_SET_SIGNAL_REQUEST: ::std::os::raw::c_int = 4024;
pub const OPUS_GET_SIGNAL_REQUEST: ::std::os::raw::c_int = 4025;
pub const OPUS_GET_LOOKAHEAD_REQUEST: ::std::os::raw::c_int = 4027;
pub const OPUS_GET_SAMPLE_RATE_REQUEST: ::std::os::raw::c_int = 4029;
pub const OPUS_GET_FINAL_RANGE_REQUEST: ::std::os::raw::c_int = 4031;
pub const OPUS_GET_PITCH_REQUEST: ::std::os::raw::c_int = 4033;
pub const OPUS_SET_GAIN_REQUEST: ::std::os::raw::c_int = 4034;
pub const OPUS_GET_GAIN_REQUEST: ::std::os::raw::c_int = 4045;
pub const OPUS_SET_LSB_DEPTH_REQUEST: ::std::os::raw::c_int = 4036;
pub const OPUS_GET_LSB_DEPTH_REQUEST: ::std::os::raw::c_int = 4037;
pub const OPUS_GET_LAST_PACKET_DURATION_REQUEST: ::std::os::raw::c_int = 4039;
pub const OPUS_SET_EXPERT_FRAME_DURATION_REQUEST: ::std::os::raw::c_int = 4040;
pub const OPUS_GET_EXPERT_FRAME_DURATION_REQUEST: ::std::os::raw::c_int = 4041;
pub const OPUS_SET_PREDICTION_DISABLED_REQUEST: ::std::os::raw::c_int = 4042;
pub const OPUS_GET_PREDICTION_DISABLED_REQUEST: ::std::os::raw::c_int = 4043;
pub const OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST: ::std::os::raw::c_int = 4046;
pub const OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST: ::std::os::raw::c_int = 4047;
pub const OPUS_GET_IN_DTX_REQUEST: ::std::os::raw::c_int = 4049;
pub const OPUS_AUTO: ::std::os::raw::c_int = -1000;
pub const OPUS_BITRATE_MAX: ::std::os::raw::c_int = -1;
pub const OPUS_APPLICATION_VOIP: ::std::os::raw::c_int = 2048;
pub const OPUS_APPLICATION_AUDIO: ::std::os::raw::c_int = 2049;
pub const OPUS_APPLICATION_RESTRICTED_LOWDELAY: ::std::os::raw::c_int = 2051;
pub const OPUS_SIGNAL_VOICE: ::std::os::raw::c_int = 3001;
pub const OPUS_SIGNAL_MUSIC: ::std::os::raw::c_int = 3002;
pub const OPUS_BANDWIDTH_NARROWBAND: ::std::os::raw::c_int = 1101;
pub const OPUS_BANDWIDTH_MEDIUMBAND: ::std::os::raw::c_int = 1102;
pub const OPUS_BANDWIDTH_WIDEBAND: ::std::os::raw::c_int = 1103;
pub const OPUS_BANDWIDTH_SUPERWIDEBAND: ::std::os::raw::c_int = 1104;
pub const OPUS_BANDWIDTH_FULLBAND: ::std::os::raw::c_int = 1105;
pub const OPUS_FRAMESIZE_ARG: ::std::os::raw::c_int = 5000;
pub const OPUS_FRAMESIZE_2_5_MS: ::std::os::raw::c_int = 5001;
pub const OPUS_FRAMESIZE_5_MS: ::std::os::raw::c_int = 5002;
pub const OPUS_FRAMESIZE_10_MS: ::std::os::raw::c_int = 5003;
pub const OPUS_FRAMESIZE_20_MS: ::std::os::raw::c_int = 5004;
pub const OPUS_FRAMESIZE_40_MS: ::std::os::raw::c_int = 5005;
pub const OPUS_FRAMESIZE_60_MS: ::std::os::raw::c_int = 5006;
pub const OPUS_FRAMESIZE_80_MS: ::std::os::raw::c_int = 5007;
pub const OPUS_FRAMESIZE_100_MS: ::std::os::raw::c_int = 5008;
pub const OPUS_FRAMESIZE_120_MS: ::std::os::raw::c_int = 5009;
pub const OPUS_RESET_STATE: ::std::os::raw::c_int = 4028;
pub const OPUS_MULTISTREAM_GET_ENCODER_STATE_REQUEST: ::std::os::raw::c_int = 5120;
pub const OPUS_MULTISTREAM_GET_DECODER_STATE_REQUEST: ::std::os::raw::c_int = 5122;
pub type opus_int32 = ::std::os::raw::c_int;
pub type opus_uint32 = ::std::os::raw::c_uint;
pub type opus_int16 = ::std::os::raw::c_short;
pub type opus_uint16 = ::std::os::raw::c_ushort;
extern "C" {
    pub fn opus_strerror(error: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn opus_get_version_string() -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpusEncoder {
    _unused: [u8; 0],
}
extern "C" {
    pub fn opus_encoder_get_size(channels: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_encoder_create(
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        application: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut OpusEncoder;
}
extern "C" {
    pub fn opus_encoder_init(
        st: *mut OpusEncoder,
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        application: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_encode(
        st: *mut OpusEncoder,
        pcm: *const opus_int16,
        frame_size: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_uchar,
        max_data_bytes: opus_int32,
    ) -> opus_int32;
}
extern "C" {
    pub fn opus_encode_float(
        st: *mut OpusEncoder,
        pcm: *const f32,
        frame_size: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_uchar,
        max_data_bytes: opus_int32,
    ) -> opus_int32;
}
extern "C" {
    pub fn opus_encoder_destroy(st: *mut OpusEncoder);
}
extern "C" {
    pub fn opus_encoder_ctl(
        st: *mut OpusEncoder,
        request: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpusDecoder {
    _unused: [u8; 0],
}
extern "C" {
    pub fn opus_decoder_get_size(channels: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_decoder_create(
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut OpusDecoder;
}
extern "C" {
    pub fn opus_decoder_init(
        st: *mut OpusDecoder,
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_decode(
        st: *mut OpusDecoder,
        data: *const ::std::os::raw::c_uchar,
        len: opus_int32,
        pcm: *mut opus_int16,
        frame_size: ::std::os::raw::c_int,
        decode_fec: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_decode_float(
        st: *mut OpusDecoder,
        data: *const ::std::os::raw::c_uchar,
        len: opus_int32,
        pcm: *mut f32,
        frame_size: ::std::os::raw::c_int,
        decode_fec: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_decoder_ctl(
        st: *mut OpusDecoder,
        request: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_decoder_destroy(st: *mut OpusDecoder);
}
extern "C" {
    pub fn opus_packet_parse(
        data: *const ::std::os::raw::c_uchar,
        len: opus_int32,
        out_toc: *mut ::std::os::raw::c_uchar,
        frames: *mut *const ::std::os::raw::c_uchar,
        size: *mut opus_int16,
        payload_offset: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_packet_get_bandwidth(data: *const ::std::os::raw::c_uchar)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_packet_get_samples_per_frame(
        data: *const ::std::os::raw::c_uchar,
        Fs: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_packet_get_nb_channels(
        data: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_packet_get_nb_frames(
        packet: *const ::std::os::raw::c_uchar,
        len: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_packet_get_nb_samples(
        packet: *const ::std::os::raw::c_uchar,
        len: opus_int32,
        Fs: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_decoder_get_nb_samples(
        dec: *const OpusDecoder,
        packet: *const ::std::os::raw::c_uchar,
        len: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_pcm_soft_clip(
        pcm: *mut f32,
        frame_size: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
        softclip_mem: *mut f32,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpusRepacketizer {
    _unused: [u8; 0],
}
extern "C" {
    pub fn opus_repacketizer_get_size() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_repacketizer_init(rp: *mut OpusRepacketizer) -> *mut OpusRepacketizer;
}
extern "C" {
    pub fn opus_repacketizer_create() -> *mut OpusRepacketizer;
}
extern "C" {
    pub fn opus_repacketizer_destroy(rp: *mut OpusRepacketizer);
}
extern "C" {
    pub fn opus_repacketizer_cat(
        rp: *mut OpusRepacketizer,
        data: *const ::std::os::raw::c_uchar,
        len: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_repacketizer_out_range(
        rp: *mut OpusRepacketizer,
        begin: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_uchar,
        maxlen: opus_int32,
    ) -> opus_int32;
}
extern "C" {
    pub fn opus_repacketizer_get_nb_frames(rp: *mut OpusRepacketizer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_repacketizer_out(
        rp: *mut OpusRepacketizer,
        data: *mut ::std::os::raw::c_uchar,
        maxlen: opus_int32,
    ) -> opus_int32;
}
extern "C" {
    pub fn opus_packet_pad(
        data: *mut ::std::os::raw::c_uchar,
        len: opus_int32,
        new_len: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_packet_unpad(data: *mut ::std::os::raw::c_uchar, len: opus_int32) -> opus_int32;
}
extern "C" {
    pub fn opus_multistream_packet_pad(
        data: *mut ::std::os::raw::c_uchar,
        len: opus_int32,
        new_len: opus_int32,
        nb_streams: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_packet_unpad(
        data: *mut ::std::os::raw::c_uchar,
        len: opus_int32,
        nb_streams: ::std::os::raw::c_int,
    ) -> opus_int32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpusMSEncoder {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpusMSDecoder {
    _unused: [u8; 0],
}
extern "C" {
    pub fn opus_multistream_encoder_get_size(
        streams: ::std::os::raw::c_int,
        coupled_streams: ::std::os::raw::c_int,
    ) -> opus_int32;
}
extern "C" {
    pub fn opus_multistream_surround_encoder_get_size(
        channels: ::std::os::raw::c_int,
        mapping_family: ::std::os::raw::c_int,
    ) -> opus_int32;
}
extern "C" {
    pub fn opus_multistream_encoder_create(
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        streams: ::std::os::raw::c_int,
        coupled_streams: ::std::os::raw::c_int,
        mapping: *const ::std::os::raw::c_uchar,
        application: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut OpusMSEncoder;
}
extern "C" {
    pub fn opus_multistream_surround_encoder_create(
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        mapping_family: ::std::os::raw::c_int,
        streams: *mut ::std::os::raw::c_int,
        coupled_streams: *mut ::std::os::raw::c_int,
        mapping: *mut ::std::os::raw::c_uchar,
        application: ::std::os::raw::c_int,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut OpusMSEncoder;
}
extern "C" {
    pub fn opus_multistream_encoder_init(
        st: *mut OpusMSEncoder,
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        streams: ::std::os::raw::c_int,
        coupled_streams: ::std::os::raw::c_int,
        mapping: *const ::std::os::raw::c_uchar,
        application: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_surround_encoder_init(
        st: *mut OpusMSEncoder,
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        mapping_family: ::std::os::raw::c_int,
        streams: *mut ::std::os::raw::c_int,
        coupled_streams: *mut ::std::os::raw::c_int,
        mapping: *mut ::std::os::raw::c_uchar,
        application: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_encode(
        st: *mut OpusMSEncoder,
        pcm: *const opus_int16,
        frame_size: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_uchar,
        max_data_bytes: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_encode_float(
        st: *mut OpusMSEncoder,
        pcm: *const f32,
        frame_size: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_uchar,
        max_data_bytes: opus_int32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_encoder_destroy(st: *mut OpusMSEncoder);
}
extern "C" {
    pub fn opus_multistream_encoder_ctl(
        st: *mut OpusMSEncoder,
        request: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_decoder_get_size(
        streams: ::std::os::raw::c_int,
        coupled_streams: ::std::os::raw::c_int,
    ) -> opus_int32;
}
extern "C" {
    pub fn opus_multistream_decoder_create(
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        streams: ::std::os::raw::c_int,
        coupled_streams: ::std::os::raw::c_int,
        mapping: *const ::std::os::raw::c_uchar,
        error: *mut ::std::os::raw::c_int,
    ) -> *mut OpusMSDecoder;
}
extern "C" {
    pub fn opus_multistream_decoder_init(
        st: *mut OpusMSDecoder,
        Fs: opus_int32,
        channels: ::std::os::raw::c_int,
        streams: ::std::os::raw::c_int,
        coupled_streams: ::std::os::raw::c_int,
        mapping: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_decode(
        st: *mut OpusMSDecoder,
        data: *const ::std::os::raw::c_uchar,
        len: opus_int32,
        pcm: *mut opus_int16,
        frame_size: ::std::os::raw::c_int,
        decode_fec: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_decode_float(
        st: *mut OpusMSDecoder,
        data: *const ::std::os::raw::c_uchar,
        len: opus_int32,
        pcm: *mut f32,
        frame_size: ::std::os::raw::c_int,
        decode_fec: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_decoder_ctl(
        st: *mut OpusMSDecoder,
        request: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn opus_multistream_decoder_destroy(st: *mut OpusMSDecoder);
}