#[derive(Debug, PartialEq, Clone)]
pub struct Timer {
    pub remaining: u32,
}

impl Timer {
    pub fn new(seconds: u32) -> Self {
        Timer { remaining: seconds }
    }

    pub fn tick(&mut self) -> bool {
        if self.remaining > 0 {
            self.remaining -= 1;
        }
        self.remaining == 0
    }

    pub fn reset(&mut self, to: u32) {
        self.remaining = to;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_ticks_down() {
        let mut timer = Timer::new(3);
        assert_eq!(timer.remaining, 3);
        timer.tick();
        assert_eq!(timer.remaining, 2);
    }

    #[test]
    fn timer_triggers_phase_switch() {
        let mut timer = Timer::new(1);
        assert_eq!(timer.tick(), true);
        assert_eq!(timer.remaining, 0);
    }

    #[test]
    fn timer_reset() {
        let mut timer = Timer::new(1);
        timer.reset(2);
        assert_eq!(timer.remaining, 2)
    }
}

#[cfg(test)]
mod props {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn never_negative(s: u32) -> bool {
            let s = s % 10000;
            let mut timer = Timer::new(s);
            for _ in 0..(s + 10) {
                timer.tick();
            }
            timer.remaining == 0
        }
    }
}