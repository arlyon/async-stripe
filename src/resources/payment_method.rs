// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, PaymentMethodId};
use crate::params::{Expand, Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{Address, BillingDetails, Customer, PaymentMethodDetails};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "PaymentMethod".
///
/// For more details see [https://stripe.com/docs/api/payment_methods/object](https://stripe.com/docs/api/payment_methods/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethod {
    /// Unique identifier for the object.
    pub id: PaymentMethodId,

    pub billing_details: BillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CardPresent>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
}

impl PaymentMethod {
    /// Returns a list of PaymentMethods for a given Customer.
    pub fn list(client: &Client, params: ListPaymentMethods<'_>) -> Response<List<PaymentMethod>> {
        client.get_query("/payment_methods", &params)
    }

    /// Creates a PaymentMethod object.
    ///
    /// Read the [Stripe.js reference](https://stripe.com/docs/stripe-js/reference#stripe-create-payment-method) to learn how to create PaymentMethods via Stripe.js.
    pub fn create(client: &Client, params: CreatePaymentMethod<'_>) -> Response<PaymentMethod> {
        client.post_form("/payment_methods", &params)
    }

    /// Retrieves a PaymentMethod object.
    pub fn retrieve(
        client: &Client,
        id: &PaymentMethodId,
        expand: &[&str],
    ) -> Response<PaymentMethod> {
        client.get_query(&format!("/payment_methods/{}", id), &Expand { expand })
    }

    /// Updates a PaymentMethod object.
    ///
    /// A PaymentMethod must be attached a customer to be updated.
    pub fn update(
        client: &Client,
        id: &PaymentMethodId,
        params: UpdatePaymentMethod<'_>,
    ) -> Response<PaymentMethod> {
        client.post_form(&format!("/payment_methods/{}", id), &params)
    }
}

impl Object for PaymentMethod {
    type Id = PaymentMethodId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_method"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardDetails {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: String,

    /// Checks on Card address and CVC if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<PaymentMethodCardChecks>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who've signed up with you are using the same card number, for example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: String,

    /// Details of the original PaymentMethod that created this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_from: Option<PaymentMethodCardGeneratedCard>,

    /// The last four digits of the card.
    pub last4: String,

    /// Contains details on how this Card maybe be used for 3D Secure authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_usage: Option<ThreeDSecureUsage>,

    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<WalletDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodCardChecks {
    /// If a address line1 was provided, results of the check, one of 'pass', 'failed', 'unavailable' or 'unchecked'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,

    /// If a address postal code was provided, results of the check, one of 'pass', 'failed', 'unavailable' or 'unchecked'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check: Option<String>,

    /// If a CVC was provided, results of the check, one of 'pass', 'failed', 'unavailable' or 'unchecked'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentMethodCardGeneratedCard {
    /// The charge that created this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    /// Transaction-specific details of the payment method used in the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<PaymentMethodDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CardPresent {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<WalletAmexExpressCheckout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<WalletApplePay>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<WalletGooglePay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<WalletMasterpass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<WalletSamsungPay>,

    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: WalletDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<WalletVisaCheckout>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletAmexExpressCheckout {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletApplePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletGooglePay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletMasterpass {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletSamsungPay {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletVisaCheckout {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThreeDSecureUsage {
    /// Whether 3D Secure is supported on this card.
    pub supported: bool,
}

/// The parameters for `PaymentMethod::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreatePaymentMethod<'a> {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,

    /// The `Customer` to whom the original PaymentMethod is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The PaymentMethod to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodId>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    /// Required unless `payment_method` is specified (see the [Shared PaymentMethods](https://stripe.com/docs/payments/payment-methods/connect#shared-paymentmethods) guide).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PaymentMethodType>,
}

impl<'a> CreatePaymentMethod<'a> {
    pub fn new() -> Self {
        CreatePaymentMethod {
            billing_details: Default::default(),
            customer: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            payment_method: Default::default(),
            type_: Default::default(),
        }
    }
}

/// The parameters for `PaymentMethod::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListPaymentMethods<'a> {
    /// The ID of the customer whose PaymentMethods will be retrieved.
    pub customer: CustomerId,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a PaymentMethodId>,

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
    pub starting_after: Option<PaymentMethodId>,

    /// A required filter on the list, based on the object `type` field.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
}

impl<'a> ListPaymentMethods<'a> {
    pub fn new(customer: CustomerId, type_: PaymentMethodType) -> Self {
        ListPaymentMethods {
            customer,
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            type_,
        }
    }
}

/// The parameters for `PaymentMethod::update`.
#[derive(Clone, Debug, Serialize)]
pub struct UpdatePaymentMethod<'a> {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl<'a> UpdatePaymentMethod<'a> {
    pub fn new() -> Self {
        UpdatePaymentMethod {
            billing_details: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `PaymentMethod`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    Card,
    CardPresent,
}

impl PaymentMethodType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentMethodType::Card => "card",
            PaymentMethodType::CardPresent => "card_present",
        }
    }
}

impl AsRef<str> for PaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `WalletDetails`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WalletDetailsType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

impl WalletDetailsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WalletDetailsType::AmexExpressCheckout => "amex_express_checkout",
            WalletDetailsType::ApplePay => "apple_pay",
            WalletDetailsType::GooglePay => "google_pay",
            WalletDetailsType::Masterpass => "masterpass",
            WalletDetailsType::SamsungPay => "samsung_pay",
            WalletDetailsType::VisaCheckout => "visa_checkout",
        }
    }
}

impl AsRef<str> for WalletDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for WalletDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
