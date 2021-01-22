//! Structs of unknown origin that aren't picked
//! up by the codegen.

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CountrySpecSupportedBankAccountCurrencies {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExchangeRateRates {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingAuthorizationAmountDetails {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationEventData {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresent {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWallet {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RadarValueListItem {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionTransferData {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreeDSecureDetails {}
