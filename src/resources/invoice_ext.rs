use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::generated::billing::invoice::InvoiceSettingCustomField;
use crate::ids::{
    AccountId, CouponId, CustomerId, InvoiceId, PaymentMethodId, PaymentSourceId, PlanId,
    SubscriptionId, SubscriptionItemId,
};
use crate::params::{Metadata, SearchList, Timestamp};
use crate::resources::{CollectionMethod, Invoice};
use crate::AutomaticTax;
use crate::CreateInvoiceShippingDetails;
use crate::InvoiceSettingRenderingOptions;
use crate::InvoiceTransferData;
use crate::InvoicesPaymentSettings;
use crate::InvoicesShippingCost;
use crate::UpdateInvoiceItemDiscounts;

#[deprecated(since = "0.12.0")]
pub type InvoiceCollectionMethod = CollectionMethod;

impl Invoice {
    /// Retrieves the details of an upcoming invoice_id
    ///
    /// For more details see <https://stripe.com/docs/api#upcoming_invoice>.
    pub fn upcoming(client: &Client, params: RetrieveUpcomingInvoice) -> Response<Invoice> {
        client.get_query("/invoices/upcoming", &params)
    }

    /// Finalizes an invoice.
    ///
    /// For more details see <https://stripe.com/docs/api/invoices/finalize.>.
    pub fn finalize(
        client: &Client,
        invoice_id: &InvoiceId,
        params: FinalizeInvoiceParams,
    ) -> Response<Invoice> {
        client.post_form(&format!("/invoices/{}/finalize", invoice_id), params)
    }

    /// Pays an invoice.
    ///
    /// For more details see <https://stripe.com/docs/api#pay_invoice.>.
    pub fn pay(client: &Client, invoice_id: &InvoiceId) -> Response<Invoice> {
        client.post(&format!("/invoices/{}/pay", invoice_id))
    }

    /// Searches for an invoice.
    ///
    /// For more details see <https://stripe.com/docs/api/invoices/search>.
    pub fn search(client: &Client, params: InvoiceSearchParams) -> Response<SearchList<Invoice>> {
        client.get_query("/invoices/search", params)
    }

    /// Voids an invoice.
    ///
    /// For more details see <https://stripe.com/docs/api/invoices/void>.
    pub fn void(client: &Client, invoice_id: &InvoiceId) -> Response<Invoice> {
        client.post(&format!("/invoices/{}/void", invoice_id))
    }

    /// Updates an invoice.
    ///
    /// For more details see <https://docs.stripe.com/api/invoices/update>.
    pub fn update(
        client: &Client,
        invoice_id: &InvoiceId,
        params: UpdateInvoice,
    ) -> Response<Invoice> {
        client.post_form(&format!("/invoices/{}", invoice_id), &params)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RetrieveUpcomingInvoice {
    pub customer: CustomerId, // this is a required param
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<SubscriptionItemFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<Timestamp>,
}

impl RetrieveUpcomingInvoice {
    pub fn new(customer: CustomerId) -> Self {
        RetrieveUpcomingInvoice {
            customer,
            coupon: None,
            subscription: None,
            subscription_items: None,
            subscription_prorate: None,
            subscription_proration_date: None,
            subscription_tax_percent: None,
            subscription_trial_end: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SubscriptionItemFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SubscriptionItemId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct InvoiceSearchParams<'a> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    pub expand: &'a [&'a str],
}

impl<'a> InvoiceSearchParams<'a> {
    pub fn new() -> InvoiceSearchParams<'a> {
        InvoiceSearchParams { query: String::new(), limit: None, page: None, expand: &[] }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FinalizeInvoiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
}

/// Update the resource representing a Stripe "Invoice".
///
/// For more details see <https://stripe.com/docs/api/invoices/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateInvoice {
    /// Controls whether Stripe performs automatic collection of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Settings for automatic tax lookup for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<AutomaticTax>,

    /// Either `charge_automatically` or `send_invoice`. This field can be updated only on draft invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// An arbitrary string attached to the object. Often useful for displaying to users. Referenced as ‘memo’ in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of key-value pairs that you can attach to an object. This can be useful for storing additional information about the object in a structured format. Individual keys can be unset by posting an empty value to them. All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The account tax IDs associated with the invoice. Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// A fee in cents that will be applied to the invoice and transferred to the application owner’s Stripe account. The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee. For more information, see the application fees documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// A list of up to 4 custom fields to be displayed on the invoice. If a value for custom_fields is specified, the list specified will replace the existing custom field list on this invoice. Pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,

    /// The number of days from which the invoice is created until it is due. Only valid for invoices where collection_method=send_invoice. This field can only be updated on draft invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the invoice. It must belong to the customer associated with the invoice. If not set, defaults to the subscription’s default payment method, if any, or to the default payment method in the customer’s invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<PaymentMethodId>,

    /// ID of the default payment source for the invoice. It must belong to the customer associated with the invoice and be in a chargeable state. If not set, defaults to the subscription’s default source, if any, or to the customer’s default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<PaymentSourceId>,

    /// The tax rates that will apply to any line item that does not have tax_rates set. Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// The discounts that will apply to the invoice. Pass an empty string to remove previously-defined discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<UpdateInvoiceItemDiscounts>>,

    /// The date on which payment for this invoice is due. Only valid for invoices where collection_method=send_invoice. This field can only be updated on draft invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Timestamp>,

    /// The date when this invoice is in effect. Same as finalized_at unless overwritten. When defined, this value replaces the system-generated ‘Date of issue’ printed on the invoice PDF and receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<Timestamp>,

    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// The connected account that issues the invoice. The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdateInvoiceIssuer>,

    /// Set the number for this invoice. If no number is present then a number will be assigned automatically when the invoice is finalized. In many markets, regulations require invoices to be unique, sequential and / or gapless. You are responsible for ensuring this is true across all your different invoicing systems in the event that you edit the invoice number using our API. If you use only Stripe for your invoices and do not change invoice numbers, Stripe handles this aspect of compliance for you automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// The account (if any) for which the funds of the invoice payment are intended. If set, the invoice will be presented with the branding and support information of the specified account. See the Invoices with Connect documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<AccountId>,

    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<InvoicesPaymentSettings>,

    /// Rendering settings for the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<InvoiceSettingRenderingOptions>,

    /// Settings for the cost of shipping for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<InvoicesShippingCost>,

    /// Shipping details for the invoice. When shipping_details is set, it will be displayed on the invoice PDF and receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<CreateInvoiceShippingDetails>,

    /// Extra information about a charge for the customer's credit card statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The data with which to automatically create a Transfer for each of the invoices’s charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<InvoiceTransferData>,
}

impl UpdateInvoice {
    pub fn new() -> Self {
        UpdateInvoice::default()
    }
}

/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateInvoiceIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountId>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateInvoiceIssuerType,
}

/// Type of the account referenced in the request.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateInvoiceIssuerType {
    /// Indicates that the account being referenced is a connected account which is different from the account making the API request but related to it.
    Account,

    /// Indicates that the account being referenced is the account making the API request.
    #[serde(rename = "self")]
    Self_,
}
