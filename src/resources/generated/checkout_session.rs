// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CheckoutSessionId, CustomerId, PaymentIntentId, SubscriptionId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, Timestamp};
use crate::resources::{
    Address, CheckoutSessionItem, Currency, Customer, Discount, LinkedAccountOptionsUsBankAccount,
    PaymentIntent, PaymentLink, SetupIntent, Shipping, ShippingRate, Subscription, TaxRate,
};

/// The resource representing a Stripe "Session".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckoutSession {
    /// Unique identifier for the object.
    ///
    /// Used to pass to `redirectToCheckout` in Stripe.js.
    pub id: CheckoutSessionId,

    /// When set, provides configuration for actions to take if this Checkout Session expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<PaymentPagesCheckoutSessionAfterExpiration>,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// Total of all items before discounts or taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,

    /// Total of all items after discounts and taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,

    pub automatic_tax: PaymentPagesCheckoutSessionAutomaticTax,

    /// Describes whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<CheckoutSessionBillingAddressCollection>,

    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: String,

    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the Session with your internal systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,

    /// Results of `consent_collection` for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<PaymentPagesCheckoutSessionConsent>,

    /// When set, provides configuration for the Checkout Session to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<PaymentPagesCheckoutSessionConsentCollection>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The ID of the customer for this Session.
    /// For Checkout Sessions in `payment` or `subscription` mode, Checkout
    /// will create a new customer object based on information provided
    /// during the payment flow unless an existing customer was provided when
    /// the Session was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<CheckoutSessionCustomerCreation>,

    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
    ///
    /// Only the customer's email is present on Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<PaymentPagesCheckoutSessionCustomerDetails>,

    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once the payment flow is complete, use the `customer` attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,

    /// The timestamp at which the Checkout Session will expire.
    pub expires_at: Timestamp,

    /// The line items purchased by the customer.
    #[serde(default)]
    pub line_items: List<CheckoutSessionItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<CheckoutSessionLocale>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The mode of the Checkout Session.
    pub mode: CheckoutSessionMode,

    /// The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// The ID of the Payment Link that created this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<Expandable<PaymentLink>>,

    /// Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovered_from: Option<String>,

    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<Expandable<SetupIntent>>,

    /// Shipping information for this Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<PaymentPagesCheckoutSessionShippingAddressCollection>,

    /// The shipping rate options applied to this Session.
    pub shipping_options: Vec<PaymentPagesCheckoutSessionShippingOption>,

    /// The ID of the ShippingRate for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<Expandable<ShippingRate>>,

    /// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutSessionStatus>,

    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,

    /// The ID of the subscription for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,

    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<PaymentPagesCheckoutSessionTaxIdCollection>,

    /// Tax and discount details for the computed total amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_details: Option<PaymentPagesCheckoutSessionTotalDetails>,

    /// The URL to the Checkout Session.
    ///
    /// Redirect customers to this URL to take them to Checkout.
    /// If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain.
    /// Otherwise, it’ll use `checkout.stripe.com.`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CheckoutSession {
    /// Returns a list of Checkout Sessions.
    pub fn list(
        client: &Client,
        params: &ListCheckoutSessions<'_>,
    ) -> Response<List<CheckoutSession>> {
        client.get_query("/checkout/sessions", &params)
    }

    /// Creates a Session object.
    pub fn create(client: &Client, params: CreateCheckoutSession<'_>) -> Response<CheckoutSession> {
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
    pub oxxo: Option<CheckoutOxxoPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CheckoutP24PaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CheckoutPaynowPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CheckoutSepaDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CheckoutSofortPaymentMethodOptions>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<CheckoutAcssDebitMandateOptionsPaymentSchedule>,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,

    /// URL that creates a new Checkout Session when clicked that is a copy of this expired Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionAutomaticTax {
    /// Indicates whether automatic tax is enabled for the session.
    pub enabled: bool,

    /// The status of the most recent automated tax calculation for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentPagesCheckoutSessionAutomaticTaxStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionConsent {
    /// If `opt_in`, the customer consents to receiving promotional communications
    /// from the merchant about this Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<PaymentPagesCheckoutSessionConsentPromotions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionConsentCollection {
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<PaymentPagesCheckoutSessionConsentCollectionPromotions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    /// The customer's address after a completed Checkout Session.
    ///
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or at time of session expiry.
    /// Otherwise, if the customer has consented to promotional content, this value is the most recent valid email provided by the customer on the Checkout form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The customer's name after a completed Checkout Session.
    ///
    /// Note: This property is populated only for sessions on or after March 30, 2022.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The customer's phone number after a completed Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The customer’s tax exempt status after a completed Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,

    /// The customer’s tax IDs after a completed Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<PaymentPagesCheckoutSessionTaxId>>,
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
pub struct PaymentPagesCheckoutSessionShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,

    /// The shipping rate.
    pub shipping_rate: Expandable<ShippingRate>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTaxId {
    /// The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `eu_oss_vat`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionTaxIdType,

    /// The value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,
}

/// The parameters for `CheckoutSession::create`.
#[derive(Clone, Debug, Serialize)]
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

    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: &'a str,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// ID of an existing Customer, if one exists.
    ///
    /// In `payment` mode, the customer’s most recent card payment method will be used to prefill the email, name, card details, and billing address on the Checkout page.
    /// In `subscription` mode, the customer’s [default payment method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) will be used if it’s a card, and otherwise the most recent card will be used.
    /// A valid billing address, billing name and billing email are required on the payment method for Checkout to prefill the customer's card details.  If the Customer already has a valid [email](https://stripe.com/docs/api/customers/object#customer_object-email) set, the email will be prefilled and not editable in Checkout. If the Customer does not have a valid `email`, Checkout will set the email entered during the session on the Customer.  If blank for Checkout Sessions in `payment` or `subscription` mode, Checkout will create a new Customer object based on information provided during the payment flow.  You can set [`payment_intent_data.setup_future_usage`](https://stripe.com/docs/api/checkout/sessions/create#create_checkout_session-payment_intent_data-setup_future_usage) to have Checkout automatically attach the payment method to the Customer you pass in for future reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Configure whether a Checkout Session creates a [Customer](https://stripe.com/docs/api/customers) during Session confirmation.
    ///
    /// When a Customer is not created, you can still retrieve email, address, and other customer data entered in Checkout
    /// with [customer_details](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-customer_details).
    ///
    /// Sessions that don't create Customers instead create [Guest Customers](https://support.stripe.com/questions/guest-customer-faq)
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

    /// A list of items the customer is purchasing.
    ///
    /// Use this parameter to pass one-time or recurring [Prices](https://stripe.com/docs/api/prices).  For `payment` mode, there is a maximum of 100 line items, however it is recommended to consolidate line items if there are more than a few dozen.  For `subscription` mode, there is a maximum of 20 line items with recurring Prices and 20 line items with one-time Prices.
    /// Line items with one-time Prices in will be on the initial invoice only.
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
    /// Required when using prices or `setup` mode.
    /// Pass `subscription` if the Checkout Session includes at least one recurring item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CheckoutSessionMode>,

    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreateCheckoutSessionPaymentIntentData>,

    /// Payment-method-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateCheckoutSessionPaymentMethodOptions>,

    /// A list of the types of payment methods (e.g., `card`) this Checkout Session can accept.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
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

    /// A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent_data: Option<CreateCheckoutSessionSetupIntentData>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreateCheckoutSessionShippingAddressCollection>,

    /// The shipping rate options to apply to this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<CreateCheckoutSessionShippingOptions>>,

    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<CheckoutSessionSubmitType>,

    /// A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreateCheckoutSessionSubscriptionData>,

    /// The URL to which Stripe should send customers when payment or setup
    /// is complete.
    /// If you’d like to use information from the successful Checkout Session on your page,
    /// read the guide on [customizing your success page](https://stripe.com/docs/payments/checkout/custom-success-page).
    pub success_url: &'a str,

    /// Controls tax ID collection settings for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreateCheckoutSessionTaxIdCollection>,
}

