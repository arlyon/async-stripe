// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, OrderId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable, Timestamp};
use crate::resources::{
    Account, Address, Application, CheckoutSessionItem, Currency, Customer, Discount,
    PaymentIntent, PaymentIntentPaymentMethodOptionsAcssDebit,
    PaymentIntentPaymentMethodOptionsLink, PaymentIntentPaymentMethodOptionsSepaDebit,
    PaymentMethodOptionsAlipay, PaymentMethodOptionsBancontact,
    PaymentMethodOptionsCustomerBalance, PaymentMethodOptionsIdeal, PaymentMethodOptionsKlarna,
    PaymentMethodOptionsOxxo, PaymentMethodOptionsP24, PaymentMethodOptionsSofort,
    PaymentMethodOptionsWechatPay, ShippingRate, TaxRate,
};

/// The resource representing a Stripe "OrdersV2ResourceOrder".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Order {
    /// Unique identifier for the object.
    pub id: OrderId,

    /// Order cost before any discounts or taxes are applied.
    ///
    /// A positive integer representing the subtotal of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    pub amount_subtotal: i64,

    /// Total order cost after discounts and taxes are applied.
    ///
    /// A positive integer representing the cost of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// To submit an order, the total must be either 0 or at least $0.50 USD or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    pub amount_total: i64,

    /// ID of the Connect application that created the Order, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<OrdersV2ResourceAutomaticTax>,

    /// Customer billing details associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<OrdersV2ResourceBillingDetails>,

    /// The client secret of this Order.
    ///
    /// Used for client-side retrieval using a publishable key.
    /// The client secret can be used to complete a payment for an Order from your frontend.
    /// It should not be stored, logged, embedded in URLs, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    /// Refer to our docs for [creating and processing an order](https://stripe.com/docs/orders-beta/create-and-process) to learn about how client_secret should be handled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The customer which this orders belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The discounts applied to the order.
    ///
    /// Use `expand[]=discounts` to expand each discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<Expandable<Discount>>>,

    /// A recent IP address of the purchaser used for tax reporting and tax location inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    /// There is a maximum of 100 line items.
    #[serde(default)]
    pub line_items: List<CheckoutSessionItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    pub payment: OrdersV2ResourcePayment,

    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<OrdersV2ResourceShippingCost>,

    /// Customer shipping information associated with the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<OrdersV2ResourceShippingDetails>,

    /// The overall status of the order.
    pub status: OrderStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<OrdersV2ResourceTaxDetails>,

    pub total_details: OrdersV2ResourceTotalDetails,
}

impl Order {
    /// Returns a list of your orders.
    ///
    /// The orders are returned sorted by creation date, with the most recently created orders appearing first.
    pub fn list(client: &Client, params: &ListOrders<'_>) -> Response<List<Order>> {
        client.get_query("/orders", &params)
    }

    /// Creates a new `open` order object.
    pub fn create(client: &Client, params: CreateOrder<'_>) -> Response<Order> {
        client.post_form("/orders", &params)
    }

    /// Retrieves the details of an existing order.
    ///
    /// Supply the unique order ID from either an order creation request or the order list, and Stripe will return the corresponding order information.
    pub fn retrieve(client: &Client, id: &OrderId, expand: &[&str]) -> Response<Order> {
        client.get_query(&format!("/orders/{}", id), &Expand { expand })
    }

    /// Updates the specific order by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(client: &Client, id: &OrderId, params: UpdateOrder<'_>) -> Response<Order> {
        client.post_form(&format!("/orders/{}", id), &params)
    }
}

