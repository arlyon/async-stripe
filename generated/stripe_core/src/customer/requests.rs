#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteCustomer {}
impl DeleteCustomer {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteCustomer {
    /// Permanently deletes a customer.
    /// It cannot be undone.
    /// Also immediately cancels any active subscriptions on the customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<stripe_shared::DeletedCustomer> {
        client.send_form(&format!("/customers/{customer}"), self, http_types::Method::Delete)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteDiscountCustomer {}
impl DeleteDiscountCustomer {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteDiscountCustomer {
    /// Removes the currently applied discount on a customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<stripe_shared::DeletedDiscount> {
        client.send_form(
            &format!("/customers/{customer}/discount"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListCustomer<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A case-sensitive filter on the list based on the customer's `email` field.
    /// The value must be a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Provides a list of customers that are associated with the specified test clock.
    /// The response will not include customers with test clocks if this parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
}
impl<'a> ListCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListCustomer<'a> {
    /// Returns a list of your customers.
    /// The customers are returned sorted by creation date, with the most recent customers appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::Customer>> {
        client.get_query("/customers", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_shared::Customer>> {
        stripe::ListPaginator::from_list_params("/customers", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCustomer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveCustomer<'a> {
    /// Retrieves a Customer object.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<RetrieveCustomerReturned> {
        client.get_query(&format!("/customers/{customer}"), self)
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(untagged))]
pub enum RetrieveCustomerReturned {
    Customer(stripe_shared::Customer),
    DeletedCustomer(stripe_shared::DeletedCustomer),
}

#[derive(Default)]
pub struct RetrieveCustomerReturnedBuilder {
    inner: stripe_types::miniserde_helpers::MaybeDeletedBuilderInner,
}

const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::MapBuilder;

    use super::*;

    make_place!(Place);

    struct Builder<'a> {
        out: &'a mut Option<RetrieveCustomerReturned>,
        builder: RetrieveCustomerReturnedBuilder,
    }

    impl Deserialize for RetrieveCustomerReturned {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    impl Visitor for Place<RetrieveCustomerReturned> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: Default::default() }))
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl MapBuilder for RetrieveCustomerReturnedBuilder {
        type Out = RetrieveCustomerReturned;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.inner.key_inner(k)
        }

        fn deser_default() -> Self {
            Self::default()
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (deleted, o) = self.inner.finish_inner()?;
            Some(if deleted {
                RetrieveCustomerReturned::DeletedCustomer(FromValueOpt::from_value(Value::Object(
                    o,
                ))?)
            } else {
                RetrieveCustomerReturned::Customer(FromValueOpt::from_value(Value::Object(o))?)
            })
        }
    }

