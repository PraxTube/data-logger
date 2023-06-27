use std::error::Error;
use std::fmt;

pub struct Time {
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
pub struct TimeError {
    err_impl: Box<dyn Error>,
}

#[derive(Debug)]
struct TimeErrorImpl {
    what: String,
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.err_impl.fmt(f)
    }
}

impl fmt::Display for TimeErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl Error for TimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        self.err_impl.description()
    }
}

impl Error for TimeErrorImpl {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        return &self.what;
    }
}

impl Time {
    pub fn new(hour: u8, minute: u8) -> Result<Time, TimeError> {
        if hour > 23 {
            return Err(TimeError {
                err_impl: Box::new(TimeErrorImpl {
                    what: "invalid hour value".to_string(),
                }),
            });
        } else if minute > 59 {
            return Err(TimeError {
                err_impl: Box::new(TimeErrorImpl {
                    what: "invalid minute value".to_string(),
                }),
            });
        }
        Ok(Time { hour, minute })
    }
}

impl std::str::FromStr for Time {
    type Err = TimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(TimeError {
                err_impl: Box::new(TimeErrorImpl {
                    what: "wrong length, only HH:MM supported".to_string(),
                }),
            });
        }

        let hour: u8;
        match parts[0].parse() {
            Ok(h) => hour = h,
            Err(er) => {
                return Err(TimeError {
                    err_impl: Box::new(er),
                })
            }
        };

        let minute: u8;
        match parts[1].parse() {
            Ok(m) => minute = m,
            Err(er) => {
                return Err(TimeError {
                    err_impl: Box::new(er),
                })
            }
        };
        Time::new(hour, minute)
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
