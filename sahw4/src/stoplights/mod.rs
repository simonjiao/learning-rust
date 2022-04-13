pub enum StopLight {
    Red,
    Yellow,
    Green,
}

impl std::fmt::Display for StopLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Red => "Red",
            Self::Yellow => "Yellow",
            Self::Green => "Green",
        };
        write!(f, "{}", name)
    }
}

impl std::str::FromStr for StopLight {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let low = s.to_lowercase();
        match low.as_str() {
            "red" => Ok(Self::Red),
            "yellow" => Ok(Self::Yellow),
            "green" => Ok(Self::Green),
            _ => Err(format!("{} is not a valid stoplight color", low)),
        }
    }
}

impl StopLight {
    pub fn duration(&self) -> u8 {
        match self {
            Self::Red => 31,
            Self::Green => 22,
            Self::Yellow => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_time_is_right() {
        assert_eq!(StopLight::Red.duration(), 31);
    }

    #[test]
    fn green_time_is_right() {
        assert_eq!(StopLight::Green.duration(), 22);
    }

    #[test]
    fn yellow_time_is_too_long() {
        assert!(StopLight::Yellow.duration() < 5);
    }
}
