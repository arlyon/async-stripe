use stripe::{Client, Response};

impl stripe_misc::order::Order {
    /// Creates a new `open` order object.
    pub fn create(client: &Client, params: CreateOrder) -> Response<stripe_misc::order::Order> {
        client.send_form("/orders", params, http_types::Method::Post)
    }
    /// Returns a list of your orders.
    ///
    /// The orders are returned sorted by creation date, with the most recently created orders appearing first.
    pub fn list(
        client: &Client,
        params: ListOrder,
    ) -> Response<stripe_types::List<stripe_misc::order::Order>> {
        client.get_query("/orders", params)
    }
    /// Retrieves the details of an existing order.
    ///
    /// Supply the unique order ID from either an order creation request or the order list, and Stripe will return the corresponding order information.
    pub fn retrieve(
        client: &Client,
        id: &str,
        params: RetrieveOrder,
    ) -> Response<stripe_misc::order::Order> {
        client.get_query(&format!("/orders/{id}", id = id), params)
    }
    /// Updates the specific order by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(
        client: &Client,
        id: &str,
        params: UpdateOrder,
    ) -> Response<stripe_misc::order::Order> {
        client.send_form(&format!("/orders/{id}", id = id), params, http_types::Method::Post)
    }
    /// Submitting an Order transitions the status to `processing` and creates a PaymentIntent object so the order can be paid.
    ///
    /// If the Order has an `amount_total` of 0, no PaymentIntent object will be created.
    /// Once the order is submitted, its contents cannot be changed, unless the [reopen](https://stripe.com/docs/api#reopen_order) method is called.
    pub fn submit(
        client: &Client,
        id: &str,
        params: SubmitOrder,
    ) -> Response<stripe_misc::order::Order> {
        client.send_form(&format!("/orders/{id}/submit", id = id), params, http_types::Method::Post)
    }
    /// Cancels the order as well as the payment intent if one is attached.
    pub fn cancel(
        client: &Client,
        id: &str,
        params: CancelOrder,
    ) -> Response<stripe_misc::order::Order> {
        client.send_form(&format!("/orders/{id}/cancel", id = id), params, http_types::Method::Post)
    }
    /// Reopens a `submitted` order.
    pub fn reopen(
        client: &Client,
        id: &str,
        params: ReopenOrder,
    ) -> Response<stripe_misc::order::Order> {
        client.send_form(&format!("/orders/{id}/reopen", id = id), params, http_types::Method::Post)
    }
    /// When retrieving an order, there is an includable **line_items** property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn list_line_items(
        client: &Client,
        id: &str,
        params: ListLineItemsOrder,
    ) -> Response<stripe_types::List<stripe_core::line_item::LineItem>> {
        client.get_query(&format!("/orders/{id}/line_items", id = id), params)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrder<'a> {
    /// Settings for automatic tax calculation for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateOrderAutomaticTax>,
    /// Billing details for the customer.
    ///
    /// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateOrderBillingDetails<'a>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The customer associated with this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The coupons, promotion codes, and/or discounts to apply to the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [CreateOrderDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The IP address of the purchaser for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    pub line_items: &'a [CreateOrderLineItems<'a>],
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Payment information associated with the order, including payment settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<CreateOrderPayment<'a>>,
    /// Settings for the customer cost of shipping for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateOrderShippingCost<'a>>,
    /// Shipping details for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<CreateOrderShippingDetails<'a>>,
    /// Additional tax details about the purchaser to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<CreateOrderTaxDetails<'a>>,
}
impl<'a> CreateOrder<'a> {
    pub fn new(
        currency: stripe_types::Currency,
        line_items: &'a [CreateOrderLineItems<'a>],
    ) -> Self {
        Self {
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
/// Settings for automatic tax calculation for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderAutomaticTax {
    /// Enable automatic tax calculation which will automatically compute tax rates on this order.
    pub enabled: bool,
}
impl CreateOrderAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Billing details for the customer.
///
/// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderBillingDetails<'a> {
    /// The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateOrderBillingDetailsAddress<'a>>,
    /// The billing email provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// The billing name provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The billing phone number provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreateOrderBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The billing address provided by the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderBillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateOrderBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The coupons, promotion codes, and/or discounts to apply to the order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> CreateOrderDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A list of line items the customer is ordering.
///
/// Each line item includes information about the product, the quantity, and the resulting cost.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderLineItems<'a> {
    /// The description for the line item.
    ///
    /// Will default to the name of the associated product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The discounts applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [CreateOrderLineItemsDiscounts<'a>]>,
    /// The ID of a [Price](https://stripe.com/docs/api/prices) to add to the Order.
    ///
    /// The `price` parameter is an alternative to using the `product` parameter.
    ///
    /// If each of your products are sold at a single price, you can set `Product.default_price` and then pass the `product` parameter when creating a line item.
    /// If your products are sold at several possible prices, use the `price` parameter to explicitly specify which one to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new Price object inline.
    ///
    /// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
    ///
    /// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
    /// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
    /// This Price is hidden in both the Dashboard and API lists and cannot be reused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateOrderLineItemsPriceData<'a>>,
    /// The ID of a [Product](https://stripe.com/docs/api/products) to add to the Order.
    ///
    /// The product must have a `default_price` specified.
    ///
    /// Otherwise, specify the price by passing the `price` or `price_data` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Defines a Product inline and adds it to the Order.
    ///
    /// `product_data` is an alternative to the `product` parameter.
    ///
    /// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
    /// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<CreateOrderLineItemsProductData<'a>>,
    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateOrderLineItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The discounts applied to this line item.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderLineItemsDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> CreateOrderLineItemsDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new Price object inline.
///
/// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
///
/// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
/// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
/// This Price is hidden in both the Dashboard and API lists and cannot be reused.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderLineItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the product this price belongs to.
    ///
    /// Use this to implement a variable-pricing model in your integration.
    ///
    /// This is required if `product_data` is not specifed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateOrderLineItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateOrderLineItemsPriceData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateOrderLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateOrderLineItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderLineItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Defines a Product inline and adds it to the Order.
///
/// `product_data` is an alternative to the `product` parameter.
///
/// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
/// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
/// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderLineItemsProductData<'a> {
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// A unique identifier for this product.
    ///
    /// `product_data` automatically creates a Product with this ID.
    ///
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    /// If any of the fields in the existing Product are different from the values in `product_data`, `product_data` updates the existing Product with the new information.
    /// So set `product_data[id]` to the same string every time you sell the same product, but don't re-use the same string for different products.
    pub id: &'a str,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<CreateOrderLineItemsProductDataPackageDimensions>,
    /// Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> CreateOrderLineItemsProductData<'a> {
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            description: Default::default(),
            id,
            images: Default::default(),
            metadata: Default::default(),
            name,
            package_dimensions: Default::default(),
            shippable: Default::default(),
            tax_code: Default::default(),
            url: Default::default(),
        }
    }
}
/// The dimensions of this product for shipping purposes.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderLineItemsProductDataPackageDimensions {
    /// Height, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub height: f64,
    /// Length, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub length: f64,
    /// Weight, in ounces.
    ///
    /// Maximum precision is 2 decimal places.
    pub weight: f64,
    /// Width, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub width: f64,
}
impl CreateOrderLineItemsProductDataPackageDimensions {
    pub fn new(height: f64, length: f64, weight: f64, width: f64) -> Self {
        Self { height, length, weight, width }
    }
}
/// Payment information associated with the order, including payment settings.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderPayment<'a> {
    /// Settings describing how the order should configure generated PaymentIntents.
    pub settings: CreateOrderPaymentSettings<'a>,
}
impl<'a> CreateOrderPayment<'a> {
    pub fn new(settings: CreateOrderPaymentSettings<'a>) -> Self {
        Self { settings }
    }
}
/// Settings describing how the order should configure generated PaymentIntents.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettings<'a> {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateOrderPaymentSettingsPaymentMethodOptions<'a>>,
    /// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [CreateOrderPaymentSettingsPaymentMethodTypes]>,
    /// The URL to redirect the customer to after they authenticate their payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers' statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// Provides configuration for completing a transfer for the order after it is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateOrderPaymentSettingsTransferData<'a>>,
}
impl<'a> CreateOrderPaymentSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptions<'a> {
    /// If paying by `acss_debit`, this sub-hash contains details about the ACSS Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebit<'a>>,
    /// If paying by `afterpay_clearpay`, this sub-hash contains details about the AfterpayClearpay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay<'a>>,
    /// If paying by `alipay`, this sub-hash contains details about the Alipay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateOrderPaymentSettingsPaymentMethodOptionsAlipay>,
    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateOrderPaymentSettingsPaymentMethodOptionsBancontact>,
    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCard>,
    /// If paying by `customer_balance`, this sub-hash contains details about the Customer Balance payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance<'a>>,
    /// If paying by `ideal`, this sub-hash contains details about the iDEAL payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateOrderPaymentSettingsPaymentMethodOptionsIdeal>,
    /// If paying by `klarna`, this sub-hash contains details about the Klarna payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarna>,
    /// If paying by `link`, this sub-hash contains details about the Link payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateOrderPaymentSettingsPaymentMethodOptionsLink<'a>>,
    /// If paying by `oxxo`, this sub-hash contains details about the OXXO payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateOrderPaymentSettingsPaymentMethodOptionsOxxo>,
    /// If paying by `p24`, this sub-hash contains details about the P24 payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateOrderPaymentSettingsPaymentMethodOptionsP24>,
    /// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebit>,
    /// If paying by `sofort`, this sub-hash contains details about the Sofort payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateOrderPaymentSettingsPaymentMethodOptionsSofort>,
    /// If paying by `wechat_pay`, this sub-hash contains details about the WeChat Pay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateOrderPaymentSettingsPaymentMethodOptionsWechatPay<'a>>,
}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If paying by `acss_debit`, this sub-hash contains details about the ACSS Debit payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebit<'a> {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule,
    >,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combined" => Ok(Self::Combined),
            "interval" => Ok(Self::Interval),
            "sporadic" => Ok(Self::Sporadic),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business" => Ok(Self::Business),
            "personal" => Ok(Self::Personal),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "instant" => Ok(Self::Instant),
            "microdeposits" => Ok(Self::Microdeposits),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `afterpay_clearpay`, this sub-hash contains details about the AfterpayClearpay payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with the payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `alipay`, this sub-hash contains details about the Alipay payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "nl" => Ok(Self::Nl),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod>,
    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    Automatic,
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with the payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `customer_balance`, this sub-hash contains details about the Customer Balance payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<&'a [CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes]>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,

}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new(
        type_: CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,
    ) -> Self {
        Self {
            eu_bank_transfer: Default::default(),
            requested_address_types: Default::default(),
            type_,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
    'a,
> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a>
    CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>
{
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
    "iban" => Ok(Self::Iban),
"sepa" => Ok(Self::Sepa),
"sort_code" => Ok(Self::SortCode),
"spei" => Ok(Self::Spei),
"zengin" => Ok(Self::Zengin),

            _ => Err(())
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eu_bank_transfer" => Ok(Self::EuBankTransfer),
            "gb_bank_transfer" => Ok(Self::GbBankTransfer),
            "jp_bank_transfer" => Ok(Self::JpBankTransfer),
            "mx_bank_transfer" => Ok(Self::MxBankTransfer),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bank_transfer" => Ok(Self::BankTransfer),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `ideal`, this sub-hash contains details about the iDEAL payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `klarna`, this sub-hash contains details about the Klarna payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Preferred language of the Klarna authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    DaMinusDk,
    DeMinusAt,
    DeMinusCh,
    DeMinusDe,
    EnMinusAt,
    EnMinusAu,
    EnMinusBe,
    EnMinusCa,
    EnMinusCh,
    EnMinusDe,
    EnMinusDk,
    EnMinusEs,
    EnMinusFi,
    EnMinusFr,
    EnMinusGb,
    EnMinusIe,
    EnMinusIt,
    EnMinusNl,
    EnMinusNo,
    EnMinusNz,
    EnMinusPl,
    EnMinusPt,
    EnMinusSe,
    EnMinusUs,
    EsMinusEs,
    EsMinusUs,
    FiMinusFi,
    FrMinusBe,
    FrMinusCa,
    FrMinusCh,
    FrMinusFr,
    ItMinusCh,
    ItMinusIt,
    NbMinusNo,
    NlMinusBe,
    NlMinusNl,
    PlMinusPl,
    PtMinusPt,
    SvMinusFi,
    SvMinusSe,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DaMinusDk => "da-DK",
            Self::DeMinusAt => "de-AT",
            Self::DeMinusCh => "de-CH",
            Self::DeMinusDe => "de-DE",
            Self::EnMinusAt => "en-AT",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusBe => "en-BE",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusCh => "en-CH",
            Self::EnMinusDe => "en-DE",
            Self::EnMinusDk => "en-DK",
            Self::EnMinusEs => "en-ES",
            Self::EnMinusFi => "en-FI",
            Self::EnMinusFr => "en-FR",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIt => "en-IT",
            Self::EnMinusNl => "en-NL",
            Self::EnMinusNo => "en-NO",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusPl => "en-PL",
            Self::EnMinusPt => "en-PT",
            Self::EnMinusSe => "en-SE",
            Self::EnMinusUs => "en-US",
            Self::EsMinusEs => "es-ES",
            Self::EsMinusUs => "es-US",
            Self::FiMinusFi => "fi-FI",
            Self::FrMinusBe => "fr-BE",
            Self::FrMinusCa => "fr-CA",
            Self::FrMinusCh => "fr-CH",
            Self::FrMinusFr => "fr-FR",
            Self::ItMinusCh => "it-CH",
            Self::ItMinusIt => "it-IT",
            Self::NbMinusNo => "nb-NO",
            Self::NlMinusBe => "nl-BE",
            Self::NlMinusNl => "nl-NL",
            Self::PlMinusPl => "pl-PL",
            Self::PtMinusPt => "pt-PT",
            Self::SvMinusFi => "sv-FI",
            Self::SvMinusSe => "sv-SE",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "da-DK" => Ok(Self::DaMinusDk),
            "de-AT" => Ok(Self::DeMinusAt),
            "de-CH" => Ok(Self::DeMinusCh),
            "de-DE" => Ok(Self::DeMinusDe),
            "en-AT" => Ok(Self::EnMinusAt),
            "en-AU" => Ok(Self::EnMinusAu),
            "en-BE" => Ok(Self::EnMinusBe),
            "en-CA" => Ok(Self::EnMinusCa),
            "en-CH" => Ok(Self::EnMinusCh),
            "en-DE" => Ok(Self::EnMinusDe),
            "en-DK" => Ok(Self::EnMinusDk),
            "en-ES" => Ok(Self::EnMinusEs),
            "en-FI" => Ok(Self::EnMinusFi),
            "en-FR" => Ok(Self::EnMinusFr),
            "en-GB" => Ok(Self::EnMinusGb),
            "en-IE" => Ok(Self::EnMinusIe),
            "en-IT" => Ok(Self::EnMinusIt),
            "en-NL" => Ok(Self::EnMinusNl),
            "en-NO" => Ok(Self::EnMinusNo),
            "en-NZ" => Ok(Self::EnMinusNz),
            "en-PL" => Ok(Self::EnMinusPl),
            "en-PT" => Ok(Self::EnMinusPt),
            "en-SE" => Ok(Self::EnMinusSe),
            "en-US" => Ok(Self::EnMinusUs),
            "es-ES" => Ok(Self::EsMinusEs),
            "es-US" => Ok(Self::EsMinusUs),
            "fi-FI" => Ok(Self::FiMinusFi),
            "fr-BE" => Ok(Self::FrMinusBe),
            "fr-CA" => Ok(Self::FrMinusCa),
            "fr-CH" => Ok(Self::FrMinusCh),
            "fr-FR" => Ok(Self::FrMinusFr),
            "it-CH" => Ok(Self::ItMinusCh),
            "it-IT" => Ok(Self::ItMinusIt),
            "nb-NO" => Ok(Self::NbMinusNo),
            "nl-BE" => Ok(Self::NlMinusBe),
            "nl-NL" => Ok(Self::NlMinusNl),
            "pl-PL" => Ok(Self::PlMinusPl),
            "pt-PT" => Ok(Self::PtMinusPt),
            "sv-FI" => Ok(Self::SvMinusFi),
            "sv-SE" => Ok(Self::SvMinusSe),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `link`, this sub-hash contains details about the Link payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsLink<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage>,
}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `oxxo`, this sub-hash contains details about the OXXO payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `p24`, this sub-hash contains details about the P24 payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Debit payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {}
impl CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `sofort`, this sub-hash contains details about the Sofort payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsSofort {
    /// Language shown to the payer on redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage>,
}
impl CreateOrderPaymentSettingsPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Language shown to the payer on redirect.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Pl => "pl",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "fr" => Ok(Self::Fr),
            "it" => Ok(Self::It),
            "nl" => Ok(Self::Nl),
            "pl" => Ok(Self::Pl),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `wechat_pay`, this sub-hash contains details about the WeChat Pay payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderPaymentSettingsPaymentMethodOptionsWechatPay<'a> {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<&'a str>,
    /// The client type that the end customer will pay from.
    pub client: CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl<'a> CreateOrderPaymentSettingsPaymentMethodOptionsWechatPay<'a> {
    pub fn new(client: CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// The client type that the end customer will pay from.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "android" => Ok(Self::Android),
            "ios" => Ok(Self::Ios),
            "web" => Ok(Self::Web),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
///
/// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::AcssDebit => "acss_debit",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl std::str::FromStr for CreateOrderPaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acss_debit" => Ok(Self::AcssDebit),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "card" => Ok(Self::Card),
            "customer_balance" => Ok(Self::CustomerBalance),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "link" => Ok(Self::Link),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderPaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Provides configuration for completing a transfer for the order after it is paid.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderPaymentSettingsTransferData<'a> {
    /// The amount that will be transferred automatically when the order is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of the Connected account receiving the transfer.
    pub destination: &'a str,
}
impl<'a> CreateOrderPaymentSettingsTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
/// Settings for the customer cost of shipping for this order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateOrderShippingCostShippingRateData<'a>>,
}
impl<'a> CreateOrderShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderShippingCostShippingRateData<'a> {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateOrderShippingCostShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: &'a str,
    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateOrderShippingCostShippingRateDataFixedAmount<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateOrderShippingCostShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateOrderShippingCostShippingRateDataType>,
}
impl<'a> CreateOrderShippingCostShippingRateData<'a> {
    pub fn new(display_name: &'a str) -> Self {
        Self {
            delivery_estimate: Default::default(),
            display_name,
            fixed_amount: Default::default(),
            metadata: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            type_: Default::default(),
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimum>,
}
impl CreateOrderShippingCostShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl std::str::FromStr for CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business_day" => Ok(Self::BusinessDay),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl std::str::FromStr for CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business_day" => Ok(Self::BusinessDay),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderShippingCostShippingRateDataFixedAmount<'a> {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl<'a> CreateOrderShippingCostShippingRateDataFixedAmount<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, currency_options: Default::default() }
    }
}
/// Shipping rates defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}
impl CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: Default::default() }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
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
impl serde::Serialize
    for CreateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateOrderShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateOrderShippingCostShippingRateDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderShippingCostShippingRateDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderShippingCostShippingRateDataType {
    FixedAmount,
}

