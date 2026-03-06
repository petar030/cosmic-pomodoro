use std::time::{Instant, Duration};


#[derive(Debug, Copy, Clone)]
pub enum TimerState {
    Created,
    Running,
    Paused,
    Finished,
}
#[derive(Debug, Copy, Clone)]

pub enum TimerType{
    Work,
    Break
}

pub struct Timer {
    start_time: Instant,
    pause_time: Instant,
    duration: Duration,
    time_left: Duration,
    state: TimerState,
    my_type: TimerType

}

impl Timer {
    pub fn new(seconds: u64, timer_type: TimerType) -> Timer {
        Timer {
            start_time: Instant::now(),
            pause_time: Instant::now(),
            duration: Duration::from_secs(seconds),
            time_left: Duration::from_secs(seconds),
            state: TimerState::Created,
            my_type: timer_type
        }
    }

    pub fn time_update(&mut self) {
        if let TimerState::Running = self.state
            && (Instant::now() - self.start_time) >= self.time_left
        {
            self.finish();
        }
    }

    pub fn start(&mut self) {
        self.time_update();
        match self.state {
            TimerState::Created => {
                self.start_time = Instant::now();
                self.state = TimerState::Running;
                // println!("Timer started from Created state");
            }
            TimerState::Paused => {
                self.start_time = Instant::now();
                self.state = TimerState::Running;
                // println!("Timer resumed from Paused state");
            }
            _ => {
                // println!(
                //     "start() called, but Timer is in {:?} state – ignored",
                //     self.state
                // );
            }
        }
    }

    pub fn pause(&mut self) {
        self.time_update();
        match self.state {
            TimerState::Running => {
                self.pause_time = Instant::now();

                if self.time_left > (self.pause_time - self.start_time) {
                    self.time_left -= self.pause_time - self.start_time;
                }
                else {
                    self.time_left = Duration::from_secs(0);
                }

                self.state = TimerState::Paused;
                // println!("Timer paused");
            }
            _ => {
                // println!(
                //     "pause() called, but Timer is in {:?} state – ignored",
                //     self.state
                // );
            }
        }
    }

    pub fn reset(&mut self) {
        self.time_update();
        match self.state {
            TimerState::Running | TimerState::Paused | TimerState::Finished => {
                self.time_left = self.duration;
                self.state = TimerState::Created;
                // println!("Timer reset");
            }
            _ => {
                // println!(
                //     "reset() called, but Timer is in {:?} state – ignored",
                //     self.state
                // );
            }
        }
    }

    pub fn finish(&mut self) {
        match self.state {
            TimerState::Running | TimerState::Paused => {
                self.time_left = Duration::from_secs(0);
                self.state = TimerState::Finished;
                // println!("Timer finished");
            }
            TimerState::Finished => {
                // println!("finish() called, but Timer is already Finished – ignored");
            }
            TimerState::Created => {
                // println!("finish() called, but Timer was never started – ignored");
            }
        }
    }

    pub fn get_current_status(&self) -> (u64, TimerState, TimerType){
       if let TimerState::Running = self.state{
        let current_time: Instant = Instant::now();
         if self.time_left > (current_time - self.start_time){
            let remaining : u64 = (self.time_left - (current_time - self.start_time)).as_secs();
            return (remaining, self.state, self.my_type);
        }
        return (0, self.state, self.my_type);
       }
       (self.time_left.as_secs(), self.state, self.my_type)
    }
}
