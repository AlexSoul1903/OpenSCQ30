//! Audio Studio design tokens. Native COSMIC/iced equivalents of the UI style guide.
use std::{
    borrow::Cow,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use cosmic::{
    cosmic_theme::{ThemeBuilder, palette::Srgb},
    iced::{Color, Font, font::Weight, widget::slider},
    widget,
};

pub const SHELL: Color = Color::from_rgb8(0x01, 0x19, 0x36);
pub const SURFACE: Color = Color::from_rgb8(0x46, 0x53, 0x62);
pub const TEXT: Color = Color::from_rgb8(0xF4, 0xFF, 0xFD);
pub const ALERT: Color = Color::from_rgb8(0xED, 0x25, 0x4E);
pub const GOLD: Color = Color::from_rgb8(0xF9, 0xDC, 0x5C);
pub const GAP: u16 = 16;
pub const PAGE_PADDING: u16 = 24;
pub const COMPACT_WIDTH: f32 = 640.0;
pub const MOTION_MS: f32 = 180.0;
static REDUCED_MOTION: AtomicBool = AtomicBool::new(false);

pub fn set_reduced_motion(reduced: bool) {
    REDUCED_MOTION.store(reduced, Ordering::Relaxed);
}
pub fn motion_enabled() -> bool {
    !REDUCED_MOTION.load(Ordering::Relaxed)
}
pub const FONT: Font = Font::with_name("Sora");
pub const SEMIBOLD: Font = Font {
    weight: Weight::Semibold,
    ..FONT
};
// A bundled Sora derivative with tabular figures enabled in its default cmap.
// iced has no CSS/OpenType feature switch, so numbers retain the geometric face.
pub const NUMERIC: Font = Font {
    weight: Weight::Semibold,
    ..Font::with_name("OpenSCQ30 Metrics")
};

pub fn initialize_fonts() {
    let mut fonts = cosmic::iced::advanced::graphics::text::font_system()
        .write()
        .expect("font database lock");
    fonts.load_font(Cow::Borrowed(include_bytes!("../resources/fonts/Sora.ttf")));
    fonts.load_font(Cow::Borrowed(include_bytes!(
        "../resources/fonts/OpenSCQ30Metrics.ttf"
    )));
    drop(fonts);
    // Process-local preferences; no system font settings are written.
    let mut toolkit = cosmic::config::COSMIC_TK
        .write()
        .expect("toolkit font lock");
    toolkit.interface_font.family = "Sora".into();
}

fn rgb(color: Color) -> Srgb {
    Srgb::new(color.r, color.g, color.b)
}

pub fn studio() -> cosmic::Theme {
    let mut builder = ThemeBuilder::dark()
        .bg_color(rgb(SHELL).into())
        .primary_container_bg(rgb(SURFACE).into())
        .text_tint(rgb(TEXT))
        .accent(rgb(GOLD));
    builder.secondary_container_bg = Some(rgb(SURFACE).into());
    builder.success = Some(rgb(GOLD));
    builder.destructive = Some(rgb(ALERT));
    builder.warning = Some(rgb(ALERT));
    let mut theme = builder.build();
    theme.name = "Audio Studio".into();
    theme.accent.on = rgb(SHELL).into();
    theme.accent_button.on = rgb(SHELL).into();
    theme.accent_button.base = rgb(GOLD).into();
    theme.destructive.on = rgb(TEXT).into();
    theme.destructive_button.on = rgb(TEXT).into();
    theme.destructive_button.base = rgb(ALERT).into();
    theme.warning.on = rgb(TEXT).into();
    cosmic::Theme::custom(Arc::new(theme))
}

/// Use the theme's normal colors in the optional system/light/dark themes.
pub fn is_studio(theme: &cosmic::Theme) -> bool {
    theme.cosmic().name == "Audio Studio"
}

pub fn panel(theme: &cosmic::Theme) -> widget::container::Style {
    use cosmic::iced::{Border, Shadow, Vector};
    widget::container::Style {
        background: Some(
            if is_studio(theme) {
                SURFACE
            } else {
                theme.cosmic().primary(false).base.into()
            }
            .into(),
        ),
        text_color: Some(if is_studio(theme) {
            TEXT
        } else {
            theme.cosmic().primary(false).on.into()
        }),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: Color { a: 0.12, ..TEXT },
        },
        shadow: Shadow {
            color: Color { a: 0.15, ..SHELL },
            offset: Vector::new(0.0, 6.0),
            blur_radius: 18.0,
        },
        ..Default::default()
    }
}

pub fn audio_slider() -> cosmic::theme::style::iced::Slider {
    fn style(theme: &cosmic::Theme, active: bool) -> slider::Style {
        let accent = if is_studio(theme) {
            GOLD
        } else {
            theme.cosmic().accent_color().into()
        };
        slider::Style {
            rail: slider::Rail {
                backgrounds: (accent.into(), Color { a: 0.3, ..SHELL }.into()),
                width: 5.0,
                border: cosmic::iced::Border {
                    radius: 3.0.into(),
                    ..Default::default()
                },
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle {
                    radius: if active { 9.0 } else { 7.0 },
                },
                background: TEXT.into(),
                border_width: if active { 3.0 } else { 0.0 },
                border_color: accent,
            },
            breakpoint: slider::Breakpoint { color: TEXT },
        }
    }
    cosmic::theme::style::iced::Slider::Custom {
        active: Rc::new(|theme| style(theme, false)),
        hovered: Rc::new(|theme| style(theme, true)),
        dragging: Rc::new(|theme| style(theme, true)),
    }
}

/// CSS cubic-bezier(0.22, 1, 0.36, 1), evaluated for native canvas transitions.
pub fn ease(progress: f32) -> f32 {
    if !motion_enabled() {
        return 1.0;
    }
    let x = progress.clamp(0.0, 1.0);
    let (mut low, mut high) = (0.0f32, 1.0f32);
    for _ in 0..12 {
        let t = (low + high) * 0.5;
        let curve_x =
            3.0 * (1.0 - t).powi(2) * t * 0.22 + 3.0 * (1.0 - t) * t * t * 0.36 + t.powi(3);
        if curve_x < x {
            low = t;
        } else {
            high = t;
        }
    }
    1.0 - (1.0 - (low + high) * 0.5).powi(3)
}