    impl stripe_types::ObjectDeser for RetrieveCustomerReturned {
        type Builder = RetrieveCustomerReturnedBuilder;
    }
};

#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct BalanceTransactionsCustomer<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> BalanceTransactionsCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> BalanceTransactionsCustomer<'a> {
    /// Returns a list of transactions that updated the customer’s [balances](https://stripe.com/docs/billing/customer/balance).
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<stripe_types::List<stripe_shared::CustomerBalanceTransaction>> {
        client.get_query(&format!("/customers/{customer}/balance_transactions"), self)
    }
    pub fn paginate(
        self,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::CustomerBalanceTransaction>> {
        stripe::ListPaginator::from_list_params(
            &format!("/customers/{customer}/balance_transactions"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPaymentMethodsCustomer<'a> {
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// An optional filter on the list, based on the object `type` field.
    /// Without the filter, the list includes all current and future payment method types.
    /// If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListPaymentMethodsCustomerType>,
}
impl<'a> ListPaymentMethodsCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// An optional filter on the list, based on the object `type` field.
/// Without the filter, the list includes all current and future payment method types.
/// If your integration expects only one type of payment method in the response, make sure to provide a type value in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListPaymentMethodsCustomerType {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl ListPaymentMethodsCustomerType {
    pub fn as_str(self) -> &'static str {
        use ListPaymentMethodsCustomerType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            Klarna => "klarna",
            Konbini => "konbini",
            Link => "link",
            Oxxo => "oxxo",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for ListPaymentMethodsCustomerType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListPaymentMethodsCustomerType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for ListPaymentMethodsCustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListPaymentMethodsCustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListPaymentMethodsCustomerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListPaymentMethodsCustomerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
impl<'a> ListPaymentMethodsCustomer<'a> {
    /// Returns a list of PaymentMethods for a given Customer
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<stripe_types::List<stripe_shared::PaymentMethod>> {
        client.get_query(&format!("/customers/{customer}/payment_methods"), self)
    }
    pub fn paginate(
        self,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::PaymentMethod>> {
        stripe::ListPaginator::from_list_params(
            &format!("/customers/{customer}/payment_methods"),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentMethodCustomer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePaymentMethodCustomer<'a> {
    /// Retrieves a PaymentMethod object for a given Customer.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
        payment_method: &str,
    ) -> stripe::Response<stripe_shared::PaymentMethod> {
        client.get_query(&format!("/customers/{customer}/payment_methods/{payment_method}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct SearchCustomer<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for pagination across multiple pages of results.
    /// Don't include this parameter on the first call.
    /// Use the next_page value returned in a previous response to request subsequent results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<&'a str>,
    /// The search query string.
    /// See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for customers](https://stripe.com/docs/search#query-fields-for-customers).
    pub query: &'a str,
}
impl<'a> SearchCustomer<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { expand: None, limit: None, page: None, query }
    }
}
impl<'a> SearchCustomer<'a> {
    /// Search for customers you’ve previously created using Stripe’s [Search Query Language](https://stripe.com/docs/search#search-query-language).
    /// Don’t use search in read-after-write flows where strict consistency is necessary.
    /// Under normal operating.
    /// conditions, data is searchable in less than a minute.
    /// Occasionally, propagation of new or updated data can be up.
    /// to an hour behind during outages. Search functionality is not available to merchants in India.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::SearchList<stripe_shared::Customer>> {
        client.get_query("/customers/search", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::SearchList<stripe_shared::Customer>> {
        stripe::ListPaginator::from_search_params("/customers/search", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomer<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OptionalFieldsAddress<'a>>,
    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<CreateCustomerCashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// An arbitrary string that you can attach to a customer object.
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Customer's email address.
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The prefix for the customer used to generate unique invoice numbers.
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,
    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateCustomerInvoiceSettings<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The sequence to be used on the customer's next invoice. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<&'a [&'a str]>,
    /// The API ID of a promotion code to apply to the customer.
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// The customer's shipping information. Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<CreateCustomerTax<'a>>,
    /// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<stripe_shared::CustomerTaxExempt>,
    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_data: Option<&'a [CreateCustomerTaxIdData<'a>]>,
    /// ID of the test clock to attach to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}
impl<'a> CreateCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Balance information and default balance settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateCustomerCashBalanceSettings>,
}
impl CreateCustomerCashBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    /// Valid options are `automatic`, `manual`, or `merchant_default`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<CreateCustomerCashBalanceSettingsReconciliationMode>,
}
impl CreateCustomerCashBalanceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
/// Valid options are `automatic`, `manual`, or `merchant_default`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
    MerchantDefault,
}
impl CreateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerCashBalanceSettingsReconciliationMode::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
            MerchantDefault => "merchant_default",
        }
    }
}

impl std::str::FromStr for CreateCustomerCashBalanceSettingsReconciliationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerCashBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            "merchant_default" => Ok(MerchantDefault),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerCashBalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCustomerCashBalanceSettingsReconciliationMode",
            )
        })
    }
}
/// Default invoice settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerInvoiceSettings<'a> {
    /// The list of up to 4 default custom fields to be displayed on invoices for this customer.
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CustomFieldParams<'a>]>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateCustomerInvoiceSettingsRenderingOptions>,
}
impl<'a> CreateCustomerInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}
impl CreateCustomerInvoiceSettingsRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay",
            )
        })
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCustomerTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// A flag that indicates when Stripe should validate the customer tax location.
    /// Defaults to `deferred`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_location: Option<CreateCustomerTaxValidateLocation>,
}
impl<'a> CreateCustomerTax<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A flag that indicates when Stripe should validate the customer tax location.
/// Defaults to `deferred`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateCustomerTaxValidateLocation {
    Deferred,
    Immediately,
}
impl CreateCustomerTaxValidateLocation {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerTaxValidateLocation::*;
        match self {
            Deferred => "deferred",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for CreateCustomerTaxValidateLocation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerTaxValidateLocation::*;
        match s {
            "deferred" => Ok(Deferred),
            "immediately" => Ok(Immediately),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerTaxValidateLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerTaxValidateLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateCustomerTaxValidateLocation")
        })
    }
}
/// The customer's tax IDs.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCustomerTaxIdData<'a> {
    /// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: CreateCustomerTaxIdDataType,
    /// Value of the tax ID.
    pub value: &'a str,
}
impl<'a> CreateCustomerTaxIdData<'a> {
    pub fn new(type_: CreateCustomerTaxIdDataType, value: &'a str) -> Self {
        Self { type_, value }
    }
}
/// Type of the tax ID, one of `ad_nrt`, `ae_trn`, `ar_cuit`, `au_abn`, `au_arn`, `bg_uic`, `bo_tin`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `pe_ruc`, `ph_tin`, `ro_tin`, `rs_pib`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `sv_nit`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, `uy_ruc`, `ve_rif`, `vn_tin`, or `za_vat`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateCustomerTaxIdDataType {
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
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateCustomerTaxIdDataType {
    pub fn as_str(self) -> &'static str {
        use CreateCustomerTaxIdDataType::*;
        match self {
            AdNrt => "ad_nrt",
            AeTrn => "ae_trn",
            ArCuit => "ar_cuit",
            AuAbn => "au_abn",
            AuArn => "au_arn",
            BgUic => "bg_uic",
            BoTin => "bo_tin",
            BrCnpj => "br_cnpj",
            BrCpf => "br_cpf",
            CaBn => "ca_bn",
            CaGstHst => "ca_gst_hst",
            CaPstBc => "ca_pst_bc",
            CaPstMb => "ca_pst_mb",
            CaPstSk => "ca_pst_sk",
            CaQst => "ca_qst",
            ChVat => "ch_vat",
            ClTin => "cl_tin",
            CnTin => "cn_tin",
            CoNit => "co_nit",
            CrTin => "cr_tin",
            DoRcn => "do_rcn",
            EcRuc => "ec_ruc",
            EgTin => "eg_tin",
            EsCif => "es_cif",
            EuOssVat => "eu_oss_vat",
            EuVat => "eu_vat",
            GbVat => "gb_vat",
            GeVat => "ge_vat",
            HkBr => "hk_br",
            HuTin => "hu_tin",
            IdNpwp => "id_npwp",
            IlVat => "il_vat",
            InGst => "in_gst",
            IsVat => "is_vat",
            JpCn => "jp_cn",
            JpRn => "jp_rn",
            JpTrn => "jp_trn",
            KePin => "ke_pin",
            KrBrn => "kr_brn",
            LiUid => "li_uid",
            MxRfc => "mx_rfc",
            MyFrp => "my_frp",
            MyItn => "my_itn",
            MySst => "my_sst",
            NoVat => "no_vat",
            NzGst => "nz_gst",
            PeRuc => "pe_ruc",
            PhTin => "ph_tin",
            RoTin => "ro_tin",
            RsPib => "rs_pib",
            RuInn => "ru_inn",
            RuKpp => "ru_kpp",
            SaVat => "sa_vat",
            SgGst => "sg_gst",
            SgUen => "sg_uen",
            SiTin => "si_tin",
            SvNit => "sv_nit",
            ThVat => "th_vat",
            TrTin => "tr_tin",
            TwVat => "tw_vat",
            UaVat => "ua_vat",
            UsEin => "us_ein",
            UyRuc => "uy_ruc",
            VeRif => "ve_rif",
            VnTin => "vn_tin",
            ZaVat => "za_vat",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateCustomerTaxIdDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateCustomerTaxIdDataType::*;
        match s {
            "ad_nrt" => Ok(AdNrt),
            "ae_trn" => Ok(AeTrn),
            "ar_cuit" => Ok(ArCuit),
            "au_abn" => Ok(AuAbn),
            "au_arn" => Ok(AuArn),
            "bg_uic" => Ok(BgUic),
            "bo_tin" => Ok(BoTin),
            "br_cnpj" => Ok(BrCnpj),
            "br_cpf" => Ok(BrCpf),
            "ca_bn" => Ok(CaBn),
            "ca_gst_hst" => Ok(CaGstHst),
            "ca_pst_bc" => Ok(CaPstBc),
            "ca_pst_mb" => Ok(CaPstMb),
            "ca_pst_sk" => Ok(CaPstSk),
            "ca_qst" => Ok(CaQst),
            "ch_vat" => Ok(ChVat),
            "cl_tin" => Ok(ClTin),
            "cn_tin" => Ok(CnTin),
            "co_nit" => Ok(CoNit),
            "cr_tin" => Ok(CrTin),
            "do_rcn" => Ok(DoRcn),
            "ec_ruc" => Ok(EcRuc),
            "eg_tin" => Ok(EgTin),
            "es_cif" => Ok(EsCif),
            "eu_oss_vat" => Ok(EuOssVat),
            "eu_vat" => Ok(EuVat),
            "gb_vat" => Ok(GbVat),
            "ge_vat" => Ok(GeVat),
            "hk_br" => Ok(HkBr),
            "hu_tin" => Ok(HuTin),
            "id_npwp" => Ok(IdNpwp),
            "il_vat" => Ok(IlVat),
            "in_gst" => Ok(InGst),
            "is_vat" => Ok(IsVat),
            "jp_cn" => Ok(JpCn),
            "jp_rn" => Ok(JpRn),
            "jp_trn" => Ok(JpTrn),
            "ke_pin" => Ok(KePin),
            "kr_brn" => Ok(KrBrn),
            "li_uid" => Ok(LiUid),
            "mx_rfc" => Ok(MxRfc),
            "my_frp" => Ok(MyFrp),
            "my_itn" => Ok(MyItn),
            "my_sst" => Ok(MySst),
            "no_vat" => Ok(NoVat),
            "nz_gst" => Ok(NzGst),
            "pe_ruc" => Ok(PeRuc),
            "ph_tin" => Ok(PhTin),
            "ro_tin" => Ok(RoTin),
            "rs_pib" => Ok(RsPib),
            "ru_inn" => Ok(RuInn),
            "ru_kpp" => Ok(RuKpp),
            "sa_vat" => Ok(SaVat),
            "sg_gst" => Ok(SgGst),
            "sg_uen" => Ok(SgUen),
            "si_tin" => Ok(SiTin),
            "sv_nit" => Ok(SvNit),
            "th_vat" => Ok(ThVat),
            "tr_tin" => Ok(TrTin),
            "tw_vat" => Ok(TwVat),
            "ua_vat" => Ok(UaVat),
            "us_ein" => Ok(UsEin),
            "uy_ruc" => Ok(UyRuc),
            "ve_rif" => Ok(VeRif),
            "vn_tin" => Ok(VnTin),
            "za_vat" => Ok(ZaVat),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateCustomerTaxIdDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateCustomerTaxIdDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateCustomerTaxIdDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateCustomerTaxIdDataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
impl<'a> CreateCustomer<'a> {
    /// Creates a new customer object.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_shared::Customer> {
        client.send_form("/customers", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomer<'a> {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<OptionalFieldsAddress<'a>>,
    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<UpdateCustomerCashBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) parameter.
    ///
    /// Provide the ID of a payment source already attached to this customer to make it this customer's default payment source.
    ///
    /// If you want to add a new payment source and make it the default, see the [source](https://stripe.com/docs/api/customers/update#update_customer-source) property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    /// An arbitrary string that you can attach to a customer object.
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Customer's email address.
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The prefix for the customer used to generate unique invoice numbers.
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,
    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateCustomerInvoiceSettings<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// The sequence to be used on the customer's next invoice. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<&'a [&'a str]>,
    /// The API ID of a promotion code to apply to the customer.
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
    /// The customer's shipping information. Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShipping<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<UpdateCustomerTax<'a>>,
    /// The customer's tax exemption. One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<stripe_shared::CustomerTaxExempt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}
impl<'a> UpdateCustomer<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Balance information and default balance settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateCustomerCashBalanceSettings>,
}
impl UpdateCustomerCashBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    /// Valid options are `automatic`, `manual`, or `merchant_default`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<UpdateCustomerCashBalanceSettingsReconciliationMode>,
}
impl UpdateCustomerCashBalanceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Controls how funds transferred by the customer are applied to payment intents and invoices.
/// Valid options are `automatic`, `manual`, or `merchant_default`.
/// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
    MerchantDefault,
}
impl UpdateCustomerCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerCashBalanceSettingsReconciliationMode::*;
        match self {
            Automatic => "automatic",
            Manual => "manual",
            MerchantDefault => "merchant_default",
        }
    }
}

