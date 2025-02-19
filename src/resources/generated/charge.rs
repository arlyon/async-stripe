// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ChargeId, CustomerId, PaymentIntentId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{
    Account, Address, Application, ApplicationFee, BalanceTransaction, BillingDetails,
    ChargeSourceParams, Currency, Customer, Invoice, Mandate, PaymentIntent, PaymentMethod,
    PaymentMethodDetailsCardInstallmentsPlan, PaymentMethodDetailsCardWalletApplePay,
    PaymentMethodDetailsCardWalletGooglePay, PaymentSource, RadarRadarOptions, Refund, Review,
    Shipping, Transfer,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Charge".
///
/// For more details see <https://stripe.com/docs/api/charges/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Charge {
    /// Unique identifier for the object.
    pub id: ChargeId,

    /// Amount intended to be collected by this payment.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,

    /// Amount in cents (or local equivalent) captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,

    /// Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that created the charge.
    pub application: Option<Expandable<Application>>,

    /// The application fee (if any) for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee: Option<Expandable<ApplicationFee>>,

    /// The amount of the application fee (if any) requested for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee_amount: Option<i64>,

    /// Authorization code on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,

    /// ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    pub billing_details: BillingDetails,

    /// The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements.
    ///
    /// Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
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
    pub customer: Option<Expandable<Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// Whether the charge has been disputed.
    pub disputed: bool,

    /// ID of the balance transaction that describes the reversal of the balance on your account due to payment failure.
    pub failure_balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/error-codes) for a list of codes).
    pub failure_code: Option<String>,

    /// Message to user further explaining reason for charge failure if available.
    pub failure_message: Option<String>,

    /// Information on fraud assessments for the charge.
    pub fraud_details: Option<FraudDetails>,

    /// ID of the invoice this charge is for if one exists.
    pub invoice: Option<Expandable<Invoice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level3: Option<Level3>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The account (if any) the charge was made on behalf of without triggering an automatic transfer.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub on_behalf_of: Option<Expandable<Account>>,

    /// Details about whether the payment was accepted, and why.
    ///
    /// See [understanding declines](https://stripe.com/docs/declines) for details.
    pub outcome: Option<ChargeOutcome>,

    /// `true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,

    /// ID of the PaymentIntent associated with this charge, if one exists.
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// ID of the payment method used in this charge.
    pub payment_method: Option<String>,

    /// Details about the payment method at the time of the transaction.
    pub payment_method_details: Option<PaymentMethodDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarRadarOptions>,

    /// This is the email address that the receipt for this charge was sent to.
    pub receipt_email: Option<String>,

    /// This is the transaction number that appears on email receipts sent for this charge.
    ///
    /// This attribute will be `null` until a receipt has been sent.
    pub receipt_number: Option<String>,

    /// This is the URL to view the receipt for this charge.
    ///
    /// The receipt is kept up-to-date to the latest state of the charge, including any refunds.
    /// If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    pub receipt_url: Option<String>,

    /// Whether the charge has been fully refunded.
    ///
    /// If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the charge.
    pub refunds: Option<List<Refund>>,

    /// ID of the review associated with this charge if one exists.
    pub review: Option<Expandable<Review>>,

    /// Shipping information for the charge.
    pub shipping: Option<Shipping>,

    /// This is a legacy field that will be removed in the future.
    ///
    /// It contains the Source, Card, or BankAccount object used for the charge.
    /// For details about the payment method used for this charge, refer to `payment_method` or `payment_method_details` instead.
    pub source: Option<PaymentSource>,

    /// The transfer ID which created this charge.
    ///
    /// Only present if the charge came from another Stripe account.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub source_transfer: Option<Expandable<Transfer>>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,

    /// The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: ChargeStatus,

    /// ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<Expandable<Transfer>>,

    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub transfer_data: Option<TransferData>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}

impl Charge {
    /// Returns a list of charges you’ve previously created.
    ///
    /// The charges are returned in sorted order, with the most recent charges appearing first.
    pub fn list(client: &Client, params: &ListCharges<'_>) -> Response<List<Charge>> {
        client.get_query("/charges", params)
    }

    /// This method is no longer recommended—use the [Payment Intents API](https://stripe.com/docs/api/payment_intents)
    /// to initiate a new payment instead.
    ///
    /// Confirmation of the PaymentIntent creates the `Charge` object used to request payment.
    pub fn create(client: &Client, params: CreateCharge<'_>) -> Response<Charge> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/charges", &params)
    }

    /// Retrieves the details of a charge that has previously been created.
    ///
    /// Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information.
    /// The same information is returned when creating or refunding the charge.
    pub fn retrieve(client: &Client, id: &ChargeId, expand: &[&str]) -> Response<Charge> {
        client.get_query(&format!("/charges/{}", id), Expand { expand })
    }

    /// Updates the specified charge by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(client: &Client, id: &ChargeId, params: UpdateCharge<'_>) -> Response<Charge> {
        #[allow(clippy::needless_borrows_for_generic_args)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FraudDetails {
    /// Assessments from Stripe.
    ///
    /// If set, the value is `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_report: Option<String>,

