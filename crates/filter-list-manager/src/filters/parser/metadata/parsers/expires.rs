use super::expires_time_holder::ExpiresTimeHolder;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::opt,
    sequence::{preceded, terminated, tuple},
    IResult,
};

/// Represents a number with possible fraction: 12, 12.0, 12.45
type NumberResult<'a> = (&'a str, Option<&'a str>);

/// Parse integer from [NumberResult]
fn from_number_result_to_integer(value: &NumberResult) -> i32 {
    value.0.parse::<i32>().unwrap_or_default()
}

/// Parse float from [NumberResult]
fn from_number_result_to_decimal(value: &NumberResult) -> f32 {
    match value.1 {
        None => str::parse::<f32>(value.0).unwrap_or_default(),
        Some(remainder) => {
            // TODO: spike
            let mut str: String = String::from(value.0);
            str += ".";
            str += remainder;

            str::parse::<f32>(str.as_str()).unwrap_or_default()
        }
    }
}

/// Parse integer "12" or "12.345" from string
fn take_time_value(input: &str) -> IResult<&str, NumberResult, nom::error::Error<&str>> {
    tuple((
        digit1,                          // whole part
        opt(preceded(tag("."), digit1)), // optional fraction
    ))(input)
}

/// Parse time unit (e.g. hours, days, ...)
fn take_time_unit(input: &str) -> IResult<&str, &str, nom::error::Error<&str>> {
    let (remainder, _) = skip_many_allowed_whitespaces(input).unwrap_or_default();

    terminated(
        alt((
            tag("days"),
            tag("day"),
            tag("d"),
            tag("hours"),
            tag("hour"),
            tag("hrs"),
            tag("hr"),
            tag("h"),
            tag("minutes"),
            tag("minute"),
            tag("min"),
            tag("m"),
            tag("seconds"),
            tag("second"),
            tag("sec"),
            tag("s"),
        )),
        skip_many_allowed_whitespaces,
    )(remainder)
}

/// Skip allowed whitespaces only
fn skip_many_allowed_whitespaces(input: &str) -> IResult<&str, &str, nom::error::Error<&str>> {
    take_while(|c: char| c == '\t' || c == ' ')(input)
}

/// Parse "Expires" and returns integer value in seconds
pub(crate) fn process_expires(input: &str) -> i32 {
    let mut time_holder = ExpiresTimeHolder::new();
    let binding = input.to_lowercase();
    let mut input_copy: &str = binding.trim();

    for iter in 0..ExpiresTimeHolder::get_time_units_count() {
        let (time_value_remainder, time_value) = match take_time_value(input_copy) {
            Ok(tuple) => tuple,
            Err(_) => {
                break;
            }
        };

        let (time_unit_remainder, time_unit) = match take_time_unit(time_value_remainder) {
            Ok(tuple) => tuple,
            Err(_) => {
                // At first iter we can treat time_value as a seconds
                if iter == 0 {
                    time_holder.set_seconds(from_number_result_to_integer(&time_value));
                }

                break;
            }
        };

        let (next_iter_input, _) =
            skip_many_allowed_whitespaces(time_unit_remainder).unwrap_or_default();

        let time_value_was_set: bool = match time_unit {
            "days" | "day" | "d" => {
                time_holder.set_days(from_number_result_to_decimal(&time_value))
            }

            "hours" | "hour" | "hrs" | "hr" | "h" => {
                time_holder.set_hours(from_number_result_to_decimal(&time_value))
            }
            "minutes" | "minute" | "min" | "m" => {
                time_holder.set_minutes(from_number_result_to_decimal(&time_value))
            }

            "seconds" | "second" | "sec" | "s" => {
                time_holder.set_seconds(from_number_result_to_integer(&time_value))
            }

            _ => false,
        };

        // Order is incorrect
        if !time_value_was_set {
            break;
        }

        input_copy = next_iter_input;
    }

    time_holder.get_overall_seconds()
}

#[cfg(test)]
mod tests {
    use super::process_expires;

    #[test]
    fn decimal_hours_integer_s_both_trimmed() {
        let actual = process_expires("12.345 hours   15 s");

        assert_eq!(actual, 44457);
    }

    #[test]
    fn case_all_integer_day_hours_min() {
        let actual = process_expires("1day 5 HoUrS 12 mIn ");

        assert_eq!(actual, 105120);
    }

    #[test]
    fn tab_all_decimal_d_hour_minute_sec_both_whitespaces() {
        let actual = process_expires(" 8.834d\t5.23hour 12.8minute 9.23sec ");

        assert_eq!(actual, 782862);
    }

    #[test]
    fn tab_case_hours_decimal_other_integer_d_h_m_s_both_whitespaces() {
        let actual = process_expires("\t8D\t5.23h\t12m\t9s\t");

        assert_eq!(actual, 710757);
    }

    #[test]
    fn tab_all_decimal_days_hrs_both_whitespaces() {
        let actual = process_expires(" 5.23days\t85.913hrs(some garbage)");

        assert_eq!(actual, 761158);
    }

    #[test]
    fn all_decimal_d_h_garbage_in_the_middle() {
        let actual = process_expires("5.23d(some garbage)85.913hrs");

        assert_eq!(actual, 451872);
    }

    #[test]
    fn all_integer_d_h_m_seconds_incorrect_ordered_must_be_ignored() {
        // 1h 1m will be ignored, because of order
        let actual = process_expires("1d 1seconds 1h 1m");

        assert_eq!(actual, 86401);
    }

    #[test]
    fn garbage_at_the_start_which_can_be_treated_as_seconds() {
        // 1h 1m will be ignored, because of order
        let actual = process_expires("32fd2 1d 1second 1h 1m");

        assert_eq!(actual, 32);
    }

    #[test]
    fn garbage_at_the_start() {
        // 1h 1m will be ignored, because of order
        let actual = process_expires("asdf 1 hour");

        assert_eq!(actual, 0);
    }

    #[test]
    fn all_integer_h_seconds_newline_is_not_an_option() {
        // 1h 1m will be ignored, because of order
        let actual = process_expires("1h\n32 seconds");

        assert_eq!(actual, 3600);
    }

    #[test]
    fn tab_case_hours_decimal_other_integer_d_h_eol_after_number() {
        // 12 will be ignored
        let actual = process_expires("\t8D\t5.23h\t12");

        assert_eq!(actual, 710028);
    }

    #[test]
    fn all_decimal_d_m_hr_s_squished() {
        // 12 will be ignored
        let actual = process_expires("1.4d5.9hr8.7m9.3s");

        assert_eq!(actual, 142731);
    }

    #[test]
    fn empty_string() {
        let actual = process_expires("");

        assert_eq!(actual, 0);
    }

    #[test]
    fn integer_parse_error() {
        let actual = process_expires("343523545234523453245324534253245324534");

        assert_eq!(actual, 0);
    }

    #[test]
    fn decimal_parse_error() {
        let actual =
            process_expires("34352354523452342432423423424324.5234242343245324534253245324534");

        assert_eq!(actual, 0);
    }

    #[test]
    fn just_take_seconds_from_numeric_string() {
        let actual = process_expires("65800");

        assert_eq!(actual, 65800);
    }
}
