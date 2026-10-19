#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupIntentPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardMandateOptions>,
    /// Selected network to process this SetupIntent on.
    /// Depends on the available networks of the card attached to the setup intent.
    /// Can be only set confirm-time.
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupIntentPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupIntentPaymentMethodOptionsCardBuilder {
    mandate_options:
        Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCardMandateOptions>>,
    network: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>>,
    request_three_d_secure: Option<Option<stripe_shared::RequestThreeDSecure>>,
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

    impl Deserialize for SetupIntentPaymentMethodOptionsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupIntentPaymentMethodOptionsCard>,
        builder: SetupIntentPaymentMethodOptionsCardBuilder,
    }

    impl Visitor for Place<SetupIntentPaymentMethodOptionsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupIntentPaymentMethodOptionsCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupIntentPaymentMethodOptionsCardBuilder {
        type Out = SetupIntentPaymentMethodOptionsCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                "network" => Deserialize::begin(&mut self.network),
                "request_three_d_secure" => Deserialize::begin(&mut self.request_three_d_secure),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                mandate_options: Some(None),
                network: Some(None),
                request_three_d_secure: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(mandate_options), Some(network), Some(request_three_d_secure)) = (
                self.mandate_options.take(),
                self.network.take(),
                self.request_three_d_secure.take(),
            ) else {
                return None;
            };
            Some(Self::Out { mandate_options, network, request_three_d_secure })
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

    impl ObjectDeser for SetupIntentPaymentMethodOptionsCard {
        type Builder = SetupIntentPaymentMethodOptionsCardBuilder;
    }

    impl FromValueOpt for SetupIntentPaymentMethodOptionsCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupIntentPaymentMethodOptionsCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "request_three_d_secure" => {
                        b.request_three_d_secure = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Selected network to process this SetupIntent on.
/// Depends on the available networks of the card attached to the setup intent.
/// Can be only set confirm-time.
pub type SetupIntentPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type SetupIntentPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
