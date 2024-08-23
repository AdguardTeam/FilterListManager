use crate::storage::entities::diff_update_entity::DiffUpdateEntity;
use crate::FilterId;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::opt;
use nom::sequence::{terminated, tuple};
use nom::IResult;
use std::num::ParseIntError;

/// Processes Diff-Path string and returns [`DiffUpdateEntity`]
///
/// * `filter_id`- FilterId for entity
/// * `diff_path` - Diff-path string
pub(crate) fn process_diff_path(
    filter_id: FilterId,
    diff_path: String,
) -> Result<Option<DiffUpdateEntity>, ParseIntError> {
    let result: IResult<&str, (&str, Option<&str>, &str, &str), nom::error::Error<&str>> =
        tuple((
            terminated(take_until("-"), tag("-")),
            opt(terminated(alt((tag("h"), tag("m"), tag("s"))), tag("-"))),
            terminated(take_until("-"), tag("-")),
            terminated(take_until("."), tag(".patch")),
        ))(diff_path.as_str());

    match result {
        Err(_) => Ok(None),
        Ok((_, (_, optional_resolution, epoch_timestamp, expiration_period))) => {
            let next_check_time =
                calculate_next_check_time(optional_resolution, epoch_timestamp, expiration_period)?;

            let entity = DiffUpdateEntity {
                filter_id,
                next_path: diff_path,
                next_check_time,
            };

            Ok(Some(entity))
        }
    }
}

/// Calculates the time when this update MAY BE available
#[inline]
fn calculate_next_check_time(
    optional_resolution: Option<&str>,
    epoch_timestamp: &str,
    expiration_period: &str,
) -> Result<i64, ParseIntError> {
    let epoch_value = epoch_timestamp.parse::<i64>()?;
    let expiration_period_value = expiration_period.parse::<i64>()?;

    let multiplier = match optional_resolution {
        Some("s") => 1,
        Some("m") => 60,
        _ => 3600,
    };

    Ok((epoch_value + expiration_period_value) * multiplier)
}

#[cfg(test)]
mod tests {
    use super::{process_diff_path, DiffUpdateEntity};

    #[test]
    fn test_process_diff_patch() {
        [
            (
                "../patches/batch-m-28334120-60.patch#list1",
                Some(DiffUpdateEntity {
                    filter_id: 0,
                    next_path: String::from("../patches/batch-m-28334120-60.patch#list1"),
                    next_check_time: 1700050800,
                }),
            ),
            (
                "../patches/2/2-s-1719833995-3600.patch",
                Some(DiffUpdateEntity {
                    filter_id: 0,
                    next_path: String::from("../patches/2/2-s-1719833995-3600.patch"),
                    next_check_time: 1719837595,
                }),
            ),
            (
                "patches/v1.0.1-472235-1.patch",
                Some(DiffUpdateEntity {
                    filter_id: 0,
                    next_path: String::from("patches/v1.0.1-472235-1.patch"),
                    next_check_time: 1700049600,
                }),
            ),
        ]
        .into_iter()
        .for_each(|(path, expected)| {
            let actual = process_diff_path(0, String::from(path)).unwrap();

            assert_eq!(actual, expected)
        })
    }
}
