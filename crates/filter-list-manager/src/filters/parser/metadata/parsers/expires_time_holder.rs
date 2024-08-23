const DAYS_IS_SET_BIT: u8 = 0x1;
const HOURS_IS_SET_BIT: u8 = 0x2;
const MINUTES_IS_SET_BIT: u8 = 0x4;
const SECONDS_IS_SET_BIT: u8 = 0x8;

/// Overall time units. May be useful as max operations count
const TIME_UNITS: u8 = 4;

/// Holds parts of time (e.g. days, hours, minutes ...) of "Expires" field
/// Supports assign ordering (i.e. you can't assign hours after seconds)
/// Also you can not set one time unit twice
///
/// The implementation uses bitmask for determination of which unit was already set.
///
pub(in crate::filters) struct ExpiresTimeHolder {
    days: f32,
    hours: f32,
    minutes: f32,
    seconds: i32,

    set_units_mask: u8,
}

impl ExpiresTimeHolder {
    pub(in crate::filters) fn new() -> Self {
        Self {
            days: 0.0,
            hours: 0.0,
            minutes: 0.0,
            seconds: 0,

            set_units_mask: 0,
        }
    }

    /// Gets overall count of seconds, calculated from all time units
    pub(in crate::filters) fn get_overall_seconds(&self) -> i32 {
        let decimals_sum =
            (self.days * 86400.0 + self.hours * 3600.0 + self.minutes * 60.0).floor() as i32;

        (decimals_sum + self.seconds).max(0)
    }

    /// Sets seconds value
    pub(in crate::filters) fn set_seconds(&mut self, seconds: i32) -> bool {
        if self.set_units_mask < SECONDS_IS_SET_BIT {
            self.seconds = seconds;
            self.set_units_mask |= SECONDS_IS_SET_BIT;

            return true;
        }

        return false;
    }

    /// Sets minutes value
    pub(in crate::filters) fn set_minutes(&mut self, minutes: f32) -> bool {
        if self.set_units_mask < MINUTES_IS_SET_BIT {
            self.minutes = minutes;
            self.set_units_mask |= MINUTES_IS_SET_BIT;

            return true;
        }

        return false;
    }

    /// Sets hours value
    pub(in crate::filters) fn set_hours(&mut self, hours: f32) -> bool {
        if self.set_units_mask < HOURS_IS_SET_BIT {
            self.hours = hours;
            self.set_units_mask |= HOURS_IS_SET_BIT;

            return true;
        }

        return false;
    }

    /// Sets days value
    pub(in crate::filters) fn set_days(&mut self, days: f32) -> bool {
        if self.set_units_mask < DAYS_IS_SET_BIT {
            self.days = days;
            self.set_units_mask |= DAYS_IS_SET_BIT;

            return true;
        }

        return false;
    }

    #[inline]
    pub(in crate::filters) fn get_time_units_count() -> u8 {
        TIME_UNITS
    }
}
