// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{ChargeId, CustomerId, PaymentIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Account, Application, ApplicationFee, BalanceTransaction, BillingDetails, ChargeSourceParams,
    Currency, Customer, FraudDetailsReport, Invoice, Order, PaymentIntent, PaymentMethodDetails,
    Refund, Review, Shipping, Transfer,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Charge".
///
/// For more details see [https://stripe.com/docs/api/charges/object](https://stripe.com/docs/api/charges/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Charge {
    /// Unique identifier for the object.
    pub id: ChargeId,

    /// Amount intended to be collected by this payment.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
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

    /// The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements.
    ///
    /// Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_statement_descriptor: Option<String>,

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

    /// Whether the charge has been disputed.
    pub disputed: bool,

    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,

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
    pub payment_intent: Option<Expandable<PaymentIntent>>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,

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

    /// The transfer ID which created this charge.
    ///
    /// Only present if the charge came from another Stripe account.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transfer: Option<Expandable<Transfer>>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: String,

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
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

impl Charge {
    /// Returns a list of charges you’ve previously created.
    ///
    /// The charges are returned in sorted order, with the most recent charges appearing first.
    pub fn list(client: &Client, params: ListCharges<'_>) -> Response<List<Charge>> {
        client.get_query("/charges", &params)
    }

    /// To charge a credit card or other payment source, you create a `Charge` object.
    ///
    /// If your API key is in test mode, the supplied payment source (e.g., card) won’t actually be charged, although everything else will occur as if in live mode.
    /// (Stripe assumes that the charge would have completed successfully).
    pub fn create(client: &Client, params: CreateCharge<'_>) -> Response<Charge> {
        client.post_form("/charges", &params)
    }

    /// Retrieves the details of a charge that has previously been created.
    ///
    /// Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information.
    /// The same information is returned when creating or refunding the charge.
    pub fn retrieve(client: &Client, id: &ChargeId, expand: &[&str]) -> Response<Charge> {
        client.get_query(&format!("/charges/{}", id), &Expand { expand })
    }

