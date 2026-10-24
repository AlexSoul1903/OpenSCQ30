use std::borrow::Cow;

use cosmic::{Element, iced::alignment, widget};
use openscq30_lib::settings;

use crate::device_settings::labeled_setting_row;

pub fn i32_range<'a, M>(
    name: Cow<'a, str>,
    range: settings::Range<i32>,
    value: i32,
    on_change: impl Fn(i32) -> M + 'a,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    labeled_setting_row(
        name,
        widget::row![
            widget::text::body(value.to_string())
                .font(crate::theme::NUMERIC)
                .width(64),
            widget::slider(range.range, value, on_change)
                .step(range.step)
                .class(crate::theme::audio_slider()),
        ]
        .spacing(12)
        .align_y(alignment::Vertical::Center),
    )
}