impl std::str::FromStr for UpdateCustomerCashBalanceSettingsReconciliationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerCashBalanceSettingsReconciliationMode::*;
        match s {
            "automatic" => Ok(Automatic),
            "manual" => Ok(Manual),
            "merchant_default" => Ok(MerchantDefault),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCustomerCashBalanceSettingsReconciliationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateCustomerCashBalanceSettingsReconciliationMode",
            )
        })
    }
}
/// Default invoice settings for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerInvoiceSettings<'a> {
    /// The list of up to 4 default custom fields to be displayed on invoices for this customer.
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<&'a [CustomFieldParams<'a>]>,
    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<UpdateCustomerInvoiceSettingsRenderingOptions>,
}
impl<'a> UpdateCustomerInvoiceSettings<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Default options for invoice PDF rendering for this customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}
impl UpdateCustomerInvoiceSettingsRenderingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
/// One of `exclude_tax` or `include_inclusive_tax`.
/// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
/// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}
impl UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match self {
            ExcludeTax => "exclude_tax",
            IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl std::str::FromStr for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay::*;
        match s {
            "exclude_tax" => Ok(ExcludeTax),
            "include_inclusive_tax" => Ok(IncludeInclusiveTax),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateCustomerInvoiceSettingsRenderingOptionsAmountTaxDisplay",
            )
        })
    }
}
/// Tax details about the customer.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCustomerTax<'a> {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// A flag that indicates when Stripe should validate the customer tax location.
    /// Defaults to `deferred`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_location: Option<UpdateCustomerTaxValidateLocation>,
}
impl<'a> UpdateCustomerTax<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A flag that indicates when Stripe should validate the customer tax location.
/// Defaults to `deferred`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCustomerTaxValidateLocation {
    Deferred,
    Immediately,
}
impl UpdateCustomerTaxValidateLocation {
    pub fn as_str(self) -> &'static str {
        use UpdateCustomerTaxValidateLocation::*;
        match self {
            Deferred => "deferred",
            Immediately => "immediately",
        }
    }
}

