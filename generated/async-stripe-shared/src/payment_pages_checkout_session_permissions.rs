#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionPermissions {
    /// Determines which entity is allowed to update the shipping details.
    ///
    /// Default is `client_only`.
    /// Stripe Checkout client will automatically update the shipping details.
    /// If set to `server_only`, only your server is allowed to update the shipping details.
    ///
    /// This parameter is only supported when `ui_mode=elements`.
    pub update_shipping_details: Option<stripe_shared::UpdateShippingDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionPermissions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionPermissionsBuilder {
    update_shipping_details: Option<Option<stripe_shared::UpdateShippingDetails>>,
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

    impl Deserialize for PaymentPagesCheckoutSessionPermissions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionPermissions>,
        builder: PaymentPagesCheckoutSessionPermissionsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionPermissions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionPermissionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionPermissionsBuilder {
        type Out = PaymentPagesCheckoutSessionPermissions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "update_shipping_details" => Deserialize::begin(&mut self.update_shipping_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { update_shipping_details: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(update_shipping_details),) = (self.update_shipping_details.take(),) else {
                return None;
            };
            Some(Self::Out { update_shipping_details })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionPermissions {
        type Builder = PaymentPagesCheckoutSessionPermissionsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionPermissions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentPagesCheckoutSessionPermissionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "update_shipping_details" => {
                        b.update_shipping_details = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Determines which entity is allowed to update the shipping details.
///
/// Default is `client_only`.
/// Stripe Checkout client will automatically update the shipping details.
/// If set to `server_only`, only your server is allowed to update the shipping details.
///
/// This parameter is only supported when `ui_mode=elements`.
pub type PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails =
    stripe_shared::UpdateShippingDetails;
