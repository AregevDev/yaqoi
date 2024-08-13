pub const QOI_MAGIC: [u8; 4] = [b'q', b'o', b'i', b'f'];
pub const QOI_ENDER: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];

#[derive(Debug, Default, Copy, Clone)]
pub enum QoiChannels {
    Rgb = 3,
    #[default]
    Rgba = 4,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum QoiColorspace {
    #[default]
    Srgb,
    Linear,
}

#[repr(packed)]
#[derive(Debug, Default, Copy, Clone)]
pub struct QoiHeader {
    magic: [u8; 4],
    width: u32,
    height: u32,
    channels: QoiChannels,
    colorspace: QoiColorspace,
}
