use crate::{equalizer_line::EqualizerLine, fl, theme};
use cosmic::{
    Element,
    iced::{Length, alignment},
    widget,
};
use openscq30_lib::settings::Equalizer;

fn gain(setting: &Equalizer, value: i16) -> String {
    let digits = setting.fraction_digits.max(0) as usize;
    let divisor = 10f32.powi(digits as i32);
    format!("{:+.*}", digits, f32::from(value) / divisor)
}

fn plot_header<'a, M: 'static>(setting: &Equalizer) -> Element<'a, M> {
    widget::row![
        widget::text::heading(fl!("studio-eq-curve")).width(Length::Fill),
        widget::text::caption(format!(
            "{} / {} dB",
            gain(setting, setting.min),
            gain(setting, setting.max)
        ))
        .font(theme::NUMERIC),
    ]
    .spacing(8)
    .into()
}

pub fn horizontal_equalizer<'a, M>(
    setting: &'a Equalizer,
    value: &'a [i16],
    on_change: impl Fn(u8, i16) -> M + 'static + Clone,
) -> Vec<Element<'a, M>>
where
    M: Clone + 'static,
{
    let curve = widget::canvas(
        EqualizerLine::new(setting.min, setting.max, value.to_vec()).on_change(on_change.clone()),
    )
    .width(Length::Fill)
    .height(200);
    let bands = widget::responsive(move |size| {
        let columns = if size.width >= theme::COMPACT_WIDTH {
            2
        } else {
            1
        };
        let mut rows = widget::column::with_capacity(setting.band_hz.len()).spacing(16);
        for start in (0..setting.band_hz.len()).step_by(columns) {
            let mut row = widget::row::with_capacity(columns).spacing(24);
            for index in start..(start + columns).min(setting.band_hz.len()) {
                let hz = setting.band_hz[index];
                let current = value
                    .get(index)
                    .copied()
                    .unwrap_or(0)
                    .clamp(setting.min, setting.max);
                let slider_change = on_change.clone();
                let spin_change = on_change.clone();
                row = row.push(
                    widget::column![
                        widget::row![
                            widget::text(fl!("hz", hz = hz))
                                .font(theme::SEMIBOLD)
                                .width(Length::Fill),
                            widget::text(fl!("db", db = gain(setting, current)))
                                .font(theme::NUMERIC)
                                .width(88),
                        ]
                        .spacing(8),
                        widget::row![
                            widget::slider(setting.min..=setting.max, current, move |v| {
                                slider_change(index as u8, v)
                            })
                            .name(fl!("hz", hz = hz))
                            .class(theme::audio_slider()),
                            widget::spin_button(
                                gain(setting, current),
                                fl!("hz", hz = hz),
                                current,
                                1,
                                setting.min,
                                setting.max,
                                move |v| spin_change(index as u8, v)
                            ),
                        ]
                        .spacing(8)
                        .align_y(alignment::Vertical::Center),
                    ]
                    .spacing(8)
                    .width(Length::Fill),
                );
            }
            rows = rows.push(row);
        }
        rows.into()
    })
    .height(Length::Shrink);
    vec![
        widget::column![plot_header(setting), curve, bands]
            .spacing(16)
            .padding(8)
            .into(),
    ]
}

pub fn equalizer_preview<'a, M: Clone + 'static>(
    setting: &'a Equalizer,
    value: &'a [i16],
) -> Element<'a, M> {
    let readouts = widget::responsive(move |size| {
        let columns = ((size.width / 100.0).floor() as usize).max(1);
        let mut rows = widget::column::with_capacity(value.len()).spacing(12);
        for start in (0..setting.band_hz.len()).step_by(columns) {
            let mut row = widget::row::with_capacity(columns).spacing(12);
            for index in start..(start + columns).min(setting.band_hz.len()) {
                row = row.push(
                    widget::column![
                        widget::text::caption(fl!("hz", hz = setting.band_hz[index])),
                        widget::text(fl!(
                            "db",
                            db = gain(setting, value.get(index).copied().unwrap_or(0))
                        ))
                        .font(theme::NUMERIC),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                );
            }
            rows = rows.push(row);
        }
        rows.into()
    })
    .height(Length::Shrink);
    widget::column![
        plot_header(setting),
        widget::canvas(EqualizerLine::new(setting.min, setting.max, value.to_vec()))
            .width(Length::Fill)
            .height(200),
        readouts,
        widget::text::caption(fl!("studio-eq-readonly")),
    ]
    .spacing(16)
    .padding(8)
    .into()
}
