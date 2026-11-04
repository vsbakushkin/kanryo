use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{self, Formatter};
use std::str::FromStr;

use crate::error::CoreError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hlc {
    ms: u64,
    counter: u16,
    device: u32,
}

impl fmt::Display for Hlc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:016}-{:04x}-{:08x}",
            self.ms, self.counter, self.device
        )
    }
}

impl FromStr for Hlc {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(CoreError::InvalidHlcFormat {
                value: s.to_string(),
            });
        }

        if parts[0].len() != 16 || parts[1].len() != 4 || parts[2].len() != 8 {
            return Err(CoreError::InvalidHlcFormat {
                value: s.to_string(),
            });
        }

        if !parts[0].bytes().all(|b| b.is_ascii_digit()) {
            return Err(CoreError::InvalidHlcFormat {
                value: s.to_string(),
            });
        }

        if !parts[1].bytes().all(|b| b.is_ascii_hexdigit())
            || !parts[2].bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(CoreError::InvalidHlcFormat {
                value: s.to_string(),
            });
        }

        let ms = parts[0]
            .parse::<u64>()
            .map_err(|_| CoreError::InvalidHlcFormat {
                value: s.to_string(),
            })?;
        let counter =
            u16::from_str_radix(parts[1], 16).map_err(|_| CoreError::InvalidHlcFormat {
                value: s.to_string(),
            })?;
        let device =
            u32::from_str_radix(parts[2], 16).map_err(|_| CoreError::InvalidHlcFormat {
                value: s.to_string(),
            })?;

        Ok(Hlc {
            ms,
            counter,
            device,
        })
    }
}

impl Serialize for Hlc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Hlc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::Hlc;

    #[test]
    fn display_formats_hlc() {
        let hlc = Hlc {
            ms: 1_761_650_000_000,
            counter: 3,
            device: 0xa1b2c3d4,
        };

        assert_eq!(hlc.to_string(), "0001761650000000-0003-a1b2c3d4");
    }

    #[test]
    fn parse_and_display_round_trip() {
        let input = "0001761650000000-0003-a1b2c3d4";
        let hlc: Hlc = input.parse().expect("valid HLC string");

        assert_eq!(hlc.ms, 1_761_650_000_000);
        assert_eq!(hlc.to_string(), input);
    }

    #[test]
    fn rejects_invalid_strings() {
        let invalid = [
            "",
            "0001761650000000-0003",
            "0001761650000000-0003-a1b2c3d4-extra",
            "000176165000000-0003-a1b2c3d4",
            "00001761650000000-0003-a1b2c3d4",
            "0001761650000000-003-a1b2c3d4",
            "0001761650000000-00003-a1b2c3d4",
            "0001761650000000-0003-a1b2c3d",
            "0001761650000000-0003-0a1b2c3d4",
            "000176165000000a-0003-a1b2c3d4",
            "0001761650000000-000g-a1b2c3d4",
            "0001761650000000-0003-a1b2c3dg",
            "+001761650000000-0003-a1b2c3d4",
            "0001761650000000-+003-a1b2c3d4",
            "0001761650000000-0003-+1b2c3d4",
        ];

        for input in invalid {
            assert!(
                matches!(
                    input.parse::<Hlc>(),
                    Err(crate::error::CoreError::InvalidHlcFormat { value }) if value == input
                ),
                "expected InvalidHlcFormat for {input:?}"
            );
            assert!(
                serde_json::from_value::<Hlc>(serde_json::Value::String(input.to_string()))
                    .is_err(),
                "expected Serde to reject {input:?}"
            );
        }
    }

    #[test]
    fn serde_uses_string_format_and_preserves_hlc() {
        let input = "0001761650000000-0003-a1b2c3d4";
        let hlc: Hlc = input.parse().unwrap();

        let json = serde_json::to_string(&hlc).unwrap();
        assert_eq!(json, format!("\"{input}\""));

        let restored: Hlc = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, hlc);
    }
}
