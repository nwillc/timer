use std::fmt;
use std::time::{Duration, Instant};

use string_builder::Builder;

const MINUTE_SECONDS: u64 = 60;
const HOUR_SECONDS: u64 = MINUTE_SECONDS * 60;

#[derive(Debug)]
pub struct Timer {
    duration: Duration,
    started: Option<Instant>,
}

impl Timer {
    pub fn new(seconds: u64) -> Timer {
        Timer { duration: Duration::from_secs(seconds), started: None }
    }

    pub fn as_secs(&self) -> u64 {
        let mut elapsed: u64 = 0;
        if self.started.is_some() {
           elapsed = self.started.unwrap().elapsed().as_secs()
        }
        return if elapsed > self.duration.as_secs() {
            0
        } else {
            self.duration.as_secs() - elapsed
        };
    }

    pub fn start(&mut self) {
        self.started = Some(Instant::now());
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut seconds = self.as_secs();
        let mut builder = Builder::default();

        if seconds > HOUR_SECONDS {
            builder.append(format!("{}:", seconds / HOUR_SECONDS));
            seconds %= HOUR_SECONDS;
        }

        if seconds > MINUTE_SECONDS {
            builder.append(format!("{:02}.", seconds / MINUTE_SECONDS));
            seconds %= MINUTE_SECONDS;
        }

        builder.append(format!("{:02}", seconds));

        write!(f, "{}", builder.string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_new_timer() {
        let t = super::Timer::new(125);
        assert_eq!(t.as_secs(), 125);
        assert_eq!(t.to_string(), "02.05");
    }

    #[test]
    fn test_timer_advanced() {
        let increment = 2;
        let delay = Duration::from_secs(increment);
        let mut t = super::Timer::new(2 * increment);
        t.start();
        sleep(delay);
        assert!(t.as_secs() <= increment);
        sleep(delay);
        assert_eq!(t.as_secs(),0);
    }
}
