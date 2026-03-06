// SPDX-License-Identifier: MPL-2.0

use crate::core::pomodoro::Pomodoro;
use crate::core::timer::{TimerState, TimerType};
use crate::config::Config;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Alignment, Length, Limits, Subscription, time, window::Id};
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget::{self};
use notify_rust::{Hint, Notification};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

const NOTIFICATION_SOUND_PATH_DEV: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/sounds/cosmic-pomodoro-notification.mp3");
const NOTIFICATION_SOUND_PATH_SYSTEM: &str =
    "/usr/share/sounds/cosmic-pomodoro/cosmic-pomodoro-notification.mp3";
const NOTIFICATION_ICON_PATH_DEV: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/resources/icon-128.png");
const NOTIFICATION_ICON_PATH_SYSTEM: &str =
    "/usr/share/icons/hicolor/128x128/apps/io.github.petar030.cosmic-pomodoro.png";
const APPLET_ICON_PATH: &str = "resources/icon-symbolic.svg";
const APP_ICON_NAME: &str = "io.github.petar030.cosmic-pomodoro";
const APP_ICON_SYMBOLIC_NAME: &str = "io.github.petar030.cosmic-pomodoro-symbolic";

mod views;

/// Dva pogleda (Main i Settings), kao u starom template-u.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum PopupView {
    #[default]
    Main,
    Settings,
}

/// Snapshot stanja koji vraća `update_and_return_state`.
#[derive(Debug, Clone)]
struct PomodoroTickState {
    remaining: u64,
    timer_state: TimerState,
    timer_type: TimerType,
    started: bool,
    session_count: usize,
    cycle_count: u32,
}

impl PomodoroTickState {
    fn from_tuple(value: (u64, TimerState, TimerType, bool, usize, u32)) -> Self {
        Self {
            remaining: value.0,
            timer_state: value.1,
            timer_type: value.2,
            started: value.3,
            session_count: value.4,
            cycle_count: value.5,
        }
    }
}

impl Default for PomodoroTickState {
    fn default() -> Self {
        Self {
            remaining: 0,
            timer_state: TimerState::Created,
            timer_type: TimerType::Work,
            started: false,
            session_count: 0,
            cycle_count: 0,
        }
    }
}


struct PomodoroState {
    pomodoro: Pomodoro,
    last_tick_state: Option<PomodoroTickState>,
    settings_changed: bool,
}

impl PomodoroState {
    fn new(config: &Config) -> Self {
        let mut this = Self {
            pomodoro: Self::new_pomodoro(config),
            last_tick_state: None,
            settings_changed: false,
        };

        this.refresh_state();
        this
    }

    fn new_pomodoro(config: &Config) -> Pomodoro {
        Pomodoro::new(
            config.long_break_interval,
            config.work_time * 60,
            config.short_break_time * 60,
            config.long_break_time * 60,
            config.auto_start_work,
            config.auto_start_break,
        )
    }

    fn mark_settings_changed(&mut self) {
        self.settings_changed = true;
    }

    fn apply_settings_if_needed(&mut self, config: &Config) {
        if self.settings_changed {
            self.pomodoro = Self::new_pomodoro(config);
            self.settings_changed = false;
            self.refresh_state();
        }
    }

    fn refresh_state(&mut self) {
        let previous_timer_type = self.last_tick_state.as_ref().map(|s| s.timer_type);

        self.last_tick_state = self
            .pomodoro
            .update_and_return_state()
            .map(PomodoroTickState::from_tuple);

        let next_timer_type = self.last_tick_state.as_ref().map(|s| s.timer_type);
        let next_remaining_seconds = self.last_tick_state.as_ref().map(|s| s.remaining);

        let phase_duration_minutes = next_remaining_seconds.map(|seconds| seconds.div_ceil(60));

        match (previous_timer_type, next_timer_type) {
            (Some(TimerType::Work), Some(TimerType::Break)) => {
                let minutes = phase_duration_minutes.unwrap_or(0);
                self.notify(&format!(
                    "Great work — take a well-earned {} minute break.",
                    minutes
                ));
            }
            (Some(TimerType::Break), Some(TimerType::Work)) => {
                let minutes = phase_duration_minutes.unwrap_or(0);
                self.notify(&format!(
                    "Break is over — let’s focus for {} minutes.",
                    minutes
                ));
            }
            _ => {}
        }
    }

