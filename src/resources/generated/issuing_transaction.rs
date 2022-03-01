// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::IssuingTransactionId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    BalanceTransaction, Currency, IssuingAuthorization, IssuingCard, IssuingCardholder,
    IssuingDispute, IssuingTransactionType, MerchantData,
};

/// The resource representing a Stripe "IssuingTransaction".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransaction {
    /// Unique identifier for the object.
    pub id: IssuingTransactionId,

    /// The transaction amount, which will be reflected in your balance.
    ///
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<Box<IssuingTransactionAmountDetails>>,

    /// The `Authorization` object that led to this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Box<Expandable<IssuingAuthorization>>>,

    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Box<Expandable<BalanceTransaction>>>,

    /// The card used to make this transaction.
    pub card: Expandable<IssuingCard>,

    /// The cardholder to whom this transaction belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<Box<Expandable<IssuingCardholder>>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// If you've disputed the transaction, the ID of the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<Box<Expandable<IssuingDispute>>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,

    /// The currency with which the merchant is taking payment.
    pub merchant_currency: Currency,

    pub merchant_data: MerchantData,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Additional purchase information that is optionally provided by the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<Box<IssuingTransactionPurchaseDetails>>,

    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,

    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Box<IssuingTransactionWallet>>,
}

impl Object for IssuingTransaction {
    type Id = IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.transaction"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atm_fee: Option<Box<i64>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<Box<IssuingTransactionFlightData>>,

    /// Information about fuel that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<Box<IssuingTransactionFuelData>>,

    /// Information about lodging that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lodging: Option<Box<IssuingTransactionLodgingData>>,

    /// The line items in the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Box<Vec<IssuingTransactionReceiptData>>>,

    /// A merchant-specific order number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionFlightData {
    /// The time that the flight departed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<Box<i64>>,

    /// The name of the passenger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<Box<String>>,

    /// Whether the ticket is refundable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable: Option<Box<bool>>,

    /// The legs of the trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Box<Vec<IssuingTransactionFlightDataLeg>>>,

    /// The travel agency that issued the ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_agency: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionFlightDataLeg {
    /// The three-letter IATA airport code of the flight's destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_airport_code: Option<Box<String>>,

    /// The airline carrier code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<Box<String>>,

    /// The three-letter IATA airport code that the flight departed from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<Box<String>>,

    /// The flight number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<Box<String>>,

    /// The flight's service class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_class: Option<Box<String>>,

    /// Whether a stopover is allowed on this flight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopover_allowed: Option<Box<bool>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionFuelData {
    /// The type of fuel that was purchased.
    ///
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    pub type_: String,

    /// The units for `volume_decimal`.
    ///
    /// One of `us_gallon` or `liter`.
    pub unit: String,

    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: String,

    /// The volume of the fuel that was pumped, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_decimal: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_at: Option<Box<i64>>,

    /// The number of nights stayed at the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nights: Option<Box<i64>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionReceiptData {
    /// The description of the item.
    ///
    /// The maximum length of this field is 26 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    /// The quantity of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<f64>>,

    /// The total for this line item in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Box<i64>>,

    /// The unit cost of the item in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<Box<i64>>,
}

/// An enum representing the possible values of an `IssuingTransaction`'s `wallet` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTransactionWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl IssuingTransactionWallet {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingTransactionWallet::ApplePay => "apple_pay",
            IssuingTransactionWallet::GooglePay => "google_pay",
            IssuingTransactionWallet::SamsungPay => "samsung_pay",
        }
    }
}

impl AsRef<str> for IssuingTransactionWallet {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTransactionWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingTransactionWallet {
    fn default() -> Self {
        Self::ApplePay
    }
}
