//! Key algorithms used in Matrix spec.

use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::{AsRefStr, Display, EnumString};

/// The basic key algorithms in the specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsRefStr, Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "snake_case"))]
#[non_exhaustive]
#[strum(serialize_all = "snake_case")]
pub enum DeviceKeyAlgorithm {
    /// The Ed25519 signature algorithm.
    Ed25519,

    /// The Curve25519 ECDH algorithm.
    Curve25519,

    /// The Curve25519 ECDH algorithm, but the key also contains signatures
    SignedCurve25519,
}

impl TryFrom<&'_ str> for DeviceKeyAlgorithm {
    type Error = strum::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for DeviceKeyAlgorithm {
    type Error = strum::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// The server key algorithms defined in the Matrix spec.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsRefStr, Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize), serde(rename_all = "snake_case"))]
#[non_exhaustive]
#[strum(serialize_all = "snake_case")]
pub enum ServerKeyAlgorithm {
    /// The Ed25519 signature algorithm.
    Ed25519,
}

impl TryFrom<&'_ str> for ServerKeyAlgorithm {
    type Error = strum::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for ServerKeyAlgorithm {
    type Error = strum::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// An encryption algorithm to be used to encrypt messages sent to a room.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(from = "String", into = "String")
)]
#[non_exhaustive]
pub enum EventEncryptionAlgorithm {
    /// Olm version 1 using Curve25519, AES-256, and SHA-256.
    OlmV1Curve25519AesSha2,

    /// Megolm version 1 using AES-256 and SHA-256.
    MegolmV1AesSha2,

    /// Any algorithm that is not part of the specification.
    Custom(String),
}

impl Display for EventEncryptionAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let algorithm_str = match *self {
            EventEncryptionAlgorithm::OlmV1Curve25519AesSha2 => "m.olm.v1.curve25519-aes-sha2",
            EventEncryptionAlgorithm::MegolmV1AesSha2 => "m.megolm.v1.aes-sha2",
            EventEncryptionAlgorithm::Custom(ref algorithm) => algorithm,
        };

        write!(f, "{}", algorithm_str)
    }
}

impl<T> From<T> for EventEncryptionAlgorithm
where
    T: Into<String> + AsRef<str>,
{
    fn from(s: T) -> EventEncryptionAlgorithm {
        match s.as_ref() {
            "m.olm.v1.curve25519-aes-sha2" => EventEncryptionAlgorithm::OlmV1Curve25519AesSha2,
            "m.megolm.v1.aes-sha2" => EventEncryptionAlgorithm::MegolmV1AesSha2,
            _ => EventEncryptionAlgorithm::Custom(s.into()),
        }
    }
}

impl From<EventEncryptionAlgorithm> for String {
    fn from(algorithm: EventEncryptionAlgorithm) -> String {
        algorithm.to_string()
    }
}

#[cfg(test)]
mod tests {
    use ruma_serde::test::serde_json_eq;
    use serde_json::json;

    use super::{DeviceKeyAlgorithm, EventEncryptionAlgorithm, ServerKeyAlgorithm};

    #[test]
    fn parse_device_key_algorithm() {
        assert_eq!("ed25519".parse(), Ok(DeviceKeyAlgorithm::Ed25519));
        assert_eq!("curve25519".parse(), Ok(DeviceKeyAlgorithm::Curve25519));
        assert_eq!("signed_curve25519".parse(), Ok(DeviceKeyAlgorithm::SignedCurve25519));
    }

    #[test]
    fn parse_server_key_algorithm() {
        assert_eq!("ed25519".parse(), Ok(ServerKeyAlgorithm::Ed25519));
    }

    #[test]
    fn event_encryption_algorithm_serde() {
        serde_json_eq(EventEncryptionAlgorithm::MegolmV1AesSha2, json!("m.megolm.v1.aes-sha2"));
        serde_json_eq(
            EventEncryptionAlgorithm::OlmV1Curve25519AesSha2,
            json!("m.olm.v1.curve25519-aes-sha2"),
        );
        serde_json_eq(
            EventEncryptionAlgorithm::Custom("io.ruma.test".into()),
            json!("io.ruma.test"),
        );
    }
}
