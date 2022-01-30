// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::PaymentLinkId;
use crate::params::{Expand, Expandable, List, Metadata, Object};
use crate::resources::{Account, CheckoutSessionItem};

/// The resource representing a Stripe "PaymentLink".
///
/// For more details see <https://stripe.com/docs/api/payment_links/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLink {
    /// Unique identifier for the object.
    pub id: PaymentLinkId,

    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,

    pub after_completion: PaymentLinksResourceAfterCompletion,

    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,

    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<Box<i64>>,

    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<Box<f64>>,

    pub automatic_tax: PaymentLinksResourceAutomaticTax,

    /// Configuration for collecting the customer's billing address.
    pub billing_address_collection: PaymentLinkBillingAddressCollection,

    /// The line items representing what is being sold.
    #[serde(default)]
    pub line_items: List<CheckoutSessionItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Box<Expandable<Account>>>,

    /// The list of payment method types that customers can use.
    ///
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Box<Vec<PaymentLinkPaymentMethodTypes>>>,

    pub phone_number_collection: PaymentLinksResourcePhoneNumberCollection,

    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<Box<PaymentLinksResourceShippingAddressCollection>>,

    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<Box<PaymentLinksResourceSubscriptionData>>,

    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<PaymentLinksResourceTransferData>>,

    /// The public URL that can be shared with customers.
    pub url: String,
}

impl PaymentLink {
    /// Returns a list of your payment links.
    pub fn list(client: &Client, params: ListPaymentLinks<'_>) -> Response<List<PaymentLink>> {
        client.get_query("/payment_links", &params)
    }

    /// Creates a payment link.
    pub fn create(client: &Client, params: CreatePaymentLink<'_>) -> Response<PaymentLink> {
        client.post_form("/payment_links", &params)
    }

    /// Retrieve a payment link.
    pub fn retrieve(client: &Client, id: &PaymentLinkId, expand: &[&str]) -> Response<PaymentLink> {
        client.get_query(&format!("/payment_links/{}", id), &Expand { expand })
    }

    /// Updates a payment link.
    pub fn update(
        client: &Client,
        id: &PaymentLinkId,
        params: UpdatePaymentLink<'_>,
    ) -> Response<PaymentLink> {
        client.post_form(&format!("/payment_links/{}", id), &params)
    }
}

impl Object for PaymentLink {
    type Id = PaymentLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_link"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceAfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<Box<PaymentLinksResourceCompletionBehaviorConfirmationPage>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Box<PaymentLinksResourceCompletionBehaviorRedirect>>,

    /// The specified behavior after the purchase is complete.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceAfterCompletionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    /// The custom message that is displayed to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourcePhoneNumberCollection {
    /// If `true`, a phone number will be collected during checkout.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<PaymentLinksResourceShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceSubscriptionData {
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<Box<u32>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceTransferData {
    /// The amount in %s that will be transferred to the destination account.
    ///
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    /// The connected account receiving the transfer.
    pub destination: Expandable<Account>,
}

/// The parameters for `PaymentLink::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreatePaymentLink<'a> {
    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<Box<CreatePaymentLinkAfterCompletion>>,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Can only be applied when there are no line items with recurring prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<CreatePaymentLinkAutomaticTax>>,

    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<PaymentLinkBillingAddressCollection>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Box<Vec<CreatePaymentLinkLineItems>>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// The list of payment method types that customers can use.
    ///
    /// Only `card` is supported.
    /// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Box<Vec<CreatePaymentLinkPaymentMethodTypes>>>,

    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<Box<CreatePaymentLinkPhoneNumberCollection>>,

    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<Box<CreatePaymentLinkShippingAddressCollection>>,

    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<Box<CreatePaymentLinkSubscriptionData>>,

    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<CreatePaymentLinkTransferData>>,
}

impl<'a> CreatePaymentLink<'a> {
    pub fn new() -> Self {
        CreatePaymentLink {
            after_completion: Default::default(),
            allow_promotion_codes: Default::default(),
            application_fee_amount: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            expand: Default::default(),
            line_items: Default::default(),
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            shipping_address_collection: Default::default(),
            subscription_data: Default::default(),
            transfer_data: Default::default(),
        }
    }
}

