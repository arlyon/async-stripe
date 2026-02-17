#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    /// IP address of the user initiating the OutboundPayment.
    /// Set if `present` is set to `true`.
    /// IP address collection is required for risk and compliance reasons.
    /// This will be used to help determine if the OutboundPayment is authorized or should be blocked.
    pub ip_address: Option<String>,
    /// `true` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    /// Otherwise, `false`.
    pub present: bool,
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder {
    ip_address: Option<Option<String>>,
    present: Option<bool>,
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

    impl Deserialize for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails>,
        builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder,
    }

    impl Visitor for Place<TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder {
        type Out = TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ip_address" => Deserialize::begin(&mut self.ip_address),
                "present" => Deserialize::begin(&mut self.present),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { ip_address: Deserialize::default(), present: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(ip_address), Some(present)) = (self.ip_address.take(), self.present) else {
                return None;
            };
            Some(Self::Out { ip_address, present })
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

    impl ObjectDeser for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
        type Builder = TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder;
    }

    impl FromValueOpt for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ip_address" => b.ip_address = FromValueOpt::from_value(v),
                    "present" => b.present = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
