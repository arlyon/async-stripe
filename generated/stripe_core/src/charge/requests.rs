use stripe::{Client, Response};

impl stripe_core::charge::Charge {
    /// Search for charges you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    ///
    /// Under normal operating conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up to an hour behind during outages.
    /// Search functionality is not available to merchants in India.
    pub fn search(client: &Client, params: SearchCharge) -> Response<SearchReturned> {
        client.get_query("/charges/search", params)
    }
    /// Returns a list of charges you’ve previously created.
    ///
    /// The charges are returned in sorted order, with the most recent charges appearing first.
    pub fn list(
        client: &Client,
        params: ListCharge,
    ) -> Response<stripe_types::List<stripe_core::charge::Charge>> {
        client.get_query("/charges", params)
    }
    /// To charge a credit card or other payment source, you create a `Charge` object.
    ///
    /// If your API key is in test mode, the supplied payment source (e.g., card) won’t actually be charged, although everything else will occur as if in live mode.
    /// (Stripe assumes that the charge would have completed successfully).
    pub fn create(client: &Client, params: CreateCharge) -> Response<stripe_core::charge::Charge> {
        client.send_form("/charges", params, http_types::Method::Post)
    }
    /// Retrieves the details of a charge that has previously been created.
    ///
    /// Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information.
    /// The same information is returned when creating or refunding the charge.
    pub fn retrieve(
        client: &Client,
        charge: &stripe_core::charge::ChargeId,
        params: RetrieveCharge,
    ) -> Response<stripe_core::charge::Charge> {
        client.get_query(&format!("/charges/{charge}", charge = charge), params)
    }
    /// Updates the specified charge by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(
        client: &Client,
        charge: &stripe_core::charge::ChargeId,
        params: UpdateCharge,
    ) -> Response<stripe_core::charge::Charge> {
        client.send_form(
            &format!("/charges/{charge}", charge = charge),
            params,
            http_types::Method::Post,
        )
    }
    /// Capture the payment of an existing, uncaptured, charge.
    ///
    /// This is the second half of the two-step payment flow, where first you [created a charge](https://stripe.com/docs/api#create_charge) with the capture option set to false.  Uncaptured payments expire a set number of days after they are created ([7 by default](https://stripe.com/docs/charges/placing-a-hold)).
    /// If they are not captured by that point in time, they will be marked as refunded and will no longer be capturable.
    pub fn capture(
        client: &Client,
        charge: &stripe_core::charge::ChargeId,
        params: CaptureCharge,
    ) -> Response<stripe_core::charge::Charge> {
        client.send_form(
            &format!("/charges/{charge}/capture", charge = charge),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SearchReturned {
    pub data: Vec<stripe_core::charge::Charge>,
    pub has_more: bool,
    pub next_page: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SearchReturnedObject,
    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SearchReturned {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum SearchReturnedObject {
    SearchResult,
}

impl SearchReturnedObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SearchResult => "search_result",
        }
    }
}

impl AsRef<str> for SearchReturnedObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SearchReturnedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchCharge<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for pagination across multiple pages of results.
    ///
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<&'a str>,
    /// The search query string.
    ///
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for charges](https://stripe.com/docs/search#query-fields-for-charges).
    pub query: &'a str,
}
impl<'a> SearchCharge<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            expand: Default::default(),
            limit: Default::default(),
            page: Default::default(),
            query,
        }
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCharge<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return charges for the customer specified by this customer ID.
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
    /// Only return charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    /// Only return charges for this transfer group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> ListCharge<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    pub currency: Option<stripe_types::Currency>,
    /// The ID of an existing customer that will be charged in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string which you can attach to a `Charge` object.
    ///
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<CreateChargeDestination<'a>>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The Stripe account ID for which these funds are intended.
    ///
    /// Automatically set if you use the `destination` parameter.
    /// For details, see [Creating Separate Charges and Transfers](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreateChargeRadarOptions<'a>>,
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
    pub shipping: Option<CreateChargeShipping<'a>>,
    /// A payment source to be charged.
    ///
    /// This can be the ID of a [card](https://stripe.com/docs/api#cards) (i.e., credit or debit card), a [bank account](https://stripe.com/docs/api#bank_accounts), a [source](https://stripe.com/docs/api#sources), a [token](https://stripe.com/docs/api#tokens), or a [connected account](https://stripe.com/docs/connect/account-debits#charging-a-connected-account).
    /// For certain sources---namely, [cards](https://stripe.com/docs/api#cards), [bank accounts](https://stripe.com/docs/api#bank_accounts), and attached [sources](https://stripe.com/docs/api#sources)---you must also pass the ID of the associated customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
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
    pub transfer_data: Option<CreateChargeTransferData<'a>>,
    /// A string that identifies this transaction as part of a group.
    ///
    /// For details, see [Grouping transactions](https://stripe.com/docs/connect/charges-transfers#transfer-options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> CreateCharge<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateChargeDestination<'a> {
    /// ID of an existing, connected Stripe account.
    pub account: &'a str,
    /// The amount to transfer to the destination account without creating an `Application Fee` object.
    ///
    /// Cannot be combined with the `application_fee` parameter.
    /// Must be less than or equal to the charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl<'a> CreateChargeDestination<'a> {
    pub fn new(account: &'a str) -> Self {
        Self { account, amount: Default::default() }
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateChargeRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> CreateChargeRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Shipping information for the charge.
///
/// Helps prevent fraud on charges for physical goods.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateChargeShipping<'a> {
    /// Shipping address.
    pub address: CreateChargeShippingAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> CreateChargeShipping<'a> {
    pub fn new(address: CreateChargeShippingAddress<'a>, name: &'a str) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name,
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateChargeShippingAddress<'a> {
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
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateChargeShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An optional dictionary including the account to automatically transfer to as part of a destination charge.
///
/// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateChargeTransferData<'a> {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateChargeTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), destination }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCharge<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCharge<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCharge<'a> {
    /// The ID of an existing customer that will be associated with this request.
    ///
    /// This field may only be updated if there is no existing associated customer with this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string which you can attach to a charge object.
    ///
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A set of key-value pairs you can attach to a charge giving information about its riskiness.
    ///
    /// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
    /// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
    /// Stripe will use the information you send to improve our fraud detection algorithms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<UpdateChargeFraudDetails>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// This is the email address that the receipt for this charge will be sent to.
    ///
    /// If this field is updated, then a new email receipt will be sent to the updated address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    /// Shipping information for the charge.
    ///
    /// Helps prevent fraud on charges for physical goods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<UpdateChargeShipping<'a>>,
    /// A string that identifies this transaction as part of a group.
    ///
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> UpdateCharge<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A set of key-value pairs you can attach to a charge giving information about its riskiness.
///
/// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
/// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
/// Stripe will use the information you send to improve our fraud detection algorithms.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateChargeFraudDetails {
    /// Either `safe` or `fraudulent`.
    pub user_report: UpdateChargeFraudDetailsUserReport,
}
impl UpdateChargeFraudDetails {
    pub fn new(user_report: UpdateChargeFraudDetailsUserReport) -> Self {
        Self { user_report }
    }
}
/// Either `safe` or `fraudulent`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateChargeFraudDetailsUserReport {
    Fraudulent,
    Safe,
}