/// The parameters for `PaymentLink::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPaymentLinks<'a> {
    /// Only return payment links that are active or inactive (e.g., pass `false` to list all inactive payment links).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PaymentLinkId>,

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
    pub starting_after: Option<PaymentLinkId>,
}

impl<'a> ListPaymentLinks<'a> {
    pub fn new() -> Self {
        ListPaymentLinks {
            active: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `PaymentLink::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePaymentLink<'a> {
    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<Box<UpdatePaymentLinkAfterCompletion>>,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<UpdatePaymentLinkAutomaticTax>>,

    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<PaymentLinkBillingAddressCollection>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Box<Vec<UpdatePaymentLinkLineItems>>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The list of payment method types that customers can use.
    ///
    /// Only `card` is supported.
    /// Pass an empty string to enable automatic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Box<Vec<UpdatePaymentLinkPaymentMethodTypes>>>,

    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<Box<UpdatePaymentLinkShippingAddressCollection>>,
}

impl<'a> UpdatePaymentLink<'a> {
    pub fn new() -> Self {
        UpdatePaymentLink {
            active: Default::default(),
            after_completion: Default::default(),
            allow_promotion_codes: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            expand: Default::default(),
            line_items: Default::default(),
            metadata: Default::default(),
            payment_method_types: Default::default(),
            shipping_address_collection: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<Box<CreatePaymentLinkAfterCompletionHostedConfirmation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Box<CreatePaymentLinkAfterCompletionRedirect>>,

    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkAfterCompletionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkLineItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<Box<CreatePaymentLinkLineItemsAdjustableQuantity>>,

    pub price: String,

    pub quantity: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkPhoneNumberCollection {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkShippingAddressCollection {
    pub allowed_countries: Vec<CreatePaymentLinkShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkSubscriptionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<Box<u32>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<Box<UpdatePaymentLinkAfterCompletionHostedConfirmation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<Box<UpdatePaymentLinkAfterCompletionRedirect>>,

    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkAfterCompletionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkLineItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<Box<UpdatePaymentLinkLineItemsAdjustableQuantity>>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkShippingAddressCollection {
    pub allowed_countries: Vec<UpdatePaymentLinkShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAfterCompletionHostedConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAfterCompletionRedirect {
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkLineItemsAdjustableQuantity {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Box<i64>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAfterCompletionHostedConfirmation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<Box<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAfterCompletionRedirect {
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkLineItemsAdjustableQuantity {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Box<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Box<i64>>,
}

/// An enum representing the possible values of an `CreatePaymentLinkAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl CreatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            CreatePaymentLinkAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `CreatePaymentLink`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentMethodTypes {
    Card,
}

impl CreatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkPaymentMethodTypes::Card => "card",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkPaymentMethodTypes {
    fn default() -> Self {
        Self::Card
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkShippingAddressCollectionAllowedCountries {
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

impl CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ac => "AC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ad => "AD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ae => "AE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Af => "AF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ag => "AG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ai => "AI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Al => "AL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Am => "AM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ao => "AO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Aq => "AQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ar => "AR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::At => "AT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Au => "AU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Aw => "AW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ax => "AX",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Az => "AZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ba => "BA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bb => "BB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bd => "BD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Be => "BE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bf => "BF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bg => "BG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bh => "BH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bi => "BI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bj => "BJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bl => "BL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bm => "BM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bn => "BN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bo => "BO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bq => "BQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Br => "BR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bs => "BS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bt => "BT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bv => "BV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bw => "BW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::By => "BY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bz => "BZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ca => "CA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cd => "CD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cf => "CF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cg => "CG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ch => "CH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ci => "CI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ck => "CK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cl => "CL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cm => "CM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cn => "CN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Co => "CO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cr => "CR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cv => "CV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cw => "CW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cy => "CY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cz => "CZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::De => "DE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dj => "DJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dk => "DK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dm => "DM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Do => "DO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dz => "DZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ec => "EC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ee => "EE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Eg => "EG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Eh => "EH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Er => "ER",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Es => "ES",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Et => "ET",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fi => "FI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fj => "FJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fk => "FK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fo => "FO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fr => "FR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ga => "GA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gb => "GB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gd => "GD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ge => "GE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gf => "GF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gg => "GG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gh => "GH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gi => "GI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gl => "GL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gm => "GM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gn => "GN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gp => "GP",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gq => "GQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gr => "GR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gs => "GS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gt => "GT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gu => "GU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gw => "GW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gy => "GY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hk => "HK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hn => "HN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hr => "HR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ht => "HT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hu => "HU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Id => "ID",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ie => "IE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Il => "IL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Im => "IM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::In => "IN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Io => "IO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Iq => "IQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Is => "IS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::It => "IT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Je => "JE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Jm => "JM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Jo => "JO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Jp => "JP",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ke => "KE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kg => "KG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kh => "KH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ki => "KI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Km => "KM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kn => "KN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kr => "KR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kw => "KW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ky => "KY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kz => "KZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::La => "LA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lb => "LB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lc => "LC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Li => "LI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lk => "LK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lr => "LR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ls => "LS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lt => "LT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lu => "LU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lv => "LV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ly => "LY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ma => "MA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mc => "MC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Md => "MD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Me => "ME",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mf => "MF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mg => "MG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mk => "MK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ml => "ML",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mm => "MM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mn => "MN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mo => "MO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mq => "MQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mr => "MR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ms => "MS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mt => "MT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mu => "MU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mv => "MV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mw => "MW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mx => "MX",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::My => "MY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mz => "MZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Na => "NA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nc => "NC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ne => "NE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ng => "NG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ni => "NI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nl => "NL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::No => "NO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Np => "NP",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nr => "NR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nu => "NU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nz => "NZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Om => "OM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pa => "PA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pe => "PE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pf => "PF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pg => "PG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ph => "PH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pk => "PK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pl => "PL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pm => "PM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pn => "PN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pr => "PR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ps => "PS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pt => "PT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Py => "PY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Qa => "QA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Re => "RE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ro => "RO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Rs => "RS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ru => "RU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Rw => "RW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sa => "SA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sb => "SB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sc => "SC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Se => "SE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sg => "SG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sh => "SH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Si => "SI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sj => "SJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sk => "SK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sl => "SL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sm => "SM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sn => "SN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::So => "SO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sr => "SR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ss => "SS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::St => "ST",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sv => "SV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sx => "SX",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sz => "SZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ta => "TA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tc => "TC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Td => "TD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tf => "TF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tg => "TG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Th => "TH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tj => "TJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tk => "TK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tl => "TL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tm => "TM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tn => "TN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::To => "TO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tr => "TR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tt => "TT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tv => "TV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tw => "TW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tz => "TZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ua => "UA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ug => "UG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Us => "US",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Uy => "UY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Uz => "UZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Va => "VA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vc => "VC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ve => "VE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vg => "VG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vn => "VN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vu => "VU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Wf => "WF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ws => "WS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Xk => "XK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ye => "YE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Yt => "YT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Za => "ZA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Zm => "ZM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Zw => "ZW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn default() -> Self {
        Self::Ac
    }
}

/// An enum representing the possible values of an `PaymentLink`'s `billing_address_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl PaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinkBillingAddressCollection::Auto => "auto",
            PaymentLinkBillingAddressCollection::Required => "required",
        }
    }
}

impl AsRef<str> for PaymentLinkBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinkBillingAddressCollection {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `PaymentLink`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkPaymentMethodTypes {
    Card,
}

impl PaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinkPaymentMethodTypes::Card => "card",
        }
    }
}

