use std::collections::HashMap;

use derive_more::Display;

use super::{separated, to_string_or_empty};

#[derive(Display)]
pub enum Font {
    #[display(fmt = "pango:{_0}")]
    Pango(FontDescription),
    Normal(FontDescription),
}

#[derive(Display)]
#[display(
    fmt = "{} {style_options} {} {}",
    "separated(families, ',')",
    "to_string_or_empty(size)",
    "separated(variations.iter().map(|(axis,value)| format!(\"`{axis}`={value}\")), ',')"
)]
pub struct FontDescription {
    families: Vec<String>,
    style_options: FontStyleOptions,
    size: Option<FontSize>,
    variations: HashMap<String, String>,
}

#[derive(Display, Default)]
#[display(
    fmt = "{} {} {} {} {}",
    "to_string_or_empty(style)",
    "to_string_or_empty(variant)",
    "to_string_or_empty(weight)",
    "to_string_or_empty(stretch)",
    "to_string_or_empty(gravity)"
)]
pub struct FontStyleOptions {
    style: Option<FontStyle>,
    variant: Option<FontVariant>,
    weight: Option<FontWeight>,
    stretch: Option<FontStretch>,
    gravity: Option<FontGravity>,
}

#[derive(Display)]
pub enum FontStyle {
    #[display(fmt = "Normal")]
    Normal,
    #[display(fmt = "Roman")]
    Roman,
    #[display(fmt = "Oblique")]
    Oblique,
    #[display(fmt = "Italic")]
    Italic,
}

#[derive(Display)]
pub enum FontVariant {
    #[display(fmt = "Small-Caps")]
    SmallCaps,
    #[display(fmt = "All-Small-Caps")]
    AllSmallCaps,
    #[display(fmt = "Petite-Caps")]
    PetiteCaps,
    #[display(fmt = "All-Petite-Caps")]
    AllPetiteCaps,
    #[display(fmt = "Unicase")]
    Unicase,
    #[display(fmt = "Title-Caps")]
    TitleCaps,
}

#[derive(Display)]
pub enum FontWeight {
    #[display(fmt = "Thin")]
    Thin,
    #[display(fmt = "Ultra-Light")]
    UltraLight,
    #[display(fmt = "Extra-Light")]
    ExtraLight,
    #[display(fmt = "Light")]
    Light,
    #[display(fmt = "Semi-Light")]
    SemiLight,
    #[display(fmt = "Demi-Light")]
    DemiLight,
    #[display(fmt = "Book")]
    Book,
    #[display(fmt = "Regular")]
    Regular,
    #[display(fmt = "Medium")]
    Medium,
    #[display(fmt = "Semi-Bold")]
    SemiBold,
    #[display(fmt = "Demi-Bold")]
    DemiBold,
    #[display(fmt = "Bold")]
    Bold,
    #[display(fmt = "Ultra-Bold")]
    UltraBold,
    #[display(fmt = "Extra-Bold")]
    ExtraBold,
    #[display(fmt = "Heavy")]
    Heavy,
    #[display(fmt = "Black")]
    Black,
    #[display(fmt = "Ultra-Black")]
    UltraBlack,
    #[display(fmt = "Extra-Black")]
    ExtraBlack,
}

#[derive(Display)]
pub enum FontStretch {
    #[display(fmt = "Ultra-Condensed")]
    UltraCondensed,
    #[display(fmt = "Extra-Condensed")]
    ExtraCondensed,
    #[display(fmt = "Condensed")]
    Condensed,
    #[display(fmt = "Semi-Condensed")]
    SemiCondensed,
    #[display(fmt = "Semi-Expanded")]
    SemiExpanded,
    #[display(fmt = "Expanded")]
    Expanded,
    #[display(fmt = "Extra-Expanded")]
    ExtraExpanded,
    #[display(fmt = "Ultra-Expanded")]
    UltraExpanded,
}

#[derive(Display)]
pub enum FontGravity {
    #[display(fmt = "Not-Rotated")]
    NotRotated,
    #[display(fmt = "South")]
    South,
    #[display(fmt = "Upside-Down")]
    UpsideDown,
    #[display(fmt = "North")]
    North,
    #[display(fmt = "Rotated-Left")]
    RotatedLeft,
    #[display(fmt = "East")]
    East,
    #[display(fmt = "Rotated-Right")]
    RotatedRight,
    #[display(fmt = "West")]
    West,
}

#[derive(Display)]
pub enum FontSize {
    Pt(f32),
    #[display(fmt = "{_0} px")]
    Px(f32),
}
