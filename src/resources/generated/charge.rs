// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{ChargeId, CustomerId, PaymentIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Account, Address, Application, ApplicationFee, BalanceTransaction, BillingDetails,
    ChargeSourceParams, Currency, Customer, Invoice, Mandate, Order, PaymentIntent, PaymentMethod,
    PaymentMethodDetailsCardInstallmentsPlan, PaymentMethodDetailsCardPresent, Refund, Review,
    Shipping, ThreeDSecureDetails, Transfer,
};

/// The resource representing a Stripe "Charge".
///
/// For more details see <https://stripe.com/docs/api/charges/object>
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

    /// Amount in %s captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,

    /// Amount in %s refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<Expandable<Application>>>,

    /// The application fee (if any) for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<Box<Expandable<ApplicationFee>>>,

    /// The amount of the application fee (if any) requested for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<Box<i64>>,

    /// ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Box<Expandable<BalanceTransaction>>>,

    pub billing_details: BillingDetails,

    /// The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements.
    ///
    /// Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_statement_descriptor: Option<Box<String>>,

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
    pub customer: Option<Box<Expandable<Customer>>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    /// Whether the charge has been disputed.
    pub disputed: bool,

    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<Box<String>>,

    /// Message to user further explaining reason for charge failure if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<Box<String>>,

    /// Information on fraud assessments for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<Box<FraudDetails>>,

    /// ID of the invoice this charge is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Box<Expandable<Invoice>>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The account (if any) the charge was made on behalf of without triggering an automatic transfer.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<Expandable<Account>>>,

    /// ID of the order this charge is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Box<Expandable<Order>>>,

    /// Details about whether the payment was accepted, and why.
    ///
    /// See [understanding declines](https://stripe.com/docs/declines) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Box<ChargeOutcome>>,

    /// `true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,

    /// ID of the PaymentIntent associated with this charge, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Box<Expandable<PaymentIntent>>>,

    /// ID of the payment method used in this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Box<String>>,

    /// Details about the payment method at the time of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<Box<PaymentMethodDetails>>,

    /// This is the email address that the receipt for this charge was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<Box<String>>,

    /// This is the transaction number that appears on email receipts sent for this charge.
    ///
    /// This attribute will be `null` until a receipt has been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<Box<String>>,

    /// This is the URL to view the receipt for this charge.
    ///
    /// The receipt is kept up-to-date to the latest state of the charge, including any refunds.
    /// If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<Box<String>>,

    /// Whether the charge has been fully refunded.
    ///
    /// If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the charge.
    pub refunds: List<Refund>,

    /// ID of the review associated with this charge if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<Box<Expandable<Review>>>,

    /// Shipping information for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Box<Shipping>>,

    /// The transfer ID which created this charge.
    ///
    /// Only present if the charge came from another Stripe account.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transfer: Option<Box<Expandable<Transfer>>>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<Box<String>>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<Box<String>>,

    /// The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: ChargeStatus,

    /// ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<Box<Expandable<Transfer>>>,

    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<TransferData>>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<Box<String>>,
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
    pub stripe_report: Option<Box<String>>,

    /// Assessments reported by you.
    ///
    /// If set, possible values of are `safe` and `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_report: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChargeOutcome {
    /// Possible values are `approved_by_network`, `declined_by_network`, `not_sent_to_network`, and `reversed_after_approval`.
    ///
    /// The value `reversed_after_approval` indicates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments) after bank authorization, and may temporarily appear as "pending" on a cardholder's statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_status: Option<Box<String>>,

    /// An enumerated value providing a more detailed explanation of the outcome's `type`.
    ///
    /// Charges blocked by Radar's default block rule have the value `highest_risk_level`.
    /// Charges placed in review by Radar's default review rule have the value `elevated_risk_level`.
    /// Charges authorized, blocked, or placed in review by custom rules have the value `rule`.
    /// See [understanding declines](https://stripe.com/docs/declines) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Box<String>>,

    /// Stripe Radar's evaluation of the riskiness of the payment.
    ///
    /// Possible values for evaluated payments are `normal`, `elevated`, `highest`.
    /// For non-card payments, and card-based payments predating the public assignment of risk levels, this field will have the value `not_assessed`.
    /// In the event of an error in the evaluation, this field will have the value `unknown`.
    /// This field is only available with Radar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<Box<String>>,

    /// Stripe Radar's evaluation of the riskiness of the payment.
    ///
    /// Possible values for evaluated payments are between 0 and 100.
    /// For non-card payments, card-based payments predating the public assignment of risk scores, or in the event of an error during evaluation, this field will not be present.
    /// This field is only available with Radar for Fraud Teams.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<Box<i64>>,

    /// The ID of the Radar rule that matched the payment, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Box<Expandable<Rule>>>,

    /// A human-readable description of the outcome type and reason, designed for you (the recipient of the payment), not your customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_message: Option<Box<String>>,

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
    pub amount: Option<Box<i64>>,

    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: Expandable<Account>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<Box<PaymentMethodDetailsAchCreditTransfer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<Box<PaymentMethodDetailsAchDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<PaymentMethodDetailsAcssDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<Box<PaymentMethodDetailsAfterpayClearpay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<Box<PaymentFlowsPrivatePaymentMethodsAlipayDetails>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<Box<PaymentMethodDetailsAuBecsDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<Box<PaymentMethodDetailsBacsDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<Box<PaymentMethodDetailsBancontact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Box<PaymentMethodDetailsBoleto>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<PaymentMethodDetailsCard>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<Box<PaymentMethodDetailsCardPresent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<Box<PaymentMethodDetailsEps>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<Box<PaymentMethodDetailsFpx>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<Box<PaymentMethodDetailsGiropay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<Box<PaymentMethodDetailsGrabpay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<Box<PaymentMethodDetailsIdeal>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<Box<PaymentMethodDetailsInteracPresent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<Box<PaymentMethodDetailsKlarna>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<Box<PaymentMethodDetailsMultibanco>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<Box<PaymentMethodDetailsOxxo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<Box<PaymentMethodDetailsP24>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<Box<PaymentMethodDetailsSepaDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<Box<PaymentMethodDetailsSofort>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_account: Option<Box<PaymentMethodDetailsStripeAccount>>,

    /// The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<Box<PaymentMethodDetailsWechat>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<Box<PaymentMethodDetailsWechatPay>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    /// Uniquely identifies this particular Alipay account.
    ///
    /// You can use this attribute to check whether two Alipay accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<Box<String>>,

    /// Uniquely identifies this particular Alipay account.
    ///
    /// You can use this attribute to check whether two Alipay accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Transaction ID of this particular Alipay transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAchCreditTransfer {
    /// Account number to transfer funds to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<Box<String>>,

    /// Name of the bank associated with the routing number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<String>>,

    /// Routing transit number for the bank account to transfer funds to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<Box<String>>,

    /// SWIFT code of the bank associated with the routing number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAchDebit {
    /// Type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<Box<PaymentMethodDetailsAchDebitAccountHolderType>>,

    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<String>>,

    /// Two-letter ISO code representing the country the bank account is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// Routing transit number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAcssDebit {
    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<String>>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Institution number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_number: Option<Box<String>>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Box<String>>,

    /// Transit number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_number: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAfterpayClearpay {
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAuBecsDebit {
    /// Bank-State-Branch number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<Box<String>>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsBacsDebit {
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Box<String>>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsBancontact {
    /// Bank code of bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<Box<String>>,

    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<String>>,

    /// Bank Identifier Code of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Box<String>>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<Box<Expandable<PaymentMethod>>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<Box<Expandable<Mandate>>>,

    /// Last four characters of the IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<Box<String>>,

    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<Box<PaymentMethodDetailsBancontactPreferredLanguage>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Bancontact directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsBoleto {
    /// The tax ID of the customer (CPF for individuals consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCard {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<Box<String>>,

    /// Check results by Card networks on Card address and CVC at time of payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Box<PaymentMethodDetailsCardChecks>>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<Box<String>>,

    /// Installment details for this payment (Mexico only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<Box<PaymentMethodDetailsCardInstallments>>,

    /// The last four digits of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Box<String>>,

    /// Populated if this transaction used 3D Secure authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<Box<ThreeDSecureDetails>>,

    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Box<PaymentMethodDetailsCardWallet>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardChecks {
    /// If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<Box<String>>,

    /// If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check: Option<Box<String>>,

    /// If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardInstallments {
    /// Installment plan selected for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<PaymentMethodDetailsCardInstallmentsPlan>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<Box<PaymentMethodDetailsCardWalletAmexExpressCheckout>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<Box<PaymentMethodDetailsCardWalletApplePay>>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<Box<PaymentMethodDetailsCardWalletGooglePay>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<Box<PaymentMethodDetailsCardWalletMasterpass>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<Box<PaymentMethodDetailsCardWalletSamsungPay>>,

    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodDetailsCardWalletType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<Box<PaymentMethodDetailsCardWalletVisaCheckout>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletAmexExpressCheckout {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletApplePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletGooglePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletMasterpass {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Box<Address>>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Box<Address>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletSamsungPay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletVisaCheckout {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Box<Address>>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<String>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<String>>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Box<Address>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsEps {
    /// The customer's bank.
    ///
    /// Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<PaymentMethodDetailsEpsBank>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by EPS directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. EPS rarely provides this information so the attribute is usually empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsFpx {
    /// The customer's bank.
    ///
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, or `pb_enterprise`.
    pub bank: PaymentMethodDetailsFpxBank,

    /// Unique transaction id generated by FPX for every request from the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsGiropay {
    /// Bank code of bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<Box<String>>,

    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<String>>,

    /// Bank Identifier Code of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Box<String>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Giropay directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. Giropay rarely provides this information so the attribute is usually empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsGrabpay {
    /// Unique transaction id generated by GrabPay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsIdeal {
    /// The customer's bank.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<PaymentMethodDetailsIdealBank>>,

    /// The Bank Identifier Code of the customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Box<PaymentMethodDetailsIdealBic>>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<Box<Expandable<PaymentMethod>>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<Box<Expandable<Mandate>>>,

    /// Last four characters of the IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<Box<String>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by iDEAL directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsInteracPresent {
    /// Card brand.
    ///
    /// Can be `interac`, `mastercard` or `visa`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<Box<String>>,

    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    ///
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<Box<String>>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Authorization response cryptogram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_data: Option<Box<String>>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *Starting May 1, 2021, card fingerprint in India for Connect will change to allow two fingerprints for the same card --- one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<Box<String>>,

    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<Box<String>>,

    /// The last four digits of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Box<String>>,

    /// EMV tag 5F2D.
    ///
    /// Preferred languages specified by the integrated circuit chip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Box<Vec<String>>>,

    /// How card details were read in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_method: Option<Box<PaymentMethodDetailsInteracPresentReadMethod>>,

    /// A collection of fields required to be displayed on receipts.
    ///
    /// Only required for EMV transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Box<PaymentMethodDetailsInteracPresentReceipt>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsInteracPresentReceipt {
    /// The type of account being debited or credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<Box<PaymentMethodDetailsInteracPresentReceiptAccountType>>,

    /// EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_cryptogram: Option<Box<String>>,

    /// Mnenomic of the Application Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_preferred_name: Option<Box<String>>,

    /// Identifier for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<Box<String>>,

    /// EMV tag 8A.
    ///
    /// A code returned by the card issuer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_response_code: Option<Box<String>>,

    /// How the cardholder verified ownership of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_verification_method: Option<Box<String>>,

    /// EMV tag 84.
    ///
    /// Similar to the application identifier stored on the integrated circuit chip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_file_name: Option<Box<String>>,

    /// The outcome of a series of EMV functions performed by the card reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_verification_results: Option<Box<String>>,

    /// An indication of various EMV functions performed during the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status_information: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsKlarna {
    /// The Klarna payment method used for this transaction.
    /// Can be one of `pay_later`, `pay_now`, `pay_with_financing`, or `pay_in_installments`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_category: Option<Box<String>>,

    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    /// Can be one of `de-AT`, `en-AT`, `nl-BE`, `fr-BE`, `en-BE`, `de-DE`, `en-DE`, `da-DK`, `en-DK`, `es-ES`, `en-ES`, `fi-FI`, `sv-FI`, `en-FI`, `en-GB`, `en-IE`, `it-IT`, `en-IT`, `nl-NL`, `en-NL`, `nb-NO`, `en-NO`, `sv-SE`, `en-SE`, `en-US`, `es-US`, `fr-FR`, or `en-FR`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsMultibanco {
    /// Entity number associated with this Multibanco payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Box<String>>,

    /// Reference number associated with this Multibanco payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsOxxo {
    /// OXXO reference number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsP24 {
    /// The customer's bank.
    ///
    /// Can be one of `ing`, `citi_handlowy`, `tmobile_usbugi_bankowe`, `plus_bank`, `etransfer_pocztowy24`, `banki_spbdzielcze`, `bank_nowy_bfg_sa`, `getin_bank`, `blik`, `noble_pay`, `ideabank`, `envelobank`, `santander_przelew24`, `nest_przelew`, `mbank_mtransfer`, `inteligo`, `pbac_z_ipko`, `bnp_paribas`, `credit_agricole`, `toyota_bank`, `bank_pekao_sa`, `volkswagen_bank`, `bank_millennium`, `alior_bank`, or `boz`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<Box<PaymentMethodDetailsP24Bank>>,

    /// Unique reference for this Przelewy24 payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<String>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Przelewy24 directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. Przelewy24 rarely provides this information so the attribute is usually empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsSepaDebit {
    /// Bank code of bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<Box<String>>,

    /// Branch code of bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<Box<String>>,

    /// Two-letter ISO code representing the country the bank account is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Last four characters of the IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<Box<String>>,

    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsSofort {
    /// Bank code of bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<Box<String>>,

    /// Name of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<String>>,

    /// Bank Identifier Code of the bank associated with the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Box<String>>,

    /// Two-letter ISO code representing the country the bank account is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Box<String>>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<Box<Expandable<PaymentMethod>>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<Box<Expandable<Mandate>>>,

    /// Last four characters of the IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<Box<String>>,

    /// Preferred language of the SOFORT authorization page that the customer is redirected to.
    /// Can be one of `de`, `en`, `es`, `fr`, `it`, `nl`, or `pl`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<Box<PaymentMethodDetailsSofortPreferredLanguage>>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by SOFORT directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Box<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsStripeAccount {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsWechat {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodDetailsWechatPay {
    /// Uniquely identifies this particular WeChat Pay account.
    ///
    /// You can use this attribute to check whether two WeChat accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Box<String>>,

    /// Transaction ID of this particular WeChat Pay transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<Box<String>>,
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
    /// Uncaptured charges expire after a set number of days (7 by default).
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

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    pub transfer_data: Option<Box<TransferDataParams>>,

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
    pub fraud_details: Option<Box<FraudDetailsParams>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    pub user_report: FraudDetailsParamsUserReport,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferDataParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    pub destination: String,
}

/// An enum representing the possible values of an `Charge`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChargeStatus {
    Failed,
    Pending,
    Succeeded,
}

impl ChargeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ChargeStatus::Failed => "failed",
            ChargeStatus::Pending => "pending",
            ChargeStatus::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for ChargeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ChargeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `FraudDetailsParams`'s `user_report` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FraudDetailsParamsUserReport {
    Fraudulent,
    Safe,
}

impl FraudDetailsParamsUserReport {
    pub fn as_str(self) -> &'static str {
        match self {
            FraudDetailsParamsUserReport::Fraudulent => "fraudulent",
            FraudDetailsParamsUserReport::Safe => "safe",
        }
    }
}

impl AsRef<str> for FraudDetailsParamsUserReport {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FraudDetailsParamsUserReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsAchDebit`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsAchDebitAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodDetailsAchDebitAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsAchDebitAccountHolderType::Company => "company",
            PaymentMethodDetailsAchDebitAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsAchDebitAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsAchDebitAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl PaymentMethodDetailsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsBancontactPreferredLanguage::De => "de",
            PaymentMethodDetailsBancontactPreferredLanguage::En => "en",
            PaymentMethodDetailsBancontactPreferredLanguage::Fr => "fr",
            PaymentMethodDetailsBancontactPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardWallet`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

impl PaymentMethodDetailsCardWalletType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardWalletType::AmexExpressCheckout => "amex_express_checkout",
            PaymentMethodDetailsCardWalletType::ApplePay => "apple_pay",
            PaymentMethodDetailsCardWalletType::GooglePay => "google_pay",
            PaymentMethodDetailsCardWalletType::Masterpass => "masterpass",
            PaymentMethodDetailsCardWalletType::SamsungPay => "samsung_pay",
            PaymentMethodDetailsCardWalletType::VisaCheckout => "visa_checkout",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardWalletType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsEps`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl PaymentMethodDetailsEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsEpsBank::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            PaymentMethodDetailsEpsBank::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            PaymentMethodDetailsEpsBank::BankAustria => "bank_austria",
            PaymentMethodDetailsEpsBank::BankhausCarlSpangler => "bankhaus_carl_spangler",
            PaymentMethodDetailsEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            PaymentMethodDetailsEpsBank::BawagPskAg => "bawag_psk_ag",
            PaymentMethodDetailsEpsBank::BksBankAg => "bks_bank_ag",
            PaymentMethodDetailsEpsBank::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            PaymentMethodDetailsEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            PaymentMethodDetailsEpsBank::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            PaymentMethodDetailsEpsBank::Dolomitenbank => "dolomitenbank",
            PaymentMethodDetailsEpsBank::EasybankAg => "easybank_ag",
            PaymentMethodDetailsEpsBank::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            PaymentMethodDetailsEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            PaymentMethodDetailsEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            PaymentMethodDetailsEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            PaymentMethodDetailsEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            PaymentMethodDetailsEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            PaymentMethodDetailsEpsBank::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            PaymentMethodDetailsEpsBank::MarchfelderBank => "marchfelder_bank",
            PaymentMethodDetailsEpsBank::OberbankAg => "oberbank_ag",
            PaymentMethodDetailsEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
            PaymentMethodDetailsEpsBank::SchoellerbankAg => "schoellerbank_ag",
            PaymentMethodDetailsEpsBank::SpardaBankWien => "sparda_bank_wien",
            PaymentMethodDetailsEpsBank::VolksbankGruppe => "volksbank_gruppe",
            PaymentMethodDetailsEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            PaymentMethodDetailsEpsBank::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsFpx`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl PaymentMethodDetailsFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsFpxBank::AffinBank => "affin_bank",
            PaymentMethodDetailsFpxBank::Agrobank => "agrobank",
            PaymentMethodDetailsFpxBank::AllianceBank => "alliance_bank",
            PaymentMethodDetailsFpxBank::Ambank => "ambank",
            PaymentMethodDetailsFpxBank::BankIslam => "bank_islam",
            PaymentMethodDetailsFpxBank::BankMuamalat => "bank_muamalat",
            PaymentMethodDetailsFpxBank::BankRakyat => "bank_rakyat",
            PaymentMethodDetailsFpxBank::Bsn => "bsn",
            PaymentMethodDetailsFpxBank::Cimb => "cimb",
            PaymentMethodDetailsFpxBank::DeutscheBank => "deutsche_bank",
            PaymentMethodDetailsFpxBank::HongLeongBank => "hong_leong_bank",
            PaymentMethodDetailsFpxBank::Hsbc => "hsbc",
            PaymentMethodDetailsFpxBank::Kfh => "kfh",
            PaymentMethodDetailsFpxBank::Maybank2e => "maybank2e",
            PaymentMethodDetailsFpxBank::Maybank2u => "maybank2u",
            PaymentMethodDetailsFpxBank::Ocbc => "ocbc",
            PaymentMethodDetailsFpxBank::PbEnterprise => "pb_enterprise",
            PaymentMethodDetailsFpxBank::PublicBank => "public_bank",
            PaymentMethodDetailsFpxBank::Rhb => "rhb",
            PaymentMethodDetailsFpxBank::StandardChartered => "standard_chartered",
            PaymentMethodDetailsFpxBank::Uob => "uob",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsIdeal`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl PaymentMethodDetailsIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsIdealBank::AbnAmro => "abn_amro",
            PaymentMethodDetailsIdealBank::AsnBank => "asn_bank",
            PaymentMethodDetailsIdealBank::Bunq => "bunq",
            PaymentMethodDetailsIdealBank::Handelsbanken => "handelsbanken",
            PaymentMethodDetailsIdealBank::Ing => "ing",
            PaymentMethodDetailsIdealBank::Knab => "knab",
            PaymentMethodDetailsIdealBank::Moneyou => "moneyou",
            PaymentMethodDetailsIdealBank::Rabobank => "rabobank",
            PaymentMethodDetailsIdealBank::Regiobank => "regiobank",
            PaymentMethodDetailsIdealBank::Revolut => "revolut",
            PaymentMethodDetailsIdealBank::SnsBank => "sns_bank",
            PaymentMethodDetailsIdealBank::TriodosBank => "triodos_bank",
            PaymentMethodDetailsIdealBank::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsIdeal`'s `bic` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsIdealBic {
    #[serde(rename = "ABNANL2A")]
    Abnanl2a,
    #[serde(rename = "ASNBNL21")]
    Asnbnl21,
    #[serde(rename = "BUNQNL2A")]
    Bunqnl2a,
    #[serde(rename = "FVLBNL22")]
    Fvlbnl22,
    #[serde(rename = "HANDNL2A")]
    Handnl2a,
    #[serde(rename = "INGBNL2A")]
    Ingbnl2a,
    #[serde(rename = "KNABNL2H")]
    Knabnl2h,
    #[serde(rename = "MOYONL21")]
    Moyonl21,
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
    #[serde(rename = "REVOLT21")]
    Revolt21,
    #[serde(rename = "SNSBNL2A")]
    Snsbnl2a,
    #[serde(rename = "TRIONL2U")]
    Trionl2u,
}

impl PaymentMethodDetailsIdealBic {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsIdealBic::Abnanl2a => "ABNANL2A",
            PaymentMethodDetailsIdealBic::Asnbnl21 => "ASNBNL21",
            PaymentMethodDetailsIdealBic::Bunqnl2a => "BUNQNL2A",
            PaymentMethodDetailsIdealBic::Fvlbnl22 => "FVLBNL22",
            PaymentMethodDetailsIdealBic::Handnl2a => "HANDNL2A",
            PaymentMethodDetailsIdealBic::Ingbnl2a => "INGBNL2A",
            PaymentMethodDetailsIdealBic::Knabnl2h => "KNABNL2H",
            PaymentMethodDetailsIdealBic::Moyonl21 => "MOYONL21",
            PaymentMethodDetailsIdealBic::Rabonl2u => "RABONL2U",
            PaymentMethodDetailsIdealBic::Rbrbnl21 => "RBRBNL21",
            PaymentMethodDetailsIdealBic::Revolt21 => "REVOLT21",
            PaymentMethodDetailsIdealBic::Snsbnl2a => "SNSBNL2A",
            PaymentMethodDetailsIdealBic::Trionl2u => "TRIONL2U",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsInteracPresent`'s `read_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsInteracPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

impl PaymentMethodDetailsInteracPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsInteracPresentReadMethod::ContactEmv => "contact_emv",
            PaymentMethodDetailsInteracPresentReadMethod::ContactlessEmv => "contactless_emv",
            PaymentMethodDetailsInteracPresentReadMethod::ContactlessMagstripeMode => {
                "contactless_magstripe_mode"
            }
            PaymentMethodDetailsInteracPresentReadMethod::MagneticStripeFallback => {
                "magnetic_stripe_fallback"
            }
            PaymentMethodDetailsInteracPresentReadMethod::MagneticStripeTrack2 => {
                "magnetic_stripe_track2"
            }
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsInteracPresentReadMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsInteracPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsInteracPresentReceipt`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsInteracPresentReceiptAccountType {
    Checking,
    Savings,
    Unknown,
}

impl PaymentMethodDetailsInteracPresentReceiptAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsInteracPresentReceiptAccountType::Checking => "checking",
            PaymentMethodDetailsInteracPresentReceiptAccountType::Savings => "savings",
            PaymentMethodDetailsInteracPresentReceiptAccountType::Unknown => "unknown",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsInteracPresentReceiptAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsInteracPresentReceiptAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsP24`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl PaymentMethodDetailsP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsP24Bank::AliorBank => "alior_bank",
            PaymentMethodDetailsP24Bank::BankMillennium => "bank_millennium",
            PaymentMethodDetailsP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            PaymentMethodDetailsP24Bank::BankPekaoSa => "bank_pekao_sa",
            PaymentMethodDetailsP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            PaymentMethodDetailsP24Bank::Blik => "blik",
            PaymentMethodDetailsP24Bank::BnpParibas => "bnp_paribas",
            PaymentMethodDetailsP24Bank::Boz => "boz",
            PaymentMethodDetailsP24Bank::CitiHandlowy => "citi_handlowy",
            PaymentMethodDetailsP24Bank::CreditAgricole => "credit_agricole",
            PaymentMethodDetailsP24Bank::Envelobank => "envelobank",
            PaymentMethodDetailsP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            PaymentMethodDetailsP24Bank::GetinBank => "getin_bank",
            PaymentMethodDetailsP24Bank::Ideabank => "ideabank",
            PaymentMethodDetailsP24Bank::Ing => "ing",
            PaymentMethodDetailsP24Bank::Inteligo => "inteligo",
            PaymentMethodDetailsP24Bank::MbankMtransfer => "mbank_mtransfer",
            PaymentMethodDetailsP24Bank::NestPrzelew => "nest_przelew",
            PaymentMethodDetailsP24Bank::NoblePay => "noble_pay",
            PaymentMethodDetailsP24Bank::PbacZIpko => "pbac_z_ipko",
            PaymentMethodDetailsP24Bank::PlusBank => "plus_bank",
            PaymentMethodDetailsP24Bank::SantanderPrzelew24 => "santander_przelew24",
            PaymentMethodDetailsP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            PaymentMethodDetailsP24Bank::ToyotaBank => "toyota_bank",
            PaymentMethodDetailsP24Bank::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsSofort`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl PaymentMethodDetailsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsSofortPreferredLanguage::De => "de",
            PaymentMethodDetailsSofortPreferredLanguage::En => "en",
            PaymentMethodDetailsSofortPreferredLanguage::Es => "es",
            PaymentMethodDetailsSofortPreferredLanguage::Fr => "fr",
            PaymentMethodDetailsSofortPreferredLanguage::It => "it",
            PaymentMethodDetailsSofortPreferredLanguage::Nl => "nl",
            PaymentMethodDetailsSofortPreferredLanguage::Pl => "pl",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
