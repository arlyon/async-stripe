// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{
    CheckoutSessionId, CustomerId, PaymentIntentId, PaymentLinkId, PaymentMethodConfigurationId,
    SubscriptionId,
};
use crate::params::{
    CurrencyMap, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{
    Address, CheckoutSessionItem, ConnectAccountReference, Currency, Customer, Discount, Invoice,
    InvoiceSettingRenderingOptions, LinkedAccountOptionsUsBankAccount, PaymentIntent, PaymentLink,
    PaymentMethodConfigBizPaymentMethodConfigurationDetails,
    PaymentMethodOptionsCustomerBalanceEuBankAccount, SetupIntent, Shipping, ShippingRate,
    Subscription, TaxId, TaxRate,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Session".
///
/// For more details see <https://stripe.com/docs/api/checkout/sessions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSession {
    /// Unique identifier for the object.
    pub id: CheckoutSessionId,

    /// When set, provides configuration for actions to take if this Checkout Session expires.
    pub after_expiration: Option<PaymentPagesCheckoutSessionAfterExpiration>,

    /// Enables user redeemable promotion codes.
    pub allow_promotion_codes: Option<bool>,

    /// Total of all items before discounts or taxes are applied.
    pub amount_subtotal: Option<i64>,

    /// Total of all items after discounts and taxes are applied.
    pub amount_total: Option<i64>,

    pub automatic_tax: PaymentPagesCheckoutSessionAutomaticTax,

    /// Describes whether Checkout should collect the customer's billing address.
    pub billing_address_collection: Option<CheckoutSessionBillingAddressCollection>,

    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    pub cancel_url: Option<String>,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the Session with your internal systems.
    pub client_reference_id: Option<String>,

    /// Client secret to be used when initializing Stripe.js embedded checkout.
    pub client_secret: Option<String>,

    /// Results of `consent_collection` for this session.
    pub consent: Option<PaymentPagesCheckoutSessionConsent>,

    /// When set, provides configuration for the Checkout Session to gather active consent from customers.
    pub consent_collection: Option<PaymentPagesCheckoutSessionConsentCollection>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<Currency>,

    /// Currency conversion details for automatic currency conversion sessions.
    pub currency_conversion: Option<PaymentPagesCheckoutSessionCurrencyConversion>,

    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 3 fields are supported.
    pub custom_fields: Vec<PaymentPagesCheckoutSessionCustomFields>,

    pub custom_text: PaymentPagesCheckoutSessionCustomText,

    /// The ID of the customer for this Session.
    /// For Checkout Sessions in `subscription` mode or Checkout Sessions with `customer_creation` set as `always` in `payment` mode, Checkout
    /// will create a new customer object based on information provided
    /// during the payment flow unless an existing customer was provided when
    /// the Session was created.
    pub customer: Option<Expandable<Customer>>,

    /// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    pub customer_creation: Option<CheckoutSessionCustomerCreation>,

    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
    ///
    /// Only the customer's email is present on Sessions in `setup` mode.
    pub customer_details: Option<PaymentPagesCheckoutSessionCustomerDetails>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once the payment flow is complete, use the `customer` attribute.
    pub customer_email: Option<String>,

    /// The timestamp at which the Checkout Session will expire.
    pub expires_at: Timestamp,

    /// ID of the invoice created by the Checkout Session, if it exists.
    pub invoice: Option<Expandable<Invoice>>,

    /// Details on the state of invoice creation for the Checkout Session.
    pub invoice_creation: Option<PaymentPagesCheckoutSessionInvoiceCreation>,

    /// The line items purchased by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<List<CheckoutSessionItem>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    pub locale: Option<CheckoutSessionLocale>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// The mode of the Checkout Session.
    pub mode: CheckoutSessionMode,

    /// The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// The ID of the Payment Link that created this Session.
    pub payment_link: Option<Expandable<PaymentLink>>,

    /// Configure whether a Checkout Session should collect a payment method.
    pub payment_method_collection: Option<CheckoutSessionPaymentMethodCollection>,

    /// Information about the payment method configuration used for this Checkout session if using dynamic payment methods.
    pub payment_method_configuration_details:
        Option<PaymentMethodConfigBizPaymentMethodConfigurationDetails>,

    /// Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    pub payment_method_options: Option<CheckoutSessionPaymentMethodOptions>,

    /// A list of the types of payment methods (e.g.
    ///
    /// card) this Checkout Session is allowed to accept.
    pub payment_method_types: Vec<String>,

    /// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
    /// You can use this value to decide when to fulfill your customer's order.
    pub payment_status: CheckoutSessionPaymentStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<PaymentPagesCheckoutSessionPhoneNumberCollection>,

    /// The ID of the original expired Checkout Session that triggered the recovery flow.
    pub recovered_from: Option<String>,

    /// Applies to Checkout Sessions with `ui_mode: embedded`.
    ///
    /// By default, Stripe will always redirect to your return_url after a successful confirmation.
    /// If you set `redirect_on_completion: 'if_required'`, then we will only redirect if your user chooses a redirect-based payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<CheckoutSessionRedirectOnCompletion>,

    /// Applies to Checkout Sessions with `ui_mode: embedded`.
    ///
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    pub setup_intent: Option<Expandable<SetupIntent>>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    pub shipping_address_collection: Option<PaymentPagesCheckoutSessionShippingAddressCollection>,

    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<PaymentPagesCheckoutSessionShippingCost>,

    /// Shipping information for this Checkout Session.
    pub shipping_details: Option<Shipping>,

    /// The shipping rate options applied to this Session.
    pub shipping_options: Vec<PaymentPagesCheckoutSessionShippingOption>,

    /// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    pub status: Option<CheckoutSessionStatus>,

    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode.
    /// If blank or `auto`, `pay` is used.
    pub submit_type: Option<CheckoutSessionSubmitType>,

    /// The ID of the subscription for Checkout Sessions in `subscription` mode.
    pub subscription: Option<Expandable<Subscription>>,

    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<PaymentPagesCheckoutSessionTaxIdCollection>,

    /// Tax and discount details for the computed total amount.
    pub total_details: Option<PaymentPagesCheckoutSessionTotalDetails>,

    /// The UI mode of the Session.
    ///
    /// Can be `hosted` (default) or `embedded`.
    pub ui_mode: Option<CheckoutSessionUiMode>,

    /// The URL to the Checkout Session.
    ///
    /// Redirect customers to this URL to take them to Checkout.
    /// If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain.
    /// Otherwise, it’ll use `checkout.stripe.com.` This value is only present when the session is active.
    pub url: Option<String>,
}

impl CheckoutSession {
    /// Returns a list of Checkout Sessions.
    pub fn list(
        client: &Client,
        params: &ListCheckoutSessions<'_>,
    ) -> Response<List<CheckoutSession>> {
        client.get_query("/checkout/sessions", params)
    }

    /// Creates a Session object.
    pub fn create(client: &Client, params: CreateCheckoutSession<'_>) -> Response<CheckoutSession> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/checkout/sessions", &params)
    }
}

impl Object for CheckoutSession {
    type Id = CheckoutSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "checkout.session"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSessionPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CheckoutAcssDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CheckoutAffirmPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CheckoutAfterpayClearpayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CheckoutAlipayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CheckoutAuBecsDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CheckoutBacsDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CheckoutBancontactPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CheckoutBoletoPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CheckoutCardPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CheckoutCashappPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CheckoutCustomerBalancePaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CheckoutEpsPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CheckoutFpxPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CheckoutGiropayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CheckoutGrabPayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CheckoutIdealPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CheckoutKlarnaPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CheckoutKonbiniPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CheckoutLinkPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CheckoutOxxoPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CheckoutP24PaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CheckoutPaynowPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CheckoutPaypalPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CheckoutPixPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CheckoutRevolutPayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CheckoutSepaDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CheckoutSofortPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CheckoutSwishPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CheckoutUsBankAccountPaymentMethodOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutAcssDebitPaymentMethodOptions {
    /// Currency supported by the bank account.
    ///
    /// Returned when the Session is in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CheckoutAcssDebitMandateOptions>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<CheckoutAcssDebitPaymentMethodOptionsVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutAcssDebitMandateOptions {
    /// A URL for custom mandate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// List of Stripe products where this mandate can be selected automatically.
    ///
    /// Returned when the Session is in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<CheckoutAcssDebitMandateOptionsDefaultFor>>,

    /// Description of the interval.
    ///
    /// Only required if the 'payment_schedule' parameter is 'interval' or 'combined'.
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    pub payment_schedule: Option<CheckoutAcssDebitMandateOptionsPaymentSchedule>,

    /// Transaction type of the mandate.
    pub transaction_type: Option<CheckoutAcssDebitMandateOptionsTransactionType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutAffirmPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutAfterpayClearpayPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutAlipayPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutAlipayPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutAuBecsDebitPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutBacsDebitPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutBancontactPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutBancontactPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutBoletoPaymentMethodOptions {
    /// The number of calendar days before a Boleto voucher expires.
    ///
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto voucher will expire on Wednesday at 23:59 America/Sao_Paulo time.
    pub expires_after_days: u32,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutBoletoPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutCardPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CheckoutCardInstallmentsOptions>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutCardPaymentMethodOptionsSetupFutureUsage>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutCardInstallmentsOptions {
    /// Indicates if installments are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutCashappPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutCashappPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutCustomerBalancePaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptions>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    pub funding_type: Option<CheckoutCustomerBalancePaymentMethodOptionsFundingType>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<PaymentMethodOptionsCustomerBalanceEuBankAccount>,

    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types:
        Option<Vec<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes>>,

    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    pub type_: Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutEpsPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutEpsPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutFpxPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutFpxPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutGiropayPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutGiropayPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutGrabPayPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutIdealPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutIdealPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutKlarnaPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutKonbiniPaymentMethodOptions {
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    ///
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    pub expires_after_days: Option<u32>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutLinkPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutLinkPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutOxxoPaymentMethodOptions {
    /// The number of calendar days before an OXXO invoice expires.
    ///
    /// For example, if you create an OXXO invoice on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    pub expires_after_days: u32,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutOxxoPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutP24PaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutP24PaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutPaynowPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutPaynowPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutPaypalPaymentMethodOptions {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CheckoutPaypalPaymentMethodOptionsCaptureMethod>,

    /// Preferred locale of the PayPal checkout page that the customer is redirected to.
    pub preferred_locale: Option<String>,

    /// A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID.
    ///
    /// This must be a globally unique ID if you have configured in your PayPal settings to block multiple payments per invoice ID.
    pub reference: Option<String>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutPaypalPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutPixPaymentMethodOptions {
    /// The number of seconds after which Pix payment will expire.
    pub expires_after_seconds: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutRevolutPayPaymentMethodOptions {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSepaDebitPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSofortPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutSofortPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSwishPaymentMethodOptions {
    /// The order reference that will be displayed to customers in the Swish application.
    ///
    /// Defaults to the `id` of the Payment Intent.
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutUsBankAccountPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<LinkedAccountOptionsUsBankAccount>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionAfterExpiration {
    /// When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<PaymentPagesCheckoutSessionAfterExpirationRecovery>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions.
    ///
    /// Defaults to `false`.
    pub allow_promotion_codes: bool,

    /// If `true`, a recovery url will be generated to recover this Checkout Session if it
    /// expires before a transaction is completed.
    ///
    /// It will be attached to the Checkout Session object upon expiration.
    pub enabled: bool,

    /// The timestamp at which the recovery URL will expire.
    pub expires_at: Option<Timestamp>,

    /// URL that creates a new Checkout Session when clicked that is a copy of this expired Checkout Session.
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionAutomaticTax {
    /// Indicates whether automatic tax is enabled for the session.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<ConnectAccountReference>,

    /// The status of the most recent automated tax calculation for this session.
    pub status: Option<PaymentPagesCheckoutSessionAutomaticTaxStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionConsent {
    /// If `opt_in`, the customer consents to receiving promotional communications
    /// from the merchant about this Checkout Session.
    pub promotions: Option<PaymentPagesCheckoutSessionConsentPromotions>,

    /// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
    pub terms_of_service: Option<PaymentPagesCheckoutSessionConsentTermsOfService>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionConsentCollection {
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    pub payment_method_reuse_agreement:
        Option<PaymentPagesCheckoutSessionPaymentMethodReuseAgreement>,

    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    pub promotions: Option<PaymentPagesCheckoutSessionConsentCollectionPromotions>,

    /// If set to `required`, it requires customers to accept the terms of service before being able to pay.
    pub terms_of_service: Option<PaymentPagesCheckoutSessionConsentCollectionTermsOfService>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCurrencyConversion {
    /// Total of all items in source currency before discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total of all items in source currency after discounts and taxes are applied.
    pub amount_total: i64,

    /// Exchange rate used to convert source currency amounts to customer currency amounts.
    pub fx_rate: String,

    /// Creation currency of the CheckoutSession before localization.
    pub source_currency: Currency,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<PaymentPagesCheckoutSessionCustomFieldsDropdown>,

    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,

    pub label: PaymentPagesCheckoutSessionCustomFieldsLabel,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<PaymentPagesCheckoutSessionCustomFieldsNumeric>,

    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    pub optional: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PaymentPagesCheckoutSessionCustomFieldsText>,

    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionCustomFieldsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdown {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: Vec<PaymentPagesCheckoutSessionCustomFieldsOption>,

    /// The option selected by the customer.
    ///
    /// This will be the `value` for the option.
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: Option<String>,

    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionCustomFieldsLabelType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,

    /// The value entered by the customer, containing only digits.
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsOption {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: String,

    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,

    /// The value entered by the customer.
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    pub after_submit: Option<PaymentPagesCheckoutSessionCustomTextPosition>,

    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<PaymentPagesCheckoutSessionCustomTextPosition>,

    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<PaymentPagesCheckoutSessionCustomTextPosition>,

    /// Custom text that should be displayed in place of the default terms of service agreement text.
    pub terms_of_service_acceptance: Option<PaymentPagesCheckoutSessionCustomTextPosition>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomTextPosition {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    /// The customer's address after a completed Checkout Session.
    ///
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub address: Option<Address>,

    /// The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or at time of session expiry.
    /// Otherwise, if the customer has consented to promotional content, this value is the most recent valid email provided by the customer on the Checkout form.
    pub email: Option<String>,

    /// The customer's name after a completed Checkout Session.
    ///
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    pub name: Option<String>,

    /// The customer's phone number after a completed Checkout Session.
    pub phone: Option<String>,

    /// The customer’s tax exempt status after a completed Checkout Session.
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,

    /// The customer’s tax IDs after a completed Checkout Session.
    pub tax_ids: Option<Vec<PaymentPagesCheckoutSessionTaxId>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionInvoiceCreation {
    /// Indicates whether invoice creation is enabled for the Checkout Session.
    pub enabled: bool,

    pub invoice_data: PaymentPagesCheckoutSessionInvoiceSettings,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionInvoiceSettings {
    /// The account tax IDs associated with the invoice.
    pub account_tax_ids: Option<Vec<Expandable<TaxId>>>,

    /// Custom fields displayed on the invoice.
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// Footer displayed on the invoice.
    pub footer: Option<String>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    pub issuer: Option<ConnectAccountReference>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// Options for invoice PDF rendering.
    pub rendering_options: Option<InvoiceSettingRenderingOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,

    /// The value of the custom field.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    ///
    /// When set to `auto`, Stripe's defaults will be used.  When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionPhoneNumberCollection {
    /// Indicates whether phone number collection is enabled for the session.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries:
        Vec<PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionShippingCost {
    /// Total shipping cost before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total tax amount applied due to shipping costs.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,

    /// Total shipping cost after discounts and taxes are applied.
    pub amount_total: i64,

    /// The ID of the ShippingRate for this order.
    pub shipping_rate: Option<Expandable<ShippingRate>>,

    /// The taxes applied to the shipping rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<LineItemsTaxAmountTaxabilityReason>,

    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,

    /// The shipping rate.
    pub shipping_rate: Expandable<ShippingRate>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTaxId {
    /// The type of the tax ID, one of `ad_nrt`, `ar_cuit`, `eu_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eu_oss_vat`, `pe_ruc`, `ro_tin`, `rs_pib`, `sv_nit`, `uy_ruc`, `ve_rif`, `vn_tin`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, `ke_pin`, `tr_tin`, `eg_tin`, `ph_tin`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionTaxIdType,

    /// The value of the tax ID.
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTaxIdCollection {
    /// Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,

    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,

    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,

    /// The aggregated tax amounts by rate.
    pub taxes: Vec<LineItemsTaxAmount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsDiscountAmount {
    /// The amount discounted.
    pub amount: i64,

    pub discount: Discount,
}

/// The parameters for `CheckoutSession::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateCheckoutSession<'a> {
    /// Configure actions after a Checkout Session has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<CreateCheckoutSessionAfterExpiration>,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateCheckoutSessionAutomaticTax>,

    /// Specify whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CheckoutSessionBillingAddressCollection>,

    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<&'a str>,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,

    /// Configure fields for the Checkout Session to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<CreateCheckoutSessionConsentCollection>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Required in `setup` mode when `payment_method_types` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 3 fields are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateCheckoutSessionCustomFields>>,

    /// Display additional text for your customers using custom text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<CreateCheckoutSessionCustomText>,

    /// ID of an existing Customer, if one exists.
    ///
    /// In `payment` mode, the customer’s most recently saved card payment method will be used to prefill the email, name, card details, and billing address on the Checkout page.
    /// In `subscription` mode, the customer’s [default payment method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) will be used if it’s a card, otherwise the most recently saved card will be used.
    /// A valid billing address, billing name and billing email are required on the payment method for Checkout to prefill the customer's card details.  If the Customer already has a valid [email](https://stripe.com/docs/api/customers/object#customer_object-email) set, the email will be prefilled and not editable in Checkout. If the Customer does not have a valid `email`, Checkout will set the email entered during the session on the Customer.  If blank for Checkout Sessions in `subscription` mode or with `customer_creation` set as `always` in `payment` mode, Checkout will create a new Customer object based on information provided during the payment flow.  You can set [`payment_intent_data.setup_future_usage`](https://stripe.com/docs/api/checkout/sessions/create#create_checkout_session-payment_intent_data-setup_future_usage) to have Checkout automatically attach the payment method to the Customer you pass in for future reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Configure whether a Checkout Session creates a [Customer](https://stripe.com/docs/api/customers) during Session confirmation.
    ///
    /// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout
    /// with [customer_details](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-customer_details).
    ///
    /// Sessions that don't create Customers instead are grouped by [guest customers](https://stripe.com/docs/payments/checkout/guest-customers)
    /// in the Dashboard.
    ///
    /// Promotion codes limited to first time customers will return invalid for these Sessions.  Can only be set in `payment` and `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CheckoutSessionCustomerCreation>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once a session is complete, use the `customer` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<&'a str>,

    /// Controls what fields on Customer can be updated by the Checkout Session.
    ///
    /// Can only be provided when `customer` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<CreateCheckoutSessionCustomerUpdate>,

    /// The coupon or promotion code to apply to this Session.
    ///
    /// Currently, only up to one may be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CreateCheckoutSessionDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The Epoch time in seconds at which the Checkout Session will expire.
    ///
    /// It can be anywhere from 30 minutes to 24 hours after Checkout Session creation.
    /// By default, this value is 24 hours from creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,

    /// Generate a post-purchase Invoice for one-time payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<CreateCheckoutSessionInvoiceCreation>,

    /// A list of items the customer is purchasing.
    ///
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).  For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.  For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
    /// Line items with one-time Prices will be on the initial invoice only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateCheckoutSessionLineItems>>,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CheckoutSessionLocale>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The mode of the Checkout Session.
    ///
    /// Pass `subscription` if the Checkout Session includes at least one recurring item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CheckoutSessionMode>,

    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreateCheckoutSessionPaymentIntentData>,

    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0. This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on configuring [subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<CheckoutSessionPaymentMethodCollection>,

    /// The ID of the payment method configuration to use with this Checkout session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<PaymentMethodConfigurationId>,

    /// Payment-method-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateCheckoutSessionPaymentMethodOptions>,

    /// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
    ///
    /// You can omit this attribute to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    /// See [Dynamic Payment Methods](https://stripe.com/docs/payments/payment-methods/integration-options#using-dynamic-payment-methods) for more details.
    ///
    /// Read more about the supported payment methods and their requirements in our [payment
    /// method details guide](/docs/payments/checkout/payment-methods).
    ///
    /// If multiple payment methods are passed, Checkout will dynamically reorder them to
    /// prioritize the most relevant payment methods based on the customer's location and
    /// other characteristics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<CreateCheckoutSessionPaymentMethodTypes>>,

    /// Controls phone number collection settings for the session.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts
    /// before using this feature.
    ///
    /// Learn more about [collecting phone numbers with Checkout](https://stripe.com/docs/payments/checkout/phone-numbers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreateCheckoutSessionPhoneNumberCollection>,

    /// This parameter applies to `ui_mode: embedded`.
    ///
    /// By default, Stripe will always redirect to your return_url after a successful confirmation.
    /// If you set `redirect_on_completion: 'if_required'`, then we will only redirect if your user chooses a redirect-based payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<CheckoutSessionRedirectOnCompletion>,

    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the
    /// payment method's app or site.
    ///
    /// This parameter is required if ui_mode is `embedded` and redirect-based payment methods are enabled on the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,

    /// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent_data: Option<CreateCheckoutSessionSetupIntentData>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreateCheckoutSessionShippingAddressCollection>,

    /// The shipping rate options to apply to this Session.
    ///
    /// Up to a maximum of 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<CreateCheckoutSessionShippingOptions>>,

    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode.
    /// If blank or `auto`, `pay` is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,

    /// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreateCheckoutSessionSubscriptionData>,

    /// The URL to which Stripe should send customers when payment or setup
    /// is complete.
    /// This parameter is not allowed if ui_mode is `embedded`.
    ///
    /// If you’d like to use information from the successful Checkout Session on your page, read the guide on [customizing your success page](https://stripe.com/docs/payments/checkout/custom-success-page).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<&'a str>,

    /// Controls tax ID collection settings for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreateCheckoutSessionTaxIdCollection>,

    /// `ui_mode` can be `hosted` or `embedded`.
    ///
    /// The default is `hosted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_mode: Option<CheckoutSessionUiMode>,
}

impl<'a> CreateCheckoutSession<'a> {
    pub fn new() -> Self {
        CreateCheckoutSession {
            after_expiration: Default::default(),
            allow_promotion_codes: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            cancel_url: Default::default(),
            client_reference_id: Default::default(),
            consent_collection: Default::default(),
            currency: Default::default(),
            custom_fields: Default::default(),
            custom_text: Default::default(),
            customer: Default::default(),
            customer_creation: Default::default(),
            customer_email: Default::default(),
            customer_update: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            expires_at: Default::default(),
            invoice_creation: Default::default(),
            line_items: Default::default(),
            locale: Default::default(),
            metadata: Default::default(),
            mode: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_collection: Default::default(),
            payment_method_configuration: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            redirect_on_completion: Default::default(),
            return_url: Default::default(),
            setup_intent_data: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_options: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            success_url: Default::default(),
            tax_id_collection: Default::default(),
            ui_mode: Default::default(),
        }
    }
}

/// The parameters for `CheckoutSession::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCheckoutSessions<'a> {
    /// Only return the Checkout Sessions that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return the Checkout Sessions for the Customer specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Only return the Checkout Sessions for the Customer details specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<ListCheckoutSessionsCustomerDetails>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<CheckoutSessionId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return the Checkout Session for the PaymentIntent specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<PaymentIntentId>,

    /// Only return the Checkout Sessions for the Payment Link specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<PaymentLinkId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<CheckoutSessionId>,

    /// Only return the Checkout Sessions matching the given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutSessionStatus>,

    /// Only return the Checkout Session for the subscription specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,
}

impl<'a> ListCheckoutSessions<'a> {
    pub fn new() -> Self {
        ListCheckoutSessions {
            created: Default::default(),
            customer: Default::default(),
            customer_details: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_intent: Default::default(),
            payment_link: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
            subscription: Default::default(),
        }
    }
}
impl Paginable for ListCheckoutSessions<'_> {
    type O = CheckoutSession;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAfterExpiration {
    /// Configure a Checkout Session that can be used to recover an expired session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CreateCheckoutSessionAfterExpirationRecovery>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAutomaticTax {
    /// Set to true to enable automatic taxes.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateCheckoutSessionAutomaticTaxLiability>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionConsentCollection {
    /// Determines the display of payment method reuse agreement text in the UI.
    ///
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement:
        Option<CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement>,

    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreateCheckoutSessionConsentCollectionPromotions>,

    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreateCheckoutSessionConsentCollectionTermsOfService>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomFields {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CreateCheckoutSessionCustomFieldsDropdown>,

    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,

    /// The label for the field, displayed to the customer.
    pub label: CreateCheckoutSessionCustomFieldsLabel,

    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CreateCheckoutSessionCustomFieldsNumeric>,

    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CreateCheckoutSessionCustomFieldsText>,

    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionCustomFieldsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<CreateCheckoutSessionCustomTextAfterSubmit>,

    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreateCheckoutSessionCustomTextShippingAddress>,

    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CreateCheckoutSessionCustomTextSubmit>,

    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance:
        Option<CreateCheckoutSessionCustomTextTermsOfServiceAcceptance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomerUpdate {
    /// Describes whether Checkout saves the billing address onto `customer.address`.
    /// To always collect a full billing address, use `billing_address_collection`.
    ///
    /// Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateCheckoutSessionCustomerUpdateAddress>,

    /// Describes whether Checkout saves the name onto `customer.name`.
    ///
    /// Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CreateCheckoutSessionCustomerUpdateName>,

    /// Describes whether Checkout saves shipping information onto `customer.shipping`.
    /// To collect shipping information, use `shipping_address_collection`.
    ///
    /// Defaults to `never`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionCustomerUpdateShipping>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionDiscounts {
    /// The ID of the coupon to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// The ID of a promotion code to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionInvoiceCreation {
    /// Set to `true` to enable invoice creation.
    pub enabled: bool,

    /// Parameters passed when creating invoices for payment-mode Checkout Sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreateCheckoutSessionInvoiceCreationInvoiceData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionLineItemsAdjustableQuantity>,

    /// The [tax rates](https://stripe.com/docs/api/tax_rates) that will be applied to this line item depending on the customer's billing/shipping address.
    ///
    /// We currently support the following countries: US, GB, AU, and all countries in the EU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<Vec<String>>,

    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateCheckoutSessionLineItemsPriceData>,

    /// The quantity of the line item being purchased.
    ///
    /// Quantity should not be defined when `recurring.usage_type=metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The [tax rates](https://stripe.com/docs/api/tax_rates) which apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentData {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// The amount of the application fee collected will be capped at the total payment amount.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentIntentDataCaptureMethod>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The Stripe account ID for which these funds are intended.
    ///
    /// For details, see the PaymentIntents [use case for connected accounts](/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Email address that the receipt for the resulting payment will be sent to.
    ///
    /// If `receipt_email` is specified for a payment in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    /// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment
    /// method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the
    /// customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the
    /// customer that their payment details will be saved and used for future
    /// payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,
    /// Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached
    /// to a Customer.
    ///
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentIntentDataSetupFutureUsage>,

    /// Shipping information for this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionPaymentIntentDataShipping>,

    /// Extra information about the payment.
    ///
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// The parameters used to automatically create a Transfer when the payment succeeds.
    /// For more information, see the PaymentIntents [use case for connected accounts](https://stripe.com/docs/payments/connected-accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionPaymentIntentDataTransferData>,

    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptions {
    /// contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebit>,

    /// contains details about the Affirm payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreateCheckoutSessionPaymentMethodOptionsAffirm>,

    /// contains details about the Afterpay Clearpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay>,

    /// contains details about the Alipay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateCheckoutSessionPaymentMethodOptionsAlipay>,

    /// contains details about the AU Becs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit>,

    /// contains details about the Bacs Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebit>,

    /// contains details about the Bancontact payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateCheckoutSessionPaymentMethodOptionsBancontact>,

    /// contains details about the Boleto payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateCheckoutSessionPaymentMethodOptionsBoleto>,

    /// contains details about the Card payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateCheckoutSessionPaymentMethodOptionsCard>,

    /// contains details about the Cashapp Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<CreateCheckoutSessionPaymentMethodOptionsCashapp>,

    /// contains details about the Customer Balance payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalance>,

    /// contains details about the EPS payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateCheckoutSessionPaymentMethodOptionsEps>,

    /// contains details about the FPX payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateCheckoutSessionPaymentMethodOptionsFpx>,

    /// contains details about the Giropay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreateCheckoutSessionPaymentMethodOptionsGiropay>,

    /// contains details about the Grabpay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreateCheckoutSessionPaymentMethodOptionsGrabpay>,

    /// contains details about the Ideal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateCheckoutSessionPaymentMethodOptionsIdeal>,

    /// contains details about the Klarna payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateCheckoutSessionPaymentMethodOptionsKlarna>,

    /// contains details about the Konbini payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateCheckoutSessionPaymentMethodOptionsKonbini>,

    /// contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateCheckoutSessionPaymentMethodOptionsLink>,

    /// contains details about the OXXO payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateCheckoutSessionPaymentMethodOptionsOxxo>,

    /// contains details about the P24 payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateCheckoutSessionPaymentMethodOptionsP24>,

    /// contains details about the PayNow payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateCheckoutSessionPaymentMethodOptionsPaynow>,

    /// contains details about the PayPal payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreateCheckoutSessionPaymentMethodOptionsPaypal>,

    /// contains details about the Pix payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateCheckoutSessionPaymentMethodOptionsPix>,

    /// contains details about the RevolutPay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPay>,

    /// contains details about the Sepa Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebit>,

    /// contains details about the Sofort payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateCheckoutSessionPaymentMethodOptionsSofort>,

    /// contains details about the Swish payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<CreateCheckoutSessionPaymentMethodOptionsSwish>,

    /// contains details about the Us Bank Account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccount>,

    /// contains details about the WeChat Pay payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateCheckoutSessionPaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSetupIntentData {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The Stripe account for which the setup is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<CreateCheckoutSessionShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptions {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    /// Parameters to be passed to Shipping Rate creation for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateCheckoutSessionShippingOptionsShippingRateData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// To use an application fee percent, the request must be made on behalf of another account, using the `Stripe-Account` header or an OAuth key.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// A future timestamp to anchor the subscription's billing cycle for new subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<Timestamp>,

    /// The tax rates that will apply to any subscription item that does not have
    /// `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription
    /// for rendering in the [customer portal](https://stripe.com/docs/customer-management).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettings>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Determines how to handle prorations resulting from the `billing_cycle_anchor`.
    ///
    /// If no value is passed, the default is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<CreateCheckoutSessionSubscriptionDataProrationBehavior>,

    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionSubscriptionDataTransferData>,

    /// Unix timestamp representing the end of the trial period the customer
    /// will get before being charged for the first time.
    ///
    /// Has to be at least 48 hours in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Timestamp>,

    /// Integer representing the number of trial period days before the
    /// customer is charged for the first time.
    ///
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,

    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreateCheckoutSessionSubscriptionDataTrialSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionTaxIdCollection {
    /// Set to true to enable Tax ID collection.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListCheckoutSessionsCustomerDetails {
    /// Customer's email address.
    pub email: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAfterExpirationRecovery {
    /// Enables user redeemable promotion codes on the recovered Checkout Sessions.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// If `true`, a recovery URL will be generated to recover this Checkout Session if it
    /// expires before a successful transaction is completed.
    ///
    /// It will be attached to the Checkout Session object upon expiration.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionAutomaticTaxLiabilityType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    ///
    /// When set to `auto`, Stripe's defaults will be used.
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdown {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: Vec<CreateCheckoutSessionCustomFieldsDropdownOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: String,

    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionCustomFieldsLabelType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomTextAfterSubmit {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomTextShippingAddress {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomTextSubmit {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomTextTermsOfServiceAcceptance {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceData {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative integer.
    ///
    /// By default customers will be able to remove the line item by setting the quantity to 0.
    pub enabled: bool,

    /// The maximum quantity the customer can purchase for the Checkout Session.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 999999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    /// The minimum quantity the customer must purchase for the Checkout Session.
    ///
    /// By default this value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the product that this price will belong to.
    ///
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// Data used to generate a new product object inline.
    ///
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateCheckoutSessionLineItemsPriceDataProductData>,

    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateCheckoutSessionLineItemsPriceDataRecurring>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionLineItemsPriceDataTaxBehavior>,

    /// A non-negative integer in cents (or local equivalent) representing how much to charge.
    ///
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShipping {
    /// Shipping address.
    pub address: CreateCheckoutSessionPaymentIntentDataShippingAddress,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Recipient name.
    pub name: String,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataTransferData {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account.
    ///
    /// The ID of the resulting transfer will be returned on the successful charge's `transfer` field.
    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// This is only accepted for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAffirm {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBancontact {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBoleto {
    /// The number of calendar days before a Boleto voucher expires.
    ///
    /// For example, if you create a Boleto voucher on Monday and you set expires_after_days to 2, the Boleto invoice will expire on Wednesday at 23:59 America/Sao_Paulo time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCard {
    /// Installment options for card payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateCheckoutSessionPaymentMethodOptionsCardInstallments>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kana prefix (shortened Kana descriptor) or Kana statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the Kanji prefix (shortened Kanji descriptor) or Kanji statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 17 characters.
    /// On card statements, the *concatenation* of both prefix and suffix (including separators) will appear truncated to 17 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCashapp {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsEps {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsFpx {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGiropay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarna {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKonbini {
    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    ///
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    /// Defaults to 3 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsLink {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage>,

    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaynow {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaypal {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod>,

    /// [Preferred locale](https://stripe.com/docs/payments/paypal/supported-locales) of the PayPal checkout page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale>,

    /// A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID.
    ///
    /// This must be a globally unique ID if you have configured in your PayPal settings to block multiple payments per invoice ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// The risk correlation ID for an on-session payment using a saved PayPal payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_correlation_id: Option<String>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPix {
    /// The number of seconds (between 10 and 1209600) after which Pix payment will expire.
    ///
    /// Defaults to 86400 seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsRevolutPay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSofort {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSwish {
    /// The order reference that will be displayed to customers in the Swish application.
    ///
    /// Defaults to the `id` of the Payment Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    /// The client type that the end customer will pay from.
    pub client: CreateCheckoutSessionPaymentMethodOptionsWechatPayClient,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate>,

    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: String,

    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateCheckoutSessionShippingOptionsShippingRateDataType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettings {
    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomFieldsDropdownOptions {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: String,

    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataCustomFields {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: String,

    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataProductData {
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The product's name, meant to be displayable to the customer.
    pub name: String,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreateCheckoutSessionLineItemsPriceDataRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShippingAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: String,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// List of Stripe products where this mandate can be selected automatically.
    ///
    /// Only usable in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<Vec<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor>>,

    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this Checkout Session.
    /// Setting to false will prevent any installment plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer {

    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,

    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        Vec<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>,
    >,

    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<
        Vec<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum>,

    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        CurrencyMap<CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions>,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method:
        CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<
        CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior,
    >,
}

/// An enum representing the possible values of an `CheckoutAcssDebitMandateOptions`'s `default_for` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl CheckoutAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitMandateOptionsDefaultFor::Invoice => "invoice",
            CheckoutAcssDebitMandateOptionsDefaultFor::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAcssDebitMandateOptionsDefaultFor {
    fn default() -> Self {
        Self::Invoice
    }
}

/// An enum representing the possible values of an `CheckoutAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CheckoutAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CheckoutAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAcssDebitMandateOptionsPaymentSchedule {
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `CheckoutAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CheckoutAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitMandateOptionsTransactionType::Business => "business",
            CheckoutAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAcssDebitMandateOptionsTransactionType {
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `CheckoutAcssDebitPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutAcssDebitPaymentMethodOptions`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Automatic => "automatic",
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Instant => "instant",
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CheckoutAffirmPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAffirmPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutAfterpayClearpayPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAfterpayClearpayPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutAlipayPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAlipayPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutAlipayPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAlipayPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutAlipayPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAlipayPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAlipayPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutAuBecsDebitPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutAuBecsDebitPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutBacsDebitPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutBacsDebitPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutBancontactPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutBancontactPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutBoletoPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutBoletoPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CheckoutBoletoPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutBoletoPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutBoletoPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutBoletoPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CheckoutBoletoPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutBoletoPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutBoletoPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutCardPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutCardPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutCardPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutCardPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutCardPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutCashappPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCashappPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutCashappPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutCashappPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutCashappPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutCashappPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutCashappPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutCustomerBalanceBankTransferPaymentMethodOptions`'s `requested_address_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::Aba => "aba",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::Iban => "iban",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::Sepa => "sepa",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::SortCode => "sort_code",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::Spei => "spei",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::Swift => "swift",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn default() -> Self {
        Self::Aba
    }
}

/// An enum representing the possible values of an `CheckoutCustomerBalanceBankTransferPaymentMethodOptions`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::EuBankTransfer => {
                "eu_bank_transfer"
            }
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::GbBankTransfer => {
                "gb_bank_transfer"
            }
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::JpBankTransfer => {
                "jp_bank_transfer"
            }
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::MxBankTransfer => {
                "mx_bank_transfer"
            }
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::UsBankTransfer => {
                "us_bank_transfer"
            }
        }
    }
}

impl AsRef<str> for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

/// An enum representing the possible values of an `CheckoutCustomerBalancePaymentMethodOptions`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    BankTransfer,
}

impl CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutCustomerBalancePaymentMethodOptionsFundingType::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}

/// An enum representing the possible values of an `CheckoutCustomerBalancePaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutCustomerBalancePaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutEpsPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutEpsPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutEpsPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutEpsPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutEpsPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutEpsPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutEpsPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutFpxPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutFpxPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutFpxPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutFpxPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutFpxPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutFpxPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutFpxPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutGiropayPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutGiropayPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutGiropayPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutGiropayPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutGiropayPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutGiropayPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutGiropayPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutGrabPayPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutGrabPayPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutIdealPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutIdealPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutIdealPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutIdealPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutIdealPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutIdealPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutIdealPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutKlarnaPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutKlarnaPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutKonbiniPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutKonbiniPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutLinkPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutLinkPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
}

impl CheckoutLinkPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutLinkPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutLinkPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CheckoutLinkPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutLinkPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutLinkPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutOxxoPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutOxxoPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutOxxoPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutOxxoPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutOxxoPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutOxxoPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutOxxoPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutP24PaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutP24PaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutP24PaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutP24PaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutP24PaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutP24PaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutP24PaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutPaynowPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPaynowPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutPaynowPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutPaynowPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutPaynowPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutPaynowPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutPaynowPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutPaypalPaymentMethodOptions`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPaypalPaymentMethodOptionsCaptureMethod {
    Manual,
}

impl CheckoutPaypalPaymentMethodOptionsCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutPaypalPaymentMethodOptionsCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CheckoutPaypalPaymentMethodOptionsCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutPaypalPaymentMethodOptionsCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutPaypalPaymentMethodOptionsCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `CheckoutPaypalPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPaypalPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
}

impl CheckoutPaypalPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutPaypalPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutPaypalPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CheckoutPaypalPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutPaypalPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutPaypalPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutSepaDebitPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSepaDebitPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `billing_address_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionBillingAddressCollection {
    Auto,
    Required,
}

impl CheckoutSessionBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionBillingAddressCollection::Auto => "auto",
            CheckoutSessionBillingAddressCollection::Required => "required",
        }
    }
}

impl AsRef<str> for CheckoutSessionBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionBillingAddressCollection {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `customer_creation` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionCustomerCreation {
    Always,
    IfRequired,
}

impl CheckoutSessionCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionCustomerCreation::Always => "always",
            CheckoutSessionCustomerCreation::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for CheckoutSessionCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionCustomerCreation {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `locale` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    #[serde(rename = "en-GB")]
    EnGb,
    Es,
    #[serde(rename = "es-419")]
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    #[serde(rename = "pt-BR")]
    PtBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    #[serde(rename = "zh-HK")]
    ZhHk,
    #[serde(rename = "zh-TW")]
    ZhTw,
}

impl CheckoutSessionLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionLocale::Auto => "auto",
            CheckoutSessionLocale::Bg => "bg",
            CheckoutSessionLocale::Cs => "cs",
            CheckoutSessionLocale::Da => "da",
            CheckoutSessionLocale::De => "de",
            CheckoutSessionLocale::El => "el",
            CheckoutSessionLocale::En => "en",
            CheckoutSessionLocale::EnGb => "en-GB",
            CheckoutSessionLocale::Es => "es",
            CheckoutSessionLocale::Es419 => "es-419",
            CheckoutSessionLocale::Et => "et",
            CheckoutSessionLocale::Fi => "fi",
            CheckoutSessionLocale::Fil => "fil",
            CheckoutSessionLocale::Fr => "fr",
            CheckoutSessionLocale::FrCa => "fr-CA",
            CheckoutSessionLocale::Hr => "hr",
            CheckoutSessionLocale::Hu => "hu",
            CheckoutSessionLocale::Id => "id",
            CheckoutSessionLocale::It => "it",
            CheckoutSessionLocale::Ja => "ja",
            CheckoutSessionLocale::Ko => "ko",
            CheckoutSessionLocale::Lt => "lt",
            CheckoutSessionLocale::Lv => "lv",
            CheckoutSessionLocale::Ms => "ms",
            CheckoutSessionLocale::Mt => "mt",
            CheckoutSessionLocale::Nb => "nb",
            CheckoutSessionLocale::Nl => "nl",
            CheckoutSessionLocale::Pl => "pl",
            CheckoutSessionLocale::Pt => "pt",
            CheckoutSessionLocale::PtBr => "pt-BR",
            CheckoutSessionLocale::Ro => "ro",
            CheckoutSessionLocale::Ru => "ru",
            CheckoutSessionLocale::Sk => "sk",
            CheckoutSessionLocale::Sl => "sl",
            CheckoutSessionLocale::Sv => "sv",
            CheckoutSessionLocale::Th => "th",
            CheckoutSessionLocale::Tr => "tr",
            CheckoutSessionLocale::Vi => "vi",
            CheckoutSessionLocale::Zh => "zh",
            CheckoutSessionLocale::ZhHk => "zh-HK",
            CheckoutSessionLocale::ZhTw => "zh-TW",
        }
    }
}

impl AsRef<str> for CheckoutSessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionLocale {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
}

impl CheckoutSessionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionMode::Payment => "payment",
            CheckoutSessionMode::Setup => "setup",
            CheckoutSessionMode::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CheckoutSessionMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionMode {
    fn default() -> Self {
        Self::Payment
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `payment_method_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionPaymentMethodCollection {
    Always,
    IfRequired,
}

impl CheckoutSessionPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionPaymentMethodCollection::Always => "always",
            CheckoutSessionPaymentMethodCollection::IfRequired => "if_required",
        }
    }
}

impl AsRef<str> for CheckoutSessionPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionPaymentMethodCollection {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `payment_status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
}

impl CheckoutSessionPaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionPaymentStatus::NoPaymentRequired => "no_payment_required",
            CheckoutSessionPaymentStatus::Paid => "paid",
            CheckoutSessionPaymentStatus::Unpaid => "unpaid",
        }
    }
}

impl AsRef<str> for CheckoutSessionPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionPaymentStatus {
    fn default() -> Self {
        Self::NoPaymentRequired
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `redirect_on_completion` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionRedirectOnCompletion {
    Always,
    IfRequired,
    Never,
}

impl CheckoutSessionRedirectOnCompletion {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionRedirectOnCompletion::Always => "always",
            CheckoutSessionRedirectOnCompletion::IfRequired => "if_required",
            CheckoutSessionRedirectOnCompletion::Never => "never",
        }
    }
}

impl AsRef<str> for CheckoutSessionRedirectOnCompletion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionRedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionRedirectOnCompletion {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionStatus {
    Complete,
    Expired,
    Open,
}

impl CheckoutSessionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionStatus::Complete => "complete",
            CheckoutSessionStatus::Expired => "expired",
            CheckoutSessionStatus::Open => "open",
        }
    }
}

impl AsRef<str> for CheckoutSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionStatus {
    fn default() -> Self {
        Self::Complete
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `submit_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl CheckoutSessionSubmitType {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionSubmitType::Auto => "auto",
            CheckoutSessionSubmitType::Book => "book",
            CheckoutSessionSubmitType::Donate => "donate",
            CheckoutSessionSubmitType::Pay => "pay",
        }
    }
}

impl AsRef<str> for CheckoutSessionSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionSubmitType {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CheckoutSession`'s `ui_mode` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionUiMode {
    Embedded,
    Hosted,
}

impl CheckoutSessionUiMode {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSessionUiMode::Embedded => "embedded",
            CheckoutSessionUiMode::Hosted => "hosted",
        }
    }
}

impl AsRef<str> for CheckoutSessionUiMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSessionUiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSessionUiMode {
    fn default() -> Self {
        Self::Embedded
    }
}

/// An enum representing the possible values of an `CheckoutSofortPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSofortPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutSofortPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutSofortPaymentMethodOptionsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutSofortPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutSofortPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutSofortPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutUsBankAccountPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CheckoutUsBankAccountPaymentMethodOptions`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
}

impl CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Automatic => "automatic",
            CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::Instant => "instant",
        }
    }
}

impl AsRef<str> for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionAutomaticTaxLiability`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionAutomaticTaxLiabilityType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreateCheckoutSessionAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionAutomaticTaxLiabilityType::Account => "account",
            CreateCheckoutSessionAutomaticTaxLiabilityType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionAutomaticTaxLiabilityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionAutomaticTaxLiabilityType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreement`'s `position` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}

impl CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition::Auto => {
                "auto"
            }
            CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition::Hidden => {
                "hidden"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionConsentCollectionPaymentMethodReuseAgreementPosition
{
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionConsentCollection`'s `promotions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionConsentCollectionPromotions {
    Auto,
    None,
}

impl CreateCheckoutSessionConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionConsentCollectionPromotions::Auto => "auto",
            CreateCheckoutSessionConsentCollectionPromotions::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionConsentCollectionPromotions {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionConsentCollection`'s `terms_of_service` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionConsentCollectionTermsOfService {
    None,
    Required,
}

impl CreateCheckoutSessionConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionConsentCollectionTermsOfService::None => "none",
            CreateCheckoutSessionConsentCollectionTermsOfService::Required => "required",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionConsentCollectionTermsOfService {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomFieldsLabel`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomFieldsLabelType {
    Custom,
}

impl CreateCheckoutSessionCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomFieldsLabelType::Custom => "custom",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionCustomFieldsLabelType {
    fn default() -> Self {
        Self::Custom
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomFields`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl CreateCheckoutSessionCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomFieldsType::Dropdown => "dropdown",
            CreateCheckoutSessionCustomFieldsType::Numeric => "numeric",
            CreateCheckoutSessionCustomFieldsType::Text => "text",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionCustomFieldsType {
    fn default() -> Self {
        Self::Dropdown
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomerUpdate`'s `address` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomerUpdateAddress {
    Auto,
    Never,
}

impl CreateCheckoutSessionCustomerUpdateAddress {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomerUpdateAddress::Auto => "auto",
            CreateCheckoutSessionCustomerUpdateAddress::Never => "never",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomerUpdateAddress {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionCustomerUpdateAddress {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomerUpdate`'s `name` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomerUpdateName {
    Auto,
    Never,
}

impl CreateCheckoutSessionCustomerUpdateName {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomerUpdateName::Auto => "auto",
            CreateCheckoutSessionCustomerUpdateName::Never => "never",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomerUpdateName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionCustomerUpdateName {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionCustomerUpdate`'s `shipping` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionCustomerUpdateShipping {
    Auto,
    Never,
}

impl CreateCheckoutSessionCustomerUpdateShipping {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionCustomerUpdateShipping::Auto => "auto",
            CreateCheckoutSessionCustomerUpdateShipping::Never => "never",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionCustomerUpdateShipping {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionCustomerUpdateShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionCustomerUpdateShipping {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionInvoiceCreationInvoiceDataIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType::Account => "account",
            CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionInvoiceCreationInvoiceDataIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptions`'s `amount_tax_display` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn default() -> Self {
        Self::ExcludeTax
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionLineItemsPriceDataRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Day => "day",
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Month => "month",
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Week => "week",
            CreateCheckoutSessionLineItemsPriceDataRecurringInterval::Year => "year",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionLineItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionLineItemsPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionLineItemsPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateCheckoutSessionLineItemsPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateCheckoutSessionLineItemsPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionLineItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentIntentData`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

impl CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentIntentDataCaptureMethod::Automatic => "automatic",
            CreateCheckoutSessionPaymentIntentDataCaptureMethod::AutomaticAsync => {
                "automatic_async"
            }
            CreateCheckoutSessionPaymentIntentDataCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentIntentData`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            CreateCheckoutSessionPaymentIntentDataSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentIntentDataSetupFutureUsage {
    fn default() -> Self {
        Self::OffSession
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions`'s `default_for` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Invoice => "invoice",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor
{
    fn default() -> Self {
        Self::Invoice
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => {
                "automatic"
            }
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Instant => {
                "instant"
            }
            CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAffirm`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage::None => {
                "none"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAlipay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsBacsDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsBancontact`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsBoleto`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCard`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage {
    fn default() -> Self {
        Self::OffSession
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCashapp`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsCashappSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer`'s `requested_address_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
}

impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Aba => "aba",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Iban => "iban",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Sepa => "sepa",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::SortCode => "sort_code",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Spei => "spei",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Swift => "swift",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Zengin => "zengin",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    fn default() -> Self {
        Self::Aba
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransfer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}

impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer => "eu_bank_transfer",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer => "gb_bank_transfer",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer => "jp_bank_transfer",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer => "mx_bank_transfer",
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType::UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCustomerBalance`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer => {
                "bank_transfer"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCustomerBalance`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage::None => {
                "none"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsEps`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsFpx`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsGiropay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsGrabpay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsIdeal`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsKlarna`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsKonbini`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsLink`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsLinkSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsOxxo`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsP24`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsPaynow`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsPaypal`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    Manual,
}

impl CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsPaypalCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsPaypal`'s `preferred_locale` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    #[serde(rename = "cs-CZ")]
    CsCz,
    #[serde(rename = "da-DK")]
    DaDk,
    #[serde(rename = "de-AT")]
    DeAt,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "de-LU")]
    DeLu,
    #[serde(rename = "el-GR")]
    ElGr,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "fi-FI")]
    FiFi,
    #[serde(rename = "fr-BE")]
    FrBe,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "fr-LU")]
    FrLu,
    #[serde(rename = "hu-HU")]
    HuHu,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "nl-BE")]
    NlBe,
    #[serde(rename = "nl-NL")]
    NlNl,
    #[serde(rename = "pl-PL")]
    PlPl,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "sk-SK")]
    SkSk,
    #[serde(rename = "sv-SE")]
    SvSe,
}

impl CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::CsCz => "cs-CZ",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::DaDk => "da-DK",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::DeAt => "de-AT",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::DeDe => "de-DE",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::DeLu => "de-LU",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::ElGr => "el-GR",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::EnGb => "en-GB",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::EnUs => "en-US",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::EsEs => "es-ES",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::FiFi => "fi-FI",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::FrBe => "fr-BE",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::FrFr => "fr-FR",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::FrLu => "fr-LU",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::HuHu => "hu-HU",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::ItIt => "it-IT",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::NlBe => "nl-BE",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::NlNl => "nl-NL",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::PlPl => "pl-PL",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::PtPt => "pt-PT",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::SkSk => "sk-SK",
            CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale::SvSe => "sv-SE",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsPaypalPreferredLocale {
    fn default() -> Self {
        Self::CsCz
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsPaypal`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    None,
    OffSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsPaypalSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsRevolutPay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    None,
    OffSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsRevolutPaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsSepaDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsSofort`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Balances => "balances",
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Ownership => "ownership",
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::PaymentMethod => "payment_method",
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Transactions,
}

impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Balances => "balances",
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsUsBankAccount`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage::None => "none",
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
}

impl CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => {
                "automatic"
            }
            CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => {
                "instant"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsWechatPay`'s `client` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::Android => "android",
            CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::Ios => "ios",
            CreateCheckoutSessionPaymentMethodOptionsWechatPayClient::Web => "web",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsWechatPayClient {
    fn default() -> Self {
        Self::Android
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsWechatPay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateCheckoutSession`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodTypes {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    Zip,
}

impl CreateCheckoutSessionPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodTypes::AcssDebit => "acss_debit",
            CreateCheckoutSessionPaymentMethodTypes::Affirm => "affirm",
            CreateCheckoutSessionPaymentMethodTypes::AfterpayClearpay => "afterpay_clearpay",
            CreateCheckoutSessionPaymentMethodTypes::Alipay => "alipay",
            CreateCheckoutSessionPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            CreateCheckoutSessionPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreateCheckoutSessionPaymentMethodTypes::Bancontact => "bancontact",
            CreateCheckoutSessionPaymentMethodTypes::Blik => "blik",
            CreateCheckoutSessionPaymentMethodTypes::Boleto => "boleto",
            CreateCheckoutSessionPaymentMethodTypes::Card => "card",
            CreateCheckoutSessionPaymentMethodTypes::Cashapp => "cashapp",
            CreateCheckoutSessionPaymentMethodTypes::CustomerBalance => "customer_balance",
            CreateCheckoutSessionPaymentMethodTypes::Eps => "eps",
            CreateCheckoutSessionPaymentMethodTypes::Fpx => "fpx",
            CreateCheckoutSessionPaymentMethodTypes::Giropay => "giropay",
            CreateCheckoutSessionPaymentMethodTypes::Grabpay => "grabpay",
            CreateCheckoutSessionPaymentMethodTypes::Ideal => "ideal",
            CreateCheckoutSessionPaymentMethodTypes::Klarna => "klarna",
            CreateCheckoutSessionPaymentMethodTypes::Konbini => "konbini",
            CreateCheckoutSessionPaymentMethodTypes::Link => "link",
            CreateCheckoutSessionPaymentMethodTypes::Oxxo => "oxxo",
            CreateCheckoutSessionPaymentMethodTypes::P24 => "p24",
            CreateCheckoutSessionPaymentMethodTypes::Paynow => "paynow",
            CreateCheckoutSessionPaymentMethodTypes::Paypal => "paypal",
            CreateCheckoutSessionPaymentMethodTypes::Pix => "pix",
            CreateCheckoutSessionPaymentMethodTypes::Promptpay => "promptpay",
            CreateCheckoutSessionPaymentMethodTypes::RevolutPay => "revolut_pay",
            CreateCheckoutSessionPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateCheckoutSessionPaymentMethodTypes::Sofort => "sofort",
            CreateCheckoutSessionPaymentMethodTypes::Swish => "swish",
            CreateCheckoutSessionPaymentMethodTypes::UsBankAccount => "us_bank_account",
            CreateCheckoutSessionPaymentMethodTypes::WechatPay => "wechat_pay",
            CreateCheckoutSessionPaymentMethodTypes::Zip => "zip",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodTypes {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ac => "AC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ad => "AD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ae => "AE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Af => "AF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ag => "AG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ai => "AI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Al => "AL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Am => "AM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ao => "AO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Aq => "AQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ar => "AR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::At => "AT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Au => "AU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Aw => "AW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ax => "AX",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Az => "AZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ba => "BA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bb => "BB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bd => "BD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Be => "BE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bf => "BF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bg => "BG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bh => "BH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bi => "BI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bj => "BJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bl => "BL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bm => "BM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bn => "BN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bo => "BO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bq => "BQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Br => "BR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bs => "BS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bt => "BT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bv => "BV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bw => "BW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::By => "BY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Bz => "BZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ca => "CA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cd => "CD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cf => "CF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cg => "CG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ch => "CH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ci => "CI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ck => "CK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cl => "CL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cm => "CM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cn => "CN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Co => "CO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cr => "CR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cv => "CV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cw => "CW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cy => "CY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Cz => "CZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::De => "DE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dj => "DJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dk => "DK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dm => "DM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Do => "DO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Dz => "DZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ec => "EC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ee => "EE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Eg => "EG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Eh => "EH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Er => "ER",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Es => "ES",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Et => "ET",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fi => "FI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fj => "FJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fk => "FK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fo => "FO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Fr => "FR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ga => "GA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gb => "GB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gd => "GD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ge => "GE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gf => "GF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gg => "GG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gh => "GH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gi => "GI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gl => "GL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gm => "GM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gn => "GN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gp => "GP",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gq => "GQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gr => "GR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gs => "GS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gt => "GT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gu => "GU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gw => "GW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Gy => "GY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hk => "HK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hn => "HN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hr => "HR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ht => "HT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Hu => "HU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Id => "ID",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ie => "IE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Il => "IL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Im => "IM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::In => "IN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Io => "IO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Iq => "IQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Is => "IS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::It => "IT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Je => "JE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Jm => "JM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Jo => "JO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Jp => "JP",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ke => "KE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kg => "KG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kh => "KH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ki => "KI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Km => "KM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kn => "KN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kr => "KR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kw => "KW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ky => "KY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Kz => "KZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::La => "LA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lb => "LB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lc => "LC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Li => "LI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lk => "LK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lr => "LR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ls => "LS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lt => "LT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lu => "LU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Lv => "LV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ly => "LY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ma => "MA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mc => "MC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Md => "MD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Me => "ME",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mf => "MF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mg => "MG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mk => "MK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ml => "ML",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mm => "MM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mn => "MN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mo => "MO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mq => "MQ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mr => "MR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ms => "MS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mt => "MT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mu => "MU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mv => "MV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mw => "MW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mx => "MX",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::My => "MY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Mz => "MZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Na => "NA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nc => "NC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ne => "NE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ng => "NG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ni => "NI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nl => "NL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::No => "NO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Np => "NP",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nr => "NR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nu => "NU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Nz => "NZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Om => "OM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pa => "PA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pe => "PE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pf => "PF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pg => "PG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ph => "PH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pk => "PK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pl => "PL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pm => "PM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pn => "PN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pr => "PR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ps => "PS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Pt => "PT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Py => "PY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Qa => "QA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Re => "RE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ro => "RO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Rs => "RS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ru => "RU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Rw => "RW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sa => "SA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sb => "SB",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sc => "SC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Se => "SE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sg => "SG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sh => "SH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Si => "SI",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sj => "SJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sk => "SK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sl => "SL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sm => "SM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sn => "SN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::So => "SO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sr => "SR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ss => "SS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::St => "ST",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sv => "SV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sx => "SX",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Sz => "SZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ta => "TA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tc => "TC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Td => "TD",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tf => "TF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tg => "TG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Th => "TH",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tj => "TJ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tk => "TK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tl => "TL",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tm => "TM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tn => "TN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::To => "TO",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tr => "TR",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tt => "TT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tv => "TV",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tw => "TW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Tz => "TZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ua => "UA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ug => "UG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Us => "US",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Uy => "UY",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Uz => "UZ",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Va => "VA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vc => "VC",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ve => "VE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vg => "VG",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vn => "VN",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Vu => "VU",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Wf => "WF",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ws => "WS",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Xk => "XK",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Ye => "YE",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Yt => "YT",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Za => "ZA",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Zm => "ZM",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Zw => "ZW",
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn default() -> Self {
        Self::Ac
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay => "business_day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Day => "day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Month => "month",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit::Week => "week",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit
{
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::BusinessDay => "business_day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Day => "day",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Hour => "hour",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Month => "month",
            CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit::Week => "week",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit
{
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::Exclusive => {
                "exclusive"
            }
            CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::Inclusive => {
                "inclusive"
            }
            CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior::Unspecified => {
                "unspecified"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionShippingOptionsShippingRateData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionShippingOptionsShippingRateDataType {
    FixedAmount,
}

impl CreateCheckoutSessionShippingOptionsShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionShippingOptionsShippingRateDataType::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionShippingOptionsShippingRateDataType {
    fn default() -> Self {
        Self::FixedAmount
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType::Account => "account",
            CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionSubscriptionDataInvoiceSettingsIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionSubscriptionData`'s `proration_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionSubscriptionDataProrationBehavior {
    CreateProrations,
    None,
}

impl CreateCheckoutSessionSubscriptionDataProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionSubscriptionDataProrationBehavior::CreateProrations => {
                "create_prorations"
            }
            CreateCheckoutSessionSubscriptionDataProrationBehavior::None => "none",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionSubscriptionDataProrationBehavior {
    fn default() -> Self {
        Self::CreateProrations
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehavior`'s `missing_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => "create_invoice",
            CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
        }
    }
}

impl AsRef<str>
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn default() -> Self {
        Self::Cancel
    }
}

/// An enum representing the possible values of an `LineItemsTaxAmount`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LineItemsTaxAmountTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

impl LineItemsTaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            LineItemsTaxAmountTaxabilityReason::CustomerExempt => "customer_exempt",
            LineItemsTaxAmountTaxabilityReason::NotCollecting => "not_collecting",
            LineItemsTaxAmountTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            LineItemsTaxAmountTaxabilityReason::NotSupported => "not_supported",
            LineItemsTaxAmountTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            LineItemsTaxAmountTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            LineItemsTaxAmountTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            LineItemsTaxAmountTaxabilityReason::ProductExempt => "product_exempt",
            LineItemsTaxAmountTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            LineItemsTaxAmountTaxabilityReason::ProportionallyRated => "proportionally_rated",
            LineItemsTaxAmountTaxabilityReason::ReducedRated => "reduced_rated",
            LineItemsTaxAmountTaxabilityReason::ReverseCharge => "reverse_charge",
            LineItemsTaxAmountTaxabilityReason::StandardRated => "standard_rated",
            LineItemsTaxAmountTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            LineItemsTaxAmountTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for LineItemsTaxAmountTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LineItemsTaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for LineItemsTaxAmountTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionAutomaticTax`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl PaymentPagesCheckoutSessionAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionAutomaticTaxStatus::Complete => "complete",
            PaymentPagesCheckoutSessionAutomaticTaxStatus::Failed => "failed",
            PaymentPagesCheckoutSessionAutomaticTaxStatus::RequiresLocationInputs => {
                "requires_location_inputs"
            }
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn default() -> Self {
        Self::Complete
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionConsentCollection`'s `promotions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentCollectionPromotions {
    Auto,
    None,
}

impl PaymentPagesCheckoutSessionConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionConsentCollectionPromotions::Auto => "auto",
            PaymentPagesCheckoutSessionConsentCollectionPromotions::None => "none",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionConsentCollection`'s `terms_of_service` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    None,
    Required,
}

impl PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionConsentCollectionTermsOfService::None => "none",
            PaymentPagesCheckoutSessionConsentCollectionTermsOfService::Required => "required",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionConsent`'s `promotions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentPromotions {
    OptIn,
    OptOut,
}

impl PaymentPagesCheckoutSessionConsentPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionConsentPromotions::OptIn => "opt_in",
            PaymentPagesCheckoutSessionConsentPromotions::OptOut => "opt_out",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionConsentPromotions {
    fn default() -> Self {
        Self::OptIn
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionConsent`'s `terms_of_service` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentTermsOfService {
    Accepted,
}

impl PaymentPagesCheckoutSessionConsentTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionConsentTermsOfService::Accepted => "accepted",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn default() -> Self {
        Self::Accepted
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionCustomFieldsLabel`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionCustomFieldsLabelType {
    Custom,
}

impl PaymentPagesCheckoutSessionCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionCustomFieldsLabelType::Custom => "custom",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionCustomFieldsLabelType {
    fn default() -> Self {
        Self::Custom
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionCustomFields`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl PaymentPagesCheckoutSessionCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionCustomFieldsType::Dropdown => "dropdown",
            PaymentPagesCheckoutSessionCustomFieldsType::Numeric => "numeric",
            PaymentPagesCheckoutSessionCustomFieldsType::Text => "text",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionCustomFieldsType {
    fn default() -> Self {
        Self::Dropdown
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionCustomerDetails`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Exempt => "exempt",
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::None => "none",
            PaymentPagesCheckoutSessionCustomerDetailsTaxExempt::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionCustomerDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionPaymentMethodReuseAgreement`'s `position` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}

impl PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Auto => "auto",
            PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Hidden => "hidden",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ac => "AC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ad => "AD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ae => "AE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Af => "AF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ag => "AG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ai => "AI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Al => "AL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Am => "AM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ao => "AO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Aq => "AQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ar => "AR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::At => "AT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Au => "AU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Aw => "AW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ax => "AX",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Az => "AZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ba => "BA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bb => "BB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bd => "BD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Be => "BE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bf => "BF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bg => "BG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bh => "BH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bi => "BI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bj => "BJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bl => "BL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bm => "BM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bn => "BN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bo => "BO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bq => "BQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Br => "BR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bs => "BS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bt => "BT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bv => "BV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bw => "BW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::By => "BY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Bz => "BZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ca => "CA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cd => "CD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cf => "CF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cg => "CG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ch => "CH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ci => "CI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ck => "CK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cl => "CL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cm => "CM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cn => "CN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Co => "CO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cr => "CR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cv => "CV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cw => "CW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cy => "CY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Cz => "CZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::De => "DE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Dj => "DJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Dk => "DK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Dm => "DM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Do => "DO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Dz => "DZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ec => "EC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ee => "EE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Eg => "EG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Eh => "EH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Er => "ER",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Es => "ES",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Et => "ET",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Fi => "FI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Fj => "FJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Fk => "FK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Fo => "FO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Fr => "FR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ga => "GA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gb => "GB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gd => "GD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ge => "GE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gf => "GF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gg => "GG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gh => "GH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gi => "GI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gl => "GL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gm => "GM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gn => "GN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gp => "GP",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gq => "GQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gr => "GR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gs => "GS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gt => "GT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gu => "GU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gw => "GW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Gy => "GY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Hk => "HK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Hn => "HN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Hr => "HR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ht => "HT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Hu => "HU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Id => "ID",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ie => "IE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Il => "IL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Im => "IM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::In => "IN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Io => "IO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Iq => "IQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Is => "IS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::It => "IT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Je => "JE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Jm => "JM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Jo => "JO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Jp => "JP",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ke => "KE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Kg => "KG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Kh => "KH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ki => "KI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Km => "KM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Kn => "KN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Kr => "KR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Kw => "KW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ky => "KY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Kz => "KZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::La => "LA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Lb => "LB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Lc => "LC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Li => "LI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Lk => "LK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Lr => "LR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ls => "LS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Lt => "LT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Lu => "LU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Lv => "LV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ly => "LY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ma => "MA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mc => "MC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Md => "MD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Me => "ME",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mf => "MF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mg => "MG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mk => "MK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ml => "ML",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mm => "MM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mn => "MN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mo => "MO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mq => "MQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mr => "MR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ms => "MS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mt => "MT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mu => "MU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mv => "MV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mw => "MW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mx => "MX",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::My => "MY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Mz => "MZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Na => "NA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Nc => "NC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ne => "NE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ng => "NG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ni => "NI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Nl => "NL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::No => "NO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Np => "NP",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Nr => "NR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Nu => "NU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Nz => "NZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Om => "OM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pa => "PA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pe => "PE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pf => "PF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pg => "PG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ph => "PH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pk => "PK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pl => "PL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pm => "PM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pn => "PN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pr => "PR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ps => "PS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Pt => "PT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Py => "PY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Qa => "QA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Re => "RE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ro => "RO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Rs => "RS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ru => "RU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Rw => "RW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sa => "SA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sb => "SB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sc => "SC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Se => "SE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sg => "SG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sh => "SH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Si => "SI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sj => "SJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sk => "SK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sl => "SL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sm => "SM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sn => "SN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::So => "SO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sr => "SR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ss => "SS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::St => "ST",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sv => "SV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sx => "SX",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Sz => "SZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ta => "TA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tc => "TC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Td => "TD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tf => "TF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tg => "TG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Th => "TH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tj => "TJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tk => "TK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tl => "TL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tm => "TM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tn => "TN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::To => "TO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tr => "TR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tt => "TT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tv => "TV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tw => "TW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Tz => "TZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ua => "UA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ug => "UG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Us => "US",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Uy => "UY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Uz => "UZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Va => "VA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Vc => "VC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ve => "VE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Vg => "VG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Vn => "VN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Vu => "VU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Wf => "WF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ws => "WS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Xk => "XK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Ye => "YE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Yt => "YT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Za => "ZA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Zm => "ZM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Zw => "ZW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountries
{
    fn default() -> Self {
        Self::Ac
    }
}

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionTaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionTaxIdType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BoTin,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    Unknown,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
}

impl PaymentPagesCheckoutSessionTaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionTaxIdType::AdNrt => "ad_nrt",
            PaymentPagesCheckoutSessionTaxIdType::AeTrn => "ae_trn",
            PaymentPagesCheckoutSessionTaxIdType::ArCuit => "ar_cuit",
            PaymentPagesCheckoutSessionTaxIdType::AuAbn => "au_abn",
            PaymentPagesCheckoutSessionTaxIdType::AuArn => "au_arn",
            PaymentPagesCheckoutSessionTaxIdType::BgUic => "bg_uic",
            PaymentPagesCheckoutSessionTaxIdType::BoTin => "bo_tin",
            PaymentPagesCheckoutSessionTaxIdType::BrCnpj => "br_cnpj",
            PaymentPagesCheckoutSessionTaxIdType::BrCpf => "br_cpf",
            PaymentPagesCheckoutSessionTaxIdType::CaBn => "ca_bn",
            PaymentPagesCheckoutSessionTaxIdType::CaGstHst => "ca_gst_hst",
            PaymentPagesCheckoutSessionTaxIdType::CaPstBc => "ca_pst_bc",
            PaymentPagesCheckoutSessionTaxIdType::CaPstMb => "ca_pst_mb",
            PaymentPagesCheckoutSessionTaxIdType::CaPstSk => "ca_pst_sk",
            PaymentPagesCheckoutSessionTaxIdType::CaQst => "ca_qst",
            PaymentPagesCheckoutSessionTaxIdType::ChVat => "ch_vat",
            PaymentPagesCheckoutSessionTaxIdType::ClTin => "cl_tin",
            PaymentPagesCheckoutSessionTaxIdType::CnTin => "cn_tin",
            PaymentPagesCheckoutSessionTaxIdType::CoNit => "co_nit",
            PaymentPagesCheckoutSessionTaxIdType::CrTin => "cr_tin",
            PaymentPagesCheckoutSessionTaxIdType::DoRcn => "do_rcn",
            PaymentPagesCheckoutSessionTaxIdType::EcRuc => "ec_ruc",
            PaymentPagesCheckoutSessionTaxIdType::EgTin => "eg_tin",
            PaymentPagesCheckoutSessionTaxIdType::EsCif => "es_cif",
            PaymentPagesCheckoutSessionTaxIdType::EuOssVat => "eu_oss_vat",
            PaymentPagesCheckoutSessionTaxIdType::EuVat => "eu_vat",
            PaymentPagesCheckoutSessionTaxIdType::GbVat => "gb_vat",
            PaymentPagesCheckoutSessionTaxIdType::GeVat => "ge_vat",
            PaymentPagesCheckoutSessionTaxIdType::HkBr => "hk_br",
            PaymentPagesCheckoutSessionTaxIdType::HuTin => "hu_tin",
            PaymentPagesCheckoutSessionTaxIdType::IdNpwp => "id_npwp",
            PaymentPagesCheckoutSessionTaxIdType::IlVat => "il_vat",
            PaymentPagesCheckoutSessionTaxIdType::InGst => "in_gst",
            PaymentPagesCheckoutSessionTaxIdType::IsVat => "is_vat",
            PaymentPagesCheckoutSessionTaxIdType::JpCn => "jp_cn",
            PaymentPagesCheckoutSessionTaxIdType::JpRn => "jp_rn",
            PaymentPagesCheckoutSessionTaxIdType::JpTrn => "jp_trn",
            PaymentPagesCheckoutSessionTaxIdType::KePin => "ke_pin",
            PaymentPagesCheckoutSessionTaxIdType::KrBrn => "kr_brn",
            PaymentPagesCheckoutSessionTaxIdType::LiUid => "li_uid",
            PaymentPagesCheckoutSessionTaxIdType::MxRfc => "mx_rfc",
            PaymentPagesCheckoutSessionTaxIdType::MyFrp => "my_frp",
            PaymentPagesCheckoutSessionTaxIdType::MyItn => "my_itn",
            PaymentPagesCheckoutSessionTaxIdType::MySst => "my_sst",
            PaymentPagesCheckoutSessionTaxIdType::NoVat => "no_vat",
            PaymentPagesCheckoutSessionTaxIdType::NzGst => "nz_gst",
            PaymentPagesCheckoutSessionTaxIdType::PeRuc => "pe_ruc",
            PaymentPagesCheckoutSessionTaxIdType::PhTin => "ph_tin",
            PaymentPagesCheckoutSessionTaxIdType::RoTin => "ro_tin",
            PaymentPagesCheckoutSessionTaxIdType::RsPib => "rs_pib",
            PaymentPagesCheckoutSessionTaxIdType::RuInn => "ru_inn",
            PaymentPagesCheckoutSessionTaxIdType::RuKpp => "ru_kpp",
            PaymentPagesCheckoutSessionTaxIdType::SaVat => "sa_vat",
            PaymentPagesCheckoutSessionTaxIdType::SgGst => "sg_gst",
            PaymentPagesCheckoutSessionTaxIdType::SgUen => "sg_uen",
            PaymentPagesCheckoutSessionTaxIdType::SiTin => "si_tin",
            PaymentPagesCheckoutSessionTaxIdType::SvNit => "sv_nit",
            PaymentPagesCheckoutSessionTaxIdType::ThVat => "th_vat",
            PaymentPagesCheckoutSessionTaxIdType::TrTin => "tr_tin",
            PaymentPagesCheckoutSessionTaxIdType::TwVat => "tw_vat",
            PaymentPagesCheckoutSessionTaxIdType::UaVat => "ua_vat",
            PaymentPagesCheckoutSessionTaxIdType::Unknown => "unknown",
            PaymentPagesCheckoutSessionTaxIdType::UsEin => "us_ein",
            PaymentPagesCheckoutSessionTaxIdType::UyRuc => "uy_ruc",
            PaymentPagesCheckoutSessionTaxIdType::VeRif => "ve_rif",
            PaymentPagesCheckoutSessionTaxIdType::VnTin => "vn_tin",
            PaymentPagesCheckoutSessionTaxIdType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionTaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionTaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentPagesCheckoutSessionTaxIdType {
    fn default() -> Self {
        Self::AdNrt
    }
}
