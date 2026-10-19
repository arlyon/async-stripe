#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
    /// The card brands to block.
    /// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
    pub brands_blocked: Option<Vec<stripe_shared::BrandsBlocked>>,
    /// Card funding types to block for this Checkout Session.
    /// Supported values are `credit`, `debit`, and `prepaid`.
    pub funding_types_blocked: Option<Vec<stripe_shared::FundingTypesBlocked>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder {
    brands_blocked: Option<Option<Vec<stripe_shared::BrandsBlocked>>>,
    funding_types_blocked: Option<Option<Vec<stripe_shared::FundingTypesBlocked>>>,
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

    impl Deserialize for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions>,
        builder: PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder,
    }

    impl Visitor for Place<PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder {
        type Out = PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "brands_blocked" => Deserialize::begin(&mut self.brands_blocked),
                "funding_types_blocked" => Deserialize::begin(&mut self.funding_types_blocked),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { brands_blocked: Some(None), funding_types_blocked: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(brands_blocked), Some(funding_types_blocked)) =
                (self.brands_blocked.take(), self.funding_types_blocked.take())
            else {
                return None;
            };
            Some(Self::Out { brands_blocked, funding_types_blocked })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
        type Builder = PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder;
    }

    impl FromValueOpt for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "brands_blocked" => b.brands_blocked = FromValueOpt::from_value(v),
                    "funding_types_blocked" => {
                        b.funding_types_blocked = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The card brands to block.
/// If a customer enters or selects a card belonging to a blocked brand, they can't complete the payment.
pub type PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlocked =
    stripe_shared::BrandsBlocked;
/// Card funding types to block for this Checkout Session.
/// Supported values are `credit`, `debit`, and `prepaid`.
pub type PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlocked =
    stripe_shared::FundingTypesBlocked;
