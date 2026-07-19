/// Indicates the status of a specific payment method on a payment method domain.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    /// The status of the payment method on the domain.
    pub status: PaymentMethodDomainResourcePaymentMethodStatusStatus,
    pub status_details:
        Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDomainResourcePaymentMethodStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDomainResourcePaymentMethodStatus").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusBuilder {
    status: Option<PaymentMethodDomainResourcePaymentMethodStatusStatus>,
    status_details:
        Option<Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatusDetails>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for PaymentMethodDomainResourcePaymentMethodStatus {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDomainResourcePaymentMethodStatus>,
        builder: PaymentMethodDomainResourcePaymentMethodStatusBuilder,
    }

    impl Visitor for Place<PaymentMethodDomainResourcePaymentMethodStatus> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDomainResourcePaymentMethodStatusBuilder {
                    status: Deserialize::default(),
                    status_details: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.builder.status),
                "status_details" => Deserialize::begin(&mut self.builder.status_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(status), Some(status_details)) =
                (self.builder.status.take(), self.builder.status_details.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(PaymentMethodDomainResourcePaymentMethodStatus { status, status_details });
            Ok(())
        }
    }
};
/// The status of the payment method on the domain.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDomainResourcePaymentMethodStatusStatus {
    Active,
    Inactive,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodDomainResourcePaymentMethodStatusStatus {
    pub fn as_str(&self) -> &str {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodDomainResourcePaymentMethodStatusStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodDomainResourcePaymentMethodStatusStatus))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodDomainResourcePaymentMethodStatusStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDomainResourcePaymentMethodStatusStatus::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