impl std::str::FromStr for UpdateCustomerTaxValidateLocation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateCustomerTaxValidateLocation::*;
        match s {
            "deferred" => Ok(Deferred),
            "immediately" => Ok(Immediately),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for UpdateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateCustomerTaxValidateLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateCustomerTaxValidateLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateCustomerTaxValidateLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateCustomerTaxValidateLocation")
        })
    }
}
impl<'a> UpdateCustomer<'a> {
    /// Updates the specified customer by setting the values of the parameters passed.
    /// Any parameters not provided will be left unchanged.
    /// For example, if you pass the **source** parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future.
    /// When you update a customer to a new valid card source by passing the **source** parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the `past_due` state, then the latest open invoice for the subscription with automatic collection enabled will be retried.
    /// This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice.
    /// Changing the **default_source** for a customer will not trigger this behavior.
    ///
    /// This request accepts mostly the same arguments as the customer creation call.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<stripe_shared::Customer> {
        client.send_form(&format!("/customers/{customer}"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomer<'a> {
    /// Additional parameters for `bank_transfer` funding types
    pub bank_transfer: CreateFundingInstructionsCustomerBankTransfer<'a>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The `funding_type` to get the instructions for.
    pub funding_type: CreateFundingInstructionsCustomerFundingType,
}
impl<'a> CreateFundingInstructionsCustomer<'a> {
    pub fn new(
        bank_transfer: CreateFundingInstructionsCustomerBankTransfer<'a>,
        currency: stripe_types::Currency,
        funding_type: CreateFundingInstructionsCustomerFundingType,
    ) -> Self {
        Self { bank_transfer, currency, expand: None, funding_type }
    }
}
/// Additional parameters for `bank_transfer` funding types
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomerBankTransfer<'a> {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a>>,
    /// List of address types that should be returned in the financial_addresses response.
    /// If not specified, all valid types will be returned.
    ///
    /// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types:
        Option<&'a [CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes]>,
    /// The type of the `bank_transfer`
    #[serde(rename = "type")]
    pub type_: CreateFundingInstructionsCustomerBankTransferType,
}
impl<'a> CreateFundingInstructionsCustomerBankTransfer<'a> {
    pub fn new(type_: CreateFundingInstructionsCustomerBankTransferType) -> Self {
        Self { eu_bank_transfer: None, requested_address_types: None, type_ }
    }
}
/// Configuration for eu_bank_transfer funding type.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a> {
    /// The desired country code of the bank account information.
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: &'a str,
}
impl<'a> CreateFundingInstructionsCustomerBankTransferEuBankTransfer<'a> {
    pub fn new(country: &'a str) -> Self {
        Self { country }
    }
}
/// List of address types that should be returned in the financial_addresses response.
/// If not specified, all valid types will be returned.
///
/// Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    Iban,
    SortCode,
    Spei,
    Zengin,
}
impl CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        use CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes::*;
        match self {
            Iban => "iban",
            SortCode => "sort_code",
            Spei => "spei",
            Zengin => "zengin",
        }
    }
}