impl CreateOrderShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for CreateOrderShippingCostShippingRateDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_amount" => Ok(Self::FixedAmount),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderShippingCostShippingRateDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Shipping details for the order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderShippingDetails<'a> {
    /// The shipping address for the order.
    pub address: CreateOrderShippingDetailsAddress<'a>,
    /// The name of the recipient of the order.
    pub name: &'a str,
    /// The phone number (including extension) for the recipient of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreateOrderShippingDetails<'a> {
    pub fn new(address: CreateOrderShippingDetailsAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// The shipping address for the order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderShippingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateOrderShippingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional tax details about the purchaser to be used for this order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateOrderTaxDetails<'a> {
    /// The purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CreateOrderTaxDetailsTaxExempt>,
    /// The purchaser's tax IDs to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<&'a [CreateOrderTaxDetailsTaxIds<'a>]>,
}
impl<'a> CreateOrderTaxDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The purchaser's tax exemption status.
///
/// One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateOrderTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl CreateOrderTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for CreateOrderTaxDetailsTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exempt" => Ok(Self::Exempt),
            "none" => Ok(Self::None),
            "reverse" => Ok(Self::Reverse),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderTaxDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The purchaser's tax IDs to be used for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateOrderTaxDetailsTaxIds<'a> {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreateOrderTaxDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> CreateOrderTaxDetailsTaxIds<'a> {
    pub fn new(type_: CreateOrderTaxDetailsTaxIdsType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    PhTin,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    ZaVat,
}

impl CreateOrderTaxDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AeTrn => "ae_trn",
            Self::AuAbn => "au_abn",
            Self::AuArn => "au_arn",
            Self::BgUic => "bg_uic",
            Self::BrCnpj => "br_cnpj",
            Self::BrCpf => "br_cpf",
            Self::CaBn => "ca_bn",
            Self::CaGstHst => "ca_gst_hst",
            Self::CaPstBc => "ca_pst_bc",
            Self::CaPstMb => "ca_pst_mb",
            Self::CaPstSk => "ca_pst_sk",
            Self::CaQst => "ca_qst",
            Self::ChVat => "ch_vat",
            Self::ClTin => "cl_tin",
            Self::EgTin => "eg_tin",
            Self::EsCif => "es_cif",
            Self::EuOssVat => "eu_oss_vat",
            Self::EuVat => "eu_vat",
            Self::GbVat => "gb_vat",
            Self::GeVat => "ge_vat",
            Self::HkBr => "hk_br",
            Self::HuTin => "hu_tin",
            Self::IdNpwp => "id_npwp",
            Self::IlVat => "il_vat",
            Self::InGst => "in_gst",
            Self::IsVat => "is_vat",
            Self::JpCn => "jp_cn",
            Self::JpRn => "jp_rn",
            Self::JpTrn => "jp_trn",
            Self::KePin => "ke_pin",
            Self::KrBrn => "kr_brn",
            Self::LiUid => "li_uid",
            Self::MxRfc => "mx_rfc",
            Self::MyFrp => "my_frp",
            Self::MyItn => "my_itn",
            Self::MySst => "my_sst",
            Self::NoVat => "no_vat",
            Self::NzGst => "nz_gst",
            Self::PhTin => "ph_tin",
            Self::RuInn => "ru_inn",
            Self::RuKpp => "ru_kpp",
            Self::SaVat => "sa_vat",
            Self::SgGst => "sg_gst",
            Self::SgUen => "sg_uen",
            Self::SiTin => "si_tin",
            Self::ThVat => "th_vat",
            Self::TrTin => "tr_tin",
            Self::TwVat => "tw_vat",
            Self::UaVat => "ua_vat",
            Self::UsEin => "us_ein",
            Self::ZaVat => "za_vat",
        }
    }
}

