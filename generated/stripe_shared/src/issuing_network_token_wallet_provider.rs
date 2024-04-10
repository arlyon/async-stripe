#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingNetworkTokenWalletProvider {
    /// The wallet provider-given account ID of the digital wallet the token belongs to.
    pub account_id: Option<String>,
    /// An evaluation on the trustworthiness of the wallet account between 1 and 5.
    /// A higher score indicates more trustworthy.
    pub account_trust_score: Option<i64>,
    /// The method used for tokenizing a card.
    pub card_number_source: Option<IssuingNetworkTokenWalletProviderCardNumberSource>,
    pub cardholder_address: Option<stripe_shared::IssuingNetworkTokenAddress>,
    /// The name of the cardholder tokenizing the card.
    pub cardholder_name: Option<String>,
    /// An evaluation on the trustworthiness of the device. A higher score indicates more trustworthy.
    pub device_trust_score: Option<i64>,
    /// The hashed email address of the cardholder's account with the wallet provider.
    pub hashed_account_email_address: Option<String>,
    /// The reasons for suggested tokenization given by the card network.
    pub reason_codes: Option<Vec<IssuingNetworkTokenWalletProviderReasonCodes>>,
    /// The recommendation on responding to the tokenization request.
    pub suggested_decision: Option<IssuingNetworkTokenWalletProviderSuggestedDecision>,
    /// The version of the standard for mapping reason codes followed by the wallet provider.
    pub suggested_decision_version: Option<String>,
}
#[doc(hidden)]
pub struct IssuingNetworkTokenWalletProviderBuilder {
    account_id: Option<Option<String>>,
    account_trust_score: Option<Option<i64>>,
    card_number_source: Option<Option<IssuingNetworkTokenWalletProviderCardNumberSource>>,
    cardholder_address: Option<Option<stripe_shared::IssuingNetworkTokenAddress>>,
    cardholder_name: Option<Option<String>>,
    device_trust_score: Option<Option<i64>>,
    hashed_account_email_address: Option<Option<String>>,
    reason_codes: Option<Option<Vec<IssuingNetworkTokenWalletProviderReasonCodes>>>,
    suggested_decision: Option<Option<IssuingNetworkTokenWalletProviderSuggestedDecision>>,
    suggested_decision_version: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingNetworkTokenWalletProvider {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingNetworkTokenWalletProvider>,
        builder: IssuingNetworkTokenWalletProviderBuilder,
    }

