// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingTransactionId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    BalanceTransaction, Currency, IssuingAuthorization, IssuingCard, IssuingCardholder,
    IssuingDispute, IssuingToken, IssuingTransactionType, MerchantData,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingTransaction".
///
/// For more details see <https://stripe.com/docs/api/issuing/transactions/object>
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
    pub amount_details: Option<IssuingTransactionAmountDetails>,

    /// The `Authorization` object that led to this transaction.
    pub authorization: Option<Expandable<IssuingAuthorization>>,

    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// The card used to make this transaction.
    pub card: Expandable<IssuingCard>,

    /// The cardholder to whom this transaction belongs.
    pub cardholder: Option<Expandable<IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// If you've disputed the transaction, the ID of the dispute.
    pub dispute: Option<Expandable<IssuingDispute>>,

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

    /// Details about the transaction, such as processing dates, set by the card network.
    pub network_data: Option<IssuingTransactionNetworkData>,

    /// Additional purchase information that is optionally provided by the merchant.
    pub purchase_details: Option<IssuingTransactionPurchaseDetails>,

    /// [Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this transaction.
    ///
    /// If a network token was not used for this transaction, this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<Expandable<IssuingToken>>,

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<IssuingTransactionTreasury>,

    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,

    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<IssuingTransactionWallet>,
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
    pub atm_fee: Option<i64>,

    /// The amount of cash requested by the cardholder.
    pub cashback_amount: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionNetworkData {
    /// A code created by Stripe which is shared with the merchant to validate the authorization.
    ///
    /// This field will be populated if the authorization message was approved.
    /// The code typically starts with the letter "S", followed by a six-digit number.
    /// For example, "S498162".
    /// Please note that the code is not guaranteed to be unique across authorizations.
    pub authorization_code: Option<String>,

    /// The date the transaction was processed by the card network.
    ///
    /// This can be different from the date the seller recorded the transaction depending on when the acquirer submits the transaction to the network.
    pub processing_date: Option<String>,

    /// Unique identifier for the authorization assigned by the card network used to match subsequent messages, disputes, and transactions.
    pub transaction_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<IssuingTransactionFlightData>,

    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<IssuingTransactionFuelData>,

    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<IssuingTransactionLodgingData>,

    /// The line items in the purchase.
    pub receipt: Option<Vec<IssuingTransactionReceiptData>>,

    /// A merchant-specific order number.
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionFlightData {
    /// The time that the flight departed.
    pub departure_at: Option<i64>,

    /// The name of the passenger.
    pub passenger_name: Option<String>,

    /// Whether the ticket is refundable.
    pub refundable: Option<bool>,

    /// The legs of the trip.
    pub segments: Option<Vec<IssuingTransactionFlightDataLeg>>,

    /// The travel agency that issued the ticket.
    pub travel_agency: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionFlightDataLeg {
    /// The three-letter IATA airport code of the flight's destination.
    pub arrival_airport_code: Option<String>,

    /// The airline carrier code.
    pub carrier: Option<String>,

    /// The three-letter IATA airport code that the flight departed from.
    pub departure_airport_code: Option<String>,

    /// The flight number.
    pub flight_number: Option<String>,

    /// The flight's service class.
    pub service_class: Option<String>,

    /// Whether a stopover is allowed on this flight.
    pub stopover_allowed: Option<bool>,
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
    pub volume_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    pub check_in_at: Option<i64>,

    /// The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionReceiptData {
    /// The description of the item.
    ///
    /// The maximum length of this field is 26 characters.
    pub description: Option<String>,

    /// The quantity of the item.
    pub quantity: Option<f64>,

    /// The total for this line item in cents.
    pub total: Option<i64>,

    /// The unit cost of the item in cents.
    pub unit_cost: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingTransactionTreasury {
    /// The Treasury [ReceivedCredit](https://stripe.com/docs/api/treasury/received_credits) representing this Issuing transaction if it is a refund.
    pub received_credit: Option<String>,

    /// The Treasury [ReceivedDebit](https://stripe.com/docs/api/treasury/received_debits) representing this Issuing transaction if it is a capture.
    pub received_debit: Option<String>,
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
