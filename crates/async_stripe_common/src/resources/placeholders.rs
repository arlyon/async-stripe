//! Structs of unknown origin that aren't picked
//! up by the codegen.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CountrySpecSupportedBankAccountCurrencies {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeRateRates {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationAmountDetails {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotificationEventData {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresent {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWallet {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarValueListItem {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionTransferData {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThreeDSecureDetails {}
