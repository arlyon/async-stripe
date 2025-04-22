#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutCardPaymentMethodOptions {
    pub installments: Option<stripe_shared::CheckoutCardInstallmentsOptions>,
    /// Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for this CheckoutSession.
    pub request_extended_authorization:
        Option<CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization>,
    /// Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession.
    pub request_incremental_authorization:
        Option<CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization>,
    /// Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession.
    pub request_multicapture: Option<CheckoutCardPaymentMethodOptionsRequestMulticapture>,
    /// Request ability to [overcapture](/payments/overcapture) for this CheckoutSession.
    pub request_overcapture: Option<CheckoutCardPaymentMethodOptionsRequestOvercapture>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// If not provided, this value defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: CheckoutCardPaymentMethodOptionsRequestThreeDSecure,
    pub restrictions:
        Option<stripe_shared::PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<CheckoutCardPaymentMethodOptionsSetupFutureUsage>,
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
#[doc(hidden)]
pub struct CheckoutCardPaymentMethodOptionsBuilder {
    installments: Option<Option<stripe_shared::CheckoutCardInstallmentsOptions>>,
    request_extended_authorization:
        Option<Option<CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization>>,
    request_incremental_authorization:
        Option<Option<CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization>>,
    request_multicapture: Option<Option<CheckoutCardPaymentMethodOptionsRequestMulticapture>>,
    request_overcapture: Option<Option<CheckoutCardPaymentMethodOptionsRequestOvercapture>>,
    request_three_d_secure: Option<CheckoutCardPaymentMethodOptionsRequestThreeDSecure>,
    restrictions: Option<
        Option<stripe_shared::PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions>,
    >,
    setup_future_usage: Option<Option<CheckoutCardPaymentMethodOptionsSetupFutureUsage>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutCardPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutCardPaymentMethodOptions>,
        builder: CheckoutCardPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutCardPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutCardPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutCardPaymentMethodOptionsBuilder {
        type Out = CheckoutCardPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "installments" => Deserialize::begin(&mut self.installments),
                "request_extended_authorization" => {
                    Deserialize::begin(&mut self.request_extended_authorization)
                }
                "request_incremental_authorization" => {
                    Deserialize::begin(&mut self.request_incremental_authorization)
                }
                "request_multicapture" => Deserialize::begin(&mut self.request_multicapture),
                "request_overcapture" => Deserialize::begin(&mut self.request_overcapture),
                "request_three_d_secure" => Deserialize::begin(&mut self.request_three_d_secure),
                "restrictions" => Deserialize::begin(&mut self.restrictions),
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
                installments: Deserialize::default(),
                request_extended_authorization: Deserialize::default(),
                request_incremental_authorization: Deserialize::default(),
                request_multicapture: Deserialize::default(),
                request_overcapture: Deserialize::default(),
                request_three_d_secure: Deserialize::default(),
                restrictions: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                statement_descriptor_suffix_kana: Deserialize::default(),
                statement_descriptor_suffix_kanji: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(installments),
                Some(request_extended_authorization),
                Some(request_incremental_authorization),
                Some(request_multicapture),
                Some(request_overcapture),
                Some(request_three_d_secure),
                Some(restrictions),
                Some(setup_future_usage),
                Some(statement_descriptor_suffix_kana),
                Some(statement_descriptor_suffix_kanji),
            ) = (
                self.installments,
                self.request_extended_authorization,
                self.request_incremental_authorization,
                self.request_multicapture,
                self.request_overcapture,
                self.request_three_d_secure,
                self.restrictions.take(),
                self.setup_future_usage,
                self.statement_descriptor_suffix_kana.take(),
                self.statement_descriptor_suffix_kanji.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                installments,
                request_extended_authorization,
                request_incremental_authorization,
                request_multicapture,
                request_overcapture,
                request_three_d_secure,
                restrictions,
                setup_future_usage,
                statement_descriptor_suffix_kana,
                statement_descriptor_suffix_kanji,
            })
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

    impl ObjectDeser for CheckoutCardPaymentMethodOptions {
        type Builder = CheckoutCardPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutCardPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutCardPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "installments" => b.installments = FromValueOpt::from_value(v),
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
                    "restrictions" => b.restrictions = FromValueOpt::from_value(v),
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
/// Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    IfAvailable,
    Never,
}
impl CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    pub fn as_str(self) -> &'static str {
        use CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization",
            )
        })
    }
}
/// Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization {
    IfAvailable,
    Never,
}
impl CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization {
    pub fn as_str(self) -> &'static str {
        use CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutCardPaymentMethodOptionsRequestIncrementalAuthorization",
            )
        })
    }
}
/// Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsRequestMulticapture {
    IfAvailable,
    Never,
}
impl CheckoutCardPaymentMethodOptionsRequestMulticapture {
    pub fn as_str(self) -> &'static str {
        use CheckoutCardPaymentMethodOptionsRequestMulticapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CheckoutCardPaymentMethodOptionsRequestMulticapture {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCardPaymentMethodOptionsRequestMulticapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptionsRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCardPaymentMethodOptionsRequestMulticapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutCardPaymentMethodOptionsRequestMulticapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutCardPaymentMethodOptionsRequestMulticapture {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutCardPaymentMethodOptionsRequestMulticapture> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCardPaymentMethodOptionsRequestMulticapture::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutCardPaymentMethodOptionsRequestMulticapture);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutCardPaymentMethodOptionsRequestMulticapture {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutCardPaymentMethodOptionsRequestMulticapture",
            )
        })
    }
}
/// Request ability to [overcapture](/payments/overcapture) for this CheckoutSession.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsRequestOvercapture {
    IfAvailable,
    Never,
}
impl CheckoutCardPaymentMethodOptionsRequestOvercapture {
    pub fn as_str(self) -> &'static str {
        use CheckoutCardPaymentMethodOptionsRequestOvercapture::*;
        match self {
            IfAvailable => "if_available",
            Never => "never",
        }
    }
}

