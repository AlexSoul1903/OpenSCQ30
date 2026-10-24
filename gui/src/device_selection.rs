use std::sync::Arc;

use cosmic::{
    Apply, Element, Task,
    iced::{Length, alignment},
    widget,
};
use openscq30_i18n::Translate;
use openscq30_lib::{OpenSCQ30Session, storage::PairedDevice};

use crate::{fl, handle_soft_error, theme, utils::coalesce_result};

pub struct DeviceSelectionModel {
    paired_devices: Vec<PairedDevice>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConnectToDevice(usize),
    RemoveDevice(usize),
    AddDevice,
    SetPairedDevices(Vec<PairedDevice>),
    Warning(String),
}

pub enum Action {
    ConnectToDevice(PairedDevice),
    RemoveDevice(PairedDevice),
    AddDevice,
    None,
    Warning(String),
}

impl DeviceSelectionModel {
    pub fn new(session: Arc<OpenSCQ30Session>) -> (Self, Task<Message>) {
        let model = Self {
            paired_devices: Vec::new(),
        };
        (model, Self::refresh_paired_devices(session))
    }

    pub fn refresh_paired_devices(session: Arc<OpenSCQ30Session>) -> Task<Message> {
        Task::future(async move {
            Ok(Message::SetPairedDevices(
                session
                    .paired_devices()
                    .await
                    .map_err(handle_soft_error!())?,
            ))
        })
        .map(coalesce_result)
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::responsive(move |size| {
            let compact = size.width < theme::COMPACT_WIDTH;
            let intro = widget::column![
                widget::text::caption_heading(fl!("openscq30").to_uppercase()),
                widget::text::title1(fl!("studio-title")),
                widget::text::body(fl!("studio-intro")),
            ]
            .spacing(8)
            .width(Length::Fill);
            let add = widget::button::suggested(fl!("add-device"))
                .leading_icon(widget::icon::from_name("list-add-symbolic"))
                .on_press(Message::AddDevice);
            let header: Element<'_, Message> = if compact {
                widget::column![intro, add].spacing(theme::GAP).into()
            } else {
                widget::row![intro, add]
                    .spacing(32)
                    .align_y(alignment::Vertical::Center)
                    .into()
            };
            let mut content = widget::column![header].spacing(24);
            if self.paired_devices.is_empty() {
                content = content.push(
                    widget::container(
                        widget::column![
                            widget::icon::from_name("audio-headphones-symbolic").size(48),
                            widget::text::title3(fl!("studio-ready")),
                            widget::text::body(fl!("studio-empty")),
                            widget::text::body(fl!("studio-setup")),
                        ]
                        .spacing(12),
                    )
                    .width(Length::Fill)
                    .padding(28)
                    .class(cosmic::theme::Container::custom(theme::panel)),
                );
            } else {
                content = content.push(widget::text::heading(fl!("select-device")));
                for item in self.items(compact) {
                    content = content.push(item);
                }
            }
            content = content.push(widget::text::caption(fl!("studio-guide")));
            widget::scrollable(
                widget::container(content)
                    .max_width(1120)
                    .padding(if compact { 16 } else { theme::PAGE_PADDING }),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        })
        .into()
    }

    fn items(&self, compact: bool) -> impl Iterator<Item = Element<'_, Message>> {
        self.paired_devices
            .iter()
            .enumerate()
            .map(move |(index, device)| {
                let identity = widget::row![
                    widget::icon::from_name("audio-headphones-symbolic").size(40),
                    widget::column![
                        widget::text::title4(device.model.translate()),
                        widget::text::caption(device.mac_address.to_string()).font(theme::NUMERIC),
                    ]
                    .spacing(6)
                    .push_maybe(
                        device
                            .is_demo
                            .then(|| widget::text::caption(fl!("demo-mode")))
                    ),
                ]
                .spacing(theme::GAP)
                .align_y(alignment::Vertical::Center)
                .width(Length::Fill);
                let actions = widget::row![
                    widget::button::text(fl!("remove")).on_press(Message::RemoveDevice(index)),
                    widget::button::suggested(fl!("connect"))
                        .on_press(Message::ConnectToDevice(index)),
                ]
                .spacing(8)
                .align_y(alignment::Vertical::Center);
                let layout: Element<'_, Message> = if compact {
                    widget::column![identity, actions]
                        .spacing(theme::GAP)
                        .into()
                } else {
                    widget::row![identity, actions]
                        .spacing(theme::GAP)
                        .align_y(alignment::Vertical::Center)
                        .into()
                };
                layout
                    .apply(widget::container)
                    .width(Length::Fill)
                    .padding(24)
                    .class(cosmic::theme::Container::custom(theme::panel))
                    .into()
            })
    }

    #[must_use]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ConnectToDevice(index) => {
                return Action::ConnectToDevice(self.paired_devices[index]);
            }
            Message::RemoveDevice(index) => {
                return Action::RemoveDevice(self.paired_devices[index]);
            }
            Message::AddDevice => return Action::AddDevice,
            Message::SetPairedDevices(paired_devices) => self.paired_devices = paired_devices,
            Message::Warning(message) => return Action::Warning(message),
        }
        Action::None
    }
}
