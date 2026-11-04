use serde::{Deserialize, Serialize, Serializer};
use std::cmp;
use std::fmt::{self, Formatter};
use std::str::FromStr;

use crate::error::CoreError;

pub trait TimeSource {
    fn now_ms(&self) -> u64;
}

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

pub struct Clock<T: TimeSource> {
    time_source: T,
    device: u32,
    last: Option<Hlc>,
}

impl<T: TimeSource> Clock<T> {
    pub fn new(time_source: T, device: u32) -> Self {
        Self {
            time_source,
            device,
            last: None,
        }
    }

    pub fn tick(&mut self) -> Result<Hlc, CoreError> {
        let now_ms = self.time_source.now_ms();

        if now_ms >= 10_u64.pow(16) {
            return Err(CoreError::HlcTimestampOutOfRange { ms: now_ms });
        }

        let next = match self.last {
            None => Hlc {
                ms: now_ms,
                counter: 0,
                device: self.device,
            },
            Some(last) if now_ms > last.ms => Hlc {
                ms: now_ms,
                counter: 0,
                device: self.device,
            },
            Some(last) => Hlc {
                ms: last.ms,
                counter: last
                    .counter
                    .checked_add(1)
                    .ok_or(CoreError::HlcCounterOverflow)?,
                device: self.device,
            },
        };

        self.last = Some(next);
        Ok(next)
    }