impl UpdateChargeFraudDetailsUserReport {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fraudulent => "fraudulent",
            Self::Safe => "safe",
        }
    }
}

impl AsRef<str> for UpdateChargeFraudDetailsUserReport {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateChargeFraudDetailsUserReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Shipping information for the charge.
///
/// Helps prevent fraud on charges for physical goods.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateChargeShipping<'a> {
    /// Shipping address.
    pub address: UpdateChargeShippingAddress<'a>,
    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// Recipient name.
    pub name: &'a str,
    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<&'a str>,
}
impl<'a> UpdateChargeShipping<'a> {
    pub fn new(address: UpdateChargeShippingAddress<'a>, name: &'a str) -> Self {
        Self {
            address,
            carrier: Default::default(),
            name,
            phone: Default::default(),
            tracking_number: Default::default(),
        }
    }
}
/// Shipping address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateChargeShippingAddress<'a> {
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
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> UpdateChargeShippingAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureCharge<'a> {
    /// The amount to capture, which must be less than or equal to the original amount.
    ///
    /// Any additional amount will be automatically refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// An application fee to add on to this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<i64>,
    /// An application fee amount to add on to this charge, which must be less than or equal to the original amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The email address to send this charge's receipt to.
    ///
    /// This will override the previously-specified email address for this charge, if one was set.
    /// Receipts will not be sent in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
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
    pub transfer_data: Option<CaptureChargeTransferData>,
    /// A string that identifies this transaction as part of a group.
    ///
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
}
impl<'a> CaptureCharge<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An optional dictionary including the account to automatically transfer to as part of a destination charge.
///
/// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureChargeTransferData {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
impl CaptureChargeTransferData {
    pub fn new() -> Self {
        Self::default()
    }
}
