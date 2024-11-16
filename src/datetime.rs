use chrono::*;
use pyo3::prelude::*;

#[derive(Debug)]
pub struct DateTimeError {
    pub message: String,
}

impl DateTimeError {
    pub fn new(message: &str) -> Self {
        DateTimeError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Datetime error: {}", self.message)
    }
}

impl std::error::Error for DateTimeError {}

#[pyclass]
pub struct DateTime {
    datetime: NaiveDateTime,
}

#[pymethods]
impl DateTime {
    #[new]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Self, DateTimeError> {
        Ok(DateTime {
            datetime: NaiveDate::from_ymd_opt(year, month, day)
                .ok_or(DateTimeError::new("Invalid month or day"))?
                .and_hms_opt(hour, minute, second)
                .ok_or(DateTimeError::new("Invalid hour, minute or second"))?,
        })
    }

    #[staticmethod]
    fn from_string(date: &str) -> Result<DateTime, DateTimeError> {
        let datetime_patterns = ["%Y-%m-%d %H:%M:%S"];
        let date_patterns = ["%Y-%m-%d"];

        if let Some(datetime) = datetime_patterns
            .iter()
            .find_map(|pattern| NaiveDateTime::parse_from_str(date, pattern).ok())
        {
            return Ok(DateTime { datetime });
        }

        if let Some(date) = date_patterns
            .iter()
            .find_map(|pattern| NaiveDate::parse_from_str(date, pattern).ok())
        {
            let datetime = date.and_hms_opt(0, 0, 0).unwrap();
            return Ok(DateTime { datetime });
        }

        Err(DateTimeError::new("Failed to match any ISO pattern"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let datetime = DateTime::new(2021, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(
            datetime.datetime,
            NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn test_from_string() {
        let datetime = DateTime::from_string("2021-01-01 00:00:00").unwrap();
        assert_eq!(
            datetime.datetime,
            NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );

        let datetime = DateTime::from_string("2021-01-01").unwrap();
        assert_eq!(
            datetime.datetime,
            NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn test_from_string_error() {
        let datetime = DateTime::from_string("2021-01-01 00:00:00:00");
        assert!(datetime.is_err());
    }
}