impl std::str::FromStr for CreateOrderTaxDetailsTaxIdsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ae_trn" => Ok(Self::AeTrn),
            "au_abn" => Ok(Self::AuAbn),
            "au_arn" => Ok(Self::AuArn),
            "bg_uic" => Ok(Self::BgUic),
            "br_cnpj" => Ok(Self::BrCnpj),
            "br_cpf" => Ok(Self::BrCpf),
            "ca_bn" => Ok(Self::CaBn),
            "ca_gst_hst" => Ok(Self::CaGstHst),
            "ca_pst_bc" => Ok(Self::CaPstBc),
            "ca_pst_mb" => Ok(Self::CaPstMb),
            "ca_pst_sk" => Ok(Self::CaPstSk),
            "ca_qst" => Ok(Self::CaQst),
            "ch_vat" => Ok(Self::ChVat),
            "cl_tin" => Ok(Self::ClTin),
            "eg_tin" => Ok(Self::EgTin),
            "es_cif" => Ok(Self::EsCif),
            "eu_oss_vat" => Ok(Self::EuOssVat),
            "eu_vat" => Ok(Self::EuVat),
            "gb_vat" => Ok(Self::GbVat),
            "ge_vat" => Ok(Self::GeVat),
            "hk_br" => Ok(Self::HkBr),
            "hu_tin" => Ok(Self::HuTin),
            "id_npwp" => Ok(Self::IdNpwp),
            "il_vat" => Ok(Self::IlVat),
            "in_gst" => Ok(Self::InGst),
            "is_vat" => Ok(Self::IsVat),
            "jp_cn" => Ok(Self::JpCn),
            "jp_rn" => Ok(Self::JpRn),
            "jp_trn" => Ok(Self::JpTrn),
            "ke_pin" => Ok(Self::KePin),
            "kr_brn" => Ok(Self::KrBrn),
            "li_uid" => Ok(Self::LiUid),
            "mx_rfc" => Ok(Self::MxRfc),
            "my_frp" => Ok(Self::MyFrp),
            "my_itn" => Ok(Self::MyItn),
            "my_sst" => Ok(Self::MySst),
            "no_vat" => Ok(Self::NoVat),
            "nz_gst" => Ok(Self::NzGst),
            "ph_tin" => Ok(Self::PhTin),
            "ru_inn" => Ok(Self::RuInn),
            "ru_kpp" => Ok(Self::RuKpp),
            "sa_vat" => Ok(Self::SaVat),
            "sg_gst" => Ok(Self::SgGst),
            "sg_uen" => Ok(Self::SgUen),
            "si_tin" => Ok(Self::SiTin),
            "th_vat" => Ok(Self::ThVat),
            "tr_tin" => Ok(Self::TrTin),
            "tw_vat" => Ok(Self::TwVat),
            "ua_vat" => Ok(Self::UaVat),
            "us_ein" => Ok(Self::UsEin),
            "za_vat" => Ok(Self::ZaVat),

            _ => Err(()),
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
impl serde::Serialize for CreateOrderTaxDetailsTaxIdsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListOrder<'a> {
    /// Only return orders for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveOrder<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrder<'a> {
    /// Settings for automatic tax calculation for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateOrderAutomaticTax>,
    /// Billing details for the customer.
    ///
    /// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<UpdateOrderBillingDetails<'a>>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The customer associated with this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The coupons, promotion codes, and/or discounts to apply to the order.
    ///
    /// Pass the empty string `""` to unset this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpdateOrderDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The IP address of the purchaser for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [UpdateOrderLineItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Payment information associated with the order, including payment settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<UpdateOrderPayment<'a>>,
    /// Settings for the customer cost of shipping for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<UpdateOrderShippingCost<'a>>,
    /// Shipping details for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<UpdateOrderShippingDetails<'a>>,
    /// Additional tax details about the purchaser to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<UpdateOrderTaxDetails<'a>>,
}
impl<'a> UpdateOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings for automatic tax calculation for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderAutomaticTax {
    /// Enable automatic tax calculation which will automatically compute tax rates on this order.
    pub enabled: bool,
}
impl UpdateOrderAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Billing details for the customer.
///
/// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderBillingDetails<'a> {
    /// The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateOrderBillingDetailsAddress<'a>>,
    /// The billing email provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// The billing name provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The billing phone number provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdateOrderBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The billing address provided by the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderBillingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateOrderBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The coupons, promotion codes, and/or discounts to apply to the order.
