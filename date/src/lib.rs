pub const FAKE_NOW: &str = "2024-10-12:00:00:00";
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    inner: chrono::NaiveDate,
}

impl Date {
    const DEFAULT_FORMAT: &'static str = "%Y-%m-%d:00:00:00";
    #[cfg(not(test))]
    pub fn now() -> Self {
        Self {
            inner: chrono::Local::now().naive_local().date(),
        }
    }
    #[cfg(test)]
    pub fn now() -> Self {
        Self {
            inner: chrono::NaiveDate::parse_from_str(FAKE_NOW, "%Y-%m-%d:00:00:00")
                .expect("FAKE_NOW is invalid"),
        }
    }
    pub fn from_str<T: AsRef<str>>(value: T) -> Result<Self, DateError> {
        Self::parse_from_str(value.as_ref(), Self::DEFAULT_FORMAT)
    }
    pub fn parse_from_str(value: &str, format: &str) -> Result<Self, DateError> {
        let inner = chrono::NaiveDate::parse_from_str(value, format)
            .map_err(|_| DateError::ParseError(value.to_string()))?;
        Ok(Self { inner })
    }
    pub fn to_string(&self) -> String {
        self.inner.format(Self::DEFAULT_FORMAT).to_string()
    }
}

#[derive(Debug)]
pub enum DateError {
    ParseError(String),
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn cmp() {
        let format = "%a, %d %b %Y %H:%M:%S %z";
        let date1 = Date::parse_from_str("Fri, 21 Jun 2024 02:22:32 +0000", format).unwrap();
        let date2 = Date::parse_from_str("Sat, 22 Jun 2024 02:22:32 +0000", format).unwrap();
        assert!(date1 < date2);
    }
    #[test]
    fn to_string() {
        let s = "2021-01-01:00:00:00";
        let date = Date::from_str(s).unwrap();
        assert_eq!(date.to_string(), s);
    }
}
