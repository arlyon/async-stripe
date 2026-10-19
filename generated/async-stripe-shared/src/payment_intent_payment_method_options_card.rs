#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>,
    /// Installment details for this payment.
    ///
    /// For more information, see the [installments integration guide](https://docs.stripe.com/payments/installments).
    pub installments: Option<stripe_shared::PaymentMethodOptionsCardInstallments>,
    /// Configuration options for setting up an eMandate for cards issued in India.
    pub mandate_options: Option<stripe_shared::PaymentMethodOptionsCardMandateOptions>,
    /// Selected network to process this payment intent on.
    /// Depends on the available networks of the card attached to the payment intent.
    /// Can be only set confirm-time.
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// Request ability to [capture beyond the standard authorization validity window](https://docs.stripe.com/payments/extended-authorization) for this PaymentIntent.
    pub request_extended_authorization:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// Request ability to [increment the authorization](https://docs.stripe.com/payments/incremental-authorization) for this PaymentIntent.
    pub request_incremental_authorization:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// Request ability to make [multiple captures](https://docs.stripe.com/payments/multicapture) for this PaymentIntent.
    pub request_multicapture:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// Request ability to [overcapture](https://docs.stripe.com/payments/overcapture) for this PaymentIntent.
    pub request_overcapture:
        Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
    /// When enabled, using a card that is attached to a customer will require the CVC to be provided again (i.e.
    /// using the cvc_token parameter).
    pub require_cvc_recollection: Option<bool>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<stripe_shared::SetupFutureUsage>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    pub statement_descriptor_suffix_kana: Option<String>,
    /// Provides information about a card payment that customers see on their statements.
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    pub statement_descriptor_suffix_kanji: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentPaymentMethodOptionsCardBuilder {
    capture_method: Option<Option<stripe_shared::PaymentMethodOptionsAlmaCaptureMethod>>,
    installments: Option<Option<stripe_shared::PaymentMethodOptionsCardInstallments>>,
    mandate_options: Option<Option<stripe_shared::PaymentMethodOptionsCardMandateOptions>>,
    network: Option<Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>>,
    request_extended_authorization:
        Option<Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>>,
    request_incremental_authorization:
        Option<Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>>,
    request_multicapture:
        Option<Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>>,
    request_overcapture:
        Option<Option<stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture>>,
    request_three_d_secure: Option<Option<stripe_shared::RequestThreeDSecure>>,
    require_cvc_recollection: Option<Option<bool>>,
    setup_future_usage: Option<Option<stripe_shared::SetupFutureUsage>>,
    statement_descriptor_suffix_kana: Option<Option<String>>,
    statement_descriptor_suffix_kanji: Option<Option<String>>,
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

    impl Deserialize for PaymentIntentPaymentMethodOptionsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptionsCard>,
        builder: PaymentIntentPaymentMethodOptionsCardBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptionsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentPaymentMethodOptionsCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsCardBuilder {
        type Out = PaymentIntentPaymentMethodOptionsCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => Deserialize::begin(&mut self.capture_method),
                "installments" => Deserialize::begin(&mut self.installments),
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                "network" => Deserialize::begin(&mut self.network),
                "request_extended_authorization" => {
                    Deserialize::begin(&mut self.request_extended_authorization)
                }
                "request_incremental_authorization" => {
                    Deserialize::begin(&mut self.request_incremental_authorization)
                }
                "request_multicapture" => Deserialize::begin(&mut self.request_multicapture),
                "request_overcapture" => Deserialize::begin(&mut self.request_overcapture),
                "request_three_d_secure" => Deserialize::begin(&mut self.request_three_d_secure),
                "require_cvc_recollection" => {
                    Deserialize::begin(&mut self.require_cvc_recollection)
                }
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                "statement_descriptor_suffix_kana" => {
                    Deserialize::begin(&mut self.statement_descriptor_suffix_kana)
                }
                "statement_descriptor_suffix_kanji" => {
                    Deserialize::begin(&mut self.statement_descriptor_suffix_kanji)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_method: Some(None),
                installments: Some(None),
                mandate_options: Some(None),
                network: Some(None),
                request_extended_authorization: Some(None),
                request_incremental_authorization: Some(None),
                request_multicapture: Some(None),
                request_overcapture: Some(None),
                request_three_d_secure: Some(None),
                require_cvc_recollection: Some(None),
                setup_future_usage: Some(None),
                statement_descriptor_suffix_kana: Some(None),
                statement_descriptor_suffix_kanji: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(capture_method),
                Some(installments),
                Some(mandate_options),
                Some(network),
                Some(request_extended_authorization),
                Some(request_incremental_authorization),
                Some(request_multicapture),
                Some(request_overcapture),
                Some(request_three_d_secure),
                Some(require_cvc_recollection),
                Some(setup_future_usage),
                Some(statement_descriptor_suffix_kana),
                Some(statement_descriptor_suffix_kanji),
            ) = (
                self.capture_method.take(),
                self.installments.take(),
                self.mandate_options.take(),
                self.network.take(),
                self.request_extended_authorization.take(),
                self.request_incremental_authorization.take(),
                self.request_multicapture.take(),
                self.request_overcapture.take(),
                self.request_three_d_secure.take(),
                self.require_cvc_recollection,
                self.setup_future_usage.take(),
                self.statement_descriptor_suffix_kana.take(),
                self.statement_descriptor_suffix_kanji.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                capture_method,
                installments,
                mandate_options,
                network,
                request_extended_authorization,
                request_incremental_authorization,
                request_multicapture,
                request_overcapture,
                request_three_d_secure,
                require_cvc_recollection,
                setup_future_usage,
                statement_descriptor_suffix_kana,
                statement_descriptor_suffix_kanji,
            })
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptionsCard {
        type Builder = PaymentIntentPaymentMethodOptionsCardBuilder;
    }

    impl FromValueOpt for PaymentIntentPaymentMethodOptionsCard {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentPaymentMethodOptionsCardBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "capture_method" => b.capture_method = FromValueOpt::from_value(v),
                    "installments" => b.installments = FromValueOpt::from_value(v),
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    "network" => b.network = FromValueOpt::from_value(v),
                    "request_extended_authorization" => {
                        b.request_extended_authorization = FromValueOpt::from_value(v)
                    }
                    "request_incremental_authorization" => {
                        b.request_incremental_authorization = FromValueOpt::from_value(v)
                    }
                    "request_multicapture" => b.request_multicapture = FromValueOpt::from_value(v),
                    "request_overcapture" => b.request_overcapture = FromValueOpt::from_value(v),
                    "request_three_d_secure" => {
                        b.request_three_d_secure = FromValueOpt::from_value(v)
                    }
                    "require_cvc_recollection" => {
                        b.require_cvc_recollection = FromValueOpt::from_value(v)
                    }
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    "statement_descriptor_suffix_kana" => {
                        b.statement_descriptor_suffix_kana = FromValueOpt::from_value(v)
                    }
                    "statement_descriptor_suffix_kanji" => {
                        b.statement_descriptor_suffix_kanji = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls when the funds will be captured from the customer's account.
pub type PaymentIntentPaymentMethodOptionsCardCaptureMethod =
    stripe_shared::PaymentMethodOptionsAlmaCaptureMethod;
/// Selected network to process this payment intent on.
/// Depends on the available networks of the card attached to the payment intent.
/// Can be only set confirm-time.
pub type PaymentIntentPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// Request ability to [capture beyond the standard authorization validity window](https://docs.stripe.com/payments/extended-authorization) for this PaymentIntent.
pub type PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// Request ability to [increment the authorization](https://docs.stripe.com/payments/incremental-authorization) for this PaymentIntent.
pub type PaymentIntentPaymentMethodOptionsCardRequestIncrementalAuthorization =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// Request ability to make [multiple captures](https://docs.stripe.com/payments/multicapture) for this PaymentIntent.
pub type PaymentIntentPaymentMethodOptionsCardRequestMulticapture =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// Request ability to [overcapture](https://docs.stripe.com/payments/overcapture) for this PaymentIntent.
pub type PaymentIntentPaymentMethodOptionsCardRequestOvercapture =
    stripe_shared::CheckoutCardPaymentMethodOptionsRequestOvercapture;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
pub type PaymentIntentPaymentMethodOptionsCardSetupFutureUsage = stripe_shared::SetupFutureUsage;