impl<'a> CreateCheckoutSession<'a> {
    pub fn new(cancel_url: &'a str, success_url: &'a str) -> Self {
        CreateCheckoutSession {
            after_expiration: Default::default(),
            allow_promotion_codes: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            cancel_url,
            client_reference_id: Default::default(),
            consent_collection: Default::default(),
            currency: Default::default(),
            customer: Default::default(),
            customer_creation: Default::default(),
            customer_email: Default::default(),
            customer_update: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            expires_at: Default::default(),
            line_items: Default::default(),
            locale: Default::default(),
            metadata: Default::default(),
            mode: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_options: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            setup_intent_data: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_options: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            success_url,
            tax_id_collection: Default::default(),
        }
    }
}

/// The parameters for `CheckoutSession::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListCheckoutSessions<'a> {
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

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<CheckoutSessionId>,

    /// Only return the Checkout Session for the subscription specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,
}

impl<'a> ListCheckoutSessions<'a> {
    pub fn new() -> Self {
        ListCheckoutSessions {
            customer: Default::default(),
            customer_details: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            payment_intent: Default::default(),
            starting_after: Default::default(),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery: Option<CreateCheckoutSessionAfterExpirationRecovery>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionConsentCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreateCheckoutSessionConsentCollectionPromotions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionCustomerUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateCheckoutSessionCustomerUpdateAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CreateCheckoutSessionCustomerUpdateName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionCustomerUpdateShipping>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionDiscounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreateCheckoutSessionLineItemsAdjustableQuantity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateCheckoutSessionLineItemsPriceData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateCheckoutSessionPaymentIntentDataCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentIntentDataSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCheckoutSessionPaymentIntentDataShipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionPaymentIntentDataTransferData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreateCheckoutSessionPaymentMethodOptionsAffirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateCheckoutSessionPaymentMethodOptionsAlipay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateCheckoutSessionPaymentMethodOptionsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateCheckoutSessionPaymentMethodOptionsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateCheckoutSessionPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateCheckoutSessionPaymentMethodOptionsEps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateCheckoutSessionPaymentMethodOptionsFpx>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreateCheckoutSessionPaymentMethodOptionsGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreateCheckoutSessionPaymentMethodOptionsGrabpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateCheckoutSessionPaymentMethodOptionsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateCheckoutSessionPaymentMethodOptionsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateCheckoutSessionPaymentMethodOptionsKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateCheckoutSessionPaymentMethodOptionsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateCheckoutSessionPaymentMethodOptionsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateCheckoutSessionPaymentMethodOptionsPaynow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateCheckoutSessionPaymentMethodOptionsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateCheckoutSessionPaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPhoneNumberCollection {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSetupIntentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingAddressCollection {
    pub allowed_countries: Vec<CreateCheckoutSessionShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateCheckoutSessionShippingOptionsShippingRateData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateCheckoutSessionSubscriptionDataItems>>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateCheckoutSessionSubscriptionDataTransferData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionTaxIdCollection {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListCheckoutSessionsCustomerDetails {
    pub email: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionAfterExpirationRecovery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsAdjustableQuantity {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceData {
    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateCheckoutSessionLineItemsPriceDataProductData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateCheckoutSessionLineItemsPriceDataRecurring>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionLineItemsPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShipping {
    pub address: CreateCheckoutSessionPaymentIntentDataShippingAddress,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAffirm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAffirmSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAlipay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAuBecsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBacsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBacsDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsBancontactSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsBoleto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsBoletoSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateCheckoutSessionPaymentMethodOptionsCardInstallments>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsCardSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsEps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsEpsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsFpx {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsFpxSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGiropay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGiropaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsGrabpay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsGrabpaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsIdeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsKlarnaSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsKonbini {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsOxxo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsP24SetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsPaynow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsPaynowSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsSofort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateCheckoutSessionPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsWechatPay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    pub client: CreateCheckoutSessionPaymentMethodOptionsWechatPayClient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateCheckoutSessionPaymentMethodOptionsWechatPaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate>,

    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateCheckoutSessionShippingOptionsShippingRateDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateCheckoutSessionShippingOptionsShippingRateDataType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataItems {
    pub plan: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionSubscriptionDataTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataProductData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionLineItemsPriceDataRecurring {
    pub interval: CreateCheckoutSessionLineItemsPriceDataRecurringInterval,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentIntentDataShippingAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    pub line1: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<Vec<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateCheckoutSessionPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardInstallments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlan>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        Vec<CreateCheckoutSessionPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmount {
    pub amount: i64,

    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlan {
    pub count: u64,

    pub interval: CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanInterval,

    #[serde(rename = "type")]
    pub type_: CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximum {
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMaximumUnit,

    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimum {
    pub unit: CreateCheckoutSessionShippingOptionsShippingRateDataDeliveryEstimateMinimumUnit,

    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateCheckoutSessionShippingOptionsShippingRateDataFixedAmountCurrencyOptions {
    pub amount: i64,

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
    Manual,
}

impl CreateCheckoutSessionPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentIntentDataCaptureMethod::Automatic => "automatic",
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

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlan`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}

impl CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanInterval::Month => "month",
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn default() -> Self {
        Self::Month
    }
}

/// An enum representing the possible values of an `CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlan`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}

impl CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanType::FixedCount => {
                "fixed_count"
            }
        }
    }
}

impl AsRef<str> for CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateCheckoutSessionPaymentMethodOptionsCardInstallmentsPlanType {
    fn default() -> Self {
        Self::FixedCount
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
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Oxxo,
    P24,
    Paynow,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
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
            CreateCheckoutSessionPaymentMethodTypes::Eps => "eps",
            CreateCheckoutSessionPaymentMethodTypes::Fpx => "fpx",
            CreateCheckoutSessionPaymentMethodTypes::Giropay => "giropay",
            CreateCheckoutSessionPaymentMethodTypes::Grabpay => "grabpay",
            CreateCheckoutSessionPaymentMethodTypes::Ideal => "ideal",
            CreateCheckoutSessionPaymentMethodTypes::Klarna => "klarna",
            CreateCheckoutSessionPaymentMethodTypes::Konbini => "konbini",
            CreateCheckoutSessionPaymentMethodTypes::Oxxo => "oxxo",
            CreateCheckoutSessionPaymentMethodTypes::P24 => "p24",
            CreateCheckoutSessionPaymentMethodTypes::Paynow => "paynow",
            CreateCheckoutSessionPaymentMethodTypes::Promptpay => "promptpay",
            CreateCheckoutSessionPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateCheckoutSessionPaymentMethodTypes::Sofort => "sofort",
            CreateCheckoutSessionPaymentMethodTypes::UsBankAccount => "us_bank_account",
            CreateCheckoutSessionPaymentMethodTypes::WechatPay => "wechat_pay",
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
    AeTrn,
    AuAbn,
    AuArn,
    BgUic,
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
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    ThVat,
    TwVat,
    UaVat,
    Unknown,
    UsEin,
    ZaVat,
}

impl PaymentPagesCheckoutSessionTaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionTaxIdType::AeTrn => "ae_trn",
            PaymentPagesCheckoutSessionTaxIdType::AuAbn => "au_abn",
            PaymentPagesCheckoutSessionTaxIdType::AuArn => "au_arn",
            PaymentPagesCheckoutSessionTaxIdType::BgUic => "bg_uic",
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
            PaymentPagesCheckoutSessionTaxIdType::KrBrn => "kr_brn",
            PaymentPagesCheckoutSessionTaxIdType::LiUid => "li_uid",
            PaymentPagesCheckoutSessionTaxIdType::MxRfc => "mx_rfc",
            PaymentPagesCheckoutSessionTaxIdType::MyFrp => "my_frp",
            PaymentPagesCheckoutSessionTaxIdType::MyItn => "my_itn",
            PaymentPagesCheckoutSessionTaxIdType::MySst => "my_sst",
            PaymentPagesCheckoutSessionTaxIdType::NoVat => "no_vat",
            PaymentPagesCheckoutSessionTaxIdType::NzGst => "nz_gst",
            PaymentPagesCheckoutSessionTaxIdType::RuInn => "ru_inn",
            PaymentPagesCheckoutSessionTaxIdType::RuKpp => "ru_kpp",
            PaymentPagesCheckoutSessionTaxIdType::SaVat => "sa_vat",
            PaymentPagesCheckoutSessionTaxIdType::SgGst => "sg_gst",
            PaymentPagesCheckoutSessionTaxIdType::SgUen => "sg_uen",
            PaymentPagesCheckoutSessionTaxIdType::SiTin => "si_tin",
            PaymentPagesCheckoutSessionTaxIdType::ThVat => "th_vat",
            PaymentPagesCheckoutSessionTaxIdType::TwVat => "tw_vat",
            PaymentPagesCheckoutSessionTaxIdType::UaVat => "ua_vat",
            PaymentPagesCheckoutSessionTaxIdType::Unknown => "unknown",
            PaymentPagesCheckoutSessionTaxIdType::UsEin => "us_ein",
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
        Self::AeTrn
    }
}