///
/// Pass the empty string `""` to unset this field.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> UpdateOrderDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A list of line items the customer is ordering.
///
/// Each line item includes information about the product, the quantity, and the resulting cost.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderLineItems<'a> {
    /// The description for the line item.
    ///
    /// Will default to the name of the associated product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The discounts applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpdateOrderLineItemsDiscounts<'a>]>,
    /// The ID of an existing line item on the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// The ID of a [Price](https://stripe.com/docs/api/prices) to add to the Order.
    ///
    /// The `price` parameter is an alternative to using the `product` parameter.
    ///
    /// If each of your products are sold at a single price, you can set `Product.default_price` and then pass the `product` parameter when creating a line item.
    /// If your products are sold at several possible prices, use the `price` parameter to explicitly specify which one to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new Price object inline.
    ///
    /// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
    ///
    /// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
    /// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
    /// This Price is hidden in both the Dashboard and API lists and cannot be reused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateOrderLineItemsPriceData<'a>>,
    /// The ID of a [Product](https://stripe.com/docs/api/products) to add to the Order.
    ///
    /// The product must have a `default_price` specified.
    ///
    /// Otherwise, specify the price by passing the `price` or `price_data` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Defines a Product inline and adds it to the Order.
    ///
    /// `product_data` is an alternative to the `product` parameter.
    ///
    /// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
    /// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<UpdateOrderLineItemsProductData<'a>>,
    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateOrderLineItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The discounts applied to this line item.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderLineItemsDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpdateOrderLineItemsDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new Price object inline.
