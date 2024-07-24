/// Indicates the status of a specific payment method on a payment method domain.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    /// The status of the payment method on the domain.
    pub status: PaymentMethodDomainResourcePaymentMethodStatusStatus,
    pub status_details:
        Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}
#[doc(hidden)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusBuilder {
    status: Option<PaymentMethodDomainResourcePaymentMethodStatusStatus>,
    status_details:
        Option<Option<stripe_payment::PaymentMethodDomainResourcePaymentMethodStatusDetails>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: PaymentMethodDomainResourcePaymentMethodStatusBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDomainResourcePaymentMethodStatusBuilder {
        type Out = PaymentMethodDomainResourcePaymentMethodStatus;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.status),
                "status_details" => Deserialize::begin(&mut self.status_details),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { status: Deserialize::default(), status_details: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out { status: self.status?, status_details: self.status_details.take()? })
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

    impl ObjectDeser for PaymentMethodDomainResourcePaymentMethodStatus {
        type Builder = PaymentMethodDomainResourcePaymentMethodStatusBuilder;
    }

    impl FromValueOpt for PaymentMethodDomainResourcePaymentMethodStatus {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDomainResourcePaymentMethodStatusBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "status_details" => b.status_details = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The status of the payment method on the domain.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDomainResourcePaymentMethodStatusStatus {
    Active,
    Inactive,
}
impl PaymentMethodDomainResourcePaymentMethodStatusStatus {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDomainResourcePaymentMethodStatusStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDomainResourcePaymentMethodStatusStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentMethodDomainResourcePaymentMethodStatusStatus::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDomainResourcePaymentMethodStatusStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentMethodDomainResourcePaymentMethodStatusStatus",
            )
        })
    }
}