    /// Assessments reported by you.
    ///
    /// If set, possible values of are `safe` and `fraudulent`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_report: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChargeOutcome {
    /// Possible values are `approved_by_network`, `declined_by_network`, `not_sent_to_network`, and `reversed_after_approval`.
    ///
    /// The value `reversed_after_approval` indicates the payment was [blocked by Stripe](https://stripe.com/docs/declines#blocked-payments) after bank authorization, and may temporarily appear as "pending" on a cardholder's statement.
    pub network_status: Option<String>,

    /// An enumerated value providing a more detailed explanation of the outcome's `type`.
    ///
    /// Charges blocked by Radar's default block rule have the value `highest_risk_level`.
    /// Charges placed in review by Radar's default review rule have the value `elevated_risk_level`.
    /// Charges authorized, blocked, or placed in review by custom rules have the value `rule`.
    /// See [understanding declines](https://stripe.com/docs/declines) for more details.
    pub reason: Option<String>,

    /// Stripe Radar's evaluation of the riskiness of the payment.
    ///
    /// Possible values for evaluated payments are `normal`, `elevated`, `highest`.
    /// For non-card payments, and card-based payments predating the public assignment of risk levels, this field will have the value `not_assessed`.
    /// In the event of an error in the evaluation, this field will have the value `unknown`.
    /// This field is only available with Radar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,

    /// Stripe Radar's evaluation of the riskiness of the payment.
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
    pub seller_message: Option<String>,

