use crate::config::Config;
use crate::core::timer::{TimerState, TimerType};
use cosmic::iced::{Alignment, Length, window::Id};
use cosmic::prelude::*;
use cosmic::widget;

use super::super::{Message, PomodoroState, PopupView};

pub(crate) fn view_main<'a>(
    core: &'a cosmic::Core,
    _current_view: PopupView,
    _id: Id,
    config: &'a Config,
    pomodoro_state: &'a PomodoroState,
) -> Element<'a, Message> {
    let (remaining, timer_state, timer_type, started, session_count, cycle_count) = pomodoro_state
        .last_tick_state
        .as_ref()
        .map(|s| {
            (
                s.remaining,
                s.timer_state,
                s.timer_type,
                s.started,
                s.session_count,
                s.cycle_count,
            )
        })
        .unwrap_or((0, TimerState::Created, TimerType::Work, false, 0, 0));

    let (work_class, break_class) = match timer_type {
        TimerType::Work => (
            cosmic::theme::Container::Primary,
            cosmic::theme::Container::Transparent,
        ),
        TimerType::Break => (
            cosmic::theme::Container::Transparent,
            cosmic::theme::Container::Primary,
        ),
    };

    let work_segment = widget::container(
        widget::row()
            .width(Length::Fill)
            .align_y(Alignment::Center)
            .push(widget::space::horizontal())
            .push(widget::icon::from_name("alarm-symbolic").size(16).icon())
            .push(widget::space::horizontal()),
    )
    .width(Length::FillPortion(1))
    .padding(6)
    .class(work_class);

    let mut break_icon_handle = widget::icon::from_svg_bytes(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/icons/coffee-symbolic.svg"
        ))
        .as_slice(),
    );
    break_icon_handle.symbolic = true;

    let break_segment = widget::container(
        widget::row()
            .width(Length::Fill)
            .align_y(Alignment::Center)
            .push(widget::space::horizontal())
            .push(break_icon_handle.icon().size(16))
            .push(widget::space::horizontal()),
    )
    .width(Length::FillPortion(1))
    .padding(6)
    .class(break_class);

    let phase_row = widget::container(
        widget::row()
            .width(Length::Fill)
            .spacing(4)
            .push(work_segment)
            .push(break_segment),
    )
    .width(Length::Fill)
    .padding(2)
    .class(cosmic::theme::Container::Transparent);

    let timer_text = widget::text(format_timer(remaining)).size(56);
    let _cycle_text = widget::text(format!("Session: {cycle_count}")).size(14);

    let total_sessions = usize::try_from(config.long_break_interval)
        .ok()
        .filter(|value| *value > 0)
        .unwrap_or(1);
    let active_index = session_count.min(total_sessions.saturating_sub(1));

    let session_dots = (0..total_sessions).fold(
        widget::row().spacing(6).align_y(Alignment::Center),
        |row, index| {
            let symbol = if index <= active_index {
                "●"
            }  else {
                "○"
            };

            row.push(widget::text(symbol).size(12))
        },
    );

    let (center_icon, center_action) = match timer_state {
        TimerState::Running => ("media-playback-pause-symbolic", Message::PausePomodoro),
        _ => ("media-playback-start-symbolic", Message::StartPomodoro),
    };

    let center_button = widget::button::custom(
        widget::container(widget::icon::from_name(center_icon).size(20).icon())
            .width(Length::Fill)
            .center_x(Length::Fill),
    )
    .class(widget::button::ButtonClass::Suggested)
    .on_press(center_action)
    .width(Length::Fixed(156.0));

    let controls = widget::row()
        .width(Length::Fill)
        .padding(0)
        .spacing(8)
        .align_y(Alignment::Center)
        .push(
            core.applet
                .icon_button("view-refresh-symbolic")
                .on_press(Message::RestartPomodoro),
        )
        .push(widget::space::horizontal())
        .push(center_button)
        .push(widget::space::horizontal())
        .push(
            core.applet
                .icon_button("media-skip-forward-symbolic")
                .on_press(Message::ForwardPomodoro),
        );

    let settings_row = if started {
        widget::row().width(Length::Fill)
    } else {
        widget::row()
            .width(Length::Fill)
            .push(widget::space::horizontal())
            .push(
                widget::button::text("Configure")
                    .on_press(Message::OpenSettingsView)
                    .padding([2, 8]),
            )
            .push(widget::space::horizontal())
    };

    let content_list = widget::column()
        .width(Length::Fill)
        .padding(0)
        .spacing(10)
        .align_x(Alignment::Center)
        .push(phase_row)
        .push(timer_text)
        .push(session_dots)
        .push(controls)
        .push(settings_row);

    core.applet
        .popup_container(content_list.padding(10).width(Length::Fixed(320.0)))
        .into()
}

fn format_timer(total_seconds: u64) -> String {
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
