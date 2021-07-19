use std::fmt;
use std::time::{Duration, Instant};

const MINUTE_SECONDS: u64 = 60;
const HOUR_SECONDS: u64 = MINUTE_SECONDS * 60;

#[derive(Debug)]
pub struct Timer {
    duration: Duration,
    elapsed: Option<Instant>,
}

impl Timer {
    pub fn new(seconds: u64) -> Timer {
        Timer { duration: Duration::from_secs(seconds), elapsed: None }
    }

    pub fn as_secs(&self) -> u64 {
        let mut elapsed: u64 = 0;
        if self.elapsed.is_some() {
            elapsed = self.elapsed.unwrap().elapsed().as_secs()
        }
        return if elapsed > self.duration.as_secs() {
            0
        } else {
            self.duration.as_secs() - elapsed
        };
    }

    pub fn start(&mut self) {
        self.elapsed = Some(Instant::now());
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut seconds: u64 = self.as_secs();

        let mut hh: String = "".to_string();
        let mut mm: String = "".to_string();

        if seconds >= HOUR_SECONDS {
            hh = format!("{}:", seconds / HOUR_SECONDS);
            seconds %= HOUR_SECONDS;
        }

        if seconds >= MINUTE_SECONDS || hh.len() > 0 {
            mm = format!("{:02}.", seconds / MINUTE_SECONDS);
            seconds %= MINUTE_SECONDS;
        }

        let ss = format!("{:02}", seconds);

        write!(f, "{}", &[hh, mm, ss].concat())
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use table_test::table_test;

    #[test]
    fn test_timer_display() {
        let table = vec![
            (59, "59"),
            (60, "01.00"),
            (61, "01.01"),
            (120, "02.00"),
            (125, "02.05"),
            (3600, "1:00.00"),
            (3601, "1:00.01"),
            (3720, "1:02.00"),
            (3675, "1:01.15"),
        ];

        for (validator, seconds, expected) in table_test!(table) {
            let timer = super::Timer::new(seconds);
            let actual = timer.to_string();

            validator
                .given(&format!("{}", seconds))
                .when("to_string")
                .then(&format!("should be {}", expected))
                .assert_eq(expected, actual.as_str());
        }
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
        assert_eq!(t.as_secs(), 0);
    }
}
