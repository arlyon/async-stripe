#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RefundDestinationDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_bank_transfer: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub br_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_shared::RefundDestinationDetailsCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_cash_balance: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    /// The type of transaction-specific details of the payment method used in the refund (e.g., `card`).
    /// An additional hash is included on `destination_details` with a name matching this value.
    /// It contains information specific to the refund transaction.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_shared::DestinationDetailsUnimplemented>,
}
