use crate::config::Config;
use cosmic::iced::{Alignment, Length, window::Id};
use cosmic::prelude::*;
use cosmic::widget;

use super::super::{Message, PopupView, SettingsMessage};

pub(crate) fn view_settings<'a>(
    core: &'a cosmic::Core,
    _current_view: PopupView,
    _id: Id,
    config: &'a Config,
) -> Element<'a, Message> {
    let header = widget::row().padding(2).spacing(0).push(
        core.applet
            .icon_button("go-previous-symbolic")
            .on_press(Message::BackToMainView),
    );

    let work_time_row = widget::row()
        .padding(0)
        .spacing(0)
        .push(widget::spin_button(
            format!("{}", config.work_time),
            config.work_time,
            5,
            5,
            180,
            |v| Message::Settings(SettingsMessage::SetWorkTime(v)),
        ));
    let short_break_time_row = widget::row()
        .padding(0)
        .spacing(0)
        .push(widget::spin_button(
            format!("{}", config.short_break_time),
            config.short_break_time,
            1,
            1,
            60,
            |v| Message::Settings(SettingsMessage::SetShortBreakTime(v)),
        ));
    let long_break_time_row = widget::row()
        .padding(0)
        .spacing(0)
        .push(widget::spin_button(
            format!("{}", config.long_break_time),
            config.long_break_time,
            1,
            1,
            180,
            |v| Message::Settings(SettingsMessage::SetLongBreakTime(v)),
        ));
    let long_break_interval_row = widget::row()
        .padding(0)
        .spacing(0)
        .push(widget::spin_button(
            format!("{}", config.long_break_interval),
            config.long_break_interval,
            1,
            1,
            10,
            |v| Message::Settings(SettingsMessage::SetLongBreakInterval(v)),
        ));

    let content_list = widget::column()
        .width(Length::Fill)
        .padding(0)
        .spacing(10)
        .push(header)
        .push(
            widget::column()
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .spacing(16)
                .push(widget::text("Work time (minutes)").size(15))
                .push(work_time_row)
                .push(widget::text("Short break time (minutes)").size(15))
                .push(short_break_time_row)
                .push(widget::text("Long break time (minutes)").size(15))
                .push(long_break_time_row)
                .push(widget::text("Long break interval").size(15))
                .push(long_break_interval_row)
                .push(
                    widget::column()
                        .spacing(12)
                        .push(
                            widget::toggler(config.auto_start_work)
                                .spacing(10)
                                .label("Auto start work timer".to_string())
                                .size(15)
                                .on_toggle(|v| {
                                    Message::Settings(SettingsMessage::SetAutoStartWork(v))
                                }),
                        )
                        .push(
                            widget::toggler(config.auto_start_break)
                                .spacing(10)
                                .label("Auto start break timer".to_string())
                                .size(15)
                                .on_toggle(|v| {
                                    Message::Settings(SettingsMessage::SetAutoStartBreak(v))
                                }),
                        ),
                )
                .push(widget::row().spacing(10).push(core.applet.text_button(
                    "Reset to default settings",
                    Message::Settings(SettingsMessage::ResetToDefault),
                ))),
        );

    core.applet.popup_container(content_list.padding(10)).into()
}
