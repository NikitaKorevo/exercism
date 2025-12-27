use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours as f64, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::get_clock(
            Self {
                hours: 0,
                minutes: 0,
            },
            Self { hours, minutes },
        )
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let time = Self::get_clock(
            Self {
                hours: self.hours,
                minutes: self.minutes,
            },
            Self { hours: 0, minutes },
        );

        self.hours = time.hours;
        self.minutes = time.minutes;

        Self {
            hours: time.hours,
            minutes: time.minutes,
        }
    }

    fn get_clock(mut initial_clock: Self, mut additional_clock: Self) -> Self {
        initial_clock = Self::handle_negative_clock(initial_clock);
        additional_clock = Self::handle_negative_clock(additional_clock);
        let minutes = (initial_clock.minutes + additional_clock.minutes) % 60;
        let extra_minutes_in_hours = (initial_clock.minutes + additional_clock.minutes) / 60;
        let hours = (initial_clock.hours + additional_clock.hours + extra_minutes_in_hours) % 24;

        Self { hours, minutes }
    }

    fn handle_negative_clock(clock: Self) -> Self {
        let mut minutes = clock.minutes;
        let mut hours = clock.hours;
        let mut extra_minutes_in_hours: i32 = 0;

        if minutes < 0 {
            extra_minutes_in_hours = (minutes.abs() as f64 / 60.0).ceil() as i32;
            minutes = extra_minutes_in_hours * 60 - minutes.abs();
            hours -= extra_minutes_in_hours;
        }

        if hours < 0 {
            hours = (hours.abs() as f64 / 24.0).ceil() as i32 * 24 - hours.abs();
        }

        Self { hours, minutes }
    }
}
