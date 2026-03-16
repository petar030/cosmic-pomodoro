use crate::core::timer::{Timer, TimerState, TimerType};

pub struct Pomodoro {
    auto_start_work: bool,
    auto_start_break: bool,
    timers: Vec<Timer>,
    cycle_count: u32,
    timer_pointer: usize,
}

impl Pomodoro {
    pub fn new(
        long_break_interval: u64,
        work_time: u64,
        short_break_time: u64,
        long_break_time: u64,
        asw: bool,
        asb: bool,
    ) -> Pomodoro {
        let mut timers: Vec<Timer> = vec![];
        for _i in 0..long_break_interval {
            timers.push(Timer::new(work_time, TimerType::Work));
            if _i < long_break_interval - 1 {
                timers.push(Timer::new(short_break_time, TimerType::Break));
            }
        }
        timers.push(Timer::new(long_break_time, TimerType::Break));

        Pomodoro {
            auto_start_work: asw,
            auto_start_break: asb,
            timers,
            cycle_count: 0,
            timer_pointer: 0,
        }
    }

    pub fn update_and_return_state(&mut self) -> Option<(u64, TimerState, TimerType, bool, usize, u32)> {
        if let Some(current_timer) = self.timers.get_mut(self.timer_pointer) {
            current_timer.time_update();
            let (remaining, status, timer_type) = current_timer.get_current_status();
            let session_count = self.timer_pointer / 2;
            let cycle_count = self.cycle_count;

            if !matches!(status, TimerState::Finished) {
                return Some((
                    remaining,
                    status,
                    timer_type,
                    if matches!(status, TimerState::Created) {
                        false
                    } else {
                        true
                    },
                    session_count,
                    cycle_count,
                ));
            }
        }

        self.forward();

        if let Some(current_timer) = self.timers.get_mut(self.timer_pointer) {
            let (remaining, status, timer_type) = current_timer.get_current_status();
            let session_count = self.timer_pointer / 2;
            let cycle_count = self.cycle_count;

            // if (self.auto_start_work && matches!(timer_type, TimerType::Work))
            //     || (self.auto_start_break && matches!(timer_type, TimerType::Break))
            // {
            //     current_timer.start();
            // }

            return Some((
                remaining,
                status,
                timer_type,
                if matches!(status, TimerState::Created) {
                    false
                } else {
                    true
                },
                session_count,
                cycle_count,
            ));
        }

        None
    }

    pub fn start(&mut self) {
        if let Some(current_timer) = self.timers.get_mut(self.timer_pointer) {
            let (_, status, _) = current_timer.get_current_status();

            if matches!(status, TimerState::Created | TimerState::Paused) {
                current_timer.start();
            }
        }
    }

    pub fn pause(&mut self) {
        if let Some(current_timer) = self.timers.get_mut(self.timer_pointer) {
            let (_, status, _) = current_timer.get_current_status();

            if matches!(status, TimerState::Running) {
                current_timer.pause();
            }
        }
    }

    pub fn reset_current(&mut self) {
        if let Some(current_timer) = self.timers.get_mut(self.timer_pointer) {
            let (_, status, _) = current_timer.get_current_status();

            if !matches!(status, TimerState::Created) {
                current_timer.reset();
            }
        }
    }

    pub fn forward(&mut self) {
        self.reset_current();

        self.timer_pointer = (self.timer_pointer + 1) % self.timers.len();
        if self.timer_pointer == 0 {
            for _i in 0..self.timers.len() {
                self.reset_current();
                self.timer_pointer += 1;
            }
            self.timer_pointer = 0;
            self.cycle_count += 1;
        }

        if let Some(current_timer) = self.timers.get_mut(self.timer_pointer) {
            let (_, _, timer_type) = current_timer.get_current_status();

            if (self.auto_start_work && matches!(timer_type, TimerType::Work))
                || (self.auto_start_break && matches!(timer_type, TimerType::Break))
            {
                current_timer.start();
            }

        }

      
    }

    // Shouldn't be used, gui should make new one when needed
    // pub fn reset_global(&mut self) {
    //     self.reset_current();
    //     self.timer_pointer = 0;
    //     self.cycle_count = 0;
    // }
}
