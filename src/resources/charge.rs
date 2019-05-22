use crate::config::{Client, Response};
use crate::error::ErrorCode;
use crate::ids::{ChargeId, CustomerId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Account, Application, ApplicationFee, BalanceTransaction, BillingDetails, Currency, Customer,
    Dispute, Invoice, Order, PaymentMethodDetails, PaymentSource, PaymentSourceParams, Refund,
    Review, Shipping, Transfer,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Charge".
///
/// For more details see [https://stripe.com/docs/api/charges/object](https://stripe.com/docs/api/charges/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Charge {
    /// Unique identifier for the object.
    pub id: ChargeId,

    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    ///
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://support.stripe.com/questions/what-is-the-minimum-amount-i-can-charge-with-stripe).
    pub amount: i64,

    /// Amount in %s refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    /// The application fee (if any) for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<Expandable<ApplicationFee>>,

    /// The amount of the application fee (if any) for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    pub billing_details: BillingDetails,

    /// If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been captured.
    pub captured: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// ID of the customer this charge is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Details about the dispute if the charge has been disputed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<Expandable<Dispute>>,

    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<ErrorCode>,

    /// Message to user further explaining reason for charge failure if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,

    /// Information on fraud assessments for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetails>,

    /// ID of the invoice this charge is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The account (if any) the charge was made on behalf of without triggering an automatic transfer.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Expandable<Account>>,

    /// ID of the order this charge is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Expandable<Order>>,

    /// Details about whether the payment was accepted, and why.
    ///
    /// See [understanding declines](https://stripe.com/docs/declines) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<ChargeOutcome>,

    /// `true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,

    /// ID of the PaymentIntent associated with this charge, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,

    /// ID of the payment method used in this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    /// Details about the payment method at the time of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<PaymentMethodDetails>,

    /// This is the email address that the receipt for this charge was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    /// This is the transaction number that appears on email receipts sent for this charge.
    ///
    /// This attribute will be `null` until a receipt has been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,

    /// This is the URL to view the receipt for this charge.
    ///
    /// The receipt is kept up-to-date to the latest state of the charge, including any refunds.
    /// If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    pub receipt_url: String,

    /// Whether the charge has been fully refunded.
    ///
    /// If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the charge.
    pub refunds: List<Refund>,

    /// ID of the review associated with this charge if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<Expandable<Review>>,

    /// Shipping information for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// For most Stripe users, the source of every charge is a credit or debit card.
    ///
    /// This hash is then the [card object](#card_object) describing that card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSource>,

    /// The transfer ID which created this charge.
    ///
    /// Only present if the charge came from another Stripe account.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transfer: Option<Expandable<Transfer>>,

    /// Extra information about a charge.
    ///
    /// This will appear on your customer's credit card statement.
    /// It must contain at least one letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: ChargeStatus,

    /// ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<Expandable<Transfer>>,

    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#grouping-transactions) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

impl Charge {
    /// Creates a new charge.
    ///
    /// For more details see [https://stripe.com/docs/api#create_charge](https://stripe.com/docs/api#create_charge).
    pub fn create(client: &Client, params: ChargeParams<'_>) -> Response<Charge> {
        client.post_form("/charges", params)
    }

    /// Retrieves the details of a charge.
    ///
    /// For more details see [https://stripe.com/docs/api#retrieve_charge](https://stripe.com/docs/api#retrieve_charge).
    pub fn retrieve(client: &Client, charge_id: &ChargeId, expand: &[&str]) -> Response<Charge> {
        client.get_query(&format!("/charges/{}", charge_id), &Expand { expand })
    }

    /// Updates a charge's properties.
    ///
    /// For more details see [https://stripe.com/docs/api#update_charge](https://stripe.com/docs/api#update_charge).
    pub fn update(
        client: &Client,
        charge_id: &ChargeId,
        params: ChargeParams<'_>,
    ) -> Response<Charge> {
        client.post_form(&format!("/charges/{}", charge_id), params)
    }

    /// Capture captures a previously created charge with capture set to false.
    ///
    /// For more details see [https://stripe.com/docs/api#charge_capture](https://stripe.com/docs/api#charge_capture).
    pub fn capture(
        client: &Client,
        charge_id: &ChargeId,
        params: CaptureParams<'_>,
    ) -> Response<Charge> {
        client.post_form(&format!("/charges/{}/capture", charge_id), params)
    }

    /// List all charges.
    ///
    /// For more details see [https://stripe.com/docs/api#list_charges](https://stripe.com/docs/api#list_charges).
    pub fn list(client: &Client, params: ChargeListParams<'_>) -> Response<List<Charge>> {
        client.get_query("/charges", &params)
    }
}

impl Object for Charge {
    type Id = ChargeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "charge"
    }
}

