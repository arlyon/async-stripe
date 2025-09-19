/// ConfirmationTokens help transport client side data collected by Stripe JS over
/// to your server for confirming a PaymentIntent or SetupIntent. If the confirmation
/// is successful, values present on the ConfirmationToken are written onto the Intent.
///
/// To learn more about how to use ConfirmationToken, visit the related guides:
/// - [Finalize payments on the server](https://stripe.com/docs/payments/finalize-payments-on-the-server).
/// - [Build two-step confirmation](https://stripe.com/docs/payments/build-a-two-step-confirmation).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationToken {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which this ConfirmationToken expires and can no longer be used to confirm a PaymentIntent or SetupIntent.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_payment::ConfirmationTokenId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Data used for generating a Mandate.
    pub mandate_data: Option<stripe_payment::ConfirmationTokensResourceMandateData>,
    /// ID of the PaymentIntent that this ConfirmationToken was used to confirm, or null if this ConfirmationToken has not yet been used.
    pub payment_intent: Option<String>,
    /// Payment-method-specific configuration for this ConfirmationToken.
    pub payment_method_options:
        Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptions>,
    /// Payment details collected by the Payment Element, used to create a PaymentMethod when a PaymentIntent or SetupIntent is confirmed with this ConfirmationToken.
    pub payment_method_preview:
        Option<stripe_payment::ConfirmationTokensResourcePaymentMethodPreview>,
    /// Return URL used to confirm the Intent.
    pub return_url: Option<String>,
    /// Indicates that you intend to make future payments with this ConfirmationToken's payment method.
    ///
    /// The presence of this property will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    pub setup_future_usage: Option<stripe_payment::ConfirmationTokenSetupFutureUsage>,
    /// ID of the SetupIntent that this ConfirmationToken was used to confirm, or null if this ConfirmationToken has not yet been used.
    pub setup_intent: Option<String>,
    /// Shipping information collected on this ConfirmationToken.
    pub shipping: Option<stripe_payment::ConfirmationTokensResourceShipping>,
    /// Indicates whether the Stripe SDK is used to handle confirmation flow.
    /// Defaults to `true` on ConfirmationToken.
    pub use_stripe_sdk: bool,
}
#[doc(hidden)]
pub struct ConfirmationTokenBuilder {
    created: Option<stripe_types::Timestamp>,
    expires_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_payment::ConfirmationTokenId>,
    livemode: Option<bool>,
    mandate_data: Option<Option<stripe_payment::ConfirmationTokensResourceMandateData>>,
    payment_intent: Option<Option<String>>,
    payment_method_options:
        Option<Option<stripe_payment::ConfirmationTokensResourcePaymentMethodOptions>>,
    payment_method_preview:
        Option<Option<stripe_payment::ConfirmationTokensResourcePaymentMethodPreview>>,
    return_url: Option<Option<String>>,
    setup_future_usage: Option<Option<stripe_payment::ConfirmationTokenSetupFutureUsage>>,
    setup_intent: Option<Option<String>>,
    shipping: Option<Option<stripe_payment::ConfirmationTokensResourceShipping>>,
    use_stripe_sdk: Option<bool>,
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

    impl Deserialize for ConfirmationToken {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationToken>,
        builder: ConfirmationTokenBuilder,
    }

    impl Visitor for Place<ConfirmationToken> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokenBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConfirmationTokenBuilder {
        type Out = ConfirmationToken;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "mandate_data" => Deserialize::begin(&mut self.mandate_data),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_method_options" => Deserialize::begin(&mut self.payment_method_options),
                "payment_method_preview" => Deserialize::begin(&mut self.payment_method_preview),
                "return_url" => Deserialize::begin(&mut self.return_url),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                "setup_intent" => Deserialize::begin(&mut self.setup_intent),
                "shipping" => Deserialize::begin(&mut self.shipping),
                "use_stripe_sdk" => Deserialize::begin(&mut self.use_stripe_sdk),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                mandate_data: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_method_options: Deserialize::default(),
                payment_method_preview: Deserialize::default(),
                return_url: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                setup_intent: Deserialize::default(),
                shipping: Deserialize::default(),
                use_stripe_sdk: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(expires_at),
                Some(id),
                Some(livemode),
                Some(mandate_data),
                Some(payment_intent),
                Some(payment_method_options),
                Some(payment_method_preview),
                Some(return_url),
                Some(setup_future_usage),
                Some(setup_intent),
                Some(shipping),
                Some(use_stripe_sdk),
            ) = (
                self.created,
                self.expires_at,
                self.id.take(),
                self.livemode,
                self.mandate_data.take(),
                self.payment_intent.take(),
                self.payment_method_options.take(),
                self.payment_method_preview.take(),
                self.return_url.take(),
                self.setup_future_usage,
                self.setup_intent.take(),
                self.shipping.take(),
                self.use_stripe_sdk,
            )
            else {
                return None;
            };
            Some(Self::Out {
                created,
                expires_at,
                id,
                livemode,
                mandate_data,
                payment_intent,
                payment_method_options,
                payment_method_preview,
                return_url,
                setup_future_usage,
                setup_intent,
                shipping,
                use_stripe_sdk,
            })
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

    impl ObjectDeser for ConfirmationToken {
        type Builder = ConfirmationTokenBuilder;
    }

    impl FromValueOpt for ConfirmationToken {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConfirmationTokenBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "mandate_data" => b.mandate_data = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "payment_method_options" => {
                        b.payment_method_options = FromValueOpt::from_value(v)
                    }
                    "payment_method_preview" => {
                        b.payment_method_preview = FromValueOpt::from_value(v)
                    }
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    "setup_intent" => b.setup_intent = FromValueOpt::from_value(v),
                    "shipping" => b.shipping = FromValueOpt::from_value(v),
                    "use_stripe_sdk" => b.use_stripe_sdk = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ConfirmationToken {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ConfirmationToken", 14)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("mandate_data", &self.mandate_data)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("payment_method_options", &self.payment_method_options)?;
        s.serialize_field("payment_method_preview", &self.payment_method_preview)?;
        s.serialize_field("return_url", &self.return_url)?;
        s.serialize_field("setup_future_usage", &self.setup_future_usage)?;
        s.serialize_field("setup_intent", &self.setup_intent)?;
        s.serialize_field("shipping", &self.shipping)?;
        s.serialize_field("use_stripe_sdk", &self.use_stripe_sdk)?;

        s.serialize_field("object", "confirmation_token")?;
        s.end()
    }
}
impl stripe_types::Object for ConfirmationToken {
    type Id = stripe_payment::ConfirmationTokenId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ConfirmationTokenId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmationTokenSetupFutureUsage {
    OffSession,
    OnSession,
}
impl ConfirmationTokenSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use ConfirmationTokenSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for ConfirmationTokenSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmationTokenSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmationTokenSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmationTokenSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ConfirmationTokenSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ConfirmationTokenSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ConfirmationTokenSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ConfirmationTokenSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ConfirmationTokenSetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmationTokenSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ConfirmationTokenSetupFutureUsage")
        })
    }
}