    fn notify(&self, message: &str) {
        let icon = Self::first_existing_path(&[
            NOTIFICATION_ICON_PATH_DEV,
            NOTIFICATION_ICON_PATH_SYSTEM,
        ])
        .unwrap_or_else(|| APP_ICON_NAME.to_string());

        let _ = Notification::new()
            .appname("Cosmic Pomodoro")
            .summary("Pomodoro")
            .body(message)
            .icon(&icon)
            .hint(Hint::DesktopEntry(APP_ICON_NAME.to_string()))
            .show();

        if let Some(sound_path) = Self::first_existing_path(&[
            NOTIFICATION_SOUND_PATH_DEV,
            NOTIFICATION_SOUND_PATH_SYSTEM,
        ]) {
            let _ = Command::new("paplay")
                .arg(&sound_path)
                .spawn()
                .or_else(|_| Command::new("aplay").arg(&sound_path).spawn());
        }
    }

    fn first_existing_path(candidates: &[&str]) -> Option<String> {
        candidates
            .iter()
            .find(|candidate| Path::new(candidate).exists())
            .map(|candidate| (*candidate).to_string())
    }

}

/// Aplet model — stanje aplikacije.
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: Config,
    current_view: PopupView,
    pomodoro_state: PomodoroState,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    PomodoroTick,
    StartPomodoro,
    PausePomodoro,
    ForwardPomodoro,
    RestartPomodoro,
    UpdateConfig(Config),
    OpenSettingsView,
    BackToMainView,
    Settings(SettingsMessage),
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ResetToDefault,
    SetLongBreakInterval(u64),
    SetWorkTime(u64),
    SetShortBreakTime(u64),
    SetLongBreakTime(u64),
    SetAutoStartWork(bool),
    SetAutoStartBreak(bool),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "io.github.petar030.cosmic-pomodoro";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let config = cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .map(|context| match Config::get_entry(&context) {
                Ok(config) => config,
                Err((_errors, config)) => config,
            })
            .unwrap_or_default();

        let pomodoro_state = PomodoroState::new(&config);

        (
            AppModel {
                core,
                popup: None,
                config,
                current_view: PopupView::Main,
                pomodoro_state,
            },
            Task::none(),
        )
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let started = self
            .pomodoro_state
            .last_tick_state
            .as_ref()
            .is_some_and(|s| s.started);

        if !started {
            let icon_button = if let Ok(path) = std::fs::canonicalize(APPLET_ICON_PATH) {
                let mut icon_handle = widget::icon::from_path(path);
                icon_handle.symbolic = true;
                self.core.applet.icon_button_from_handle(icon_handle)
            } else {
                self.core.applet.icon_button(APP_ICON_SYMBOLIC_NAME)
            };

            return icon_button.on_press(Message::TogglePopup).into();
        }

        let progress = self.panel_progress_fraction();
        let timer_type = self
            .pomodoro_state
            .last_tick_state
            .as_ref()
            .map_or(TimerType::Work, |s| s.timer_type);

        let panel_phase_icon: Element<'_, Message> = match timer_type {
            TimerType::Work => widget::icon::from_name("alarm-symbolic").size(14).icon().into(),
            TimerType::Break => {
                let mut break_icon_handle = widget::icon::from_svg_bytes(
                    include_bytes!("../resources/icons/coffee-symbolic.svg").as_slice(),
                );
                break_icon_handle.symbolic = true;
                break_icon_handle.icon().size(14).into()
            }
        };

        let panel_content = widget::column()
            .width(Length::Fixed(18.0))
            .align_x(Alignment::Center)
            .spacing(1)
            .push(panel_phase_icon)
            .push(
                widget::progress_bar(0.0..=1.0, progress)
                    .height(Length::Fixed(2.0))
                    .width(Length::Fixed(16.0)),
            );

        self.core
            .applet
            .button_from_element(panel_content, true)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, id: Id) -> Element<'_, Self::Message> {
        match self.current_view {
            PopupView::Main => views::main::view_main(
                &self.core,
                self.current_view,
                id,
                &self.config,
                &self.pomodoro_state,
            ),
            PopupView::Settings => {
                views::settings::view_settings(&self.core, self.current_view, id, &self.config)
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            time::every(Duration::from_millis(250)).map(|_| Message::PomodoroTick),
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
        ])
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::PomodoroTick => {
                self.pomodoro_state.refresh_state();
            }
            Message::StartPomodoro => {
                self.pomodoro_state.pomodoro.start();
                self.pomodoro_state.refresh_state();
            }
            Message::PausePomodoro => {
                self.pomodoro_state.pomodoro.pause();
                self.pomodoro_state.refresh_state();
            }
            Message::ForwardPomodoro => {
                self.pomodoro_state.pomodoro.forward();
                self.pomodoro_state.refresh_state();
            }
            Message::RestartPomodoro => {
                self.pomodoro_state = PomodoroState::new(&self.config);
            }
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::OpenSettingsView => {
                self.current_view = PopupView::Settings;
            }
            Message::BackToMainView => {
                self.pomodoro_state.apply_settings_if_needed(&self.config);
                self.current_view = PopupView::Main;
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::Settings(msg) => match msg {
                SettingsMessage::ResetToDefault => {
                    self.config = Config::default();
                    self.pomodoro_state.mark_settings_changed();
                    if let Ok(ctx) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                        let _ = self.config.write_entry(&ctx);
                    }
                }
                SettingsMessage::SetLongBreakInterval(v) => {
                    if self.config.long_break_interval != v {
                        self.config.long_break_interval = v;
                        self.pomodoro_state.mark_settings_changed();
                        if let Ok(ctx) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                            let _ = self.config.write_entry(&ctx);
                        }
                    }
                }
                SettingsMessage::SetWorkTime(v) => {
                    if self.config.work_time != v {
                        self.config.work_time = v;
                        self.pomodoro_state.mark_settings_changed();
                        if let Ok(ctx) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                            let _ = self.config.write_entry(&ctx);
                        }
                    }
                }
                SettingsMessage::SetShortBreakTime(v) => {
                    if self.config.short_break_time != v {
                        self.config.short_break_time = v;
                        self.pomodoro_state.mark_settings_changed();
                        if let Ok(ctx) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                            let _ = self.config.write_entry(&ctx);
                        }
                    }
                }
                SettingsMessage::SetLongBreakTime(v) => {
                    if self.config.long_break_time != v {
                        self.config.long_break_time = v;
                        self.pomodoro_state.mark_settings_changed();
                        if let Ok(ctx) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                            let _ = self.config.write_entry(&ctx);
                        }
                    }
                }
                SettingsMessage::SetAutoStartWork(v) => {
                    if self.config.auto_start_work != v {
                        self.config.auto_start_work = v;
                        self.pomodoro_state.mark_settings_changed();
                        if let Ok(ctx) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                            let _ = self.config.write_entry(&ctx);
                        }
                    }
                }
                SettingsMessage::SetAutoStartBreak(v) => {
                    if self.config.auto_start_break != v {
                        self.config.auto_start_break = v;
                        self.pomodoro_state.mark_settings_changed();
                        if let Ok(ctx) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                            let _ = self.config.write_entry(&ctx);
                        }
                    }
                }
            },
        }

        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}

impl AppModel {
    fn panel_progress_fraction(&self) -> f32 {
        let Some(state) = self.pomodoro_state.last_tick_state.as_ref() else {
            return 0.0;
        };

        let total_seconds = match state.timer_type {
            TimerType::Work => self.config.work_time.saturating_mul(60),
            TimerType::Break => {
                let short_break = self.config.short_break_time.saturating_mul(60);
                let long_break = self.config.long_break_time.saturating_mul(60);

                if state.remaining > short_break {
                    long_break.max(state.remaining)
                } else {
                    short_break.max(state.remaining)
                }
            }
        };

        if total_seconds == 0 {
            return 0.0;
        }

        let elapsed = total_seconds.saturating_sub(state.remaining);
        (elapsed as f32 / total_seconds as f32).clamp(0.0, 1.0)
    }
}