    impl Visitor for Place<IssuingNetworkTokenWalletProvider> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingNetworkTokenWalletProviderBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingNetworkTokenWalletProviderBuilder {
        type Out = IssuingNetworkTokenWalletProvider;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_id" => Deserialize::begin(&mut self.account_id),
                "account_trust_score" => Deserialize::begin(&mut self.account_trust_score),
                "card_number_source" => Deserialize::begin(&mut self.card_number_source),
                "cardholder_address" => Deserialize::begin(&mut self.cardholder_address),
                "cardholder_name" => Deserialize::begin(&mut self.cardholder_name),
                "device_trust_score" => Deserialize::begin(&mut self.device_trust_score),
                "hashed_account_email_address" => {
                    Deserialize::begin(&mut self.hashed_account_email_address)
                }
                "reason_codes" => Deserialize::begin(&mut self.reason_codes),
                "suggested_decision" => Deserialize::begin(&mut self.suggested_decision),
                "suggested_decision_version" => {
                    Deserialize::begin(&mut self.suggested_decision_version)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_id: Deserialize::default(),
                account_trust_score: Deserialize::default(),
                card_number_source: Deserialize::default(),
                cardholder_address: Deserialize::default(),
                cardholder_name: Deserialize::default(),
                device_trust_score: Deserialize::default(),
                hashed_account_email_address: Deserialize::default(),
                reason_codes: Deserialize::default(),
                suggested_decision: Deserialize::default(),
                suggested_decision_version: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account_id: self.account_id.take()?,
                account_trust_score: self.account_trust_score?,
                card_number_source: self.card_number_source?,
                cardholder_address: self.cardholder_address.take()?,
                cardholder_name: self.cardholder_name.take()?,
                device_trust_score: self.device_trust_score?,
                hashed_account_email_address: self.hashed_account_email_address.take()?,
                reason_codes: self.reason_codes.take()?,
                suggested_decision: self.suggested_decision?,
                suggested_decision_version: self.suggested_decision_version.take()?,
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

    impl ObjectDeser for IssuingNetworkTokenWalletProvider {
        type Builder = IssuingNetworkTokenWalletProviderBuilder;
    }

    impl FromValueOpt for IssuingNetworkTokenWalletProvider {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingNetworkTokenWalletProviderBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_id" => b.account_id = Some(FromValueOpt::from_value(v)?),
                    "account_trust_score" => {
                        b.account_trust_score = Some(FromValueOpt::from_value(v)?)
                    }
                    "card_number_source" => {
                        b.card_number_source = Some(FromValueOpt::from_value(v)?)
                    }
                    "cardholder_address" => {
                        b.cardholder_address = Some(FromValueOpt::from_value(v)?)
                    }
                    "cardholder_name" => b.cardholder_name = Some(FromValueOpt::from_value(v)?),
                    "device_trust_score" => {
                        b.device_trust_score = Some(FromValueOpt::from_value(v)?)
                    }
                    "hashed_account_email_address" => {
                        b.hashed_account_email_address = Some(FromValueOpt::from_value(v)?)
                    }
                    "reason_codes" => b.reason_codes = Some(FromValueOpt::from_value(v)?),
                    "suggested_decision" => {
                        b.suggested_decision = Some(FromValueOpt::from_value(v)?)
                    }
                    "suggested_decision_version" => {
                        b.suggested_decision_version = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The method used for tokenizing a card.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenWalletProviderCardNumberSource {
    App,
    Manual,
    OnFile,
    Other,
}
impl IssuingNetworkTokenWalletProviderCardNumberSource {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenWalletProviderCardNumberSource::*;
        match self {
            App => "app",
            Manual => "manual",
            OnFile => "on_file",
            Other => "other",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenWalletProviderCardNumberSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenWalletProviderCardNumberSource::*;
        match s {
            "app" => Ok(App),
            "manual" => Ok(Manual),
            "on_file" => Ok(OnFile),
            "other" => Ok(Other),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingNetworkTokenWalletProviderCardNumberSource> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingNetworkTokenWalletProviderCardNumberSource::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingNetworkTokenWalletProviderCardNumberSource);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenWalletProviderCardNumberSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingNetworkTokenWalletProviderCardNumberSource",
            )
        })
    }
}
/// The reasons for suggested tokenization given by the card network.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingNetworkTokenWalletProviderReasonCodes {
    AccountCardTooNew,
    AccountRecentlyChanged,
    AccountTooNew,
    AccountTooNewSinceLaunch,
    AdditionalDevice,
    DataExpired,
    DeferIdVDecision,
    DeviceRecentlyLost,
    GoodActivityHistory,
    HasSuspendedTokens,
    HighRisk,
    InactiveAccount,
    LongAccountTenure,
    LowAccountScore,
    LowDeviceScore,
    LowPhoneNumberScore,
    NetworkServiceError,
    OutsideHomeTerritory,
    ProvisioningCardholderMismatch,
    ProvisioningDeviceAndCardholderMismatch,
    ProvisioningDeviceMismatch,
    SameDeviceNoPriorAuthentication,
    SameDeviceSuccessfulPriorAuthentication,
    SoftwareUpdate,
    SuspiciousActivity,
    TooManyDifferentCardholders,
    TooManyRecentAttempts,
    TooManyRecentTokens,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl IssuingNetworkTokenWalletProviderReasonCodes {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenWalletProviderReasonCodes::*;
        match self {
            AccountCardTooNew => "account_card_too_new",
            AccountRecentlyChanged => "account_recently_changed",
            AccountTooNew => "account_too_new",
            AccountTooNewSinceLaunch => "account_too_new_since_launch",
            AdditionalDevice => "additional_device",
            DataExpired => "data_expired",
            DeferIdVDecision => "defer_id_v_decision",
            DeviceRecentlyLost => "device_recently_lost",
            GoodActivityHistory => "good_activity_history",
            HasSuspendedTokens => "has_suspended_tokens",
            HighRisk => "high_risk",
            InactiveAccount => "inactive_account",
            LongAccountTenure => "long_account_tenure",
            LowAccountScore => "low_account_score",
            LowDeviceScore => "low_device_score",
            LowPhoneNumberScore => "low_phone_number_score",
            NetworkServiceError => "network_service_error",
            OutsideHomeTerritory => "outside_home_territory",
            ProvisioningCardholderMismatch => "provisioning_cardholder_mismatch",
            ProvisioningDeviceAndCardholderMismatch => {
                "provisioning_device_and_cardholder_mismatch"
            }
            ProvisioningDeviceMismatch => "provisioning_device_mismatch",
            SameDeviceNoPriorAuthentication => "same_device_no_prior_authentication",
            SameDeviceSuccessfulPriorAuthentication => {
                "same_device_successful_prior_authentication"
            }
            SoftwareUpdate => "software_update",
            SuspiciousActivity => "suspicious_activity",
            TooManyDifferentCardholders => "too_many_different_cardholders",
            TooManyRecentAttempts => "too_many_recent_attempts",
            TooManyRecentTokens => "too_many_recent_tokens",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenWalletProviderReasonCodes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenWalletProviderReasonCodes::*;
        match s {
            "account_card_too_new" => Ok(AccountCardTooNew),
            "account_recently_changed" => Ok(AccountRecentlyChanged),
            "account_too_new" => Ok(AccountTooNew),
            "account_too_new_since_launch" => Ok(AccountTooNewSinceLaunch),
            "additional_device" => Ok(AdditionalDevice),
            "data_expired" => Ok(DataExpired),
            "defer_id_v_decision" => Ok(DeferIdVDecision),
            "device_recently_lost" => Ok(DeviceRecentlyLost),
            "good_activity_history" => Ok(GoodActivityHistory),
            "has_suspended_tokens" => Ok(HasSuspendedTokens),
            "high_risk" => Ok(HighRisk),
            "inactive_account" => Ok(InactiveAccount),
            "long_account_tenure" => Ok(LongAccountTenure),
            "low_account_score" => Ok(LowAccountScore),
            "low_device_score" => Ok(LowDeviceScore),
            "low_phone_number_score" => Ok(LowPhoneNumberScore),
            "network_service_error" => Ok(NetworkServiceError),
            "outside_home_territory" => Ok(OutsideHomeTerritory),
            "provisioning_cardholder_mismatch" => Ok(ProvisioningCardholderMismatch),
            "provisioning_device_and_cardholder_mismatch" => {
                Ok(ProvisioningDeviceAndCardholderMismatch)
            }
            "provisioning_device_mismatch" => Ok(ProvisioningDeviceMismatch),
            "same_device_no_prior_authentication" => Ok(SameDeviceNoPriorAuthentication),
            "same_device_successful_prior_authentication" => {
                Ok(SameDeviceSuccessfulPriorAuthentication)
            }
            "software_update" => Ok(SoftwareUpdate),
            "suspicious_activity" => Ok(SuspiciousActivity),
            "too_many_different_cardholders" => Ok(TooManyDifferentCardholders),
            "too_many_recent_attempts" => Ok(TooManyRecentAttempts),
            "too_many_recent_tokens" => Ok(TooManyRecentTokens),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenWalletProviderReasonCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenWalletProviderReasonCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingNetworkTokenWalletProviderReasonCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingNetworkTokenWalletProviderReasonCodes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingNetworkTokenWalletProviderReasonCodes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingNetworkTokenWalletProviderReasonCodes::from_str(s)
                .unwrap_or(IssuingNetworkTokenWalletProviderReasonCodes::Unknown),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingNetworkTokenWalletProviderReasonCodes);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenWalletProviderReasonCodes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
/// The recommendation on responding to the tokenization request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IssuingNetworkTokenWalletProviderSuggestedDecision {
    Approve,
    Decline,
    RequireAuth,
}
impl IssuingNetworkTokenWalletProviderSuggestedDecision {
    pub fn as_str(self) -> &'static str {
        use IssuingNetworkTokenWalletProviderSuggestedDecision::*;
        match self {
            Approve => "approve",
            Decline => "decline",
            RequireAuth => "require_auth",
        }
    }
}

impl std::str::FromStr for IssuingNetworkTokenWalletProviderSuggestedDecision {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingNetworkTokenWalletProviderSuggestedDecision::*;
        match s {
            "approve" => Ok(Approve),
            "decline" => Ok(Decline),
            "require_auth" => Ok(RequireAuth),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingNetworkTokenWalletProviderSuggestedDecision> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingNetworkTokenWalletProviderSuggestedDecision::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingNetworkTokenWalletProviderSuggestedDecision);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingNetworkTokenWalletProviderSuggestedDecision {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for IssuingNetworkTokenWalletProviderSuggestedDecision",
            )
        })
    }
}