impl std::str::FromStr for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes::*;
        match s {
            "iban" => Ok(Iban),
            "sort_code" => Ok(SortCode),
            "spei" => Ok(Spei),
            "zengin" => Ok(Zengin),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateFundingInstructionsCustomerBankTransferRequestedAddressTypes"))
    }
}
/// The type of the `bank_transfer`
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateFundingInstructionsCustomerBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
}
impl CreateFundingInstructionsCustomerBankTransferType {
    pub fn as_str(self) -> &'static str {
        use CreateFundingInstructionsCustomerBankTransferType::*;
        match self {
            EuBankTransfer => "eu_bank_transfer",
            GbBankTransfer => "gb_bank_transfer",
            JpBankTransfer => "jp_bank_transfer",
            MxBankTransfer => "mx_bank_transfer",
            UsBankTransfer => "us_bank_transfer",
        }
    }
}

impl std::str::FromStr for CreateFundingInstructionsCustomerBankTransferType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFundingInstructionsCustomerBankTransferType::*;
        match s {
            "eu_bank_transfer" => Ok(EuBankTransfer),
            "gb_bank_transfer" => Ok(GbBankTransfer),
            "jp_bank_transfer" => Ok(JpBankTransfer),
            "mx_bank_transfer" => Ok(MxBankTransfer),
            "us_bank_transfer" => Ok(UsBankTransfer),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateFundingInstructionsCustomerBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFundingInstructionsCustomerBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFundingInstructionsCustomerBankTransferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateFundingInstructionsCustomerBankTransferType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateFundingInstructionsCustomerBankTransferType",
            )
        })
    }
}
/// The `funding_type` to get the instructions for.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateFundingInstructionsCustomerFundingType {
    BankTransfer,
}
impl CreateFundingInstructionsCustomerFundingType {
    pub fn as_str(self) -> &'static str {
        use CreateFundingInstructionsCustomerFundingType::*;
        match self {
            BankTransfer => "bank_transfer",
        }
    }
}

