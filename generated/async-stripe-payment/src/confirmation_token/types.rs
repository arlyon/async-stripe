/// ConfirmationTokens help transport client side data collected by Stripe JS over
/// to your server for confirming a PaymentIntent or SetupIntent. If the confirmation
/// is successful, values present on the ConfirmationToken are written onto the Intent.
///
/// To learn more about how to use ConfirmationToken, visit the related guides:
/// - [Finalize payments on the server](https://docs.stripe.com/payments/finalize-payments-on-the-server).
/// - [Build two-step confirmation](https://docs.stripe.com/payments/build-a-two-step-confirmation).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationToken {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Time at which this ConfirmationToken expires and can no longer be used to confirm a PaymentIntent or SetupIntent.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_payment::ConfirmationTokenId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
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
    /// The presence of this property will [attach the payment method](https://docs.stripe.com/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    pub setup_future_usage: Option<stripe_payment::ConfirmationTokenSetupFutureUsage>,
    /// ID of the SetupIntent that this ConfirmationToken was used to confirm, or null if this ConfirmationToken has not yet been used.
    pub setup_intent: Option<String>,
    /// Shipping information collected on this ConfirmationToken.
    pub shipping: Option<stripe_payment::ConfirmationTokensResourceShipping>,
    /// Indicates whether the Stripe SDK is used to handle confirmation flow.
    /// Defaults to `true` on ConfirmationToken.
    pub use_stripe_sdk: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmationToken").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: ConfirmationTokenBuilder {
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
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "expires_at" => Deserialize::begin(&mut self.builder.expires_at),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "mandate_data" => Deserialize::begin(&mut self.builder.mandate_data),
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                "payment_method_options" => {
                    Deserialize::begin(&mut self.builder.payment_method_options)
                }
                "payment_method_preview" => {
                    Deserialize::begin(&mut self.builder.payment_method_preview)
                }
                "return_url" => Deserialize::begin(&mut self.builder.return_url),
                "setup_future_usage" => Deserialize::begin(&mut self.builder.setup_future_usage),
                "setup_intent" => Deserialize::begin(&mut self.builder.setup_intent),
                "shipping" => Deserialize::begin(&mut self.builder.shipping),
                "use_stripe_sdk" => Deserialize::begin(&mut self.builder.use_stripe_sdk),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.created,
                self.builder.expires_at,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.mandate_data.take(),
                self.builder.payment_intent.take(),
                self.builder.payment_method_options.take(),
                self.builder.payment_method_preview.take(),
                self.builder.return_url.take(),
                self.builder.setup_future_usage.take(),
                self.builder.setup_intent.take(),
                self.builder.shipping.take(),
                self.builder.use_stripe_sdk,
            )
            else {
                return Ok(());
            };
            *self.out = Some(ConfirmationToken {
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
            });
            Ok(())
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmationTokenSetupFutureUsage {
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ConfirmationTokenSetupFutureUsage {
    pub fn as_str(&self) -> &str {
        use ConfirmationTokenSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ConfirmationTokenSetupFutureUsage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmationTokenSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ConfirmationTokenSetupFutureUsage"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ConfirmationTokenSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ConfirmationTokenSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokenSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ConfirmationTokenSetupFutureUsage)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for ConfirmationTokenSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ConfirmationTokenSetupFutureUsage> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ConfirmationTokenSetupFutureUsage::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmationTokenSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
