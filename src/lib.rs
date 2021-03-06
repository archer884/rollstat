use std::{fmt::Display, num::ParseIntError, str::FromStr};

mod error;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
pub use error::{ParseEntryError, ParseVersionError};

pub struct Entry {
    pub timestamp: DateTime<Utc>,
    pub version: Version,
    pub max: i32,
    pub values: Vec<i32>,
}

impl FromStr for Entry {
    type Err = ParseEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_values(s: &str) -> Result<Vec<i32>, ParseIntError> {
            s.split(',').map(|x| x.parse()).collect()
        }
        
        let mut segments = s.split('|');

        let timestamp = segments
            .next()
            .ok_or(ParseEntryError::Format("timestamp"))?;
        let timestamp = Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(timestamp, "%F %R")?);
        let version = segments
            .next()
            .ok_or(ParseEntryError::Format("version"))?
            .parse()?;

        let max_and_values = segments.next().ok_or(ParseEntryError::Format("missing max/values"))?;
        let (max, values) = match max_and_values.find(':') {
            Some(mid) => (&max_and_values[..mid], &max_and_values[mid + 1..]),
            None => return Err(ParseEntryError::Format("missing values")),
        };

        Ok(Entry {
            timestamp,
            version,
            max: max.parse()?,
            values: parse_values(values)?,
        })
    }
}

// The intent here is to provide a way to group entries
//  by version; hence the eq/hash implementations.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
    extension: Option<String>,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.extension {
            Some(ext) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, ext),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_int<'a>(
            mut segments: impl Iterator<Item = &'a str>,
            name: &'static str,
        ) -> Result<u16, ParseVersionError> {
            let segment = segments.next().ok_or(ParseVersionError::Format(name))?;
            Ok(segment.parse()?)
        }

        let mut segments = s.split(|u| u == '.' || u == '-');
        Ok(Self {
            major: read_int(&mut segments, "major")?,
            minor: read_int(&mut segments, "minor")?,
            patch: read_int(&mut segments, "patch")?,
            extension: segments.next().map(ToOwned::to_owned),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
