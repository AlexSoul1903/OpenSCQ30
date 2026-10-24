use std::borrow::Cow;

use cosmic::{Element, iced::core::text::Wrapping, widget};

pub fn information<'a, M>(
    name: Cow<'a, str>,
    text: Cow<'a, str>,
    on_copy: M,
    is_alert: bool,
) -> Element<'a, M>
where
    M: Clone + 'static,
{
    let value = widget::text(text)
        .font(crate::theme::NUMERIC)
        .wrapping(Wrapping::WordOrGlyph);
    let value = if is_alert {
        value.class(cosmic::theme::Text::Color(crate::theme::ALERT))
    } else {
        value
    };
    let readout = widget::column![value]
        .push_maybe(is_alert.then(|| {
            widget::text::caption(crate::fl!("studio-low-battery"))
                .class(cosmic::theme::Text::Color(crate::theme::ALERT))
        }))
        .spacing(4);
    widget::settings::item::builder(name)
        // The copy button being with the title rather than the value isn't ideal, but putting it with the value causes
        // layout issues. If those layout issues are ever fixed, remove this comment and replace the flex control with
        // a row containing text and the icon button.
        .icon(widget::button::icon(widget::icon::from_name("edit-copy-symbolic")).on_press(on_copy))
        .flex_control(readout)
        .into()
}
