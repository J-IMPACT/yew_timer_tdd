use crate::timers::timer::Timer;

#[derive(Debug, PartialEq, Clone)]
pub enum Phase {
    Work,
    Break,
}

#[derive(Debug, Clone)]
pub struct TimerManager {
    pub timer: Timer,
    pub phase: Phase,
    pub work_duration: u32,
    pub break_duration: u32,
}

impl TimerManager {
    pub fn new(work_sec: u32, break_sec: u32) -> Self {
        TimerManager {
            timer: Timer::new(work_sec), 
            phase: Phase::Work, 
            work_duration: work_sec, 
            break_duration: break_sec
        }
    }

    pub fn tick(&mut self) {
        if self.timer.tick() {
            match self.phase {
                Phase::Work => {
                    self.phase = Phase::Break;
                    self.timer = Timer::new(self.break_duration);
                }
                Phase::Break => {
                    self.phase = Phase::Work;
                    self.timer = Timer::new(self.work_duration);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.phase = Phase::Work;
        self.timer.reset(self.work_duration);
    }
}