#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder {
    ip_address: Option<Option<String>>,
    present: Option<bool>,
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
                builder:
                    TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetailsBuilder {
                        ip_address: Deserialize::default(),
                        present: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ip_address" => Deserialize::begin(&mut self.builder.ip_address),
                "present" => Deserialize::begin(&mut self.builder.present),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(ip_address), Some(present)) =
                (self.builder.ip_address.take(), self.builder.present)
            else {
                return Ok(());
            };
            *self.out =
                Some(TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails {
                    ip_address,
                    present,
                });
            Ok(())
        }
    }
};
