/// A Checkout Session represents your customer's session as they pay for
/// one-time purchases or subscriptions through [Checkout](https://stripe.com/docs/payments/checkout)
/// or [Payment Links](https://stripe.com/docs/payments/payment-links). We recommend creating a
/// new Session each time your customer attempts to pay.
///
/// Once payment is successful, the Checkout Session will contain a reference
/// to the [Customer](https://stripe.com/docs/api/customers), and either the successful
/// [PaymentIntent](https://stripe.com/docs/api/payment_intents) or an active
/// [Subscription](https://stripe.com/docs/api/subscriptions).
///
/// You can create a Checkout Session on your server and redirect to its URL
/// to begin Checkout.
///
/// Related guide: [Checkout quickstart](https://stripe.com/docs/checkout/quickstart)
///
/// For more details see <<https://stripe.com/docs/api/checkout/sessions/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutSession {
    /// Settings for price localization with [Adaptive Pricing](https://docs.stripe.com/payments/checkout/adaptive-pricing).
    pub adaptive_pricing: Option<stripe_shared::PaymentPagesCheckoutSessionAdaptivePricing>,
    /// When set, provides configuration for actions to take if this Checkout Session expires.
    pub after_expiration: Option<stripe_shared::PaymentPagesCheckoutSessionAfterExpiration>,
    /// Enables user redeemable promotion codes.
    pub allow_promotion_codes: Option<bool>,
    /// Total of all items before discounts or taxes are applied.
    pub amount_subtotal: Option<i64>,
    /// Total of all items after discounts and taxes are applied.
    pub amount_total: Option<i64>,
    pub automatic_tax: stripe_shared::PaymentPagesCheckoutSessionAutomaticTax,
    /// Describes whether Checkout should collect the customer's billing address. Defaults to `auto`.
    pub billing_address_collection: Option<stripe_shared::CheckoutSessionBillingAddressCollection>,
    pub branding_settings: Option<stripe_shared::PaymentPagesCheckoutSessionBrandingSettings>,
    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    pub cancel_url: Option<String>,
    /// A unique string to reference the Checkout Session. This can be a
    /// customer ID, a cart ID, or similar, and can be used to reconcile the
    /// Session with your internal systems.
    pub client_reference_id: Option<String>,
    /// The client secret of your Checkout Session.
    /// Applies to Checkout Sessions with `ui_mode: embedded` or `ui_mode: custom`.
    /// For `ui_mode: embedded`, the client secret is to be used when initializing Stripe.js embedded checkout.
    /// For `ui_mode: custom`, use the client secret with [initCheckout](https://stripe.com/docs/js/custom_checkout/init) on your front end.
    pub client_secret: Option<String>,
    /// Information about the customer collected within the Checkout Session.
    pub collected_information:
        Option<stripe_shared::PaymentPagesCheckoutSessionCollectedInformation>,
    /// Results of `consent_collection` for this session.
    pub consent: Option<stripe_shared::PaymentPagesCheckoutSessionConsent>,
    /// When set, provides configuration for the Checkout Session to gather active consent from customers.
    pub consent_collection: Option<stripe_shared::PaymentPagesCheckoutSessionConsentCollection>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// Currency conversion details for [Adaptive Pricing](https://docs.stripe.com/payments/checkout/adaptive-pricing) sessions created before 2025-03-31.
    pub currency_conversion: Option<stripe_shared::PaymentPagesCheckoutSessionCurrencyConversion>,
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    pub custom_fields: Vec<stripe_shared::PaymentPagesCheckoutSessionCustomFields>,
    pub custom_text: stripe_shared::PaymentPagesCheckoutSessionCustomText,
    /// The ID of the customer for this Session.
    /// For Checkout Sessions in `subscription` mode or Checkout Sessions with `customer_creation` set as `always` in `payment` mode, Checkout.
    /// will create a new customer object based on information provided
    /// during the payment flow unless an existing customer was provided when
    /// the Session was created.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    pub customer_creation: Option<CheckoutSessionCustomerCreation>,
    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
    /// Customer's address details are not present on Sessions in `setup` mode.
    pub customer_details: Option<stripe_shared::PaymentPagesCheckoutSessionCustomerDetails>,
    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file. To access information about the customer once the payment flow is
    /// complete, use the `customer` attribute.
    pub customer_email: Option<String>,
    /// List of coupons and promotion codes attached to the Checkout Session.
    pub discounts: Option<Vec<stripe_shared::PaymentPagesCheckoutSessionDiscount>>,
    /// A list of the types of payment methods (e.g., `card`) that should be excluded from this Checkout Session.
    /// This should only be used when payment methods for this Checkout Session are managed through the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub excluded_payment_method_types: Option<Vec<String>>,
    /// The timestamp at which the Checkout Session will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_shared::CheckoutSessionId,
    /// ID of the invoice created by the Checkout Session, if it exists.
    pub invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    /// Details on the state of invoice creation for the Checkout Session.
    pub invoice_creation: Option<stripe_shared::PaymentPagesCheckoutSessionInvoiceCreation>,
    /// The line items purchased by the customer.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The IETF language tag of the locale Checkout is displayed in.
    /// If blank or `auto`, the browser's locale is used.
    pub locale: Option<stripe_shared::CheckoutSessionLocale>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The mode of the Checkout Session.
    pub mode: stripe_shared::CheckoutSessionMode,
    pub name_collection: Option<stripe_shared::PaymentPagesCheckoutSessionNameCollection>,
    /// The optional items presented to the customer at checkout.
    pub optional_items: Option<Vec<stripe_shared::PaymentPagesCheckoutSessionOptionalItem>>,
    /// Where the user is coming from. This informs the optimizations that are applied to the session.
    pub origin_context: Option<stripe_shared::CheckoutSessionOriginContext>,
    /// The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    /// You can't confirm or cancel the PaymentIntent for a Checkout Session.
    /// To cancel, [expire the Checkout Session](https://stripe.com/docs/api/checkout/sessions/expire) instead.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// The ID of the Payment Link that created this Session.
    pub payment_link: Option<stripe_types::Expandable<stripe_shared::PaymentLink>>,
    /// Configure whether a Checkout Session should collect a payment method. Defaults to `always`.
    pub payment_method_collection: Option<CheckoutSessionPaymentMethodCollection>,
    /// Information about the payment method configuration used for this Checkout session if using dynamic payment methods.
    pub payment_method_configuration_details:
        Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>,
    /// Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    pub payment_method_options: Option<stripe_shared::CheckoutSessionPaymentMethodOptions>,
    /// A list of the types of payment methods (e.g. card) this Checkout
    /// Session is allowed to accept.
    pub payment_method_types: Vec<String>,
    /// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
    /// You can use this value to decide when to fulfill your customer's order.
    pub payment_status: CheckoutSessionPaymentStatus,
    /// This property is used to set up permissions for various actions (e.g., update) on the CheckoutSession object.
    ///
    /// For specific permissions, please refer to their dedicated subsections, such as `permissions.update_shipping_details`.
    pub permissions: Option<stripe_shared::PaymentPagesCheckoutSessionPermissions>,
    pub phone_number_collection:
        Option<stripe_shared::PaymentPagesCheckoutSessionPhoneNumberCollection>,
    pub presentment_details: Option<stripe_shared::PaymentFlowsPaymentIntentPresentmentDetails>,
    /// The ID of the original expired Checkout Session that triggered the recovery flow.
    pub recovered_from: Option<String>,
    /// This parameter applies to `ui_mode: embedded`.
    /// Learn more about the [redirect behavior](https://stripe.com/docs/payments/checkout/custom-success-page?payment-ui=embedded-form) of embedded sessions.
    /// Defaults to `always`.
    pub redirect_on_completion: Option<stripe_shared::CheckoutSessionRedirectOnCompletion>,
    /// Applies to Checkout Sessions with `ui_mode: embedded` or `ui_mode: custom`.
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    pub return_url: Option<String>,
    /// Controls saved payment method settings for the session.
    /// Only available in `payment` and `subscription` mode.
    pub saved_payment_method_options:
        Option<stripe_shared::PaymentPagesCheckoutSessionSavedPaymentMethodOptions>,
    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    /// You can't confirm or cancel the SetupIntent for a Checkout Session.
    /// To cancel, [expire the Checkout Session](https://stripe.com/docs/api/checkout/sessions/expire) instead.
    pub setup_intent: Option<stripe_types::Expandable<stripe_shared::SetupIntent>>,
    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    pub shipping_address_collection:
        Option<stripe_shared::PaymentPagesCheckoutSessionShippingAddressCollection>,
    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<stripe_shared::PaymentPagesCheckoutSessionShippingCost>,
    /// The shipping rate options applied to this Session.
    pub shipping_options: Vec<stripe_shared::PaymentPagesCheckoutSessionShippingOption>,
    /// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    pub status: Option<stripe_shared::CheckoutSessionStatus>,
    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button. `submit_type` can only be
    /// specified on Checkout Sessions in `payment` mode. If blank or `auto`, `pay` is used.
    pub submit_type: Option<stripe_shared::CheckoutSessionSubmitType>,
    /// The ID of the [Subscription](https://stripe.com/docs/api/subscriptions) for Checkout Sessions in `subscription` mode.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: Option<String>,
    pub tax_id_collection: Option<stripe_shared::PaymentPagesCheckoutSessionTaxIdCollection>,
    /// Tax and discount details for the computed total amount.
    pub total_details: Option<stripe_shared::PaymentPagesCheckoutSessionTotalDetails>,
    /// The UI mode of the Session. Defaults to `hosted`.
    pub ui_mode: Option<stripe_shared::CheckoutSessionUiMode>,
    /// The URL to the Checkout Session.
    /// Applies to Checkout Sessions with `ui_mode: hosted`.
    /// Redirect customers to this URL to take them to Checkout.
    /// If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain.
    /// Otherwise, it’ll use `checkout.stripe.com.`.
    /// This value is only present when the session is active.
    pub url: Option<String>,
    /// Wallet-specific configuration for this Checkout Session.
    pub wallet_options: Option<stripe_shared::CheckoutSessionWalletOptions>,
}
#[doc(hidden)]
pub struct CheckoutSessionBuilder {
    adaptive_pricing: Option<Option<stripe_shared::PaymentPagesCheckoutSessionAdaptivePricing>>,
    after_expiration: Option<Option<stripe_shared::PaymentPagesCheckoutSessionAfterExpiration>>,
    allow_promotion_codes: Option<Option<bool>>,
    amount_subtotal: Option<Option<i64>>,
    amount_total: Option<Option<i64>>,
    automatic_tax: Option<stripe_shared::PaymentPagesCheckoutSessionAutomaticTax>,
    billing_address_collection:
        Option<Option<stripe_shared::CheckoutSessionBillingAddressCollection>>,
    branding_settings: Option<Option<stripe_shared::PaymentPagesCheckoutSessionBrandingSettings>>,
    cancel_url: Option<Option<String>>,
    client_reference_id: Option<Option<String>>,
    client_secret: Option<Option<String>>,
    collected_information:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionCollectedInformation>>,
    consent: Option<Option<stripe_shared::PaymentPagesCheckoutSessionConsent>>,
    consent_collection: Option<Option<stripe_shared::PaymentPagesCheckoutSessionConsentCollection>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    currency_conversion:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionCurrencyConversion>>,
    custom_fields: Option<Vec<stripe_shared::PaymentPagesCheckoutSessionCustomFields>>,
    custom_text: Option<stripe_shared::PaymentPagesCheckoutSessionCustomText>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_creation: Option<Option<CheckoutSessionCustomerCreation>>,
    customer_details: Option<Option<stripe_shared::PaymentPagesCheckoutSessionCustomerDetails>>,
    customer_email: Option<Option<String>>,
    discounts: Option<Option<Vec<stripe_shared::PaymentPagesCheckoutSessionDiscount>>>,
    excluded_payment_method_types: Option<Option<Vec<String>>>,
    expires_at: Option<stripe_types::Timestamp>,
    id: Option<stripe_shared::CheckoutSessionId>,
    invoice: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    invoice_creation: Option<Option<stripe_shared::PaymentPagesCheckoutSessionInvoiceCreation>>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    livemode: Option<bool>,
    locale: Option<Option<stripe_shared::CheckoutSessionLocale>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    mode: Option<stripe_shared::CheckoutSessionMode>,
    name_collection: Option<Option<stripe_shared::PaymentPagesCheckoutSessionNameCollection>>,
    optional_items: Option<Option<Vec<stripe_shared::PaymentPagesCheckoutSessionOptionalItem>>>,
    origin_context: Option<Option<stripe_shared::CheckoutSessionOriginContext>>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_link: Option<Option<stripe_types::Expandable<stripe_shared::PaymentLink>>>,
    payment_method_collection: Option<Option<CheckoutSessionPaymentMethodCollection>>,
    payment_method_configuration_details:
        Option<Option<stripe_shared::PaymentMethodConfigBizPaymentMethodConfigurationDetails>>,
    payment_method_options: Option<Option<stripe_shared::CheckoutSessionPaymentMethodOptions>>,
    payment_method_types: Option<Vec<String>>,
    payment_status: Option<CheckoutSessionPaymentStatus>,
    permissions: Option<Option<stripe_shared::PaymentPagesCheckoutSessionPermissions>>,
    phone_number_collection:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionPhoneNumberCollection>>,
    presentment_details: Option<Option<stripe_shared::PaymentFlowsPaymentIntentPresentmentDetails>>,
    recovered_from: Option<Option<String>>,
    redirect_on_completion: Option<Option<stripe_shared::CheckoutSessionRedirectOnCompletion>>,
    return_url: Option<Option<String>>,
    saved_payment_method_options:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionSavedPaymentMethodOptions>>,
    setup_intent: Option<Option<stripe_types::Expandable<stripe_shared::SetupIntent>>>,
    shipping_address_collection:
        Option<Option<stripe_shared::PaymentPagesCheckoutSessionShippingAddressCollection>>,
    shipping_cost: Option<Option<stripe_shared::PaymentPagesCheckoutSessionShippingCost>>,
    shipping_options: Option<Vec<stripe_shared::PaymentPagesCheckoutSessionShippingOption>>,
    status: Option<Option<stripe_shared::CheckoutSessionStatus>>,
    submit_type: Option<Option<stripe_shared::CheckoutSessionSubmitType>>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    success_url: Option<Option<String>>,
    tax_id_collection: Option<Option<stripe_shared::PaymentPagesCheckoutSessionTaxIdCollection>>,
    total_details: Option<Option<stripe_shared::PaymentPagesCheckoutSessionTotalDetails>>,
    ui_mode: Option<Option<stripe_shared::CheckoutSessionUiMode>>,
    url: Option<Option<String>>,
    wallet_options: Option<Option<stripe_shared::CheckoutSessionWalletOptions>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutSession {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSession>,
        builder: CheckoutSessionBuilder,
    }