    /// Possible values are `authorized`, `manual_review`, `issuer_declined`, `blocked`, and `invalid`.
    ///
    /// See [understanding declines](https://stripe.com/docs/declines) and [Radar reviews](https://stripe.com/docs/radar/reviews) for details.
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransferData {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: Expandable<Account>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Level3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,

    pub line_items: Vec<Level3LineItems>,

    pub merchant_reference: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_zip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_from_zip: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Level3LineItems {
    pub discount_amount: Option<i64>,

    pub product_code: String,

    pub product_description: String,

    pub quantity: Option<u64>,

    pub tax_amount: Option<i64>,

    pub unit_cost: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<PaymentMethodDetailsAchCreditTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<PaymentMethodDetailsAchDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentMethodDetailsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PaymentMethodDetailsAffirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PaymentMethodDetailsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentFlowsPrivatePaymentMethodsAlipayDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PaymentMethodDetailsAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PaymentMethodDetailsBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodDetailsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PaymentMethodDetailsBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PaymentMethodDetailsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodDetailsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<PaymentMethodDetailsCardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<PaymentMethodDetailsCashapp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodDetailsCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PaymentMethodDetailsEps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PaymentMethodDetailsFpx>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PaymentMethodDetailsGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PaymentMethodDetailsGrabpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodDetailsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PaymentMethodDetailsInteracPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodDetailsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PaymentMethodDetailsKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodDetailsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<PaymentMethodDetailsMultibanco>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodDetailsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodDetailsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PaymentMethodDetailsPaynow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodDetailsPaypal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<PaymentMethodDetailsPix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PaymentMethodDetailsPromptpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<PaymentMethodDetailsRevolutPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<PaymentMethodDetailsSepaCreditTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentMethodDetailsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodDetailsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_account: Option<PaymentMethodDetailsStripeAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<PaymentMethodDetailsSwish>,

    /// The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PaymentMethodDetailsUsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<PaymentMethodDetailsWechat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodDetailsWechatPay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<PaymentMethodDetailsZip>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsAlipayDetails {
    /// Uniquely identifies this particular Alipay account.
    ///
    /// You can use this attribute to check whether two Alipay accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,

    /// Uniquely identifies this particular Alipay account.
    ///
    /// You can use this attribute to check whether two Alipay accounts are the same.
    pub fingerprint: Option<String>,

    /// Transaction ID of this particular Alipay transaction.
    pub transaction_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAchCreditTransfer {
    /// Account number to transfer funds to.
    pub account_number: Option<String>,

    /// Name of the bank associated with the routing number.
    pub bank_name: Option<String>,

    /// Routing transit number for the bank account to transfer funds to.
    pub routing_number: Option<String>,

    /// SWIFT code of the bank associated with the routing number.
    pub swift_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAchDebit {
    /// Type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodDetailsAchDebitAccountHolderType>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Routing transit number of the bank account.
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAcssDebit {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Institution number of the bank account.
    pub institution_number: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<String>,

    /// Transit number of the bank account.
    pub transit_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAffirm {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAfterpayClearpay {
    /// The Afterpay order ID associated with this payment intent.
    pub order_id: Option<String>,

    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsAuBecsDebit {
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsBacsDebit {
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// ID of the mandate used to make this payment.
    pub mandate: Option<String>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    pub sort_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsBancontact {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<Expandable<PaymentMethod>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<Expandable<Mandate>>,

    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,

    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`.
    pub preferred_language: Option<PaymentMethodDetailsBancontactPreferredLanguage>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Bancontact directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsBlik {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsBoleto {
    /// The tax ID of the customer (CPF for individuals consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCard {

    /// The authorized amount.
    pub amount_authorized: Option<i64>,

    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,

    /// When using manual capture, a future timestamp at which the charge will be automatically refunded if uncaptured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<Timestamp>,

    /// Check results by Card networks on Card address and CVC at time of payment.
    pub checks: Option<PaymentMethodDetailsCardChecks>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_authorization: Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization>,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_authorization: Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization>,

    /// Installment details for this payment (Mexico only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    pub installments: Option<PaymentMethodDetailsCardInstallments>,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,

    /// ID of the mandate used to make this payment or created by it.
    pub mandate: Option<String>,

    /// True if this payment was marked as MOTO and out of scope for SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicapture: Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,

    /// If this card has network token credentials, this contains the details of the network token credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_token: Option<PaymentMethodDetailsCardNetworkToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overcapture: Option<PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>,

    /// Populated if this transaction used 3D Secure authentication.
    pub three_d_secure: Option<ThreeDSecureDetailsCharge>,

    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<PaymentMethodDetailsCardWallet>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization {

    /// Indicates whether or not the capture window is extended beyond the standard authorization.
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization {

    /// Indicates whether or not the incremental authorization feature is supported.
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture {

    /// The maximum amount that can be captured.
    pub maximum_amount_capturable: i64,

    /// Indicates whether or not the authorized amount can be over-captured.
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    /// Indicates whether or not multiple captures are supported.
    pub status: PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardChecks {
    /// If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,

    /// If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check: Option<String>,

    /// If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardInstallments {
    /// Installment plan selected for the payment.
    pub plan: Option<PaymentMethodDetailsCardInstallmentsPlan>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardNetworkToken {
    /// Indicates if Stripe used a network token, either user provided or Stripe managed when processing the transaction.
    pub used: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresent {
    /// The authorized amount.
    pub amount_authorized: Option<i64>,

    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,

    /// When using manual capture, a future timestamp after which the charge will be automatically refunded if uncaptured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<Timestamp>,

    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    ///
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Authorization response cryptogram.
    pub emv_auth_data: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,

    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// Whether this [PaymentIntent](https://stripe.com/docs/api/payment_intents) is eligible for incremental authorizations.
    ///
    /// Request support using [request_incremental_authorization_support](https://stripe.com/docs/api/payment_intents/create#create_payment_intent-payment_method_options-card_present-request_incremental_authorization_support).
    pub incremental_authorization_supported: bool,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,

    /// Details about payments collected offline.
    pub offline: Option<PaymentMethodDetailsCardPresentOffline>,

    /// Defines whether the authorized amount can be over-captured or not.
    pub overcapture_supported: bool,

    /// How card details were read in this transaction.
    pub read_method: Option<PaymentMethodDetailsCardPresentReadMethod>,

    /// A collection of fields required to be displayed on receipts.
    ///
    /// Only required for EMV transactions.
    pub receipt: Option<PaymentMethodDetailsCardPresentReceipt>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresentOffline {
    /// Time at which the payment was collected while offline.
    pub stored_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresentReceipt {
    /// The type of account being debited or credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PaymentMethodDetailsCardPresentReceiptAccountType>,

    /// EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    pub application_cryptogram: Option<String>,

    /// Mnenomic of the Application Identifier.
    pub application_preferred_name: Option<String>,

    /// Identifier for this transaction.
    pub authorization_code: Option<String>,

    /// EMV tag 8A.
    ///
    /// A code returned by the card issuer.
    pub authorization_response_code: Option<String>,

    /// How the cardholder verified ownership of the card.
    pub cardholder_verification_method: Option<String>,

    /// EMV tag 84.
    ///
    /// Similar to the application identifier stored on the integrated circuit chip.
    pub dedicated_file_name: Option<String>,

    /// The outcome of a series of EMV functions performed by the card reader.
    pub terminal_verification_results: Option<String>,

    /// An indication of various EMV functions performed during the transaction.
    pub transaction_status_information: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<PaymentMethodDetailsCardWalletAmexExpressCheckout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<PaymentMethodDetailsCardWalletApplePay>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<PaymentMethodDetailsCardWalletGooglePay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentMethodDetailsCardWalletLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<PaymentMethodDetailsCardWalletMasterpass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<PaymentMethodDetailsCardWalletSamsungPay>,

    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodDetailsCardWalletType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<PaymentMethodDetailsCardWalletVisaCheckout>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletAmexExpressCheckout {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletLink {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletMasterpass {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub shipping_address: Option<Address>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletSamsungPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWalletVisaCheckout {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub shipping_address: Option<Address>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCashapp {
    /// A unique and immutable identifier assigned by Cash App to every buyer.
    pub buyer_id: Option<String>,

    /// A public identifier for buyers using Cash App.
    pub cashtag: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCustomerBalance {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsEps {
    /// The customer's bank.
    ///
    /// Should be one of `arzte_und_apotheker_bank`, `austrian_anadi_bank_ag`, `bank_austria`, `bankhaus_carl_spangler`, `bankhaus_schelhammer_und_schattera_ag`, `bawag_psk_ag`, `bks_bank_ag`, `brull_kallmus_bank_ag`, `btv_vier_lander_bank`, `capital_bank_grawe_gruppe_ag`, `deutsche_bank_ag`, `dolomitenbank`, `easybank_ag`, `erste_bank_und_sparkassen`, `hypo_alpeadriabank_international_ag`, `hypo_noe_lb_fur_niederosterreich_u_wien`, `hypo_oberosterreich_salzburg_steiermark`, `hypo_tirol_bank_ag`, `hypo_vorarlberg_bank_ag`, `hypo_bank_burgenland_aktiengesellschaft`, `marchfelder_bank`, `oberbank_ag`, `raiffeisen_bankengruppe_osterreich`, `schoellerbank_ag`, `sparda_bank_wien`, `volksbank_gruppe`, `volkskreditbank_ag`, or `vr_bank_braunau`.
    pub bank: Option<PaymentMethodDetailsEpsBank>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by EPS directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. EPS rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsFpx {
    /// Account holder type, if provided.
    ///
    /// Can be one of `individual` or `company`.
    pub account_holder_type: Option<PaymentMethodDetailsFpxAccountHolderType>,

    /// The customer's bank.
    ///
    /// Can be one of `affin_bank`, `agrobank`, `alliance_bank`, `ambank`, `bank_islam`, `bank_muamalat`, `bank_rakyat`, `bsn`, `cimb`, `hong_leong_bank`, `hsbc`, `kfh`, `maybank2u`, `ocbc`, `public_bank`, `rhb`, `standard_chartered`, `uob`, `deutsche_bank`, `maybank2e`, `pb_enterprise`, or `bank_of_china`.
    pub bank: PaymentMethodDetailsFpxBank,

    /// Unique transaction id generated by FPX for every request from the merchant.
    pub transaction_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsGiropay {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Giropay directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. Giropay rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsGrabpay {
    /// Unique transaction id generated by GrabPay.
    pub transaction_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsIdeal {
    /// The customer's bank.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
    pub bank: Option<PaymentMethodDetailsIdealBank>,

    /// The Bank Identifier Code of the customer's bank.
    pub bic: Option<PaymentMethodDetailsIdealBic>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<Expandable<PaymentMethod>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<Expandable<Mandate>>,

    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by iDEAL directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsInteracPresent {
    /// Card brand.
    ///
    /// Can be `interac`, `mastercard` or `visa`.
    pub brand: Option<String>,

    /// The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format.
    ///
    /// May include alphanumeric characters, special characters and first/last name separator (`/`).
    /// In some cases, the cardholder name may not be available depending on how the issuer has configured the card.
    /// Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    pub cardholder_name: Option<String>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Authorization response cryptogram.
    pub emv_auth_data: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,

    /// ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions.
    ///
    /// Only present if it was possible to generate a card PaymentMethod.
    pub generated_card: Option<String>,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,

    /// EMV tag 5F2D.
    ///
    /// Preferred languages specified by the integrated circuit chip.
    pub preferred_locales: Option<Vec<String>>,

    /// How card details were read in this transaction.
    pub read_method: Option<PaymentMethodDetailsInteracPresentReadMethod>,

    /// A collection of fields required to be displayed on receipts.
    ///
    /// Only required for EMV transactions.
    pub receipt: Option<PaymentMethodDetailsInteracPresentReceipt>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsInteracPresentReceipt {
    /// The type of account being debited or credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PaymentMethodDetailsInteracPresentReceiptAccountType>,

    /// EMV tag 9F26, cryptogram generated by the integrated circuit chip.
    pub application_cryptogram: Option<String>,

    /// Mnenomic of the Application Identifier.
    pub application_preferred_name: Option<String>,

    /// Identifier for this transaction.
    pub authorization_code: Option<String>,

    /// EMV tag 8A.
    ///
    /// A code returned by the card issuer.
    pub authorization_response_code: Option<String>,

    /// How the cardholder verified ownership of the card.
    pub cardholder_verification_method: Option<String>,

    /// EMV tag 84.
    ///
    /// Similar to the application identifier stored on the integrated circuit chip.
    pub dedicated_file_name: Option<String>,

    /// The outcome of a series of EMV functions performed by the card reader.
    pub terminal_verification_results: Option<String>,

    /// An indication of various EMV functions performed during the transaction.
    pub transaction_status_information: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsKlarna {
    /// The Klarna payment method used for this transaction.
    /// Can be one of `pay_later`, `pay_now`, `pay_with_financing`, or `pay_in_installments`.
    pub payment_method_category: Option<String>,

    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    /// Can be one of `de-AT`, `en-AT`, `nl-BE`, `fr-BE`, `en-BE`, `de-DE`, `en-DE`, `da-DK`, `en-DK`, `es-ES`, `en-ES`, `fi-FI`, `sv-FI`, `en-FI`, `en-GB`, `en-IE`, `it-IT`, `en-IT`, `nl-NL`, `en-NL`, `nb-NO`, `en-NO`, `sv-SE`, `en-SE`, `en-US`, `es-US`, `fr-FR`, `en-FR`, `cs-CZ`, `en-CZ`, `el-GR`, `en-GR`, `en-AU`, `en-NZ`, `en-CA`, `fr-CA`, `pl-PL`, `en-PL`, `pt-PT`, `en-PT`, `de-CH`, `fr-CH`, `it-CH`, or `en-CH`.
    pub preferred_locale: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsKonbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<PaymentMethodDetailsKonbiniStore>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsKonbiniStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<PaymentMethodDetailsKonbiniStoreChain>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsLink {
    /// Two-letter ISO code representing the funding source country beneath the Link payment.
    /// You could use this attribute to get a sense of international fees.
    pub country: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsMultibanco {
    /// Entity number associated with this Multibanco payment.
    pub entity: Option<String>,

    /// Reference number associated with this Multibanco payment.
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsOxxo {
    /// OXXO reference number.
    pub number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsP24 {
    /// The customer's bank.
    ///
    /// Can be one of `ing`, `citi_handlowy`, `tmobile_usbugi_bankowe`, `plus_bank`, `etransfer_pocztowy24`, `banki_spbdzielcze`, `bank_nowy_bfg_sa`, `getin_bank`, `velobank`, `blik`, `noble_pay`, `ideabank`, `envelobank`, `santander_przelew24`, `nest_przelew`, `mbank_mtransfer`, `inteligo`, `pbac_z_ipko`, `bnp_paribas`, `credit_agricole`, `toyota_bank`, `bank_pekao_sa`, `volkswagen_bank`, `bank_millennium`, `alior_bank`, or `boz`.
    pub bank: Option<PaymentMethodDetailsP24Bank>,

    /// Unique reference for this Przelewy24 payment.
    pub reference: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Przelewy24 directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. Przelewy24 rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsPaynow {
    /// Reference number associated with this PayNow payment.
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsPaypal {
    /// Owner's email.
    ///
    /// Values are provided by PayPal directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub payer_email: Option<String>,

    /// PayPal account PayerID.
    ///
    /// This identifier uniquely identifies the PayPal customer.
    pub payer_id: Option<String>,

    /// Owner's full name.
    ///
    /// Values provided by PayPal directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub payer_name: Option<String>,

    /// The level of protection offered as defined by PayPal Seller Protection for Merchants, for this transaction.
    pub seller_protection: Option<PaypalSellerProtection>,

    /// A unique ID generated by PayPal for this transaction.
    pub transaction_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsPix {
    /// Unique transaction id generated by BCB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transaction_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsPromptpay {
    /// Bill reference generated by PromptPay.
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsRevolutPay {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsSepaCreditTransfer {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,

    /// IBAN of the bank account to transfer funds to.
    pub iban: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four characters of the IBAN.
    pub last4: Option<String>,

    /// Find the ID of the mandate used for this payment under the [payment_method_details.sepa_debit.mandate](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-sepa_debit-mandate) property on the Charge.
    ///
    /// Use this mandate ID to [retrieve the Mandate](https://stripe.com/docs/api/mandates/retrieve).
    pub mandate: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsSofort {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,

    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit: Option<Expandable<PaymentMethod>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge.
    pub generated_sepa_debit_mandate: Option<Expandable<Mandate>>,

    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,

    /// Preferred language of the SOFORT authorization page that the customer is redirected to.
    /// Can be one of `de`, `en`, `es`, `fr`, `it`, `nl`, or `pl`.
    pub preferred_language: Option<PaymentMethodDetailsSofortPreferredLanguage>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by SOFORT directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsStripeAccount {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsSwish {
    /// Uniquely identifies the payer's Swish account.
    ///
    /// You can use this attribute to check whether two Swish transactions were paid for by the same payer.
    pub fingerprint: Option<String>,

    /// Payer bank reference number for the payment.
    pub payment_reference: Option<String>,

    /// The last four digits of the Swish account phone number.
    pub verified_phone_last4: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<PaymentMethodDetailsUsBankAccountAccountHolderType>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    pub account_type: Option<PaymentMethodDetailsUsBankAccountAccountType>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,

    /// Last four digits of the bank account number.
    pub last4: Option<String>,

    /// Routing number of the bank account.
    pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsWechat {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsWechatPay {
    /// Uniquely identifies this particular WeChat Pay account.
    ///
    /// You can use this attribute to check whether two WeChat accounts are the same.
    pub fingerprint: Option<String>,

    /// Transaction ID of this particular WeChat Pay transaction.
    pub transaction_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsZip {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaypalSellerProtection {
    /// An array of conditions that are covered for the transaction, if applicable.
    pub dispute_categories: Option<Vec<PaypalSellerProtectionDisputeCategories>>,

    /// Indicates whether the transaction is eligible for PayPal's seller protection.
    pub status: PaypalSellerProtectionStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Rule {
    /// The action taken on the payment.
    pub action: String,

    /// Unique identifier for the object.
    pub id: String,

    /// The predicate to evaluate the payment against.
    pub predicate: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThreeDSecureDetailsCharge {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureDetailsChargeAuthenticationFlow>,

    /// The Electronic Commerce Indicator (ECI).
    ///
    /// A protocol-level field indicating what degree of authentication was performed.
    pub electronic_commerce_indicator: Option<ThreeDSecureDetailsChargeElectronicCommerceIndicator>,

    /// The exemption requested via 3DS and accepted by the issuer at authentication time.
    pub exemption_indicator: Option<ThreeDSecureDetailsChargeExemptionIndicator>,

    /// Whether Stripe requested the value of `exemption_indicator` in the transaction.
    ///
    /// This will depend on the outcome of Stripe's internal risk assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_indicator_applied: Option<bool>,

    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureDetailsChargeResult>,

    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureDetailsChargeResultReason>,

    /// The 3D Secure 1 XID or 3D Secure 2 Directory Server Transaction ID
    /// (dsTransId) for this payment.
    pub transaction_id: Option<String>,

    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureDetailsChargeVersion>,
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

    /// A fee in cents (or local equivalent) that will be applied to the charge and transferred to the application owner's Stripe account.
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationSpecs>,

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
    /// For details, see [Creating Separate Charges and Transfers](https://stripe.com/docs/connect/separate-charges-and-transfers#on-behalf-of).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreateChargeRadarOptions>,

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
    /// For details, see [Grouping transactions](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options).
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
            destination: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            radar_options: Default::default(),
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
impl Paginable for ListCharges<'_> {
    type O = Charge;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
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
    /// See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateChargeRadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FraudDetailsParams {
    /// Either `safe` or `fraudulent`.
    pub user_report: FraudDetailsParamsUserReport,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransferDataParams {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account.
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
impl std::default::Default for ChargeStatus {
    fn default() -> Self {
        Self::Failed
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
impl std::default::Default for FraudDetailsParamsUserReport {
    fn default() -> Self {
        Self::Fraudulent
    }
}

/// An enum representing the possible values of an `PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus
{
    Disabled,
    Enabled,
}

impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Disabled => "disabled",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Enabled => "enabled",
        }
    }
}

impl AsRef<str> for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

/// An enum representing the possible values of an `PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus
{
    Available,
    Unavailable,
}

impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Available => "available",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Unavailable => "unavailable",
        }
    }
}

impl AsRef<str> for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    fn default() -> Self {
        Self::Available
    }
}

/// An enum representing the possible values of an `PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus
{
    Available,
    Unavailable,
}

impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus::Available => "available",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus::Unavailable => "unavailable",
        }
    }
}

impl AsRef<str> for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercaptureStatus {
    fn default() -> Self {
        Self::Available
    }
}

/// An enum representing the possible values of an `PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    Available,
    Unavailable,
}

impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus::Available => "available",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus::Unavailable => "unavailable",
        }
    }
}

impl AsRef<str> for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticaptureStatus
{
    fn default() -> Self {
        Self::Available
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
impl std::default::Default for PaymentMethodDetailsAchDebitAccountHolderType {
    fn default() -> Self {
        Self::Company
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
impl std::default::Default for PaymentMethodDetailsBancontactPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardPresent`'s `read_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReadMethod {
    ContactEmv,
    ContactlessEmv,
    ContactlessMagstripeMode,
    MagneticStripeFallback,
    MagneticStripeTrack2,
}

impl PaymentMethodDetailsCardPresentReadMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardPresentReadMethod::ContactEmv => "contact_emv",
            PaymentMethodDetailsCardPresentReadMethod::ContactlessEmv => "contactless_emv",
            PaymentMethodDetailsCardPresentReadMethod::ContactlessMagstripeMode => {
                "contactless_magstripe_mode"
            }
            PaymentMethodDetailsCardPresentReadMethod::MagneticStripeFallback => {
                "magnetic_stripe_fallback"
            }
            PaymentMethodDetailsCardPresentReadMethod::MagneticStripeTrack2 => {
                "magnetic_stripe_track2"
            }
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardPresentReadMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardPresentReadMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsCardPresentReadMethod {
    fn default() -> Self {
        Self::ContactEmv
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardPresentReceipt`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardPresentReceiptAccountType {
    Checking,
    Credit,
    Prepaid,
    Unknown,
}

impl PaymentMethodDetailsCardPresentReceiptAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsCardPresentReceiptAccountType::Checking => "checking",
            PaymentMethodDetailsCardPresentReceiptAccountType::Credit => "credit",
            PaymentMethodDetailsCardPresentReceiptAccountType::Prepaid => "prepaid",
            PaymentMethodDetailsCardPresentReceiptAccountType::Unknown => "unknown",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsCardPresentReceiptAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsCardWallet`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
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
            PaymentMethodDetailsCardWalletType::Link => "link",
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
impl std::default::Default for PaymentMethodDetailsCardWalletType {
    fn default() -> Self {
        Self::AmexExpressCheckout
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
    DeutscheBankAg,
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
            PaymentMethodDetailsEpsBank::DeutscheBankAg => "deutsche_bank_ag",
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
impl std::default::Default for PaymentMethodDetailsEpsBank {
    fn default() -> Self {
        Self::ArzteUndApothekerBank
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsFpx`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsFpxAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodDetailsFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsFpxAccountHolderType::Company => "company",
            PaymentMethodDetailsFpxAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsFpxAccountHolderType {
    fn default() -> Self {
        Self::Company
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
    BankOfChina,
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
            PaymentMethodDetailsFpxBank::BankOfChina => "bank_of_china",
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
impl std::default::Default for PaymentMethodDetailsFpxBank {
    fn default() -> Self {
        Self::AffinBank
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
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
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
            PaymentMethodDetailsIdealBank::N26 => "n26",
            PaymentMethodDetailsIdealBank::Nn => "nn",
            PaymentMethodDetailsIdealBank::Rabobank => "rabobank",
            PaymentMethodDetailsIdealBank::Regiobank => "regiobank",
            PaymentMethodDetailsIdealBank::Revolut => "revolut",
            PaymentMethodDetailsIdealBank::SnsBank => "sns_bank",
            PaymentMethodDetailsIdealBank::TriodosBank => "triodos_bank",
            PaymentMethodDetailsIdealBank::VanLanschot => "van_lanschot",
            PaymentMethodDetailsIdealBank::Yoursafe => "yoursafe",
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
impl std::default::Default for PaymentMethodDetailsIdealBank {
    fn default() -> Self {
        Self::AbnAmro
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
    #[serde(rename = "BITSNL2A")]
    Bitsnl2a,
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
    #[serde(rename = "NNBANL2G")]
    Nnbanl2g,
    #[serde(rename = "NTSBDEB1")]
    Ntsbdeb1,
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
    #[serde(rename = "REVOIE23")]
    Revoie23,
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
            PaymentMethodDetailsIdealBic::Bitsnl2a => "BITSNL2A",
            PaymentMethodDetailsIdealBic::Bunqnl2a => "BUNQNL2A",
            PaymentMethodDetailsIdealBic::Fvlbnl22 => "FVLBNL22",
            PaymentMethodDetailsIdealBic::Handnl2a => "HANDNL2A",
            PaymentMethodDetailsIdealBic::Ingbnl2a => "INGBNL2A",
            PaymentMethodDetailsIdealBic::Knabnl2h => "KNABNL2H",
            PaymentMethodDetailsIdealBic::Moyonl21 => "MOYONL21",
            PaymentMethodDetailsIdealBic::Nnbanl2g => "NNBANL2G",
            PaymentMethodDetailsIdealBic::Ntsbdeb1 => "NTSBDEB1",
            PaymentMethodDetailsIdealBic::Rabonl2u => "RABONL2U",
            PaymentMethodDetailsIdealBic::Rbrbnl21 => "RBRBNL21",
            PaymentMethodDetailsIdealBic::Revoie23 => "REVOIE23",
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
impl std::default::Default for PaymentMethodDetailsIdealBic {
    fn default() -> Self {
        Self::Abnanl2a
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
impl std::default::Default for PaymentMethodDetailsInteracPresentReadMethod {
    fn default() -> Self {
        Self::ContactEmv
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
impl std::default::Default for PaymentMethodDetailsInteracPresentReceiptAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsKonbiniStore`'s `chain` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsKonbiniStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}

impl PaymentMethodDetailsKonbiniStoreChain {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsKonbiniStoreChain::Familymart => "familymart",
            PaymentMethodDetailsKonbiniStoreChain::Lawson => "lawson",
            PaymentMethodDetailsKonbiniStoreChain::Ministop => "ministop",
            PaymentMethodDetailsKonbiniStoreChain::Seicomart => "seicomart",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsKonbiniStoreChain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsKonbiniStoreChain {
    fn default() -> Self {
        Self::Familymart
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
    Velobank,
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
            PaymentMethodDetailsP24Bank::Velobank => "velobank",
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
impl std::default::Default for PaymentMethodDetailsP24Bank {
    fn default() -> Self {
        Self::AliorBank
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
impl std::default::Default for PaymentMethodDetailsSofortPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsUsBankAccount`'s `account_holder_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl PaymentMethodDetailsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsUsBankAccountAccountHolderType::Company => "company",
            PaymentMethodDetailsUsBankAccountAccountHolderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

/// An enum representing the possible values of an `PaymentMethodDetailsUsBankAccount`'s `account_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsUsBankAccountAccountType {
    Checking,
    Savings,
}

impl PaymentMethodDetailsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodDetailsUsBankAccountAccountType::Checking => "checking",
            PaymentMethodDetailsUsBankAccountAccountType::Savings => "savings",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodDetailsUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}

/// An enum representing the possible values of an `PaypalSellerProtection`'s `dispute_categories` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaypalSellerProtectionDisputeCategories {
    Fraudulent,
    ProductNotReceived,
}

impl PaypalSellerProtectionDisputeCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            PaypalSellerProtectionDisputeCategories::Fraudulent => "fraudulent",
            PaypalSellerProtectionDisputeCategories::ProductNotReceived => "product_not_received",
        }
    }
}

impl AsRef<str> for PaypalSellerProtectionDisputeCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaypalSellerProtectionDisputeCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaypalSellerProtectionDisputeCategories {
    fn default() -> Self {
        Self::Fraudulent
    }
}

/// An enum representing the possible values of an `PaypalSellerProtection`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaypalSellerProtectionStatus {
    Eligible,
    NotEligible,
    PartiallyEligible,
}

impl PaypalSellerProtectionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaypalSellerProtectionStatus::Eligible => "eligible",
            PaypalSellerProtectionStatus::NotEligible => "not_eligible",
            PaypalSellerProtectionStatus::PartiallyEligible => "partially_eligible",
        }
    }
}

impl AsRef<str> for PaypalSellerProtectionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaypalSellerProtectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaypalSellerProtectionStatus {
    fn default() -> Self {
        Self::Eligible
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetailsCharge`'s `authentication_flow` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeAuthenticationFlow {
    Challenge,
    Frictionless,
}

impl ThreeDSecureDetailsChargeAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsChargeAuthenticationFlow::Challenge => "challenge",
            ThreeDSecureDetailsChargeAuthenticationFlow::Frictionless => "frictionless",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsChargeAuthenticationFlow {
    fn default() -> Self {
        Self::Challenge
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetailsCharge`'s `electronic_commerce_indicator` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    #[serde(rename = "01")]
    V01,
    #[serde(rename = "02")]
    V02,
    #[serde(rename = "05")]
    V05,
    #[serde(rename = "06")]
    V06,
    #[serde(rename = "07")]
    V07,
}

impl ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsChargeElectronicCommerceIndicator::V01 => "01",
            ThreeDSecureDetailsChargeElectronicCommerceIndicator::V02 => "02",
            ThreeDSecureDetailsChargeElectronicCommerceIndicator::V05 => "05",
            ThreeDSecureDetailsChargeElectronicCommerceIndicator::V06 => "06",
            ThreeDSecureDetailsChargeElectronicCommerceIndicator::V07 => "07",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsChargeElectronicCommerceIndicator {
    fn default() -> Self {
        Self::V01
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetailsCharge`'s `exemption_indicator` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeExemptionIndicator {
    LowRisk,
    None,
}

impl ThreeDSecureDetailsChargeExemptionIndicator {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsChargeExemptionIndicator::LowRisk => "low_risk",
            ThreeDSecureDetailsChargeExemptionIndicator::None => "none",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsChargeExemptionIndicator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsChargeExemptionIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsChargeExemptionIndicator {
    fn default() -> Self {
        Self::LowRisk
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetailsCharge`'s `result` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}

impl ThreeDSecureDetailsChargeResult {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsChargeResult::AttemptAcknowledged => "attempt_acknowledged",
            ThreeDSecureDetailsChargeResult::Authenticated => "authenticated",
            ThreeDSecureDetailsChargeResult::Exempted => "exempted",
            ThreeDSecureDetailsChargeResult::Failed => "failed",
            ThreeDSecureDetailsChargeResult::NotSupported => "not_supported",
            ThreeDSecureDetailsChargeResult::ProcessingError => "processing_error",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsChargeResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsChargeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsChargeResult {
    fn default() -> Self {
        Self::AttemptAcknowledged
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetailsCharge`'s `result_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}

impl ThreeDSecureDetailsChargeResultReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsChargeResultReason::Abandoned => "abandoned",
            ThreeDSecureDetailsChargeResultReason::Bypassed => "bypassed",
            ThreeDSecureDetailsChargeResultReason::Canceled => "canceled",
            ThreeDSecureDetailsChargeResultReason::CardNotEnrolled => "card_not_enrolled",
            ThreeDSecureDetailsChargeResultReason::NetworkNotSupported => "network_not_supported",
            ThreeDSecureDetailsChargeResultReason::ProtocolError => "protocol_error",
            ThreeDSecureDetailsChargeResultReason::Rejected => "rejected",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsChargeResultReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsChargeResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsChargeResultReason {
    fn default() -> Self {
        Self::Abandoned
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetailsCharge`'s `version` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsChargeVersion {
    #[serde(rename = "1.0.2")]
    V1_0_2,
    #[serde(rename = "2.1.0")]
    V2_1_0,
    #[serde(rename = "2.2.0")]
    V2_2_0,
}

impl ThreeDSecureDetailsChargeVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsChargeVersion::V1_0_2 => "1.0.2",
            ThreeDSecureDetailsChargeVersion::V2_1_0 => "2.1.0",
            ThreeDSecureDetailsChargeVersion::V2_2_0 => "2.2.0",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsChargeVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsChargeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsChargeVersion {
    fn default() -> Self {
        Self::V1_0_2
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DestinationSpecs {
    /// ID of an existing, connected Stripe account.
    pub account: String,
    /// The amount to transfer to the destination account without creating an `Application Fee` object.
    ///
    /// Cannot be combined with the `application_fee` parameter.
    /// Must be less than or equal to the charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
}