    pub fn observe(&mut self, remote: Hlc) -> Result<Hlc, CoreError> {
        let now_ms = self.time_source.now_ms();

        if now_ms >= 10_u64.pow(16) {
            return Err(CoreError::HlcTimestampOutOfRange { ms: now_ms });
        }

        let next = match self.last {
            None => Hlc {
                ms: cmp::max(now_ms, remote.ms),
                counter: if now_ms > remote.ms {
                    0
                } else {
                    remote
                        .counter
                        .checked_add(1)
                        .ok_or(CoreError::HlcCounterOverflow)?
                },
                device: self.device,
            },
            Some(last) => {
                let max_ms = cmp::max(cmp::max(now_ms, last.ms), remote.ms);
                let counter = if max_ms == last.ms && max_ms == remote.ms {
                    cmp::max(last.counter, remote.counter)
                        .checked_add(1)
                        .ok_or(CoreError::HlcCounterOverflow)?
                } else if max_ms == last.ms {
                    last.counter
                        .checked_add(1)
                        .ok_or(CoreError::HlcCounterOverflow)?
                } else if max_ms == remote.ms {
                    remote
                        .counter
                        .checked_add(1)
                        .ok_or(CoreError::HlcCounterOverflow)?
                } else {
                    0
                };

                Hlc {
                    ms: max_ms,
                    counter,
                    device: self.device,
                }
            }
        };

        self.last = Some(next);
        Ok(next)
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

    struct FakeTimeSource {
        ms: std::cell::Cell<u64>,
    }

    impl FakeTimeSource {
        fn new(ms: u64) -> Self {
            Self {
                ms: std::cell::Cell::new(ms),
            }
        }
    }

    impl super::TimeSource for FakeTimeSource {
        fn now_ms(&self) -> u64 {
            self.ms.get()
        }
    }

    #[test]
    fn tick_handles_equal_backward_and_forward_time_and_saves_last() {
        let mut clock = super::Clock::new(FakeTimeSource::new(1000), 0xa1b2c3d4);

        for (now, expected_ms, expected_counter) in [
            (1000, 1000, 0),
            (1000, 1000, 1),
            (999, 1000, 2),
            (1001, 1001, 0),
        ] {
            clock.time_source.ms.set(now);
            let hlc = clock.tick().expect("tick should succeed");

            assert_eq!((hlc.ms, hlc.counter), (expected_ms, expected_counter));
            assert_eq!(hlc.device, 0xa1b2c3d4);
            assert_eq!(clock.last, Some(hlc), "successful tick must save last");
        }
    }

    #[test]
    fn counter_overflow_preserves_last_and_forward_time_allows_recovery() {
        let mut clock = super::Clock::new(FakeTimeSource::new(1000), 7);
        let last = Hlc {
            ms: 1000,
            counter: u16::MAX,
            device: 7,
        };
        clock.last = Some(last);

        for now in [1000, 999] {
            clock.time_source.ms.set(now);
            assert!(matches!(
                clock.tick(),
                Err(crate::error::CoreError::HlcCounterOverflow)
            ));
            assert_eq!(clock.last, Some(last), "overflow must preserve last");
        }

        clock.time_source.ms.set(1001);
        let recovered = clock.tick().expect("forward time resets the counter");
        assert_eq!((recovered.ms, recovered.counter), (1001, 0));
        assert_eq!(recovered.device, 7);
        assert_eq!(clock.last, Some(recovered));
    }

    #[test]
    fn tick_enforces_sixteen_digit_time_limit_and_preserves_last_on_error() {
        const MAX_MS: u64 = 9_999_999_999_999_999;
        let mut clock = super::Clock::new(FakeTimeSource::new(MAX_MS), 7);
        let last = clock.tick().expect("sixteen-digit timestamp is valid");

        assert_eq!((last.ms, last.counter), (MAX_MS, 0));
        assert_eq!(last.to_string(), "9999999999999999-0000-00000007");
        assert_eq!(last.to_string().parse::<Hlc>().unwrap(), last);
        assert_eq!(clock.last, Some(last));

        for now in [MAX_MS + 1, u64::MAX] {
            clock.time_source.ms.set(now);
            assert!(clock.tick().is_err(), "timestamp {now} exceeds 16 digits");
            assert_eq!(clock.last, Some(last), "invalid time must preserve last");
        }
    }

    #[test]
    fn invalid_initial_time_leaves_last_unset() {
        for now in [10_000_000_000_000_000, u64::MAX] {
            let mut clock = super::Clock::new(FakeTimeSource::new(now), 7);

            assert!(clock.tick().is_err(), "timestamp {now} exceeds 16 digits");
            assert_eq!(clock.last, None);

            clock.time_source.ms.set(1000);
            let hlc = clock.tick().expect("valid time after rejected first tick");
            assert_eq!((hlc.ms, hlc.counter), (1000, 0));
            assert_eq!(clock.last, Some(hlc));
        }
    }

    #[test]
    fn observe_handles_first_timestamp_and_merge_cases() {
        const LOCAL_DEVICE: u32 = 7;
        const REMOTE_DEVICE: u32 = 42;

        for (now, previous, remote_ms, remote_counter, expected_ms, expected_counter) in [
            (1001, None, 1000, 0, 1001, 0),
            (1001, None, 1000, 5, 1001, 0),
            (1000, None, 1000, 3, 1000, 4),
            (999, None, 1000, 3, 1000, 4),
            (1002, Some((1000, 3)), 1001, 5, 1002, 0),
            (999, Some((1001, 3)), 1000, 5, 1001, 4),
            (999, Some((1000, 3)), 1001, 5, 1001, 6),
            (999, Some((1000, 3)), 1000, 5, 1000, 6),
            (1000, Some((1000, 5)), 1000, 3, 1000, 6),
            (1000, Some((1000, 3)), 999, 5, 1000, 4),
            (1000, Some((999, 3)), 1000, 5, 1000, 6),
        ] {
            let mut clock = super::Clock::new(FakeTimeSource::new(now), LOCAL_DEVICE);
            let previous = previous.map(|(ms, counter)| Hlc {
                ms,
                counter,
                device: LOCAL_DEVICE,
            });
            clock.last = previous;
            let remote = Hlc {
                ms: remote_ms,
                counter: remote_counter,
                device: REMOTE_DEVICE,
            };

            let observed = clock.observe(remote).expect("observe should succeed");

            assert_eq!(
                (observed.ms, observed.counter),
                (expected_ms, expected_counter)
            );
            assert!(observed > remote, "result must exceed the remote timestamp");
            if let Some(previous) = previous {
                assert!(
                    observed > previous,
                    "result must exceed the local timestamp"
                );
            }
            assert_eq!(observed.device, LOCAL_DEVICE);
            assert_eq!(
                clock.last,
                Some(observed),
                "successful observe must save last"
            );
        }
    }

    #[test]
    fn observe_counter_overflow_preserves_last() {
        for now in [999, 1000] {
            let mut clock = super::Clock::new(FakeTimeSource::new(now), 7);
            let remote = Hlc {
                ms: 1000,
                counter: u16::MAX,
                device: 42,
            };

            assert!(matches!(
                clock.observe(remote),
                Err(crate::error::CoreError::HlcCounterOverflow)
            ));
            assert_eq!(clock.last, None, "overflow must leave last unset");
        }

        for (last_ms, last_counter, remote_ms, remote_counter) in [
            (1001, u16::MAX, 1000, 0),
            (1000, 0, 1001, u16::MAX),
            (1000, u16::MAX, 1000, 0),
            (1000, 0, 1000, u16::MAX),
            (1000, u16::MAX, 1000, u16::MAX),
        ] {
            let mut clock = super::Clock::new(FakeTimeSource::new(999), 7);
            let last = Hlc {
                ms: last_ms,
                counter: last_counter,
                device: 7,
            };
            clock.last = Some(last);
            let remote = Hlc {
                ms: remote_ms,
                counter: remote_counter,
                device: 42,
            };

            assert!(matches!(
                clock.observe(remote),
                Err(crate::error::CoreError::HlcCounterOverflow)
            ));
            assert_eq!(clock.last, Some(last), "overflow must preserve last");

            clock.time_source.ms.set(1002);
            let recovered = clock
                .observe(remote)
                .expect("forward time resets the counter");
            assert_eq!((recovered.ms, recovered.counter), (1002, 0));
            assert!(recovered > remote);
            assert!(recovered > last);
            assert_eq!(recovered.device, 7);
            assert_eq!(clock.last, Some(recovered));
        }
    }

    #[test]
    fn hlc_order_matches_lexicographic_string_order() {
        let timestamps = [
            Hlc {
                ms: 10,
                counter: 0x10,
                device: 0x10,
            },
            Hlc {
                ms: 9,
                counter: 0x10,
                device: 0x0f,
            },
            Hlc {
                ms: 10,
                counter: 0x0f,
                device: 0x10,
            },
            Hlc {
                ms: 9,
                counter: 0x0f,
                device: 0x10,
            },
            Hlc {
                ms: 10,
                counter: 0x10,
                device: 0x0f,
            },
            Hlc {
                ms: 9,
                counter: 0x0f,
                device: 0x0f,
            },
            Hlc {
                ms: 10,
                counter: 0x0f,
                device: 0x0f,
            },
            Hlc {
                ms: 9,
                counter: 0x10,
                device: 0x10,
            },
        ];

        let mut sorted_hlcs = timestamps.to_vec();
        sorted_hlcs.sort();
        let strings_from_sorted_hlcs: Vec<String> =
            sorted_hlcs.iter().map(ToString::to_string).collect();

        let mut sorted_strings: Vec<String> = timestamps.iter().map(ToString::to_string).collect();
        sorted_strings.sort();

        assert_eq!(strings_from_sorted_hlcs, sorted_strings);
    }
}