///
/// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
///
/// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
/// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
/// This Price is hidden in both the Dashboard and API lists and cannot be reused.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderLineItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// ID of the product this price belongs to.
    ///
    /// Use this to implement a variable-pricing model in your integration.
    ///
    /// This is required if `product_data` is not specifed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateOrderLineItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateOrderLineItemsPriceData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateOrderLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateOrderLineItemsPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderLineItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Defines a Product inline and adds it to the Order.
///
/// `product_data` is an alternative to the `product` parameter.
///
/// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
/// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
/// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderLineItemsProductData<'a> {
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// A unique identifier for this product.
    ///
    /// `product_data` automatically creates a Product with this ID.
    ///
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    /// If any of the fields in the existing Product are different from the values in `product_data`, `product_data` updates the existing Product with the new information.
    /// So set `product_data[id]` to the same string every time you sell the same product, but don't re-use the same string for different products.
    pub id: &'a str,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<UpdateOrderLineItemsProductDataPackageDimensions>,
    /// Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> UpdateOrderLineItemsProductData<'a> {
    pub fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            description: Default::default(),
            id,
            images: Default::default(),
            metadata: Default::default(),
            name,
            package_dimensions: Default::default(),
            shippable: Default::default(),
            tax_code: Default::default(),
            url: Default::default(),
        }
    }
}
/// The dimensions of this product for shipping purposes.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderLineItemsProductDataPackageDimensions {
    /// Height, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub height: f64,
    /// Length, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub length: f64,
    /// Weight, in ounces.
    ///
    /// Maximum precision is 2 decimal places.
    pub weight: f64,
    /// Width, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub width: f64,
}
impl UpdateOrderLineItemsProductDataPackageDimensions {
    pub fn new(height: f64, length: f64, weight: f64, width: f64) -> Self {
        Self { height, length, weight, width }
    }
}
/// Payment information associated with the order, including payment settings.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderPayment<'a> {
    /// Settings describing how the order should configure generated PaymentIntents.
    pub settings: UpdateOrderPaymentSettings<'a>,
}
impl<'a> UpdateOrderPayment<'a> {
    pub fn new(settings: UpdateOrderPaymentSettings<'a>) -> Self {
        Self { settings }
    }
}
/// Settings describing how the order should configure generated PaymentIntents.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettings<'a> {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateOrderPaymentSettingsPaymentMethodOptions<'a>>,
    /// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [UpdateOrderPaymentSettingsPaymentMethodTypes]>,
    /// The URL to redirect the customer to after they authenticate their payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// For non-card charges, you can use this value as the complete description that appears on your customers' statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<&'a str>,
    /// Provides configuration for completing a transfer for the order after it is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateOrderPaymentSettingsTransferData<'a>>,
}
impl<'a> UpdateOrderPaymentSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptions<'a> {
    /// If paying by `acss_debit`, this sub-hash contains details about the ACSS Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebit<'a>>,
    /// If paying by `afterpay_clearpay`, this sub-hash contains details about the AfterpayClearpay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay<'a>>,
    /// If paying by `alipay`, this sub-hash contains details about the Alipay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAlipay>,
    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsBancontact>,
    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCard>,
    /// If paying by `customer_balance`, this sub-hash contains details about the Customer Balance payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance<'a>>,
    /// If paying by `ideal`, this sub-hash contains details about the iDEAL payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsIdeal>,
    /// If paying by `klarna`, this sub-hash contains details about the Klarna payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna>,
    /// If paying by `link`, this sub-hash contains details about the Link payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsLink<'a>>,
    /// If paying by `oxxo`, this sub-hash contains details about the OXXO payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsOxxo>,
    /// If paying by `p24`, this sub-hash contains details about the P24 payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsP24>,
    /// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebit>,
    /// If paying by `sofort`, this sub-hash contains details about the Sofort payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSofort>,
    /// If paying by `wechat_pay`, this sub-hash contains details about the WeChat Pay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPay<'a>>,
}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If paying by `acss_debit`, this sub-hash contains details about the ACSS Debit payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebit<'a> {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule,
    >,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combined" => Ok(Self::Combined),
            "interval" => Ok(Self::Interval),
            "sporadic" => Ok(Self::Sporadic),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business" => Ok(Self::Business),
            "personal" => Ok(Self::Personal),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "instant" => Ok(Self::Instant),
            "microdeposits" => Ok(Self::Microdeposits),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `afterpay_clearpay`, this sub-hash contains details about the AfterpayClearpay payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpay<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with the payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `alipay`, this sub-hash contains details about the Alipay payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Preferred language of the Bancontact authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "nl" => Ok(Self::Nl),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod>,
    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsCard {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    Automatic,
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic" => Ok(Self::Automatic),
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with the payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `customer_balance`, this sub-hash contains details about the Customer Balance payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a>>,
    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalance<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<&'a [UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes]>,
    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,

}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer<'a> {
    pub fn new(
        type_: UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,
    ) -> Self {
        Self {
            eu_bank_transfer: Default::default(),
            requested_address_types: Default::default(),
            type_,
        }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<
    'a,
> {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a>
    UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer<'a>
{
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
///
/// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
    "iban" => Ok(Self::Iban),
"sepa" => Ok(Self::Sepa),
"sort_code" => Ok(Self::SortCode),
"spei" => Ok(Self::Spei),
"zengin" => Ok(Self::Zengin),

            _ => Err(())
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eu_bank_transfer" => Ok(Self::EuBankTransfer),
            "gb_bank_transfer" => Ok(Self::GbBankTransfer),
            "jp_bank_transfer" => Ok(Self::JpBankTransfer),
            "mx_bank_transfer" => Ok(Self::MxBankTransfer),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The funding method type to be used when there are not enough funds in the customer balance.
///
/// Permitted values include: `bank_transfer`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bank_transfer" => Ok(Self::BankTransfer),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `ideal`, this sub-hash contains details about the iDEAL payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `klarna`, this sub-hash contains details about the Klarna payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod>,
    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Preferred language of the Klarna authorization page that the customer is redirected to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    DaMinusDk,
    DeMinusAt,
    DeMinusCh,
    DeMinusDe,
    EnMinusAt,
    EnMinusAu,
    EnMinusBe,
    EnMinusCa,
    EnMinusCh,
    EnMinusDe,
    EnMinusDk,
    EnMinusEs,
    EnMinusFi,
    EnMinusFr,
    EnMinusGb,
    EnMinusIe,
    EnMinusIt,
    EnMinusNl,
    EnMinusNo,
    EnMinusNz,
    EnMinusPl,
    EnMinusPt,
    EnMinusSe,
    EnMinusUs,
    EsMinusEs,
    EsMinusUs,
    FiMinusFi,
    FrMinusBe,
    FrMinusCa,
    FrMinusCh,
    FrMinusFr,
    ItMinusCh,
    ItMinusIt,
    NbMinusNo,
    NlMinusBe,
    NlMinusNl,
    PlMinusPl,
    PtMinusPt,
    SvMinusFi,
    SvMinusSe,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DaMinusDk => "da-DK",
            Self::DeMinusAt => "de-AT",
            Self::DeMinusCh => "de-CH",
            Self::DeMinusDe => "de-DE",
            Self::EnMinusAt => "en-AT",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusBe => "en-BE",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusCh => "en-CH",
            Self::EnMinusDe => "en-DE",
            Self::EnMinusDk => "en-DK",
            Self::EnMinusEs => "en-ES",
            Self::EnMinusFi => "en-FI",
            Self::EnMinusFr => "en-FR",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIt => "en-IT",
            Self::EnMinusNl => "en-NL",
            Self::EnMinusNo => "en-NO",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusPl => "en-PL",
            Self::EnMinusPt => "en-PT",
            Self::EnMinusSe => "en-SE",
            Self::EnMinusUs => "en-US",
            Self::EsMinusEs => "es-ES",
            Self::EsMinusUs => "es-US",
            Self::FiMinusFi => "fi-FI",
            Self::FrMinusBe => "fr-BE",
            Self::FrMinusCa => "fr-CA",
            Self::FrMinusCh => "fr-CH",
            Self::FrMinusFr => "fr-FR",
            Self::ItMinusCh => "it-CH",
            Self::ItMinusIt => "it-IT",
            Self::NbMinusNo => "nb-NO",
            Self::NlMinusBe => "nl-BE",
            Self::NlMinusNl => "nl-NL",
            Self::PlMinusPl => "pl-PL",
            Self::PtMinusPt => "pt-PT",
            Self::SvMinusFi => "sv-FI",
            Self::SvMinusSe => "sv-SE",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "da-DK" => Ok(Self::DaMinusDk),
            "de-AT" => Ok(Self::DeMinusAt),
            "de-CH" => Ok(Self::DeMinusCh),
            "de-DE" => Ok(Self::DeMinusDe),
            "en-AT" => Ok(Self::EnMinusAt),
            "en-AU" => Ok(Self::EnMinusAu),
            "en-BE" => Ok(Self::EnMinusBe),
            "en-CA" => Ok(Self::EnMinusCa),
            "en-CH" => Ok(Self::EnMinusCh),
            "en-DE" => Ok(Self::EnMinusDe),
            "en-DK" => Ok(Self::EnMinusDk),
            "en-ES" => Ok(Self::EnMinusEs),
            "en-FI" => Ok(Self::EnMinusFi),
            "en-FR" => Ok(Self::EnMinusFr),
            "en-GB" => Ok(Self::EnMinusGb),
            "en-IE" => Ok(Self::EnMinusIe),
            "en-IT" => Ok(Self::EnMinusIt),
            "en-NL" => Ok(Self::EnMinusNl),
            "en-NO" => Ok(Self::EnMinusNo),
            "en-NZ" => Ok(Self::EnMinusNz),
            "en-PL" => Ok(Self::EnMinusPl),
            "en-PT" => Ok(Self::EnMinusPt),
            "en-SE" => Ok(Self::EnMinusSe),
            "en-US" => Ok(Self::EnMinusUs),
            "es-ES" => Ok(Self::EsMinusEs),
            "es-US" => Ok(Self::EsMinusUs),
            "fi-FI" => Ok(Self::FiMinusFi),
            "fr-BE" => Ok(Self::FrMinusBe),
            "fr-CA" => Ok(Self::FrMinusCa),
            "fr-CH" => Ok(Self::FrMinusCh),
            "fr-FR" => Ok(Self::FrMinusFr),
            "it-CH" => Ok(Self::ItMinusCh),
            "it-IT" => Ok(Self::ItMinusIt),
            "nb-NO" => Ok(Self::NbMinusNo),
            "nl-BE" => Ok(Self::NlMinusBe),
            "nl-NL" => Ok(Self::NlMinusNl),
            "pl-PL" => Ok(Self::PlMinusPl),
            "pt-PT" => Ok(Self::PtMinusPt),
            "sv-FI" => Ok(Self::SvMinusFi),
            "sv-SE" => Ok(Self::SvMinusSe),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `link`, this sub-hash contains details about the Link payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsLink<'a> {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod>,
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage>,
}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls when the funds will be captured from the customer's account.
///
/// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
///
/// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "manual" => Ok(Self::Manual),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `oxxo`, this sub-hash contains details about the OXXO payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `p24`, this sub-hash contains details about the P24 payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage>,
    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Debit payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `sofort`, this sub-hash contains details about the Sofort payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsSofort {
    /// Language shown to the payer on redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage>,
}
impl UpdateOrderPaymentSettingsPaymentMethodOptionsSofort {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Language shown to the payer on redirect.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Pl => "pl",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "de" => Ok(Self::De),
            "en" => Ok(Self::En),
            "es" => Ok(Self::Es),
            "fr" => Ok(Self::Fr),
            "it" => Ok(Self::It),
            "nl" => Ok(Self::Nl),
            "pl" => Ok(Self::Pl),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "off_session" => Ok(Self::OffSession),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// If paying by `wechat_pay`, this sub-hash contains details about the WeChat Pay payment method options to pass to the order's PaymentIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPay<'a> {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<&'a str>,
    /// The client type that the end customer will pay from.
    pub client: UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage>,
}
impl<'a> UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPay<'a> {
    pub fn new(client: UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient) -> Self {
        Self { app_id: Default::default(), client, setup_future_usage: Default::default() }
    }
}
/// The client type that the end customer will pay from.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "android" => Ok(Self::Android),
            "ios" => Ok(Self::Ios),
            "web" => Ok(Self::Web),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
///
/// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
///
/// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::AcssDebit => "acss_debit",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl std::str::FromStr for UpdateOrderPaymentSettingsPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acss_debit" => Ok(Self::AcssDebit),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "card" => Ok(Self::Card),
            "customer_balance" => Ok(Self::CustomerBalance),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "link" => Ok(Self::Link),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderPaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Provides configuration for completing a transfer for the order after it is paid.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderPaymentSettingsTransferData<'a> {
    /// The amount that will be transferred automatically when the order is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of the Connected account receiving the transfer.
    pub destination: &'a str,
}
impl<'a> UpdateOrderPaymentSettingsTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
/// Settings for the customer cost of shipping for this order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderShippingCost<'a> {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<&'a str>,
    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<UpdateOrderShippingCostShippingRateData<'a>>,
}
impl<'a> UpdateOrderShippingCost<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderShippingCostShippingRateData<'a> {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<UpdateOrderShippingCostShippingRateDataDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: &'a str,
    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<UpdateOrderShippingCostShippingRateDataFixedAmount<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateOrderShippingCostShippingRateDataTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<UpdateOrderShippingCostShippingRateDataType>,
}
impl<'a> UpdateOrderShippingCostShippingRateData<'a> {
    pub fn new(display_name: &'a str) -> Self {
        Self {
            delivery_estimate: Default::default(),
            display_name,
            fixed_amount: Default::default(),
            metadata: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            type_: Default::default(),
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimum>,
}
impl UpdateOrderShippingCostShippingRateDataDeliveryEstimate {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximum {
    pub fn new(
        unit: UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl std::str::FromStr for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business_day" => Ok(Self::BusinessDay),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimum {
    pub fn new(
        unit: UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
        value: i64,
    ) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl std::str::FromStr for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business_day" => Ok(Self::BusinessDay),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataFixedAmount<'a> {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions,
        >,
    >,
}
impl<'a> UpdateOrderShippingCostShippingRateDataFixedAmount<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, currency_options: Default::default() }
    }
}
/// Shipping rates defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}
impl UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptions {
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: Default::default() }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr
    for UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
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
impl serde::Serialize
    for UpdateOrderShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateOrderShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateOrderShippingCostShippingRateDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderShippingCostShippingRateDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderShippingCostShippingRateDataType {
    FixedAmount,
}

impl UpdateOrderShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for UpdateOrderShippingCostShippingRateDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_amount" => Ok(Self::FixedAmount),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderShippingCostShippingRateDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Shipping details for the order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderShippingDetails<'a> {
    /// The shipping address for the order.
    pub address: UpdateOrderShippingDetailsAddress<'a>,
    /// The name of the recipient of the order.
    pub name: &'a str,
    /// The phone number (including extension) for the recipient of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdateOrderShippingDetails<'a> {
    pub fn new(address: UpdateOrderShippingDetailsAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: Default::default() }
    }
}
/// The shipping address for the order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderShippingDetailsAddress<'a> {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<&'a str>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateOrderShippingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional tax details about the purchaser to be used for this order.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateOrderTaxDetails<'a> {
    /// The purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<UpdateOrderTaxDetailsTaxExempt>,
    /// The purchaser's tax IDs to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<&'a [UpdateOrderTaxDetailsTaxIds<'a>]>,
}
impl<'a> UpdateOrderTaxDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The purchaser's tax exemption status.
///
/// One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateOrderTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl UpdateOrderTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for UpdateOrderTaxDetailsTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exempt" => Ok(Self::Exempt),
            "none" => Ok(Self::None),
            "reverse" => Ok(Self::Reverse),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderTaxDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The purchaser's tax IDs to be used for this order.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateOrderTaxDetailsTaxIds<'a> {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: UpdateOrderTaxDetailsTaxIdsType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> UpdateOrderTaxDetailsTaxIds<'a> {
    pub fn new(type_: UpdateOrderTaxDetailsTaxIdsType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    PhTin,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    ZaVat,
}

impl UpdateOrderTaxDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AeTrn => "ae_trn",
            Self::AuAbn => "au_abn",
            Self::AuArn => "au_arn",
            Self::BgUic => "bg_uic",
            Self::BrCnpj => "br_cnpj",
            Self::BrCpf => "br_cpf",
            Self::CaBn => "ca_bn",
            Self::CaGstHst => "ca_gst_hst",
            Self::CaPstBc => "ca_pst_bc",
            Self::CaPstMb => "ca_pst_mb",
            Self::CaPstSk => "ca_pst_sk",
            Self::CaQst => "ca_qst",
            Self::ChVat => "ch_vat",
            Self::ClTin => "cl_tin",
            Self::EgTin => "eg_tin",
            Self::EsCif => "es_cif",
            Self::EuOssVat => "eu_oss_vat",
            Self::EuVat => "eu_vat",
            Self::GbVat => "gb_vat",
            Self::GeVat => "ge_vat",
            Self::HkBr => "hk_br",
            Self::HuTin => "hu_tin",
            Self::IdNpwp => "id_npwp",
            Self::IlVat => "il_vat",
            Self::InGst => "in_gst",
            Self::IsVat => "is_vat",
            Self::JpCn => "jp_cn",
            Self::JpRn => "jp_rn",
            Self::JpTrn => "jp_trn",
            Self::KePin => "ke_pin",
            Self::KrBrn => "kr_brn",
            Self::LiUid => "li_uid",
            Self::MxRfc => "mx_rfc",
            Self::MyFrp => "my_frp",
            Self::MyItn => "my_itn",
            Self::MySst => "my_sst",
            Self::NoVat => "no_vat",
            Self::NzGst => "nz_gst",
            Self::PhTin => "ph_tin",
            Self::RuInn => "ru_inn",
            Self::RuKpp => "ru_kpp",
            Self::SaVat => "sa_vat",
            Self::SgGst => "sg_gst",
            Self::SgUen => "sg_uen",
            Self::SiTin => "si_tin",
            Self::ThVat => "th_vat",
            Self::TrTin => "tr_tin",
            Self::TwVat => "tw_vat",
            Self::UaVat => "ua_vat",
            Self::UsEin => "us_ein",
            Self::ZaVat => "za_vat",
        }
    }
}