    impl Visitor for Place<CheckoutSession> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutSessionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutSessionBuilder {
        type Out = CheckoutSession;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "adaptive_pricing" => Deserialize::begin(&mut self.adaptive_pricing),
                "after_expiration" => Deserialize::begin(&mut self.after_expiration),
                "allow_promotion_codes" => Deserialize::begin(&mut self.allow_promotion_codes),
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "automatic_tax" => Deserialize::begin(&mut self.automatic_tax),
                "billing_address_collection" => {
                    Deserialize::begin(&mut self.billing_address_collection)
                }
                "branding_settings" => Deserialize::begin(&mut self.branding_settings),
                "cancel_url" => Deserialize::begin(&mut self.cancel_url),
                "client_reference_id" => Deserialize::begin(&mut self.client_reference_id),
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "collected_information" => Deserialize::begin(&mut self.collected_information),
                "consent" => Deserialize::begin(&mut self.consent),
                "consent_collection" => Deserialize::begin(&mut self.consent_collection),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "currency_conversion" => Deserialize::begin(&mut self.currency_conversion),
                "custom_fields" => Deserialize::begin(&mut self.custom_fields),
                "custom_text" => Deserialize::begin(&mut self.custom_text),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_creation" => Deserialize::begin(&mut self.customer_creation),
                "customer_details" => Deserialize::begin(&mut self.customer_details),
                "customer_email" => Deserialize::begin(&mut self.customer_email),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "excluded_payment_method_types" => {
                    Deserialize::begin(&mut self.excluded_payment_method_types)
                }
                "expires_at" => Deserialize::begin(&mut self.expires_at),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "invoice_creation" => Deserialize::begin(&mut self.invoice_creation),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "locale" => Deserialize::begin(&mut self.locale),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "mode" => Deserialize::begin(&mut self.mode),
                "name_collection" => Deserialize::begin(&mut self.name_collection),
                "optional_items" => Deserialize::begin(&mut self.optional_items),
                "origin_context" => Deserialize::begin(&mut self.origin_context),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_link" => Deserialize::begin(&mut self.payment_link),
                "payment_method_collection" => {
                    Deserialize::begin(&mut self.payment_method_collection)
                }
                "payment_method_configuration_details" => {
                    Deserialize::begin(&mut self.payment_method_configuration_details)
                }
                "payment_method_options" => Deserialize::begin(&mut self.payment_method_options),
                "payment_method_types" => Deserialize::begin(&mut self.payment_method_types),
                "payment_status" => Deserialize::begin(&mut self.payment_status),
                "permissions" => Deserialize::begin(&mut self.permissions),
                "phone_number_collection" => Deserialize::begin(&mut self.phone_number_collection),
                "presentment_details" => Deserialize::begin(&mut self.presentment_details),
                "recovered_from" => Deserialize::begin(&mut self.recovered_from),
                "redirect_on_completion" => Deserialize::begin(&mut self.redirect_on_completion),
                "return_url" => Deserialize::begin(&mut self.return_url),
                "saved_payment_method_options" => {
                    Deserialize::begin(&mut self.saved_payment_method_options)
                }
                "setup_intent" => Deserialize::begin(&mut self.setup_intent),
                "shipping_address_collection" => {
                    Deserialize::begin(&mut self.shipping_address_collection)
                }
                "shipping_cost" => Deserialize::begin(&mut self.shipping_cost),
                "shipping_options" => Deserialize::begin(&mut self.shipping_options),
                "status" => Deserialize::begin(&mut self.status),
                "submit_type" => Deserialize::begin(&mut self.submit_type),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "success_url" => Deserialize::begin(&mut self.success_url),
                "tax_id_collection" => Deserialize::begin(&mut self.tax_id_collection),
                "total_details" => Deserialize::begin(&mut self.total_details),
                "ui_mode" => Deserialize::begin(&mut self.ui_mode),
                "url" => Deserialize::begin(&mut self.url),
                "wallet_options" => Deserialize::begin(&mut self.wallet_options),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                adaptive_pricing: Deserialize::default(),
                after_expiration: Deserialize::default(),
                allow_promotion_codes: Deserialize::default(),
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_address_collection: Deserialize::default(),
                branding_settings: Deserialize::default(),
                cancel_url: Deserialize::default(),
                client_reference_id: Deserialize::default(),
                client_secret: Deserialize::default(),
                collected_information: Deserialize::default(),
                consent: Deserialize::default(),
                consent_collection: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                currency_conversion: Deserialize::default(),
                custom_fields: Deserialize::default(),
                custom_text: Deserialize::default(),
                customer: Deserialize::default(),
                customer_creation: Deserialize::default(),
                customer_details: Deserialize::default(),
                customer_email: Deserialize::default(),
                discounts: Deserialize::default(),
                excluded_payment_method_types: Deserialize::default(),
                expires_at: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                invoice_creation: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                locale: Deserialize::default(),
                metadata: Deserialize::default(),
                mode: Deserialize::default(),
                name_collection: Deserialize::default(),
                optional_items: Deserialize::default(),
                origin_context: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_link: Deserialize::default(),
                payment_method_collection: Deserialize::default(),
                payment_method_configuration_details: Deserialize::default(),
                payment_method_options: Deserialize::default(),
                payment_method_types: Deserialize::default(),
                payment_status: Deserialize::default(),
                permissions: Deserialize::default(),
                phone_number_collection: Deserialize::default(),
                presentment_details: Deserialize::default(),
                recovered_from: Deserialize::default(),
                redirect_on_completion: Deserialize::default(),
                return_url: Deserialize::default(),
                saved_payment_method_options: Deserialize::default(),
                setup_intent: Deserialize::default(),
                shipping_address_collection: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                shipping_options: Deserialize::default(),
                status: Deserialize::default(),
                submit_type: Deserialize::default(),
                subscription: Deserialize::default(),
                success_url: Deserialize::default(),
                tax_id_collection: Deserialize::default(),
                total_details: Deserialize::default(),
                ui_mode: Deserialize::default(),
                url: Deserialize::default(),
                wallet_options: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(adaptive_pricing),
                Some(after_expiration),
                Some(allow_promotion_codes),
                Some(amount_subtotal),
                Some(amount_total),
                Some(automatic_tax),
                Some(billing_address_collection),
                Some(branding_settings),
                Some(cancel_url),
                Some(client_reference_id),
                Some(client_secret),
                Some(collected_information),
                Some(consent),
                Some(consent_collection),
                Some(created),
                Some(currency),
                Some(currency_conversion),
                Some(custom_fields),
                Some(custom_text),
                Some(customer),
                Some(customer_creation),
                Some(customer_details),
                Some(customer_email),
                Some(discounts),
                Some(excluded_payment_method_types),
                Some(expires_at),
                Some(id),
                Some(invoice),
                Some(invoice_creation),
                Some(line_items),
                Some(livemode),
                Some(locale),
                Some(metadata),
                Some(mode),
                Some(name_collection),
                Some(optional_items),
                Some(origin_context),
                Some(payment_intent),
                Some(payment_link),
                Some(payment_method_collection),
                Some(payment_method_configuration_details),
                Some(payment_method_options),
                Some(payment_method_types),
                Some(payment_status),
                Some(permissions),
                Some(phone_number_collection),
                Some(presentment_details),
                Some(recovered_from),
                Some(redirect_on_completion),
                Some(return_url),
                Some(saved_payment_method_options),
                Some(setup_intent),
                Some(shipping_address_collection),
                Some(shipping_cost),
                Some(shipping_options),
                Some(status),
                Some(submit_type),
                Some(subscription),
                Some(success_url),
                Some(tax_id_collection),
                Some(total_details),
                Some(ui_mode),
                Some(url),
                Some(wallet_options),
            ) = (
                self.adaptive_pricing,
                self.after_expiration.take(),
                self.allow_promotion_codes,
                self.amount_subtotal,
                self.amount_total,
                self.automatic_tax.take(),
                self.billing_address_collection.take(),
                self.branding_settings.take(),
                self.cancel_url.take(),
                self.client_reference_id.take(),
                self.client_secret.take(),
                self.collected_information.take(),
                self.consent.take(),
                self.consent_collection.take(),
                self.created,
                self.currency.take(),
                self.currency_conversion.take(),
                self.custom_fields.take(),
                self.custom_text.take(),
                self.customer.take(),
                self.customer_creation.take(),
                self.customer_details.take(),
                self.customer_email.take(),
                self.discounts.take(),
                self.excluded_payment_method_types.take(),
                self.expires_at,
                self.id.take(),
                self.invoice.take(),
                self.invoice_creation.take(),
                self.line_items.take(),
                self.livemode,
                self.locale.take(),
                self.metadata.take(),
                self.mode.take(),
                self.name_collection,
                self.optional_items.take(),
                self.origin_context.take(),
                self.payment_intent.take(),
                self.payment_link.take(),
                self.payment_method_collection.take(),
                self.payment_method_configuration_details.take(),
                self.payment_method_options.take(),
                self.payment_method_types.take(),
                self.payment_status.take(),
                self.permissions.take(),
                self.phone_number_collection,
                self.presentment_details.take(),
                self.recovered_from.take(),
                self.redirect_on_completion.take(),
                self.return_url.take(),
                self.saved_payment_method_options.take(),
                self.setup_intent.take(),
                self.shipping_address_collection.take(),
                self.shipping_cost.take(),
                self.shipping_options.take(),
                self.status.take(),
                self.submit_type.take(),
                self.subscription.take(),
                self.success_url.take(),
                self.tax_id_collection.take(),
                self.total_details.take(),
                self.ui_mode.take(),
                self.url.take(),
                self.wallet_options.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                adaptive_pricing,
                after_expiration,
                allow_promotion_codes,
                amount_subtotal,
                amount_total,
                automatic_tax,
                billing_address_collection,
                branding_settings,
                cancel_url,
                client_reference_id,
                client_secret,
                collected_information,
                consent,
                consent_collection,
                created,
                currency,
                currency_conversion,
                custom_fields,
                custom_text,
                customer,
                customer_creation,
                customer_details,
                customer_email,
                discounts,
                excluded_payment_method_types,
                expires_at,
                id,
                invoice,
                invoice_creation,
                line_items,
                livemode,
                locale,
                metadata,
                mode,
                name_collection,
                optional_items,
                origin_context,
                payment_intent,
                payment_link,
                payment_method_collection,
                payment_method_configuration_details,
                payment_method_options,
                payment_method_types,
                payment_status,
                permissions,
                phone_number_collection,
                presentment_details,
                recovered_from,
                redirect_on_completion,
                return_url,
                saved_payment_method_options,
                setup_intent,
                shipping_address_collection,
                shipping_cost,
                shipping_options,
                status,
                submit_type,
                subscription,
                success_url,
                tax_id_collection,
                total_details,
                ui_mode,
                url,
                wallet_options,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for CheckoutSession {
        type Builder = CheckoutSessionBuilder;
    }

    impl FromValueOpt for CheckoutSession {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutSessionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "adaptive_pricing" => b.adaptive_pricing = FromValueOpt::from_value(v),
                    "after_expiration" => b.after_expiration = FromValueOpt::from_value(v),
                    "allow_promotion_codes" => {
                        b.allow_promotion_codes = FromValueOpt::from_value(v)
                    }
                    "amount_subtotal" => b.amount_subtotal = FromValueOpt::from_value(v),
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),
                    "automatic_tax" => b.automatic_tax = FromValueOpt::from_value(v),
                    "billing_address_collection" => {
                        b.billing_address_collection = FromValueOpt::from_value(v)
                    }
                    "branding_settings" => b.branding_settings = FromValueOpt::from_value(v),
                    "cancel_url" => b.cancel_url = FromValueOpt::from_value(v),
                    "client_reference_id" => b.client_reference_id = FromValueOpt::from_value(v),
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "collected_information" => {
                        b.collected_information = FromValueOpt::from_value(v)
                    }
                    "consent" => b.consent = FromValueOpt::from_value(v),
                    "consent_collection" => b.consent_collection = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "currency_conversion" => b.currency_conversion = FromValueOpt::from_value(v),
                    "custom_fields" => b.custom_fields = FromValueOpt::from_value(v),
                    "custom_text" => b.custom_text = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "customer_creation" => b.customer_creation = FromValueOpt::from_value(v),
                    "customer_details" => b.customer_details = FromValueOpt::from_value(v),
                    "customer_email" => b.customer_email = FromValueOpt::from_value(v),
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "excluded_payment_method_types" => {
                        b.excluded_payment_method_types = FromValueOpt::from_value(v)
                    }
                    "expires_at" => b.expires_at = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "invoice_creation" => b.invoice_creation = FromValueOpt::from_value(v),
                    "line_items" => b.line_items = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "locale" => b.locale = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "mode" => b.mode = FromValueOpt::from_value(v),
                    "name_collection" => b.name_collection = FromValueOpt::from_value(v),
                    "optional_items" => b.optional_items = FromValueOpt::from_value(v),
                    "origin_context" => b.origin_context = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "payment_link" => b.payment_link = FromValueOpt::from_value(v),
                    "payment_method_collection" => {
                        b.payment_method_collection = FromValueOpt::from_value(v)
                    }
                    "payment_method_configuration_details" => {
                        b.payment_method_configuration_details = FromValueOpt::from_value(v)
                    }
                    "payment_method_options" => {
                        b.payment_method_options = FromValueOpt::from_value(v)
                    }
                    "payment_method_types" => b.payment_method_types = FromValueOpt::from_value(v),
                    "payment_status" => b.payment_status = FromValueOpt::from_value(v),
                    "permissions" => b.permissions = FromValueOpt::from_value(v),
                    "phone_number_collection" => {
                        b.phone_number_collection = FromValueOpt::from_value(v)
                    }
                    "presentment_details" => b.presentment_details = FromValueOpt::from_value(v),
                    "recovered_from" => b.recovered_from = FromValueOpt::from_value(v),
                    "redirect_on_completion" => {
                        b.redirect_on_completion = FromValueOpt::from_value(v)
                    }
                    "return_url" => b.return_url = FromValueOpt::from_value(v),
                    "saved_payment_method_options" => {
                        b.saved_payment_method_options = FromValueOpt::from_value(v)
                    }
                    "setup_intent" => b.setup_intent = FromValueOpt::from_value(v),
                    "shipping_address_collection" => {
                        b.shipping_address_collection = FromValueOpt::from_value(v)
                    }
                    "shipping_cost" => b.shipping_cost = FromValueOpt::from_value(v),
                    "shipping_options" => b.shipping_options = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "submit_type" => b.submit_type = FromValueOpt::from_value(v),
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    "success_url" => b.success_url = FromValueOpt::from_value(v),
                    "tax_id_collection" => b.tax_id_collection = FromValueOpt::from_value(v),
                    "total_details" => b.total_details = FromValueOpt::from_value(v),
                    "ui_mode" => b.ui_mode = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
                    "wallet_options" => b.wallet_options = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutSession {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CheckoutSession", 65)?;
        s.serialize_field("adaptive_pricing", &self.adaptive_pricing)?;
        s.serialize_field("after_expiration", &self.after_expiration)?;
        s.serialize_field("allow_promotion_codes", &self.allow_promotion_codes)?;
        s.serialize_field("amount_subtotal", &self.amount_subtotal)?;
        s.serialize_field("amount_total", &self.amount_total)?;
        s.serialize_field("automatic_tax", &self.automatic_tax)?;
        s.serialize_field("billing_address_collection", &self.billing_address_collection)?;
        s.serialize_field("branding_settings", &self.branding_settings)?;
        s.serialize_field("cancel_url", &self.cancel_url)?;
        s.serialize_field("client_reference_id", &self.client_reference_id)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("collected_information", &self.collected_information)?;
        s.serialize_field("consent", &self.consent)?;
        s.serialize_field("consent_collection", &self.consent_collection)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("currency_conversion", &self.currency_conversion)?;
        s.serialize_field("custom_fields", &self.custom_fields)?;
        s.serialize_field("custom_text", &self.custom_text)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_creation", &self.customer_creation)?;
        s.serialize_field("customer_details", &self.customer_details)?;
        s.serialize_field("customer_email", &self.customer_email)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("excluded_payment_method_types", &self.excluded_payment_method_types)?;
        s.serialize_field("expires_at", &self.expires_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("invoice_creation", &self.invoice_creation)?;
        s.serialize_field("line_items", &self.line_items)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("locale", &self.locale)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("mode", &self.mode)?;
        s.serialize_field("name_collection", &self.name_collection)?;
        s.serialize_field("optional_items", &self.optional_items)?;
        s.serialize_field("origin_context", &self.origin_context)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("payment_link", &self.payment_link)?;
        s.serialize_field("payment_method_collection", &self.payment_method_collection)?;
        s.serialize_field(
            "payment_method_configuration_details",
            &self.payment_method_configuration_details,
        )?;
        s.serialize_field("payment_method_options", &self.payment_method_options)?;
        s.serialize_field("payment_method_types", &self.payment_method_types)?;
        s.serialize_field("payment_status", &self.payment_status)?;
        s.serialize_field("permissions", &self.permissions)?;
        s.serialize_field("phone_number_collection", &self.phone_number_collection)?;
        s.serialize_field("presentment_details", &self.presentment_details)?;
        s.serialize_field("recovered_from", &self.recovered_from)?;
        s.serialize_field("redirect_on_completion", &self.redirect_on_completion)?;
        s.serialize_field("return_url", &self.return_url)?;
        s.serialize_field("saved_payment_method_options", &self.saved_payment_method_options)?;
        s.serialize_field("setup_intent", &self.setup_intent)?;
        s.serialize_field("shipping_address_collection", &self.shipping_address_collection)?;
        s.serialize_field("shipping_cost", &self.shipping_cost)?;
        s.serialize_field("shipping_options", &self.shipping_options)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("submit_type", &self.submit_type)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("success_url", &self.success_url)?;
        s.serialize_field("tax_id_collection", &self.tax_id_collection)?;
        s.serialize_field("total_details", &self.total_details)?;
        s.serialize_field("ui_mode", &self.ui_mode)?;
        s.serialize_field("url", &self.url)?;
        s.serialize_field("wallet_options", &self.wallet_options)?;

        s.serialize_field("object", "checkout.session")?;
        s.end()
    }
}
/// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionCustomerCreation {
    Always,
    IfRequired,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionCustomerCreation {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionCustomerCreation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutSessionCustomerCreation"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutSessionCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionCustomerCreation {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionCustomerCreation> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionCustomerCreation::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionCustomerCreation);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Configure whether a Checkout Session should collect a payment method. Defaults to `always`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionPaymentMethodCollection {
    Always,
    IfRequired,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionPaymentMethodCollection {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionPaymentMethodCollection {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutSessionPaymentMethodCollection"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutSessionPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionPaymentMethodCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionPaymentMethodCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionPaymentMethodCollection::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionPaymentMethodCollection);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
/// You can use this value to decide when to fulfill your customer's order.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionPaymentStatus {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionPaymentStatus::*;
        match self {
            NoPaymentRequired => "no_payment_required",
            Paid => "paid",
            Unpaid => "unpaid",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionPaymentStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionPaymentStatus::*;
        match s {
            "no_payment_required" => Ok(NoPaymentRequired),
            "paid" => Ok(Paid),
            "unpaid" => Ok(Unpaid),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutSessionPaymentStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutSessionPaymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionPaymentStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionPaymentStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionPaymentStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionPaymentStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionPaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for CheckoutSession {
    type Id = stripe_shared::CheckoutSessionId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(CheckoutSessionId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionBillingAddressCollection {
    Auto,
    Required,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionBillingAddressCollection {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionBillingAddressCollection {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutSessionBillingAddressCollection"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionBillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionBillingAddressCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionBillingAddressCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionBillingAddressCollection::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionBillingAddressCollection);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionBillingAddressCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnMinusGb,
    Es,
    EsMinus419,
    Et,
    Fi,
    Fil,
    Fr,
    FrMinusCa,
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
    PtMinusBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhMinusHk,
    ZhMinusTw,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionLocale {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionLocale::*;
        match self {
            Auto => "auto",
            Bg => "bg",
            Cs => "cs",
            Da => "da",
            De => "de",
            El => "el",
            En => "en",
            EnMinusGb => "en-GB",
            Es => "es",
            EsMinus419 => "es-419",
            Et => "et",
            Fi => "fi",
            Fil => "fil",
            Fr => "fr",
            FrMinusCa => "fr-CA",
            Hr => "hr",
            Hu => "hu",
            Id => "id",
            It => "it",
            Ja => "ja",
            Ko => "ko",
            Lt => "lt",
            Lv => "lv",
            Ms => "ms",
            Mt => "mt",
            Nb => "nb",
            Nl => "nl",
            Pl => "pl",
            Pt => "pt",
            PtMinusBr => "pt-BR",
            Ro => "ro",
            Ru => "ru",
            Sk => "sk",
            Sl => "sl",
            Sv => "sv",
            Th => "th",
            Tr => "tr",
            Vi => "vi",
            Zh => "zh",
            ZhMinusHk => "zh-HK",
            ZhMinusTw => "zh-TW",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionLocale {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionLocale::*;
        match s {
            "auto" => Ok(Auto),
            "bg" => Ok(Bg),
            "cs" => Ok(Cs),
            "da" => Ok(Da),
            "de" => Ok(De),
            "el" => Ok(El),
            "en" => Ok(En),
            "en-GB" => Ok(EnMinusGb),
            "es" => Ok(Es),
            "es-419" => Ok(EsMinus419),
            "et" => Ok(Et),
            "fi" => Ok(Fi),
            "fil" => Ok(Fil),
            "fr" => Ok(Fr),
            "fr-CA" => Ok(FrMinusCa),
            "hr" => Ok(Hr),
            "hu" => Ok(Hu),
            "id" => Ok(Id),
            "it" => Ok(It),
            "ja" => Ok(Ja),
            "ko" => Ok(Ko),
            "lt" => Ok(Lt),
            "lv" => Ok(Lv),
            "ms" => Ok(Ms),
            "mt" => Ok(Mt),
            "nb" => Ok(Nb),
            "nl" => Ok(Nl),
            "pl" => Ok(Pl),
            "pt" => Ok(Pt),
            "pt-BR" => Ok(PtMinusBr),
            "ro" => Ok(Ro),
            "ru" => Ok(Ru),
            "sk" => Ok(Sk),
            "sl" => Ok(Sl),
            "sv" => Ok(Sv),
            "th" => Ok(Th),
            "tr" => Ok(Tr),
            "vi" => Ok(Vi),
            "zh" => Ok(Zh),
            "zh-HK" => Ok(ZhMinusHk),
            "zh-TW" => Ok(ZhMinusTw),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CheckoutSessionLocale");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionLocale {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionLocale> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionLocale::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionLocale);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionMode {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionMode::*;
        match self {
            Payment => "payment",
            Setup => "setup",
            Subscription => "subscription",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionMode::*;
        match s {
            "payment" => Ok(Payment),
            "setup" => Ok(Setup),
            "subscription" => Ok(Subscription),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CheckoutSessionMode");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionMode::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionMode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionOriginContext {
    MobileApp,
    Web,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionOriginContext {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionOriginContext::*;
        match self {
            MobileApp => "mobile_app",
            Web => "web",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionOriginContext {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionOriginContext::*;
        match s {
            "mobile_app" => Ok(MobileApp),
            "web" => Ok(Web),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutSessionOriginContext"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionOriginContext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionOriginContext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionOriginContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionOriginContext {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionOriginContext> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionOriginContext::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionOriginContext);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionOriginContext {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionRedirectOnCompletion {
    Always,
    IfRequired,
    Never,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionRedirectOnCompletion {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionRedirectOnCompletion::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
            Never => "never",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionRedirectOnCompletion {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionRedirectOnCompletion::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            "never" => Ok(Never),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutSessionRedirectOnCompletion"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionRedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionRedirectOnCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionRedirectOnCompletion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionRedirectOnCompletion {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionRedirectOnCompletion> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionRedirectOnCompletion::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionRedirectOnCompletion);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionRedirectOnCompletion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionStatus {
    Complete,
    Expired,
    Open,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionStatus {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionStatus::*;
        match self {
            Complete => "complete",
            Expired => "expired",
            Open => "open",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionStatus::*;
        match s {
            "complete" => Ok(Complete),
            "expired" => Ok(Expired),
            "open" => Ok(Open),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CheckoutSessionStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
    Subscribe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionSubmitType {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionSubmitType::*;
        match self {
            Auto => "auto",
            Book => "book",
            Donate => "donate",
            Pay => "pay",
            Subscribe => "subscribe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionSubmitType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionSubmitType::*;
        match s {
            "auto" => Ok(Auto),
            "book" => Ok(Book),
            "donate" => Ok(Donate),
            "pay" => Ok(Pay),
            "subscribe" => Ok(Subscribe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CheckoutSessionSubmitType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionSubmitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionSubmitType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionSubmitType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionSubmitType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionSubmitType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionSubmitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutSessionUiMode {
    Custom,
    Embedded,
    Hosted,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutSessionUiMode {
    pub fn as_str(&self) -> &str {
        use CheckoutSessionUiMode::*;
        match self {
            Custom => "custom",
            Embedded => "embedded",
            Hosted => "hosted",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutSessionUiMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutSessionUiMode::*;
        match s {
            "custom" => Ok(Custom),
            "embedded" => Ok(Embedded),
            "hosted" => Ok(Hosted),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CheckoutSessionUiMode");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutSessionUiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutSessionUiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CheckoutSessionUiMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutSessionUiMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutSessionUiMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CheckoutSessionUiMode::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutSessionUiMode);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutSessionUiMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