impl Object for Order {
    type Id = OrderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "order"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceAutomaticTax {
    /// Whether Stripe automatically computes tax on this Order.
    pub enabled: bool,

    /// The status of the most recent automated tax calculation for this Order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrdersV2ResourceAutomaticTaxStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceBillingDetails {
    /// Billing address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Email address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Billing phone number for the order (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourcePayment {
    /// ID of the payment intent associated with this order.
    ///
    /// Null when the order is `open`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// Settings describing how the order should configure generated PaymentIntents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<OrdersV2ResourcePaymentSettings>,

    /// The status of the underlying payment associated with this order, if any.
    ///
    /// Null when the order is `open`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrdersV2ResourcePaymentStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourcePaymentSettings {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Indicates whether order has been opted into using [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods) to manage payment method types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<OrdersV2ResourceAutomaticPaymentMethods>,

    /// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<OrdersV2ResourcePaymentMethodOptions>,

    /// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<OrdersV2ResourcePaymentSettingsPaymentMethodTypes>>,

    /// The URL to redirect the customer to after they authenticate their payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// For non-card charges, you can use this value as the complete description that appears on your customers' statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// Provides configuration for completing a transfer for the order after it is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<OrdersV2ResourceTransferData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceAutomaticPaymentMethods {
    /// Whether this Order has been opted into managing payment method types via the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourcePaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PaymentIntentPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<OrdersPaymentMethodOptionsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PaymentMethodOptionsAlipay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PaymentMethodOptionsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<OrdersV2ResourceCardPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PaymentMethodOptionsCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PaymentMethodOptionsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PaymentMethodOptionsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PaymentIntentPaymentMethodOptionsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PaymentMethodOptionsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PaymentMethodOptionsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<PaymentMethodOptionsPaypal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PaymentIntentPaymentMethodOptionsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PaymentMethodOptionsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersPaymentMethodOptionsAfterpayClearpay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod>,

    /// Order identifier shown to the user in Afterpay's online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceCardPaymentMethodOptions {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod,

    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceShippingCost {
    /// Total shipping cost before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total tax amount applied due to shipping costs.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,

    /// Total shipping cost after discounts and taxes are applied.
    pub amount_total: i64,

    /// The ID of the ShippingRate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
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
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceShippingDetails {
    /// Recipient shipping address.
    ///
    /// Required if the order includes products that are shippable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// Recipient name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceTaxDetails {
    /// Describes the purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    pub tax_exempt: OrdersV2ResourceTaxDetailsTaxExempt,

    /// The purchaser's tax IDs to be used in calculation of tax for this Order.
    pub tax_ids: Vec<OrdersV2ResourceTaxDetailsResourceTaxId>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceTaxDetailsResourceTaxId {
    /// The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `eu_oss_vat`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: OrdersV2ResourceTaxDetailsResourceTaxIdType,

    /// The value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,

    /// This is the sum of all the shipping amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_shipping: Option<i64>,

    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<OrdersV2ResourceTotalDetailsApiResourceBreakdown>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OrdersV2ResourceTotalDetailsApiResourceBreakdown {
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
pub struct OrdersV2ResourceTransferData {
    /// The amount that will be transferred automatically when the order is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of the Connected account receiving the transfer.
    pub destination: Expandable<Account>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsPaypal {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<PaymentMethodOptionsPaypalCaptureMethod>,

    /// Preferred locale of the PayPal checkout page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
}

/// The parameters for `Order::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateOrder<'a> {
    /// Settings for automatic tax calculation for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateOrderAutomaticTax>,

    /// Billing details for the customer.
    ///
    /// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateOrderBillingDetails>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The customer associated with this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// The coupons, promotion codes, and/or discounts to apply to the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CreateOrderDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The IP address of the purchaser for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,

    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    pub line_items: Vec<CreateOrderLineItems>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Payment information associated with the order, including payment settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<CreateOrderPayment>,

    /// Settings for the customer cost of shipping for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateOrderShippingCost>,

    /// Shipping details for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<CreateOrderShippingDetails>,

    /// Additional tax details about the purchaser to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<CreateOrderTaxDetails>,
}

impl<'a> CreateOrder<'a> {
    pub fn new(currency: Currency, line_items: Vec<CreateOrderLineItems>) -> Self {
        CreateOrder {
            automatic_tax: Default::default(),
            billing_details: Default::default(),
            currency,
            customer: Default::default(),
            description: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            ip_address: Default::default(),
            line_items,
            metadata: Default::default(),
            payment: Default::default(),
            shipping_cost: Default::default(),
            shipping_details: Default::default(),
            tax_details: Default::default(),
        }
    }
}

/// The parameters for `Order::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListOrders<'a> {
    /// Only return orders for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<OrderId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<OrderId>,
}

impl<'a> ListOrders<'a> {
    pub fn new() -> Self {
        ListOrders {
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListOrders<'_> {
    type O = Order;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `Order::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateOrder<'a> {
    /// Settings for automatic tax calculation for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateOrderAutomaticTax>,

    /// Billing details for the customer.
    ///
    /// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<UpdateOrderBillingDetails>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The customer associated with this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// The coupons, promotion codes, and/or discounts to apply to the order.
    ///
    /// Pass the empty string `""` to unset this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<UpdateOrderDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The IP address of the purchaser for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,

    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<UpdateOrderLineItems>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Payment information associated with the order, including payment settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<UpdateOrderPayment>,

    /// Settings for the customer cost of shipping for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<UpdateOrderShippingCost>,

    /// Shipping details for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<UpdateOrderShippingDetails>,

    /// Additional tax details about the purchaser to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<UpdateOrderTaxDetails>,
}

impl<'a> UpdateOrder<'a> {
    pub fn new() -> Self {
        UpdateOrder {
            automatic_tax: Default::default(),
            billing_details: Default::default(),
            currency: Default::default(),
            customer: Default::default(),
            description: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            ip_address: Default::default(),
            line_items: Default::default(),
            metadata: Default::default(),
            payment: Default::default(),
            shipping_cost: Default::default(),
            shipping_details: Default::default(),
            tax_details: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderBillingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateOrderBillingDetailsAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderDiscounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderLineItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CreateOrderLineItemsDiscounts>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateOrderLineItemsPriceData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateOrderLineItemsProductData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPayment {
    pub settings: CreateOrderPaymentSettings,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateOrderShippingCostShippingRateData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingDetails {
    pub address: CreateOrderShippingDetailsAddress,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderTaxDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CreateOrderTaxDetailsTaxExempt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<CreateOrderTaxDetailsTaxIds>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderBillingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateOrderBillingDetailsAddress>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderDiscounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderLineItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<UpdateOrderLineItemsDiscounts>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateOrderLineItemsPriceData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<UpdateOrderLineItemsProductData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPayment {
    pub settings: UpdateOrderPaymentSettings,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<UpdateOrderShippingCostShippingRateData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingDetails {
    pub address: UpdateOrderShippingDetailsAddress,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderTaxDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<UpdateOrderTaxDetailsTaxExempt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<UpdateOrderTaxDetailsTaxIds>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderBillingDetailsAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderLineItemsDiscounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderLineItemsPriceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateOrderLineItemsPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderLineItemsProductData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<CreateOrderLineItemsProductDataPackageDimensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateOrderPaymentSettingsPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<CreateOrderPaymentSettingsPaymentMethodTypes>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateOrderPaymentSettingsTransferData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingCostShippingRateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateOrderShippingCostShippingRateDataDeliveryEstimate>,

    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateOrderShippingCostShippingRateDataFixedAmount>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateOrderShippingCostShippingRateDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateOrderShippingCostShippingRateDataType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingDetailsAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderTaxDetailsTaxIds {
    #[serde(rename = "type")]
    pub type_: CreateOrderTaxDetailsTaxIdsType,

    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderBillingDetailsAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderLineItemsDiscounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderLineItemsPriceData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateOrderLineItemsPriceDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderLineItemsProductData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<UpdateOrderLineItemsProductDataPackageDimensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateOrderPaymentSettingsPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<UpdateOrderPaymentSettingsPaymentMethodTypes>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateOrderPaymentSettingsTransferData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingCostShippingRateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<UpdateOrderShippingCostShippingRateDataDeliveryEstimate>,

    pub display_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<UpdateOrderShippingCostShippingRateDataFixedAmount>,

    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateOrderShippingCostShippingRateDataTaxBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateOrderShippingCostShippingRateDataType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingDetailsAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderTaxDetailsTaxIds {
    #[serde(rename = "type")]
    pub type_: UpdateOrderTaxDetailsTaxIdsType,

    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderLineItemsProductDataPackageDimensions {
    pub height: f64,

    pub length: f64,

    pub weight: f64,

    pub width: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateOrderPaymentSettingsPaymentMethodOptionsAlipay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateOrderPaymentSettingsPaymentMethodOptionsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateOrderPaymentSettingsPaymentMethodOptionsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateOrderPaymentSettingsPaymentMethodOptionsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateOrderPaymentSettingsPaymentMethodOptionsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateOrderPaymentSettingsPaymentMethodOptionsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateOrderPaymentSettingsPaymentMethodOptionsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateOrderPaymentSettingsPaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingCostShippingRateDataDeliveryEstimate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimum>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingCostShippingRateDataFixedAmount {
    pub amount: i64,

    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderLineItemsProductDataPackageDimensions {
    pub height: f64,

    pub length: f64,

    pub weight: f64,

    pub width: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAlipay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataDeliveryEstimate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimum>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataFixedAmount {
    pub amount: i64,

    pub currency: Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAlipay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsIdeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsOxxo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsSofort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsWechatPay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    pub client: CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub unit: CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit,

    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub unit: CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit,

    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAlipay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsIdeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsOxxo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsP24 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsSofort {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    pub client: UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub unit: UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit,

    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub unit: UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit,

    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule,
    >,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,

    #[serde(rename = "type")]
    pub type_: CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule,
    >,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<Vec<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,

    #[serde(rename = "type")]
    pub type_: UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    pub country: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    pub country: String,
}

/// An enum representing the possible values of an `CreateOrderLineItemsPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateOrderLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderLineItemsPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateOrderLineItemsPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateOrderLineItemsPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateOrderLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderLineItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str>
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod::Automatic => "automatic",
            CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsAlipay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsBancontact`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsCard`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    Automatic,
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod::Automatic => {
                "automatic"
            }
            CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsCard`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer`'s `requested_address_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl
    CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Iban => "iban",
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Sepa => "sepa",
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::SortCode => "sort_code",
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Spei => "spei",
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn default() -> Self {
        Self::Iban
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer => "eu_bank_transfer",
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer => "gb_bank_transfer",
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer => "jp_bank_transfer",
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    fn default() -> Self {
        Self::BankTransfer
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage::None => {
                "none"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsIdeal`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsKlarna`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsKlarna`'s `preferred_locale` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    #[serde(rename = "da-DK")]
    DaDk,
    #[serde(rename = "de-AT")]
    DeAt,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "en-AT")]
    EnAt,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-BE")]
    EnBe,
    #[serde(rename = "en-CA")]
    EnCa,
    #[serde(rename = "en-DE")]
    EnDe,
    #[serde(rename = "en-DK")]
    EnDk,
    #[serde(rename = "en-ES")]
    EnEs,
    #[serde(rename = "en-FI")]
    EnFi,
    #[serde(rename = "en-FR")]
    EnFr,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-IE")]
    EnIe,
    #[serde(rename = "en-IT")]
    EnIt,
    #[serde(rename = "en-NL")]
    EnNl,
    #[serde(rename = "en-NO")]
    EnNo,
    #[serde(rename = "en-NZ")]
    EnNz,
    #[serde(rename = "en-SE")]
    EnSe,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-US")]
    EsUs,
    #[serde(rename = "fi-FI")]
    FiFi,
    #[serde(rename = "fr-BE")]
    FrBe,
    #[serde(rename = "fr-CA")]
    FrCa,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "nb-NO")]
    NbNo,
    #[serde(rename = "nl-BE")]
    NlBe,
    #[serde(rename = "nl-NL")]
    NlNl,
    #[serde(rename = "sv-FI")]
    SvFi,
    #[serde(rename = "sv-SE")]
    SvSe,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::DaDk => "da-DK",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::DeAt => "de-AT",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::DeDe => "de-DE",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnAt => "en-AT",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnAu => "en-AU",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnBe => "en-BE",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnCa => "en-CA",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnDe => "en-DE",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnDk => "en-DK",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnEs => "en-ES",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnFi => "en-FI",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnFr => "en-FR",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnGb => "en-GB",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnIe => "en-IE",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnIt => "en-IT",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnNl => "en-NL",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnNo => "en-NO",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnNz => "en-NZ",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnSe => "en-SE",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnUs => "en-US",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EsEs => "es-ES",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EsUs => "es-US",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FiFi => "fi-FI",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FrBe => "fr-BE",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FrCa => "fr-CA",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FrFr => "fr-FR",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::ItIt => "it-IT",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::NbNo => "nb-NO",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::NlBe => "nl-BE",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::NlNl => "nl-NL",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::SvFi => "sv-FI",
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::SvSe => "sv-SE",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn default() -> Self {
        Self::DaDk
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsKlarna`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsLink`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsLink`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsOxxo`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsP24`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage::OffSession => {
                "off_session"
            }
            CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsSofort`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::De => "de",
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::En => "en",
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Es => "es",
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Fr => "fr",
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::It => "it",
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Nl => "nl",
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Pl => "pl",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsSofort`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage::None => "none",
            CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsWechatPay`'s `client` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient::Android => "android",
            CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient::Ios => "ios",
            CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient::Web => "web",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn default() -> Self {
        Self::Android
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettingsPaymentMethodOptionsWechatPay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreateOrderPaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderPaymentSettingsPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Link,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl CreateOrderPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderPaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            CreateOrderPaymentSettingsPaymentMethodTypes::AfterpayClearpay => "afterpay_clearpay",
            CreateOrderPaymentSettingsPaymentMethodTypes::Alipay => "alipay",
            CreateOrderPaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            CreateOrderPaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreateOrderPaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            CreateOrderPaymentSettingsPaymentMethodTypes::Card => "card",
            CreateOrderPaymentSettingsPaymentMethodTypes::CustomerBalance => "customer_balance",
            CreateOrderPaymentSettingsPaymentMethodTypes::Eps => "eps",
            CreateOrderPaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            CreateOrderPaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            CreateOrderPaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            CreateOrderPaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            CreateOrderPaymentSettingsPaymentMethodTypes::Klarna => "klarna",
            CreateOrderPaymentSettingsPaymentMethodTypes::Link => "link",
            CreateOrderPaymentSettingsPaymentMethodTypes::Oxxo => "oxxo",
            CreateOrderPaymentSettingsPaymentMethodTypes::P24 => "p24",
            CreateOrderPaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateOrderPaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            CreateOrderPaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreateOrderPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay => {
                "business_day"
            }
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Day => "day",
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Month => "month",
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Week => "week",
        }
    }
}

impl AsRef<str> for CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::BusinessDay => {
                "business_day"
            }
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Day => "day",
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Hour => "hour",
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Month => "month",
            CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Week => "week",
        }
    }
}

impl AsRef<str> for CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateOrderShippingCostShippingRateData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateOrderShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderShippingCostShippingRateDataTaxBehavior::Exclusive => "exclusive",
            CreateOrderShippingCostShippingRateDataTaxBehavior::Inclusive => "inclusive",
            CreateOrderShippingCostShippingRateDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateOrderShippingCostShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderShippingCostShippingRateDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateOrderShippingCostShippingRateData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderShippingCostShippingRateDataType {
    FixedAmount,
}

impl CreateOrderShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderShippingCostShippingRateDataType::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for CreateOrderShippingCostShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderShippingCostShippingRateDataType {
    fn default() -> Self {
        Self::FixedAmount
    }
}

/// An enum representing the possible values of an `CreateOrderTaxDetails`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl CreateOrderTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderTaxDetailsTaxExempt::Exempt => "exempt",
            CreateOrderTaxDetailsTaxExempt::None => "none",
            CreateOrderTaxDetailsTaxExempt::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for CreateOrderTaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderTaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderTaxDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

/// An enum representing the possible values of an `CreateOrderTaxDetailsTaxIds`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateOrderTaxDetailsTaxIdsType {
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
    UsEin,
    ZaVat,
}

impl CreateOrderTaxDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateOrderTaxDetailsTaxIdsType::AeTrn => "ae_trn",
            CreateOrderTaxDetailsTaxIdsType::AuAbn => "au_abn",
            CreateOrderTaxDetailsTaxIdsType::AuArn => "au_arn",
            CreateOrderTaxDetailsTaxIdsType::BgUic => "bg_uic",
            CreateOrderTaxDetailsTaxIdsType::BrCnpj => "br_cnpj",
            CreateOrderTaxDetailsTaxIdsType::BrCpf => "br_cpf",
            CreateOrderTaxDetailsTaxIdsType::CaBn => "ca_bn",
            CreateOrderTaxDetailsTaxIdsType::CaGstHst => "ca_gst_hst",
            CreateOrderTaxDetailsTaxIdsType::CaPstBc => "ca_pst_bc",
            CreateOrderTaxDetailsTaxIdsType::CaPstMb => "ca_pst_mb",
            CreateOrderTaxDetailsTaxIdsType::CaPstSk => "ca_pst_sk",
            CreateOrderTaxDetailsTaxIdsType::CaQst => "ca_qst",
            CreateOrderTaxDetailsTaxIdsType::ChVat => "ch_vat",
            CreateOrderTaxDetailsTaxIdsType::ClTin => "cl_tin",
            CreateOrderTaxDetailsTaxIdsType::EsCif => "es_cif",
            CreateOrderTaxDetailsTaxIdsType::EuOssVat => "eu_oss_vat",
            CreateOrderTaxDetailsTaxIdsType::EuVat => "eu_vat",
            CreateOrderTaxDetailsTaxIdsType::GbVat => "gb_vat",
            CreateOrderTaxDetailsTaxIdsType::GeVat => "ge_vat",
            CreateOrderTaxDetailsTaxIdsType::HkBr => "hk_br",
            CreateOrderTaxDetailsTaxIdsType::HuTin => "hu_tin",
            CreateOrderTaxDetailsTaxIdsType::IdNpwp => "id_npwp",
            CreateOrderTaxDetailsTaxIdsType::IlVat => "il_vat",
            CreateOrderTaxDetailsTaxIdsType::InGst => "in_gst",
            CreateOrderTaxDetailsTaxIdsType::IsVat => "is_vat",
            CreateOrderTaxDetailsTaxIdsType::JpCn => "jp_cn",
            CreateOrderTaxDetailsTaxIdsType::JpRn => "jp_rn",
            CreateOrderTaxDetailsTaxIdsType::KrBrn => "kr_brn",
            CreateOrderTaxDetailsTaxIdsType::LiUid => "li_uid",
            CreateOrderTaxDetailsTaxIdsType::MxRfc => "mx_rfc",
            CreateOrderTaxDetailsTaxIdsType::MyFrp => "my_frp",
            CreateOrderTaxDetailsTaxIdsType::MyItn => "my_itn",
            CreateOrderTaxDetailsTaxIdsType::MySst => "my_sst",
            CreateOrderTaxDetailsTaxIdsType::NoVat => "no_vat",
            CreateOrderTaxDetailsTaxIdsType::NzGst => "nz_gst",
            CreateOrderTaxDetailsTaxIdsType::RuInn => "ru_inn",
            CreateOrderTaxDetailsTaxIdsType::RuKpp => "ru_kpp",
            CreateOrderTaxDetailsTaxIdsType::SaVat => "sa_vat",
            CreateOrderTaxDetailsTaxIdsType::SgGst => "sg_gst",
            CreateOrderTaxDetailsTaxIdsType::SgUen => "sg_uen",
            CreateOrderTaxDetailsTaxIdsType::SiTin => "si_tin",
            CreateOrderTaxDetailsTaxIdsType::ThVat => "th_vat",
            CreateOrderTaxDetailsTaxIdsType::TwVat => "tw_vat",
            CreateOrderTaxDetailsTaxIdsType::UaVat => "ua_vat",
            CreateOrderTaxDetailsTaxIdsType::UsEin => "us_ein",
            CreateOrderTaxDetailsTaxIdsType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for CreateOrderTaxDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateOrderTaxDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateOrderTaxDetailsTaxIdsType {
    fn default() -> Self {
        Self::AeTrn
    }
}

/// An enum representing the possible values of an `Order`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Canceled,
    Complete,
    Open,
    Processing,
    Submitted,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            OrderStatus::Canceled => "canceled",
            OrderStatus::Complete => "complete",
            OrderStatus::Open => "open",
            OrderStatus::Processing => "processing",
            OrderStatus::Submitted => "submitted",
        }
    }
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrderStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

/// An enum representing the possible values of an `OrdersPaymentMethodOptionsAfterpayClearpay`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod::Automatic => "automatic",
            OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `OrdersPaymentMethodOptionsAfterpayClearpay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `OrdersV2ResourceAutomaticTax`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl OrdersV2ResourceAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersV2ResourceAutomaticTaxStatus::Complete => "complete",
            OrdersV2ResourceAutomaticTaxStatus::Failed => "failed",
            OrdersV2ResourceAutomaticTaxStatus::RequiresLocationInputs => {
                "requires_location_inputs"
            }
        }
    }
}

impl AsRef<str> for OrdersV2ResourceAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersV2ResourceAutomaticTaxStatus {
    fn default() -> Self {
        Self::Complete
    }
}

/// An enum representing the possible values of an `OrdersV2ResourceCardPaymentMethodOptions`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod {
    Automatic,
    Manual,
}

impl OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod::Automatic => "automatic",
            OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersV2ResourceCardPaymentMethodOptionsCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `OrdersV2ResourceCardPaymentMethodOptions`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage::None => "none",
            OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersV2ResourceCardPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `OrdersV2ResourcePaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Link,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::AfterpayClearpay => {
                "afterpay_clearpay"
            }
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Alipay => "alipay",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Card => "card",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::CustomerBalance => {
                "customer_balance"
            }
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Eps => "eps",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Klarna => "klarna",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Link => "link",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Oxxo => "oxxo",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::P24 => "p24",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            OrdersV2ResourcePaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `OrdersV2ResourcePayment`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourcePaymentStatus {
    Canceled,
    Complete,
    NotRequired,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
}

impl OrdersV2ResourcePaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersV2ResourcePaymentStatus::Canceled => "canceled",
            OrdersV2ResourcePaymentStatus::Complete => "complete",
            OrdersV2ResourcePaymentStatus::NotRequired => "not_required",
            OrdersV2ResourcePaymentStatus::Processing => "processing",
            OrdersV2ResourcePaymentStatus::RequiresAction => "requires_action",
            OrdersV2ResourcePaymentStatus::RequiresCapture => "requires_capture",
            OrdersV2ResourcePaymentStatus::RequiresConfirmation => "requires_confirmation",
            OrdersV2ResourcePaymentStatus::RequiresPaymentMethod => "requires_payment_method",
        }
    }
}

impl AsRef<str> for OrdersV2ResourcePaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourcePaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersV2ResourcePaymentStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

/// An enum representing the possible values of an `OrdersV2ResourceTaxDetailsResourceTaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourceTaxDetailsResourceTaxIdType {
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

impl OrdersV2ResourceTaxDetailsResourceTaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersV2ResourceTaxDetailsResourceTaxIdType::AeTrn => "ae_trn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::AuAbn => "au_abn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::AuArn => "au_arn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::BgUic => "bg_uic",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::BrCnpj => "br_cnpj",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::BrCpf => "br_cpf",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::CaBn => "ca_bn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::CaGstHst => "ca_gst_hst",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::CaPstBc => "ca_pst_bc",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::CaPstMb => "ca_pst_mb",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::CaPstSk => "ca_pst_sk",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::CaQst => "ca_qst",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::ChVat => "ch_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::ClTin => "cl_tin",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::EsCif => "es_cif",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::EuOssVat => "eu_oss_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::EuVat => "eu_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::GbVat => "gb_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::GeVat => "ge_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::HkBr => "hk_br",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::HuTin => "hu_tin",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::IdNpwp => "id_npwp",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::IlVat => "il_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::InGst => "in_gst",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::IsVat => "is_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::JpCn => "jp_cn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::JpRn => "jp_rn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::KrBrn => "kr_brn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::LiUid => "li_uid",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::MxRfc => "mx_rfc",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::MyFrp => "my_frp",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::MyItn => "my_itn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::MySst => "my_sst",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::NoVat => "no_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::NzGst => "nz_gst",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::RuInn => "ru_inn",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::RuKpp => "ru_kpp",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::SaVat => "sa_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::SgGst => "sg_gst",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::SgUen => "sg_uen",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::SiTin => "si_tin",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::ThVat => "th_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::TwVat => "tw_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::UaVat => "ua_vat",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::Unknown => "unknown",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::UsEin => "us_ein",
            OrdersV2ResourceTaxDetailsResourceTaxIdType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for OrdersV2ResourceTaxDetailsResourceTaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourceTaxDetailsResourceTaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersV2ResourceTaxDetailsResourceTaxIdType {
    fn default() -> Self {
        Self::AeTrn
    }
}

/// An enum representing the possible values of an `OrdersV2ResourceTaxDetails`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourceTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl OrdersV2ResourceTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            OrdersV2ResourceTaxDetailsTaxExempt::Exempt => "exempt",
            OrdersV2ResourceTaxDetailsTaxExempt::None => "none",
            OrdersV2ResourceTaxDetailsTaxExempt::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for OrdersV2ResourceTaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourceTaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for OrdersV2ResourceTaxDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

/// An enum representing the possible values of an `PaymentMethodOptionsPaypal`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsPaypalCaptureMethod {
    Manual,
}

impl PaymentMethodOptionsPaypalCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsPaypalCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsPaypalCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsPaypalCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsPaypalCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `UpdateOrderLineItemsPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateOrderLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderLineItemsPriceDataTaxBehavior::Exclusive => "exclusive",
            UpdateOrderLineItemsPriceDataTaxBehavior::Inclusive => "inclusive",
            UpdateOrderLineItemsPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateOrderLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderLineItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `payment_schedule` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str>
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession => {
                "off_session"
            }
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod::Automatic => "automatic",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsAlipay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsBancontact`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsCard`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    Automatic,
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod::Automatic => {
                "automatic"
            }
            UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsCard`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage::OffSession => {
                "off_session"
            }
            UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer`'s `requested_address_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl
    UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Iban => "iban",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Sepa => "sepa",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::SortCode => "sort_code",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Spei => "spei",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn default() -> Self {
        Self::Iban
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer => "eu_bank_transfer",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer => "gb_bank_transfer",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer => "jp_bank_transfer",
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    fn default() -> Self {
        Self::BankTransfer
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage::None => {
                "none"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsIdeal`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna`'s `preferred_locale` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    #[serde(rename = "da-DK")]
    DaDk,
    #[serde(rename = "de-AT")]
    DeAt,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "en-AT")]
    EnAt,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-BE")]
    EnBe,
    #[serde(rename = "en-CA")]
    EnCa,
    #[serde(rename = "en-DE")]
    EnDe,
    #[serde(rename = "en-DK")]
    EnDk,
    #[serde(rename = "en-ES")]
    EnEs,
    #[serde(rename = "en-FI")]
    EnFi,
    #[serde(rename = "en-FR")]
    EnFr,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "en-IE")]
    EnIe,
    #[serde(rename = "en-IT")]
    EnIt,
    #[serde(rename = "en-NL")]
    EnNl,
    #[serde(rename = "en-NO")]
    EnNo,
    #[serde(rename = "en-NZ")]
    EnNz,
    #[serde(rename = "en-SE")]
    EnSe,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "es-US")]
    EsUs,
    #[serde(rename = "fi-FI")]
    FiFi,
    #[serde(rename = "fr-BE")]
    FrBe,
    #[serde(rename = "fr-CA")]
    FrCa,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "nb-NO")]
    NbNo,
    #[serde(rename = "nl-BE")]
    NlBe,
    #[serde(rename = "nl-NL")]
    NlNl,
    #[serde(rename = "sv-FI")]
    SvFi,
    #[serde(rename = "sv-SE")]
    SvSe,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::DaDk => "da-DK",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::DeAt => "de-AT",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::DeDe => "de-DE",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnAt => "en-AT",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnAu => "en-AU",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnBe => "en-BE",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnCa => "en-CA",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnDe => "en-DE",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnDk => "en-DK",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnEs => "en-ES",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnFi => "en-FI",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnFr => "en-FR",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnGb => "en-GB",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnIe => "en-IE",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnIt => "en-IT",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnNl => "en-NL",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnNo => "en-NO",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnNz => "en-NZ",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnSe => "en-SE",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EnUs => "en-US",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EsEs => "es-ES",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::EsUs => "es-US",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FiFi => "fi-FI",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FrBe => "fr-BE",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FrCa => "fr-CA",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::FrFr => "fr-FR",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::ItIt => "it-IT",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::NbNo => "nb-NO",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::NlBe => "nl-BE",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::NlNl => "nl-NL",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::SvFi => "sv-FI",
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale::SvSe => "sv-SE",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn default() -> Self {
        Self::DaDk
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsLink`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsLink`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsOxxo`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsP24`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebit`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage::OffSession => {
                "off_session"
            }
            UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage::OnSession => {
                "on_session"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsSofort`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::De => "de",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::En => "en",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Es => "es",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Fr => "fr",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::It => "it",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Nl => "nl",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage::Pl => "pl",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsSofort`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage::None => "none",
            UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage::OffSession => {
                "off_session"
            }
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPay`'s `client` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient::Android => "android",
            UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient::Ios => "ios",
            UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient::Web => "web",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn default() -> Self {
        Self::Android
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPay`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage::None => "none",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `UpdateOrderPaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderPaymentSettingsPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Link,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl UpdateOrderPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderPaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            UpdateOrderPaymentSettingsPaymentMethodTypes::AfterpayClearpay => "afterpay_clearpay",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Alipay => "alipay",
            UpdateOrderPaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            UpdateOrderPaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Card => "card",
            UpdateOrderPaymentSettingsPaymentMethodTypes::CustomerBalance => "customer_balance",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Eps => "eps",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Klarna => "klarna",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Link => "link",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Oxxo => "oxxo",
            UpdateOrderPaymentSettingsPaymentMethodTypes::P24 => "p24",
            UpdateOrderPaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            UpdateOrderPaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            UpdateOrderPaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for UpdateOrderPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AcssDebit
    }
}

/// An enum representing the possible values of an `UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay => {
                "business_day"
            }
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Day => "day",
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Hour => "hour",
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Month => "month",
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Week => "week",
        }
    }
}

