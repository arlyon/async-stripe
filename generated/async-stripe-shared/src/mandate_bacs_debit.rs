#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct MandateBacsDebit {
    /// The status of the mandate on the Bacs network.
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: MandateBacsDebitNetworkStatus,
    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,
    /// When the mandate is revoked on the Bacs network this field displays the reason for the revocation.
    pub revocation_reason: Option<MandateBacsDebitRevocationReason>,
    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}
#[doc(hidden)]
pub struct MandateBacsDebitBuilder {
    network_status: Option<MandateBacsDebitNetworkStatus>,
    reference: Option<String>,
    revocation_reason: Option<Option<MandateBacsDebitRevocationReason>>,
    url: Option<String>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for MandateBacsDebit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<MandateBacsDebit>,
        builder: MandateBacsDebitBuilder,
    }

    impl Visitor for Place<MandateBacsDebit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: MandateBacsDebitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for MandateBacsDebitBuilder {
        type Out = MandateBacsDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "network_status" => Deserialize::begin(&mut self.network_status),
                "reference" => Deserialize::begin(&mut self.reference),
                "revocation_reason" => Deserialize::begin(&mut self.revocation_reason),
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                network_status: Deserialize::default(),
                reference: Deserialize::default(),
                revocation_reason: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(network_status), Some(reference), Some(revocation_reason), Some(url)) = (
                self.network_status.take(),
                self.reference.take(),
                self.revocation_reason.take(),
                self.url.take(),
            ) else {
                return None;
            };
            Some(Self::Out { network_status, reference, revocation_reason, url })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for MandateBacsDebit {
        type Builder = MandateBacsDebitBuilder;
    }

    impl FromValueOpt for MandateBacsDebit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = MandateBacsDebitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "network_status" => b.network_status = FromValueOpt::from_value(v),
                    "reference" => b.reference = FromValueOpt::from_value(v),
                    "revocation_reason" => b.revocation_reason = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The status of the mandate on the Bacs network.
/// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateBacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateBacsDebitNetworkStatus {
    pub fn as_str(&self) -> &str {
        use MandateBacsDebitNetworkStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateBacsDebitNetworkStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateBacsDebitNetworkStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandateBacsDebitNetworkStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateBacsDebitNetworkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateBacsDebitNetworkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandateBacsDebitNetworkStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateBacsDebitNetworkStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateBacsDebitNetworkStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateBacsDebitNetworkStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateBacsDebitNetworkStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// When the mandate is revoked on the Bacs network this field displays the reason for the revocation.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandateBacsDebitRevocationReason {
    AccountClosed,
    BankAccountRestricted,
    BankOwnershipChanged,
    CouldNotProcess,
    DebitNotAuthorized,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandateBacsDebitRevocationReason {
    pub fn as_str(&self) -> &str {
        use MandateBacsDebitRevocationReason::*;
        match self {
            AccountClosed => "account_closed",
            BankAccountRestricted => "bank_account_restricted",
            BankOwnershipChanged => "bank_ownership_changed",
            CouldNotProcess => "could_not_process",
            DebitNotAuthorized => "debit_not_authorized",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandateBacsDebitRevocationReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateBacsDebitRevocationReason::*;
        match s {
            "account_closed" => Ok(AccountClosed),
            "bank_account_restricted" => Ok(BankAccountRestricted),
            "bank_ownership_changed" => Ok(BankOwnershipChanged),
            "could_not_process" => Ok(CouldNotProcess),
            "debit_not_authorized" => Ok(DebitNotAuthorized),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandateBacsDebitRevocationReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandateBacsDebitRevocationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for MandateBacsDebitRevocationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MandateBacsDebitRevocationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandateBacsDebitRevocationReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandateBacsDebitRevocationReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateBacsDebitRevocationReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandateBacsDebitRevocationReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandateBacsDebitRevocationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
