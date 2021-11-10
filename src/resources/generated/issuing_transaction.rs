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
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub amount_details: Box<Option<IssuingTransactionAmountDetails>>,

    /// The `Authorization` object that led to this transaction.
    pub authorization: Box<Option<Expandable<IssuingAuthorization>>>,

    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction: Box<Option<Expandable<BalanceTransaction>>>,

    /// The card used to make this transaction.
    pub card: Expandable<IssuingCard>,

    /// The cardholder to whom this transaction belongs.
    pub cardholder: Box<Option<Expandable<IssuingCardholder>>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// If you've disputed the transaction, the ID of the dispute.
    pub dispute: Box<Option<Expandable<IssuingDispute>>>,

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
    pub purchase_details: Box<Option<IssuingTransactionPurchaseDetails>>,

    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,

    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Box<Option<IssuingTransactionWallet>>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Box<Option<i64>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Box<Option<IssuingTransactionFlightData>>,

    /// Information about fuel that was purchased with this transaction.
    pub fuel: Box<Option<IssuingTransactionFuelData>>,

    /// Information about lodging that was purchased with this transaction.
    pub lodging: Box<Option<IssuingTransactionLodgingData>>,

    /// The line items in the purchase.
    pub receipt: Box<Option<Vec<IssuingTransactionReceiptData>>>,

    /// A merchant-specific order number.
    pub reference: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionFlightData {
    /// The time that the flight departed.
    pub departure_at: Box<Option<i64>>,

    /// The name of the passenger.
    pub passenger_name: Box<Option<String>>,

    /// Whether the ticket is refundable.
    pub refundable: Box<Option<bool>>,

    /// The legs of the trip.
    pub segments: Box<Option<Vec<IssuingTransactionFlightDataLeg>>>,

    /// The travel agency that issued the ticket.
    pub travel_agency: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionFlightDataLeg {
    /// The three-letter IATA airport code of the flight's destination.
    pub arrival_airport_code: Box<Option<String>>,

    /// The airline carrier code.
    pub carrier: Box<Option<String>>,

    /// The three-letter IATA airport code that the flight departed from.
    pub departure_airport_code: Box<Option<String>>,

    /// The flight number.
    pub flight_number: Box<Option<String>>,

    /// The flight's service class.
    pub service_class: Box<Option<String>>,

    /// Whether a stopover is allowed on this flight.
    pub stopover_allowed: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub volume_decimal: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    pub check_in_at: Box<Option<i64>>,

    /// The number of nights stayed at the lodging.
    pub nights: Box<Option<i64>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionReceiptData {
    /// The description of the item.
    ///
    /// The maximum length of this field is 26 characters.
    pub description: Box<Option<String>>,

    /// The quantity of the item.
    pub quantity: Box<Option<f64>>,

    /// The total for this line item in cents.
    pub total: Box<Option<i64>>,

    /// The unit cost of the item in cents.
    pub unit_cost: Box<Option<i64>>,
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