impl AsRef<str> for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::BusinessDay => {
                "business_day"
            }
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Day => "day",
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Hour => "hour",
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Month => "month",
            UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Week => "week",
        }
    }
}

impl AsRef<str> for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `UpdateOrderShippingCostShippingRateData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateOrderShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderShippingCostShippingRateDataTaxBehavior::Exclusive => "exclusive",
            UpdateOrderShippingCostShippingRateDataTaxBehavior::Inclusive => "inclusive",
            UpdateOrderShippingCostShippingRateDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateOrderShippingCostShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderShippingCostShippingRateDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `UpdateOrderShippingCostShippingRateData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderShippingCostShippingRateDataType {
    FixedAmount,
}

impl UpdateOrderShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderShippingCostShippingRateDataType::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for UpdateOrderShippingCostShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderShippingCostShippingRateDataType {
    fn default() -> Self {
        Self::FixedAmount
    }
}

/// An enum representing the possible values of an `UpdateOrderTaxDetails`'s `tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl UpdateOrderTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderTaxDetailsTaxExempt::Exempt => "exempt",
            UpdateOrderTaxDetailsTaxExempt::None => "none",
            UpdateOrderTaxDetailsTaxExempt::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for UpdateOrderTaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderTaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderTaxDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

/// An enum representing the possible values of an `UpdateOrderTaxDetailsTaxIds`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateOrderTaxDetailsTaxIdsType {
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
    UsEin,
    ZaVat,
}