impl std::str::FromStr for CheckoutCardPaymentMethodOptionsRequestOvercapture {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCardPaymentMethodOptionsRequestOvercapture::*;
        match s {
            "if_available" => Ok(IfAvailable),
            "never" => Ok(Never),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptionsRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCardPaymentMethodOptionsRequestOvercapture {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutCardPaymentMethodOptionsRequestOvercapture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutCardPaymentMethodOptionsRequestOvercapture {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutCardPaymentMethodOptionsRequestOvercapture> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCardPaymentMethodOptionsRequestOvercapture::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutCardPaymentMethodOptionsRequestOvercapture);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutCardPaymentMethodOptionsRequestOvercapture {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutCardPaymentMethodOptionsRequestOvercapture",
            )
        })
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// If not provided, this value defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}
impl CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        use CheckoutCardPaymentMethodOptionsRequestThreeDSecure::*;
        match self {
            Any => "any",
            Automatic => "automatic",
            Challenge => "challenge",
        }
    }
}

impl std::str::FromStr for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCardPaymentMethodOptionsRequestThreeDSecure::*;
        match s {
            "any" => Ok(Any),
            "automatic" => Ok(Automatic),
            "challenge" => Ok(Challenge),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutCardPaymentMethodOptionsRequestThreeDSecure> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCardPaymentMethodOptionsRequestThreeDSecure::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutCardPaymentMethodOptionsRequestThreeDSecure);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutCardPaymentMethodOptionsRequestThreeDSecure",
            )
        })
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CheckoutCardPaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutCardPaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutCardPaymentMethodOptionsSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutCardPaymentMethodOptionsSetupFutureUsage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutCardPaymentMethodOptionsSetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutCardPaymentMethodOptionsSetupFutureUsage",
            )
        })
    }
}