impl std::str::FromStr for UpdateOrderTaxDetailsTaxIdsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ae_trn" => Ok(Self::AeTrn),
            "au_abn" => Ok(Self::AuAbn),
            "au_arn" => Ok(Self::AuArn),
            "bg_uic" => Ok(Self::BgUic),
            "br_cnpj" => Ok(Self::BrCnpj),
            "br_cpf" => Ok(Self::BrCpf),
            "ca_bn" => Ok(Self::CaBn),
            "ca_gst_hst" => Ok(Self::CaGstHst),
            "ca_pst_bc" => Ok(Self::CaPstBc),
            "ca_pst_mb" => Ok(Self::CaPstMb),
            "ca_pst_sk" => Ok(Self::CaPstSk),
            "ca_qst" => Ok(Self::CaQst),
            "ch_vat" => Ok(Self::ChVat),
            "cl_tin" => Ok(Self::ClTin),
            "eg_tin" => Ok(Self::EgTin),
            "es_cif" => Ok(Self::EsCif),
            "eu_oss_vat" => Ok(Self::EuOssVat),
            "eu_vat" => Ok(Self::EuVat),
            "gb_vat" => Ok(Self::GbVat),
            "ge_vat" => Ok(Self::GeVat),
            "hk_br" => Ok(Self::HkBr),
            "hu_tin" => Ok(Self::HuTin),
            "id_npwp" => Ok(Self::IdNpwp),
            "il_vat" => Ok(Self::IlVat),
            "in_gst" => Ok(Self::InGst),
            "is_vat" => Ok(Self::IsVat),
            "jp_cn" => Ok(Self::JpCn),
            "jp_rn" => Ok(Self::JpRn),
            "jp_trn" => Ok(Self::JpTrn),
            "ke_pin" => Ok(Self::KePin),
            "kr_brn" => Ok(Self::KrBrn),
            "li_uid" => Ok(Self::LiUid),
            "mx_rfc" => Ok(Self::MxRfc),
            "my_frp" => Ok(Self::MyFrp),
            "my_itn" => Ok(Self::MyItn),
            "my_sst" => Ok(Self::MySst),
            "no_vat" => Ok(Self::NoVat),
            "nz_gst" => Ok(Self::NzGst),
            "ph_tin" => Ok(Self::PhTin),
            "ru_inn" => Ok(Self::RuInn),
            "ru_kpp" => Ok(Self::RuKpp),
            "sa_vat" => Ok(Self::SaVat),
            "sg_gst" => Ok(Self::SgGst),
            "sg_uen" => Ok(Self::SgUen),
            "si_tin" => Ok(Self::SiTin),
            "th_vat" => Ok(Self::ThVat),
            "tr_tin" => Ok(Self::TrTin),
            "tw_vat" => Ok(Self::TwVat),
            "ua_vat" => Ok(Self::UaVat),
            "us_ein" => Ok(Self::UsEin),
            "za_vat" => Ok(Self::ZaVat),

            _ => Err(()),
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
impl serde::Serialize for UpdateOrderTaxDetailsTaxIdsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SubmitOrder<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// `expected_total` should always be set to the order's `amount_total` field.
    ///
    /// If they don't match, submitting the order will fail.
    /// This helps detect race conditions where something else concurrently modifies the order.
    pub expected_total: i64,
}
impl<'a> SubmitOrder<'a> {
    pub fn new(expected_total: i64) -> Self {
        Self { expand: Default::default(), expected_total }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelOrder<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReopenOrder<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ReopenOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsOrder<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListLineItemsOrder<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