impl UpdateOrderTaxDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateOrderTaxDetailsTaxIdsType::AeTrn => "ae_trn",
            UpdateOrderTaxDetailsTaxIdsType::AuAbn => "au_abn",
            UpdateOrderTaxDetailsTaxIdsType::AuArn => "au_arn",
            UpdateOrderTaxDetailsTaxIdsType::BgUic => "bg_uic",
            UpdateOrderTaxDetailsTaxIdsType::BrCnpj => "br_cnpj",
            UpdateOrderTaxDetailsTaxIdsType::BrCpf => "br_cpf",
            UpdateOrderTaxDetailsTaxIdsType::CaBn => "ca_bn",
            UpdateOrderTaxDetailsTaxIdsType::CaGstHst => "ca_gst_hst",
            UpdateOrderTaxDetailsTaxIdsType::CaPstBc => "ca_pst_bc",
            UpdateOrderTaxDetailsTaxIdsType::CaPstMb => "ca_pst_mb",
            UpdateOrderTaxDetailsTaxIdsType::CaPstSk => "ca_pst_sk",
            UpdateOrderTaxDetailsTaxIdsType::CaQst => "ca_qst",
            UpdateOrderTaxDetailsTaxIdsType::ChVat => "ch_vat",
            UpdateOrderTaxDetailsTaxIdsType::ClTin => "cl_tin",
            UpdateOrderTaxDetailsTaxIdsType::EsCif => "es_cif",
            UpdateOrderTaxDetailsTaxIdsType::EuOssVat => "eu_oss_vat",
            UpdateOrderTaxDetailsTaxIdsType::EuVat => "eu_vat",
            UpdateOrderTaxDetailsTaxIdsType::GbVat => "gb_vat",
            UpdateOrderTaxDetailsTaxIdsType::GeVat => "ge_vat",
            UpdateOrderTaxDetailsTaxIdsType::HkBr => "hk_br",
            UpdateOrderTaxDetailsTaxIdsType::HuTin => "hu_tin",
            UpdateOrderTaxDetailsTaxIdsType::IdNpwp => "id_npwp",
            UpdateOrderTaxDetailsTaxIdsType::IlVat => "il_vat",
            UpdateOrderTaxDetailsTaxIdsType::InGst => "in_gst",
            UpdateOrderTaxDetailsTaxIdsType::IsVat => "is_vat",
            UpdateOrderTaxDetailsTaxIdsType::JpCn => "jp_cn",
            UpdateOrderTaxDetailsTaxIdsType::JpRn => "jp_rn",
            UpdateOrderTaxDetailsTaxIdsType::KrBrn => "kr_brn",
            UpdateOrderTaxDetailsTaxIdsType::LiUid => "li_uid",
            UpdateOrderTaxDetailsTaxIdsType::MxRfc => "mx_rfc",
            UpdateOrderTaxDetailsTaxIdsType::MyFrp => "my_frp",
            UpdateOrderTaxDetailsTaxIdsType::MyItn => "my_itn",
            UpdateOrderTaxDetailsTaxIdsType::MySst => "my_sst",
            UpdateOrderTaxDetailsTaxIdsType::NoVat => "no_vat",
            UpdateOrderTaxDetailsTaxIdsType::NzGst => "nz_gst",
            UpdateOrderTaxDetailsTaxIdsType::RuInn => "ru_inn",
            UpdateOrderTaxDetailsTaxIdsType::RuKpp => "ru_kpp",
            UpdateOrderTaxDetailsTaxIdsType::SaVat => "sa_vat",
            UpdateOrderTaxDetailsTaxIdsType::SgGst => "sg_gst",
            UpdateOrderTaxDetailsTaxIdsType::SgUen => "sg_uen",
            UpdateOrderTaxDetailsTaxIdsType::SiTin => "si_tin",
            UpdateOrderTaxDetailsTaxIdsType::ThVat => "th_vat",
            UpdateOrderTaxDetailsTaxIdsType::TwVat => "tw_vat",
            UpdateOrderTaxDetailsTaxIdsType::UaVat => "ua_vat",
            UpdateOrderTaxDetailsTaxIdsType::UsEin => "us_ein",
            UpdateOrderTaxDetailsTaxIdsType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for UpdateOrderTaxDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateOrderTaxDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateOrderTaxDetailsTaxIdsType {
    fn default() -> Self {
        Self::AeTrn
    }
}
