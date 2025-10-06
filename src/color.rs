use crate::tables::Layer;

/// Represents an indexed color.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    raw_value: i16,
}

impl Color {
    /// Returns `true` if the color defaults back to the item's layer's color.
    pub fn is_by_layer(&self) -> bool {
        self.raw_value == 256
    }
    /// Returns `true` if the color defaults back to the entity's color.
    pub fn is_by_entity(&self) -> bool {
        self.raw_value == 257
    }
    /// Returns `true` if the color defaults back to the containing block's color.
    pub fn is_by_block(&self) -> bool {
        self.raw_value == 0
    }
    /// Returns `true` if the color represents a `Layer` that is turned off.
    pub fn is_turned_off(&self) -> bool {
        self.raw_value < 0
    }
    /// Sets the color to default back to the item's layer's color.
    pub fn set_by_layer(&mut self) {
        self.raw_value = 256
    }
    /// Sets the color to default back to the containing block's color.
    pub fn set_by_block(&mut self) {
        self.raw_value = 0
    }
    /// Sets the color to default back to the containing entity's color.
    pub fn set_by_entity(&mut self) {
        self.raw_value = 257
    }
    /// Sets the color to represent a `Layer` that is turned off.
    pub fn turn_off(&mut self) {
        self.raw_value = -1
    }
    /// Returns `true` if the color represents a proper color index.
    pub fn is_index(&self) -> bool {
        self.raw_value >= 1 && self.raw_value <= 255
    }
    /// Gets an `Option<u8>` of the indexable value of the color.
    pub fn index(&self) -> Option<u8> {
        if self.is_index() {
            Some(self.raw_value as u8)
        } else {
            None
        }
    }
    pub(crate) fn raw_value(&self) -> i16 {
        self.raw_value
    }
    pub(crate) fn from_raw_value(val: i16) -> Color {
        Color { raw_value: val }
    }
    /// Creates a `Color` that defaults to the item's layer's color.
    pub fn by_layer() -> Color {
        Color { raw_value: 256 }
    }
    /// Creates a `Color` that defaults back to the containing block's color.
    pub fn by_block() -> Color {
        Color { raw_value: 0 }
    }
    /// Creates a `Color` that defaults back to the containing entity's color.
    pub fn by_entity() -> Color {
        Color { raw_value: 257 }
    }
    /// Creates a `Color` from the specified index.
    pub fn from_index(i: u8) -> Color {
        Color {
            raw_value: i16::from(i),
        }
    }
    pub(crate) fn writable_color_value(&self, layer: &Layer) -> i16 {
        let value = self.raw_value().abs();
        if layer.is_layer_on {
            value
        } else {
            -value
        }
    }
}