impl AsRef<str> for PaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinkPaymentMethodTypes {
    fn default() -> Self {
        Self::Card
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl PaymentLinksResourceAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            PaymentLinksResourceAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceShippingAddressCollectionAllowedCountries {
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

impl PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ac => "AC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ad => "AD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ae => "AE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Af => "AF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ag => "AG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ai => "AI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Al => "AL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Am => "AM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ao => "AO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Aq => "AQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ar => "AR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::At => "AT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Au => "AU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Aw => "AW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ax => "AX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Az => "AZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ba => "BA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bb => "BB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bd => "BD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Be => "BE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bf => "BF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bg => "BG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bh => "BH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bi => "BI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bj => "BJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bl => "BL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bm => "BM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bn => "BN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bo => "BO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bq => "BQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Br => "BR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bs => "BS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bt => "BT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bv => "BV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bw => "BW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::By => "BY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bz => "BZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ca => "CA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cd => "CD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cf => "CF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cg => "CG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ch => "CH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ci => "CI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ck => "CK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cl => "CL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cm => "CM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cn => "CN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Co => "CO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cr => "CR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cv => "CV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cw => "CW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cy => "CY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cz => "CZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::De => "DE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dj => "DJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dk => "DK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dm => "DM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Do => "DO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dz => "DZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ec => "EC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ee => "EE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Eg => "EG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Eh => "EH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Er => "ER",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Es => "ES",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Et => "ET",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fi => "FI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fj => "FJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fk => "FK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fo => "FO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fr => "FR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ga => "GA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gb => "GB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gd => "GD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ge => "GE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gf => "GF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gg => "GG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gh => "GH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gi => "GI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gl => "GL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gm => "GM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gn => "GN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gp => "GP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gq => "GQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gr => "GR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gs => "GS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gt => "GT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gu => "GU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gw => "GW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gy => "GY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hk => "HK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hn => "HN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hr => "HR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ht => "HT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hu => "HU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Id => "ID",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ie => "IE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Il => "IL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Im => "IM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::In => "IN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Io => "IO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Iq => "IQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Is => "IS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::It => "IT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Je => "JE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Jm => "JM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Jo => "JO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Jp => "JP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ke => "KE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kg => "KG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kh => "KH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ki => "KI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Km => "KM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kn => "KN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kr => "KR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kw => "KW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ky => "KY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kz => "KZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::La => "LA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lb => "LB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lc => "LC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Li => "LI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lk => "LK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lr => "LR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ls => "LS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lt => "LT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lu => "LU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lv => "LV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ly => "LY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ma => "MA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mc => "MC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Md => "MD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Me => "ME",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mf => "MF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mg => "MG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mk => "MK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ml => "ML",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mm => "MM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mn => "MN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mo => "MO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mq => "MQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mr => "MR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ms => "MS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mt => "MT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mu => "MU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mv => "MV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mw => "MW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mx => "MX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::My => "MY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mz => "MZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Na => "NA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nc => "NC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ne => "NE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ng => "NG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ni => "NI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nl => "NL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::No => "NO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Np => "NP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nr => "NR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nu => "NU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nz => "NZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Om => "OM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pa => "PA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pe => "PE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pf => "PF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pg => "PG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ph => "PH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pk => "PK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pl => "PL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pm => "PM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pn => "PN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pr => "PR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ps => "PS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pt => "PT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Py => "PY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Qa => "QA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Re => "RE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ro => "RO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Rs => "RS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ru => "RU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Rw => "RW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sa => "SA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sb => "SB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sc => "SC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Se => "SE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sg => "SG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sh => "SH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Si => "SI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sj => "SJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sk => "SK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sl => "SL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sm => "SM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sn => "SN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::So => "SO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sr => "SR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ss => "SS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::St => "ST",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sv => "SV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sx => "SX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sz => "SZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ta => "TA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tc => "TC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Td => "TD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tf => "TF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tg => "TG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Th => "TH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tj => "TJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tk => "TK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tl => "TL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tm => "TM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tn => "TN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::To => "TO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tr => "TR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tt => "TT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tv => "TV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tw => "TW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tz => "TZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ua => "UA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ug => "UG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Us => "US",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Uy => "UY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Uz => "UZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Va => "VA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vc => "VC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ve => "VE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vg => "VG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vn => "VN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vu => "VU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Wf => "WF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ws => "WS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Xk => "XK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ye => "YE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Yt => "YT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Za => "ZA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Zm => "ZM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Zw => "ZW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    fn default() -> Self {
        Self::Ac
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl UpdatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            UpdatePaymentLinkAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `UpdatePaymentLink`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkPaymentMethodTypes {
    Card,
}

impl UpdatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkPaymentMethodTypes::Card => "card",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkPaymentMethodTypes {
    fn default() -> Self {
        Self::Card
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
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

impl UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ac => "AC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ad => "AD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ae => "AE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Af => "AF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ag => "AG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ai => "AI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Al => "AL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Am => "AM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ao => "AO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Aq => "AQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ar => "AR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::At => "AT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Au => "AU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Aw => "AW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ax => "AX",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Az => "AZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ba => "BA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bb => "BB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bd => "BD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Be => "BE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bf => "BF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bg => "BG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bh => "BH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bi => "BI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bj => "BJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bl => "BL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bm => "BM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bn => "BN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bo => "BO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bq => "BQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Br => "BR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bs => "BS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bt => "BT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bv => "BV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bw => "BW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::By => "BY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bz => "BZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ca => "CA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cd => "CD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cf => "CF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cg => "CG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ch => "CH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ci => "CI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ck => "CK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cl => "CL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cm => "CM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cn => "CN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Co => "CO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cr => "CR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cv => "CV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cw => "CW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cy => "CY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cz => "CZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::De => "DE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dj => "DJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dk => "DK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dm => "DM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Do => "DO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dz => "DZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ec => "EC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ee => "EE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Eg => "EG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Eh => "EH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Er => "ER",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Es => "ES",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Et => "ET",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fi => "FI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fj => "FJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fk => "FK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fo => "FO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fr => "FR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ga => "GA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gb => "GB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gd => "GD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ge => "GE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gf => "GF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gg => "GG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gh => "GH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gi => "GI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gl => "GL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gm => "GM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gn => "GN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gp => "GP",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gq => "GQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gr => "GR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gs => "GS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gt => "GT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gu => "GU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gw => "GW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gy => "GY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hk => "HK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hn => "HN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hr => "HR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ht => "HT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hu => "HU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Id => "ID",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ie => "IE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Il => "IL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Im => "IM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::In => "IN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Io => "IO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Iq => "IQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Is => "IS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::It => "IT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Je => "JE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Jm => "JM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Jo => "JO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Jp => "JP",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ke => "KE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kg => "KG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kh => "KH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ki => "KI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Km => "KM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kn => "KN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kr => "KR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kw => "KW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ky => "KY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kz => "KZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::La => "LA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lb => "LB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lc => "LC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Li => "LI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lk => "LK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lr => "LR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ls => "LS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lt => "LT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lu => "LU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lv => "LV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ly => "LY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ma => "MA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mc => "MC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Md => "MD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Me => "ME",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mf => "MF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mg => "MG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mk => "MK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ml => "ML",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mm => "MM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mn => "MN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mo => "MO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mq => "MQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mr => "MR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ms => "MS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mt => "MT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mu => "MU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mv => "MV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mw => "MW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mx => "MX",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::My => "MY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mz => "MZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Na => "NA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nc => "NC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ne => "NE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ng => "NG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ni => "NI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nl => "NL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::No => "NO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Np => "NP",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nr => "NR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nu => "NU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nz => "NZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Om => "OM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pa => "PA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pe => "PE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pf => "PF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pg => "PG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ph => "PH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pk => "PK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pl => "PL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pm => "PM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pn => "PN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pr => "PR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ps => "PS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pt => "PT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Py => "PY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Qa => "QA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Re => "RE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ro => "RO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Rs => "RS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ru => "RU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Rw => "RW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sa => "SA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sb => "SB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sc => "SC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Se => "SE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sg => "SG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sh => "SH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Si => "SI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sj => "SJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sk => "SK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sl => "SL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sm => "SM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sn => "SN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::So => "SO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sr => "SR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ss => "SS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::St => "ST",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sv => "SV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sx => "SX",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sz => "SZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ta => "TA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tc => "TC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Td => "TD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tf => "TF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tg => "TG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Th => "TH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tj => "TJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tk => "TK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tl => "TL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tm => "TM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tn => "TN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::To => "TO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tr => "TR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tt => "TT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tv => "TV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tw => "TW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tz => "TZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ua => "UA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ug => "UG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Us => "US",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Uy => "UY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Uz => "UZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Va => "VA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vc => "VC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ve => "VE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vg => "VG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vn => "VN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vu => "VU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Wf => "WF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ws => "WS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Xk => "XK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ye => "YE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Yt => "YT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Za => "ZA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Zm => "ZM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Zw => "ZW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn default() -> Self {
        Self::Ac
    }
}
