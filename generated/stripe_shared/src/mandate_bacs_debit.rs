#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct MandateBacsDebit {
    /// The status of the mandate on the Bacs network.
    /// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: MandateBacsDebitNetworkStatus,
    /// The unique reference identifying the mandate on the Bacs network.
    pub reference: String,
    /// The URL that will contain the mandate that the customer has signed.
    pub url: String,
}
#[cfg(feature = "min-ser")]
pub struct MandateBacsDebitBuilder {
    network_status: Option<MandateBacsDebitNetworkStatus>,
    reference: Option<String>,
    url: Option<String>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
            Ok(Box::new(Builder { out: &mut self.out, builder: MandateBacsDebitBuilder::deser_default() }))
        }
    }

    impl MapBuilder for MandateBacsDebitBuilder {
        type Out = MandateBacsDebit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "network_status" => Deserialize::begin(&mut self.network_status),
                "reference" => Deserialize::begin(&mut self.reference),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { network_status: Deserialize::default(), reference: Deserialize::default(), url: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let network_status = self.network_status.take()?;
            let reference = self.reference.take()?;
            let url = self.url.take()?;

            Some(Self::Out { network_status, reference, url })
        }
    }

    impl<'a> Map for Builder<'a> {
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
                    "network_status" => b.network_status = Some(FromValueOpt::from_value(v)?),
                    "reference" => b.reference = Some(FromValueOpt::from_value(v)?),
                    "url" => b.url = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The status of the mandate on the Bacs network.
/// Can be one of `pending`, `revoked`, `refused`, or `accepted`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MandateBacsDebitNetworkStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}
impl MandateBacsDebitNetworkStatus {
    pub fn as_str(self) -> &'static str {
        use MandateBacsDebitNetworkStatus::*;
        match self {
            Accepted => "accepted",
            Pending => "pending",
            Refused => "refused",
            Revoked => "revoked",
        }
    }
}

impl std::str::FromStr for MandateBacsDebitNetworkStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandateBacsDebitNetworkStatus::*;
        match s {
            "accepted" => Ok(Accepted),
            "pending" => Ok(Pending),
            "refused" => Ok(Refused),
            "revoked" => Ok(Revoked),
            _ => Err(()),
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
impl serde::Serialize for MandateBacsDebitNetworkStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MandateBacsDebitNetworkStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for MandateBacsDebitNetworkStatus"))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for MandateBacsDebitNetworkStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<MandateBacsDebitNetworkStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandateBacsDebitNetworkStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(MandateBacsDebitNetworkStatus);