/// The resource representing a Stripe charge object outcome.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome](https://stripe.com/docs/api#charge_object-outcome)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChargeOutcome {
    #[serde(rename = "type")]
    pub outcome_type: OutcomeType,
    pub network_status: NetworkStatus,
    #[serde(default)]
    pub reason: Option<OutcomeReason>,
    #[serde(default)]
    pub risk_level: Option<RiskLevel>,
    #[serde(default)]
    pub seller_message: Option<String>,
    #[serde(default)]
    pub rule: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FraudDetails {
    pub user_report: Option<String>,
    #[serde(skip_serializing)]
    pub stripe_report: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferData {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: Expandable<Account>,
}

/// The set of parameters that can be used when capturing a charge object.
///
/// For more details see (https://stripe.com/docs/api#charge_capture](https://stripe.com/docs/api#charge_capture).
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CaptureParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

/// The set of parameters that can be used when creating or updating a charge object.
///
/// For more details see [https://stripe.com/docs/api#create_charge](https://stripe.com/docs/api#create_charge)
/// and [https://stripe.com/docs/api#update_charge](https://stripe.com/docs/api#update_charge).
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChargeParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>, // NOTE: if None, Stripe assumes true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
}

/// The parameters for `Charge::list`.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChargeListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// Only return charges for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a ChargeId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a ChargeId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DestinationParams<'a> {
    pub account: &'a str,
    pub amount: u64,
}

/// The resource representing a Stripe charge object status.
///
/// For more details see [https://stripe.com/docs/api#charge_object-status](https://stripe.com/docs/api#charge_object-status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum ChargeStatus {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `network_status` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-network_status](https://stripe.com/docs/api#charge_object-outcome-network_status)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkStatus {
    ApprovedByNetwork,
    DeclinedByNetwork,
    NotSentToNetwork,

    /// This value indiciates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments)
    /// after bank authorization, and may temporarily appear as “pending” on a cardholder’s statement.
    ReversedAfterApproval,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `reason` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-reason](https://stripe.com/docs/api#charge_object-outcome-reason)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeReason {
    ApprovedWithId,
    CallIssuer,
    CardNotSupported,
    CardVelocityExceeded,
    CurrencyNotSupported,
    DoNotHonor,
    DoNotTryAgain,
    DuplicateTransaction,
    ExpiredCard,
    Fraudulent,
    GenericDecline,
    IncorrectNumber,
    IncorrectCvc,
    IncorrectPin,
    IncorrectZip,
    InsufficientFunds,
    InvalidAccount,
    InvalidAmount,
    InvalidCvc,
    InvalidExpiryYear,
    InvalidNumber,
    InvalidPin,
    IssuerNotAvailable,
    LostCard,
    MerchantBlacklist,
    NewAccountInformationAvailable,
    NoActionTaken,
    NotPermitted,
    PickupCard,
    PinTryExceeded,
    ProcessingError,
    ReenterTransaction,
    RestrictedCard,
    RevocationOfAllAuthorizations,
    RevocationOfAuthorization,
    SecurityViolation,
    ServiceNotAllowed,
    StolenCard,
    StopPaymentOrder,
    TestmodeDecline,
    TransactionNotAllowed,
    TryAgainLater,
    WithdrawalCountLimitExceeded,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `type` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-type](https://stripe.com/docs/api#charge_object-outcome-type)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Copy, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeType {
    Authorized,
    ManualReview,
    IssuerDeclined,
    Blocked,
    Invalid,

    /// A variant not yet supported by the library.
    /// It is an error to send `Other` as part of a request.
    #[serde(other, skip_serializing)]
    Other,
}

/// An enum representing the possible values of a `ChargeOutcome`'s `risk_level` field.
///
/// For more details see [https://stripe.com/docs/api#charge_object-outcome-risk_level](https://stripe.com/docs/api#charge_object-outcome-risk_level)
#[derive(Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Normal,
    Elevated,
    Highest,
    NotAssessed,

    /// An unknown risk level.
    ///
    /// May also be a variant not yet supported by the library.
    #[serde(other)]
    #[serde(rename = "unknown")]
    Unknown,
}