    /// Updates the specified charge by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(client: &Client, id: &ChargeId, params: UpdateCharge<'_>) -> Response<Charge> {
        client.post_form(&format!("/charges/{}", id), &params)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FraudDetails {
    /// Assessments from Stripe.
    ///
    /// If set, the value is `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_report: Option<FraudDetailsReport>,

    /// Assessments reported by you.
    ///
    /// If set, possible values of are `safe` and `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_report: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChargeOutcome {
    /// Possible values are `approved_by_network`, `declined_by_network`, `not_sent_to_network`, and `reversed_after_approval`.
    ///
    /// The value `reversed_after_approval` indicates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments) after bank authorization, and may temporarily appear as "pending" on a cardholder's statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_status: Option<String>,

    /// An enumerated value providing a more detailed explanation of the outcome's `type`.
    ///
    /// Charges blocked by Radar's default block rule have the value `highest_risk_level`.
    /// Charges placed in review by Radar's default review rule have the value `elevated_risk_level`.
    /// Charges authorized, blocked, or placed in review by custom rules have the value `rule`.
    /// See [understanding declines](https://stripe.com/docs/declines) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// Stripe's evaluation of the riskiness of the payment.
    ///
    /// Possible values for evaluated payments are `normal`, `elevated`, `highest`.
    /// For non-card payments, and card-based payments predating the public assignment of risk levels, this field will have the value `not_assessed`.
    /// In the event of an error in the evaluation, this field will have the value `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,

    /// Stripe's evaluation of the riskiness of the payment.
    ///
    /// Possible values for evaluated payments are between 0 and 100.
    /// For non-card payments, card-based payments predating the public assignment of risk scores, or in the event of an error during evaluation, this field will not be present.
    /// This field is only available with Radar for Fraud Teams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<i64>,

    /// The ID of the Radar rule that matched the payment, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Expandable<Rule>>,

    /// A human-readable description of the outcome type and reason, designed for you (the recipient of the payment), not your customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_message: Option<String>,

    /// Possible values are `authorized`, `manual_review`, `issuer_declined`, `blocked`, and `invalid`.
    ///
    /// See [understanding declines](https://stripe.com/docs/declines) and [Radar reviews](https://stripe.com/docs/radar/reviews) for details.
    #[serde(rename = "type")]
    pub type_: String,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rule {
    /// The action taken on the payment.
    pub action: String,

    /// Unique identifier for the object.
    pub id: String,

    /// The predicate to evaluate the payment against.
    pub predicate: String,
}

/// The parameters for `Charge::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateCharge<'a> {
    /// Amount intended to be collected by this payment.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<i64>,

    /// A fee in %s that will be applied to the charge and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the `Stripe-Account` header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Whether to immediately capture the charge.
    ///
    /// Defaults to `true`.
    /// When `false`, the charge issues an authorization (or pre-authorization), and will need to be [captured](https://stripe.com/docs/api#capture_charge) later.
    /// Uncaptured charges expire in _seven days_.
    /// For more information, see the [authorizing charges and settling later](https://stripe.com/docs/charges/placing-a-hold) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The ID of an existing customer that will be charged in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string which you can attach to a `Charge` object.
    ///
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The Stripe account ID for which these funds are intended.
    ///
    /// Automatically set if you use the `destination` parameter.
    /// For details, see [Creating Separate Charges and Transfers](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// The email address to which this charge's [receipt](https://stripe.com/docs/dashboard/receipts) will be sent.
    ///
    /// The receipt will not be sent until the charge is paid, and no receipts will be sent for test mode charges.
    /// If this charge is for a [Customer](https://stripe.com/docs/api/customers/object), the email address specified here will override the customer's email address.
    /// If `receipt_email` is specified for a charge in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,

    /// Shipping information for the charge.
    ///
    /// Helps prevent fraud on charges for physical goods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// A payment source to be charged.
    ///
    /// This can be the ID of a [card](https://stripe.com/docs/api#cards) (i.e., credit or debit card), a [bank account](https://stripe.com/docs/api#bank_accounts), a [source](https://stripe.com/docs/api#sources), a [token](https://stripe.com/docs/api#tokens), or a [connected account](https://stripe.com/docs/connect/account-debits#charging-a-connected-account).
    /// For certain sources---namely, [cards](https://stripe.com/docs/api#cards), [bank accounts](https://stripe.com/docs/api#bank_accounts), and attached [sources](https://stripe.com/docs/api#sources)---you must also pass the ID of the associated customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ChargeSourceParams>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,

    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferDataParams>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// For details, see [Grouping transactions](https://stripe.com/docs/connect/charges-transfers#transfer-options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

impl<'a> CreateCharge<'a> {
    pub fn new() -> Self {
        CreateCharge {
            amount: Default::default(),
            application_fee: Default::default(),
            application_fee_amount: Default::default(),
            capture: Default::default(),
            currency: Default::default(),
            customer: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            receipt_email: Default::default(),
            shipping: Default::default(),
            source: Default::default(),
            statement_descriptor: Default::default(),
            statement_descriptor_suffix: Default::default(),
            transfer_data: Default::default(),
            transfer_group: Default::default(),
        }
    }
}

/// The parameters for `Charge::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCharges<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return charges for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<ChargeId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntentId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<ChargeId>,

    /// Only return charges for this transfer group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

impl<'a> ListCharges<'a> {
    pub fn new() -> Self {
        ListCharges {
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_intent: Default::default(),
            starting_after: Default::default(),
            transfer_group: Default::default(),
        }
    }
}

/// The parameters for `Charge::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateCharge<'a> {
    /// The ID of an existing customer that will be associated with this request.
    ///
    /// This field may only be updated if there is no existing associated customer with this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string which you can attach to a charge object.
    ///
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A set of key-value pairs you can attach to a charge giving information about its riskiness.
    ///
    /// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
    /// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
    /// Stripe will use the information you send to improve our fraud detection algorithms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetailsParams>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// This is the email address that the receipt for this charge will be sent to.
    ///
    /// If this field is updated, then a new email receipt will be sent to the updated address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,

    /// Shipping information for the charge.
    ///
    /// Helps prevent fraud on charges for physical goods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}

impl<'a> UpdateCharge<'a> {
    pub fn new() -> Self {
        UpdateCharge {
            customer: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            fraud_details: Default::default(),
            metadata: Default::default(),
            receipt_email: Default::default(),
            shipping: Default::default(),
            transfer_group: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FraudDetailsParams {
    pub user_report: FraudDetailsReport,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferDataParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    pub destination: String,
}
