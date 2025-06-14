#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionPermissions {
    /// Determines which entity is allowed to update the shipping details.
    ///
    /// Default is `client_only`.
    /// Stripe Checkout client will automatically update the shipping details.
    /// If set to `server_only`, only your server is allowed to update the shipping details.
    ///
    /// When set to `server_only`, you must add the onShippingDetailsChange event handler when initializing the Stripe Checkout client and manually update the shipping details from your server using the Stripe API.
    pub update_shipping_details:
        Option<PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails>,
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionPermissionsBuilder {
    update_shipping_details:
        Option<Option<PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails>>,
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
    use miniserde::{make_place, Deserialize, Result};
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
            Self { update_shipping_details: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(update_shipping_details),) = (self.update_shipping_details,) else {
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
            *self.out = self.builder.take_out();
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
/// When set to `server_only`, you must add the onShippingDetailsChange event handler when initializing the Stripe Checkout client and manually update the shipping details from your server using the Stripe API.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    ClientOnly,
    ServerOnly,
}
impl PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    pub fn as_str(self) -> &'static str {
        use PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::*;
        match self {
            ClientOnly => "client_only",
            ServerOnly => "server_only",
        }
    }
}

impl std::str::FromStr for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::*;
        match s {
            "client_only" => Ok(ClientOnly),
            "server_only" => Ok(ServerOnly),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails",
            )
        })
    }
}