impl std::str::FromStr for CreateFundingInstructionsCustomerFundingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateFundingInstructionsCustomerFundingType::*;
        match s {
            "bank_transfer" => Ok(BankTransfer),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateFundingInstructionsCustomerFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateFundingInstructionsCustomerFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateFundingInstructionsCustomerFundingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateFundingInstructionsCustomerFundingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateFundingInstructionsCustomerFundingType",
            )
        })
    }
}
impl<'a> CreateFundingInstructionsCustomer<'a> {
    /// Retrieve funding instructions for a customer cash balance.
    /// If funding instructions do not yet exist for the customer, new.
    /// funding instructions will be created.
    /// If funding instructions have already been created for a given customer, the same.
    /// funding instructions will be retrieved.
    /// In other words, we will return the same funding instructions each time.
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &stripe_shared::CustomerId,
    ) -> stripe::Response<stripe_shared::FundingInstructions> {
        client.send_form(
            &format!("/customers/{customer}/funding_instructions"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct FundCashBalanceCustomer<'a> {
    /// Amount to be used for this test cash balance transaction.
    /// A positive integer representing how much to fund in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to fund $1.00 or 100 to fund ¥100, a zero-decimal currency).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A description of the test funding.
    /// This simulates free-text references supplied by customers when making bank transfers to their cash balance.
    /// You can use this to test how Stripe's [reconciliation algorithm](https://stripe.com/docs/payments/customer-balance/reconciliation) applies to different user inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
}
impl<'a> FundCashBalanceCustomer<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, expand: None, reference: None }
    }
}
impl<'a> FundCashBalanceCustomer<'a> {
    /// Create an incoming testmode bank transfer
    pub fn send(
        &self,
        client: &stripe::Client,
        customer: &str,
    ) -> stripe::Response<stripe_shared::CustomerCashBalanceTransaction> {
        client.send_form(
            &format!("/test_helpers/customers/{customer}/fund_cash_balance"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct OptionalFieldsAddress<'a> {
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
impl<'a> OptionalFieldsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomFieldParams<'a> {
    /// The name of the custom field. This may be up to 30 characters.
    pub name: &'a str,
    /// The value of the custom field. This may be up to 30 characters.
    pub value: &'a str,
}
impl<'a> CustomFieldParams<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CustomerShipping<'a> {
    /// Customer shipping address.
    pub address: OptionalFieldsAddress<'a>,
    /// Customer name.
    pub name: &'a str,
    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CustomerShipping<'a> {
    pub fn new(address: OptionalFieldsAddress<'a>, name: &'a str) -> Self {
        Self { address, name, phone: None }
    }
}