const DXF_DEFAULT_COLORS: [i32; 256] = [
    0x000000, 0xFF0000, 0xFFFF00, 0x00FF00, 0x00FFFF, 0x0000FF, 0xFF00FF, 0xFFFFFF, 0x808080,
    0xC0C0C0, 0xFF0000, 0xFF7F7F, 0xA50000, 0xA55252, 0x7F0000, 0x7F3F3F, 0x4C0000, 0x4C2626,
    0x260000, 0x261313, 0xFF3F00, 0xFF9F7F, 0xA52900, 0xA56752, 0x7F1F00, 0x7F4F3F, 0x4C1300,
    0x4C2F26, 0x260900, 0x261713, 0xFF7F00, 0xFFBF7F, 0xA55200, 0xA57C52, 0x7F3F00, 0x7F5F3F,
    0x4C2600, 0x4C3926, 0x261300, 0x261C13, 0xFFBF00, 0xFFDF7F, 0xA57C00, 0xA59152, 0x7F5F00,
    0x7F6F3F, 0x4C3900, 0x4C4226, 0x261C00, 0x262113, 0xFFFF00, 0xFFFF7F, 0xA5A500, 0xA5A552,
    0x7F7F00, 0x7F7F3F, 0x4C4C00, 0x4C4C26, 0x262600, 0x262613, 0xBFFF00, 0xDFFF7F, 0x7CA500,
    0x91A552, 0x5F7F00, 0x6F7F3F, 0x394C00, 0x424C26, 0x1C2600, 0x212613, 0x7FFF00, 0xBFFF7F,
    0x52A500, 0x7CA552, 0x3F7F00, 0x5F7F3F, 0x264C00, 0x394C26, 0x132600, 0x1C2613, 0x3FFF00,
    0x9FFF7F, 0x29A500, 0x67A552, 0x1F7F00, 0x4F7F3F, 0x134C00, 0x2F4C26, 0x092600, 0x172613,
    0x00FF00, 0x7FFF7F, 0x00A500, 0x52A552, 0x007F00, 0x3F7F3F, 0x004C00, 0x264C26, 0x002600,
    0x132613, 0x00FF3F, 0x7FFF9F, 0x00A529, 0x52A567, 0x007F1F, 0x3F7F4F, 0x004C13, 0x264C2F,
    0x002609, 0x135817, 0x00FF7F, 0x7FFFBF, 0x00A552, 0x52A57C, 0x007F3F, 0x3F7F5F, 0x004C26,
    0x264C39, 0x002613, 0x13581C, 0x00FFBF, 0x7FFFDF, 0x00A57C, 0x52A591, 0x007F5F, 0x3F7F6F,
    0x004C39, 0x264C42, 0x00261C, 0x135858, 0x00FFFF, 0x7FFFFF, 0x00A5A5, 0x52A5A5, 0x007F7F,
    0x3F7F7F, 0x004C4C, 0x264C4C, 0x002626, 0x135858, 0x00BFFF, 0x7FDFFF, 0x007CA5, 0x5291A5,
    0x005F7F, 0x3F6F7F, 0x00394C, 0x26427E, 0x001C26, 0x135858, 0x007FFF, 0x7FBFFF, 0x0052A5,
    0x527CA5, 0x003F7F, 0x3F5F7F, 0x00264C, 0x26397E, 0x001326, 0x131C58, 0x003FFF, 0x7F9FFF,
    0x0029A5, 0x5267A5, 0x001F7F, 0x3F4F7F, 0x00134C, 0x262F7E, 0x000926, 0x131758, 0x0000FF,
    0x7F7FFF, 0x0000A5, 0x5252A5, 0x00007F, 0x3F3F7F, 0x00004C, 0x26267E, 0x000026, 0x131358,
    0x3F00FF, 0x9F7FFF, 0x2900A5, 0x6752A5, 0x1F007F, 0x4F3F7F, 0x13004C, 0x2F267E, 0x090026,
    0x171358, 0x7F00FF, 0xBF7FFF, 0x5200A5, 0x7C52A5, 0x3F007F, 0x5F3F7F, 0x26004C, 0x39267E,
    0x130026, 0x1C1358, 0xBF00FF, 0xDF7FFF, 0x7C00A5, 0x9152A5, 0x5F007F, 0x6F3F7F, 0x39004C,
    0x42264C, 0x1C0026, 0x581358, 0xFF00FF, 0xFF7FFF, 0xA500A5, 0xA552A5, 0x7F007F, 0x7F3F7F,
    0x4C004C, 0x4C264C, 0x260026, 0x581358, 0xFF00BF, 0xFF7FDF, 0xA5007C, 0xA55291, 0x7F005F,
    0x7F3F6F, 0x4C0039, 0x4C2642, 0x26001C, 0x581358, 0xFF007F, 0xFF7FBF, 0xA50052, 0xA5527C,
    0x7F003F, 0x7F3F5F, 0x4C0026, 0x4C2639, 0x260013, 0x58131C, 0xFF003F, 0xFF7F9F, 0xA50029,
    0xA55267, 0x7F001F, 0x7F3F4F, 0x4C0013, 0x4C262F, 0x260009, 0x581317, 0x000000, 0x656565,
    0x666666, 0x999999, 0xCCCCCC, 0xFFFFFF,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_rgba(rgba: RGBA) -> Self {
        Self::new(rgba.r, rgba.g, rgba.b)
    }

    pub fn from_i32(color: i32) -> Self {
        int_to_rgb(color.abs())
    }

    pub fn from_index(index: i16) -> Option<Self> {
        aci_to_rgb(index).ok()
    }

    pub fn to_rgba(&self, a: Option<u8>) -> RGBA {
        let mut alpha_value: u8 = 255;
        if let Some(alpha) = a {
            alpha_value = alpha;
        };
        RGBA::new(self.r, self.g, self.b, alpha_value)
    }

    pub fn to_floats(&self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        )
    }

    pub fn from_floats(rgb: (f64, f64, f64)) -> Self {
        let r = ((rgb.0 * 255.0).round() as i32).clamp(0, 255) as u8;
        let g = ((rgb.1 * 255.0).round() as i32).clamp(0, 255) as u8;
        let b = ((rgb.2 * 255.0).round() as i32).clamp(0, 255) as u8;
        RGB { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn from_hex(color: &str) -> Result<Self, std::num::ParseIntError> {
        let hex_string = color.trim_start_matches('#');

        let r_str = &hex_string[0..2];
        let g_str = &hex_string[2..4];
        let b_str = &hex_string[4..6];

        let r = u8::from_str_radix(r_str, 16)?;
        let g = u8::from_str_radix(g_str, 16)?;
        let b = u8::from_str_radix(b_str, 16)?;

        Ok(RGB { r, g, b })
    }

    pub fn get_luminance(&self) -> f64 {
        luminance_impl(&[self.r as f64, self.g as f64, self.b as f64])
    }

    pub(crate) fn writable_color_value_fallback(rgb: Option<RGB>, layer: &Layer) -> i32 {
        if let Some(received_rgb) = rgb {
            return received_rgb.writable_color_value(layer);
        }
        if let Ok(true_color_index) = aci_to_rgb(layer.color.raw_value) {
            return true_color_index.writable_color_value(layer);
        }
        if layer.is_layer_on {
            16777215
        } else {
            -16777215
        }
    }

    pub(crate) fn writable_color_value(&self, layer: &Layer) -> i32 {
        let value = rgb_to_int(*self);
        if layer.is_layer_on {
            value
        } else {
            -value
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // 0 = transparent, 255 = opaque
}

impl Default for RGBA {
    fn default() -> Self {
        RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA { r, g, b, a }
    }

    pub fn from_rgb(rgb: RGB) -> Self {
        Self::new(rgb.r, rgb.g, rgb.b, 255)
    }

    pub fn from_index(index: i16) -> Option<Self> {
        if let Ok(rgb) = aci_to_rgb(index) {
            Some(rgb.to_rgba(None))
        } else {
            None
        }
    }

    pub fn to_rgb(&self) -> RGB {
        RGB::new(self.r, self.g, self.b)
    }

    pub fn to_floats(&self) -> (f64, f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
            self.a as f64 / 255.0,
        )
    }

    pub fn from_floats(values: &[f64]) -> Self {
        let r = ((values[0] * 255.0).round() as i32).clamp(0, 255) as u8;
        let g = ((values[1] * 255.0).round() as i32).clamp(0, 255) as u8;
        let b = ((values[2] * 255.0).round() as i32).clamp(0, 255) as u8;

        let a = if values.len() > 3 {
            ((values[3] * 255.0).round() as i32).clamp(0, 255) as u8
        } else {
            255
        };
        RGBA { r, g, b, a }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    pub fn from_hex(color: &str) -> Result<Self, std::num::ParseIntError> {
        let hex_string = color.trim_start_matches('#');

        let r = u8::from_str_radix(&hex_string[0..2], 16)?;
        let g = u8::from_str_radix(&hex_string[2..4], 16)?;
        let b = u8::from_str_radix(&hex_string[4..6], 16)?;
        let a = if hex_string.len() >= 8 {
            u8::from_str_radix(&hex_string[6..8], 16)?
        } else {
            255 // Opaque by default
        };

        Ok(RGBA { r, g, b, a })
    }

    pub fn get_luminance(&self) -> f64 {
        luminance_impl(&[self.r as f64, self.g as f64, self.b as f64])
    }

    pub fn set_opacity_32_bit(&mut self, opacity: i32) {
        let opacidad_byte = opacity & 0xFF;
        self.a = opacidad_byte as u8;
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.a = opacity;
    }

    pub fn set_opacity_float(&mut self, opacity: f64) {
        let opacity_byte = float_to_transparency(opacity) & 0xFF;
        self.a = opacity_byte as u8;
    }
}

fn int_to_rgb(value: i32) -> RGB {
    let value_u32 = value as u32;
    RGB {
        r: ((value_u32 >> 16) & 0xFF) as u8, // red
        g: ((value_u32 >> 8) & 0xFF) as u8,  // green
        b: (value_u32 & 0xFF) as u8,         // blue
    }
}

fn rgb_to_int(rgb: RGB) -> i32 {
    let r = rgb.r as u32;
    let g = rgb.g as u32;
    let b = rgb.b as u32;

    (((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF)) as i32
}

fn aci_to_rgb(index: i16) -> Result<RGB, String> {
    if !(1..=255).contains(&index) {
        return Err(format!("Invalid ACI index: {index}"));
    }
    let color_value = DXF_DEFAULT_COLORS[index as usize];
    Ok(int_to_rgb(color_value))
}

fn luminance_impl(color: &[f64]) -> f64 {
    let r = color[0] / 255.0;
    let g = color[1] / 255.0;
    let b = color[2] / 255.0;

    (0.299 * r * r + 0.587 * g * g + 0.114 * b * b)
        .sqrt()
        .clamp(0.0, 1.0)
}

pub fn float_to_transparency(value: f64) -> i32 {
    // Returns the DXF opacity value as an integer in the 0 a 255 range,
    // where 0 is 100% transparent y 255 es opaque.
    // final value has the flag 0x02000000.

    // Mapping: 0 (opaque) -> 255; 1 (100% transparent) -> 0
    let t_value = ((1.0 - value).clamp(0.0, 1.0) * 255.0).round() as i32;

    // DXF formula: 0x020000TT
    t_value | 0x02000000
}
