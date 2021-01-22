// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::CheckoutSessionId;
use crate::params::{Expandable, List, Metadata, Object};
use crate::resources::{
    CheckoutSessionItem, Currency, Customer, Discount, PaymentIntent, SetupIntent, Shipping,
    Subscription, TaxRate,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Session".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CheckoutSession {
    /// Unique identifier for the object.
    ///
    /// Used to pass to `redirectToCheckout` in Stripe.js.
    pub id: CheckoutSessionId,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// Total of all items before discounts or taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,

    /// Total of all items after discounts and taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,

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

    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
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

    /// A list of the types of payment methods (e.g.
    ///
    /// card) this Checkout Session is allowed to accept.
    pub payment_method_types: Vec<String>,

    /// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
    /// You can use this value to decide when to fulfill your customer's order.
    pub payment_status: CheckoutSessionPaymentStatus,

    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<Expandable<SetupIntent>>,

    /// Shipping information for this Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,

    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<ShippingAddressCollection>,

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

    /// Tax and discount details for the computed total amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_details: Option<PaymentPagesCheckoutSessionTotalDetails>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    /// The customer’s email at time of checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The customer’s tax exempt status at time of checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,

    /// The customer’s tax IDs at time of checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<PaymentPagesCheckoutSessionTaxId>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTaxId {
    /// The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `nz_gst`, `au_abn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: PaymentPagesCheckoutSessionTaxIdType,

    /// The value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    /// This is the sum of all the line item discounts.
    pub amount_discount: i64,

    /// This is the sum of all the line item tax amounts.
    pub amount_tax: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    /// The aggregated line item discounts.
    pub discounts: Vec<LineItemsDiscountAmount>,

    /// The aggregated line item tax amounts by rate.
    pub taxes: Vec<LineItemsTaxAmount>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineItemsDiscountAmount {
    /// The amount discounted.
    pub amount: i64,

    pub discount: Discount,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: TaxRate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<ShippingAddressCollectionAllowedCountries>,
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
    Fr,
    #[serde(rename = "fr-CA")]
    FrCa,
    Hu,
    Id,
    It,
    Ja,
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
    Tr,
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
            CheckoutSessionLocale::Fr => "fr",
            CheckoutSessionLocale::FrCa => "fr-CA",
            CheckoutSessionLocale::Hu => "hu",
            CheckoutSessionLocale::Id => "id",
            CheckoutSessionLocale::It => "it",
            CheckoutSessionLocale::Ja => "ja",
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
            CheckoutSessionLocale::Tr => "tr",
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

/// An enum representing the possible values of an `PaymentPagesCheckoutSessionTaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionTaxIdType {
    AeTrn,
    AuAbn,
    BrCnpj,
    BrCpf,
    CaBn,
    CaQst,
    ChVat,
    ClTin,
    EsCif,
    EuVat,
    HkBr,
    IdNpwp,
    InGst,
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
    ThVat,
    TwVat,
    Unknown,
    UsEin,
    ZaVat,
}

impl PaymentPagesCheckoutSessionTaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentPagesCheckoutSessionTaxIdType::AeTrn => "ae_trn",
            PaymentPagesCheckoutSessionTaxIdType::AuAbn => "au_abn",
            PaymentPagesCheckoutSessionTaxIdType::BrCnpj => "br_cnpj",
            PaymentPagesCheckoutSessionTaxIdType::BrCpf => "br_cpf",
            PaymentPagesCheckoutSessionTaxIdType::CaBn => "ca_bn",
            PaymentPagesCheckoutSessionTaxIdType::CaQst => "ca_qst",
            PaymentPagesCheckoutSessionTaxIdType::ChVat => "ch_vat",
            PaymentPagesCheckoutSessionTaxIdType::ClTin => "cl_tin",
            PaymentPagesCheckoutSessionTaxIdType::EsCif => "es_cif",
            PaymentPagesCheckoutSessionTaxIdType::EuVat => "eu_vat",
            PaymentPagesCheckoutSessionTaxIdType::HkBr => "hk_br",
            PaymentPagesCheckoutSessionTaxIdType::IdNpwp => "id_npwp",
            PaymentPagesCheckoutSessionTaxIdType::InGst => "in_gst",
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
            PaymentPagesCheckoutSessionTaxIdType::ThVat => "th_vat",
            PaymentPagesCheckoutSessionTaxIdType::TwVat => "tw_vat",
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

/// An enum representing the possible values of an `ShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShippingAddressCollectionAllowedCountries {
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

impl ShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingAddressCollectionAllowedCountries::Ac => "AC",
            ShippingAddressCollectionAllowedCountries::Ad => "AD",
            ShippingAddressCollectionAllowedCountries::Ae => "AE",
            ShippingAddressCollectionAllowedCountries::Af => "AF",
            ShippingAddressCollectionAllowedCountries::Ag => "AG",
            ShippingAddressCollectionAllowedCountries::Ai => "AI",
            ShippingAddressCollectionAllowedCountries::Al => "AL",
            ShippingAddressCollectionAllowedCountries::Am => "AM",
            ShippingAddressCollectionAllowedCountries::Ao => "AO",
            ShippingAddressCollectionAllowedCountries::Aq => "AQ",
            ShippingAddressCollectionAllowedCountries::Ar => "AR",
            ShippingAddressCollectionAllowedCountries::At => "AT",
            ShippingAddressCollectionAllowedCountries::Au => "AU",
            ShippingAddressCollectionAllowedCountries::Aw => "AW",
            ShippingAddressCollectionAllowedCountries::Ax => "AX",
            ShippingAddressCollectionAllowedCountries::Az => "AZ",
            ShippingAddressCollectionAllowedCountries::Ba => "BA",
            ShippingAddressCollectionAllowedCountries::Bb => "BB",
            ShippingAddressCollectionAllowedCountries::Bd => "BD",
            ShippingAddressCollectionAllowedCountries::Be => "BE",
            ShippingAddressCollectionAllowedCountries::Bf => "BF",
            ShippingAddressCollectionAllowedCountries::Bg => "BG",
            ShippingAddressCollectionAllowedCountries::Bh => "BH",
            ShippingAddressCollectionAllowedCountries::Bi => "BI",
            ShippingAddressCollectionAllowedCountries::Bj => "BJ",
            ShippingAddressCollectionAllowedCountries::Bl => "BL",
            ShippingAddressCollectionAllowedCountries::Bm => "BM",
            ShippingAddressCollectionAllowedCountries::Bn => "BN",
            ShippingAddressCollectionAllowedCountries::Bo => "BO",
            ShippingAddressCollectionAllowedCountries::Bq => "BQ",
            ShippingAddressCollectionAllowedCountries::Br => "BR",
            ShippingAddressCollectionAllowedCountries::Bs => "BS",
            ShippingAddressCollectionAllowedCountries::Bt => "BT",
            ShippingAddressCollectionAllowedCountries::Bv => "BV",
            ShippingAddressCollectionAllowedCountries::Bw => "BW",
            ShippingAddressCollectionAllowedCountries::By => "BY",
            ShippingAddressCollectionAllowedCountries::Bz => "BZ",
            ShippingAddressCollectionAllowedCountries::Ca => "CA",
            ShippingAddressCollectionAllowedCountries::Cd => "CD",
            ShippingAddressCollectionAllowedCountries::Cf => "CF",
            ShippingAddressCollectionAllowedCountries::Cg => "CG",
            ShippingAddressCollectionAllowedCountries::Ch => "CH",
            ShippingAddressCollectionAllowedCountries::Ci => "CI",
            ShippingAddressCollectionAllowedCountries::Ck => "CK",
            ShippingAddressCollectionAllowedCountries::Cl => "CL",
            ShippingAddressCollectionAllowedCountries::Cm => "CM",
            ShippingAddressCollectionAllowedCountries::Cn => "CN",
            ShippingAddressCollectionAllowedCountries::Co => "CO",
            ShippingAddressCollectionAllowedCountries::Cr => "CR",
            ShippingAddressCollectionAllowedCountries::Cv => "CV",
            ShippingAddressCollectionAllowedCountries::Cw => "CW",
            ShippingAddressCollectionAllowedCountries::Cy => "CY",
            ShippingAddressCollectionAllowedCountries::Cz => "CZ",
            ShippingAddressCollectionAllowedCountries::De => "DE",
            ShippingAddressCollectionAllowedCountries::Dj => "DJ",
            ShippingAddressCollectionAllowedCountries::Dk => "DK",
            ShippingAddressCollectionAllowedCountries::Dm => "DM",
            ShippingAddressCollectionAllowedCountries::Do => "DO",
            ShippingAddressCollectionAllowedCountries::Dz => "DZ",
            ShippingAddressCollectionAllowedCountries::Ec => "EC",
            ShippingAddressCollectionAllowedCountries::Ee => "EE",
            ShippingAddressCollectionAllowedCountries::Eg => "EG",
            ShippingAddressCollectionAllowedCountries::Eh => "EH",
            ShippingAddressCollectionAllowedCountries::Er => "ER",
            ShippingAddressCollectionAllowedCountries::Es => "ES",
            ShippingAddressCollectionAllowedCountries::Et => "ET",
            ShippingAddressCollectionAllowedCountries::Fi => "FI",
            ShippingAddressCollectionAllowedCountries::Fj => "FJ",
            ShippingAddressCollectionAllowedCountries::Fk => "FK",
            ShippingAddressCollectionAllowedCountries::Fo => "FO",
            ShippingAddressCollectionAllowedCountries::Fr => "FR",
            ShippingAddressCollectionAllowedCountries::Ga => "GA",
            ShippingAddressCollectionAllowedCountries::Gb => "GB",
            ShippingAddressCollectionAllowedCountries::Gd => "GD",
            ShippingAddressCollectionAllowedCountries::Ge => "GE",
            ShippingAddressCollectionAllowedCountries::Gf => "GF",
            ShippingAddressCollectionAllowedCountries::Gg => "GG",
            ShippingAddressCollectionAllowedCountries::Gh => "GH",
            ShippingAddressCollectionAllowedCountries::Gi => "GI",
            ShippingAddressCollectionAllowedCountries::Gl => "GL",
            ShippingAddressCollectionAllowedCountries::Gm => "GM",
            ShippingAddressCollectionAllowedCountries::Gn => "GN",
            ShippingAddressCollectionAllowedCountries::Gp => "GP",
            ShippingAddressCollectionAllowedCountries::Gq => "GQ",
            ShippingAddressCollectionAllowedCountries::Gr => "GR",
            ShippingAddressCollectionAllowedCountries::Gs => "GS",
            ShippingAddressCollectionAllowedCountries::Gt => "GT",
            ShippingAddressCollectionAllowedCountries::Gu => "GU",
            ShippingAddressCollectionAllowedCountries::Gw => "GW",
            ShippingAddressCollectionAllowedCountries::Gy => "GY",
            ShippingAddressCollectionAllowedCountries::Hk => "HK",
            ShippingAddressCollectionAllowedCountries::Hn => "HN",
            ShippingAddressCollectionAllowedCountries::Hr => "HR",
            ShippingAddressCollectionAllowedCountries::Ht => "HT",
            ShippingAddressCollectionAllowedCountries::Hu => "HU",
            ShippingAddressCollectionAllowedCountries::Id => "ID",
            ShippingAddressCollectionAllowedCountries::Ie => "IE",
            ShippingAddressCollectionAllowedCountries::Il => "IL",
            ShippingAddressCollectionAllowedCountries::Im => "IM",
            ShippingAddressCollectionAllowedCountries::In => "IN",
            ShippingAddressCollectionAllowedCountries::Io => "IO",
            ShippingAddressCollectionAllowedCountries::Iq => "IQ",
            ShippingAddressCollectionAllowedCountries::Is => "IS",
            ShippingAddressCollectionAllowedCountries::It => "IT",
            ShippingAddressCollectionAllowedCountries::Je => "JE",
            ShippingAddressCollectionAllowedCountries::Jm => "JM",
            ShippingAddressCollectionAllowedCountries::Jo => "JO",
            ShippingAddressCollectionAllowedCountries::Jp => "JP",
            ShippingAddressCollectionAllowedCountries::Ke => "KE",
            ShippingAddressCollectionAllowedCountries::Kg => "KG",
            ShippingAddressCollectionAllowedCountries::Kh => "KH",
            ShippingAddressCollectionAllowedCountries::Ki => "KI",
            ShippingAddressCollectionAllowedCountries::Km => "KM",
            ShippingAddressCollectionAllowedCountries::Kn => "KN",
            ShippingAddressCollectionAllowedCountries::Kr => "KR",
            ShippingAddressCollectionAllowedCountries::Kw => "KW",
            ShippingAddressCollectionAllowedCountries::Ky => "KY",
            ShippingAddressCollectionAllowedCountries::Kz => "KZ",
            ShippingAddressCollectionAllowedCountries::La => "LA",
            ShippingAddressCollectionAllowedCountries::Lb => "LB",
            ShippingAddressCollectionAllowedCountries::Lc => "LC",
            ShippingAddressCollectionAllowedCountries::Li => "LI",
            ShippingAddressCollectionAllowedCountries::Lk => "LK",
            ShippingAddressCollectionAllowedCountries::Lr => "LR",
            ShippingAddressCollectionAllowedCountries::Ls => "LS",
            ShippingAddressCollectionAllowedCountries::Lt => "LT",
            ShippingAddressCollectionAllowedCountries::Lu => "LU",
            ShippingAddressCollectionAllowedCountries::Lv => "LV",
            ShippingAddressCollectionAllowedCountries::Ly => "LY",
            ShippingAddressCollectionAllowedCountries::Ma => "MA",
            ShippingAddressCollectionAllowedCountries::Mc => "MC",
            ShippingAddressCollectionAllowedCountries::Md => "MD",
            ShippingAddressCollectionAllowedCountries::Me => "ME",
            ShippingAddressCollectionAllowedCountries::Mf => "MF",
            ShippingAddressCollectionAllowedCountries::Mg => "MG",
            ShippingAddressCollectionAllowedCountries::Mk => "MK",
            ShippingAddressCollectionAllowedCountries::Ml => "ML",
            ShippingAddressCollectionAllowedCountries::Mm => "MM",
            ShippingAddressCollectionAllowedCountries::Mn => "MN",
            ShippingAddressCollectionAllowedCountries::Mo => "MO",
            ShippingAddressCollectionAllowedCountries::Mq => "MQ",
            ShippingAddressCollectionAllowedCountries::Mr => "MR",
            ShippingAddressCollectionAllowedCountries::Ms => "MS",
            ShippingAddressCollectionAllowedCountries::Mt => "MT",
            ShippingAddressCollectionAllowedCountries::Mu => "MU",
            ShippingAddressCollectionAllowedCountries::Mv => "MV",
            ShippingAddressCollectionAllowedCountries::Mw => "MW",
            ShippingAddressCollectionAllowedCountries::Mx => "MX",
            ShippingAddressCollectionAllowedCountries::My => "MY",
            ShippingAddressCollectionAllowedCountries::Mz => "MZ",
            ShippingAddressCollectionAllowedCountries::Na => "NA",
            ShippingAddressCollectionAllowedCountries::Nc => "NC",
            ShippingAddressCollectionAllowedCountries::Ne => "NE",
            ShippingAddressCollectionAllowedCountries::Ng => "NG",
            ShippingAddressCollectionAllowedCountries::Ni => "NI",
            ShippingAddressCollectionAllowedCountries::Nl => "NL",
            ShippingAddressCollectionAllowedCountries::No => "NO",
            ShippingAddressCollectionAllowedCountries::Np => "NP",
            ShippingAddressCollectionAllowedCountries::Nr => "NR",
            ShippingAddressCollectionAllowedCountries::Nu => "NU",
            ShippingAddressCollectionAllowedCountries::Nz => "NZ",
            ShippingAddressCollectionAllowedCountries::Om => "OM",
            ShippingAddressCollectionAllowedCountries::Pa => "PA",
            ShippingAddressCollectionAllowedCountries::Pe => "PE",
            ShippingAddressCollectionAllowedCountries::Pf => "PF",
            ShippingAddressCollectionAllowedCountries::Pg => "PG",
            ShippingAddressCollectionAllowedCountries::Ph => "PH",
            ShippingAddressCollectionAllowedCountries::Pk => "PK",
            ShippingAddressCollectionAllowedCountries::Pl => "PL",
            ShippingAddressCollectionAllowedCountries::Pm => "PM",
            ShippingAddressCollectionAllowedCountries::Pn => "PN",
            ShippingAddressCollectionAllowedCountries::Pr => "PR",
            ShippingAddressCollectionAllowedCountries::Ps => "PS",
            ShippingAddressCollectionAllowedCountries::Pt => "PT",
            ShippingAddressCollectionAllowedCountries::Py => "PY",
            ShippingAddressCollectionAllowedCountries::Qa => "QA",
            ShippingAddressCollectionAllowedCountries::Re => "RE",
            ShippingAddressCollectionAllowedCountries::Ro => "RO",
            ShippingAddressCollectionAllowedCountries::Rs => "RS",
            ShippingAddressCollectionAllowedCountries::Ru => "RU",
            ShippingAddressCollectionAllowedCountries::Rw => "RW",
            ShippingAddressCollectionAllowedCountries::Sa => "SA",
            ShippingAddressCollectionAllowedCountries::Sb => "SB",
            ShippingAddressCollectionAllowedCountries::Sc => "SC",
            ShippingAddressCollectionAllowedCountries::Se => "SE",
            ShippingAddressCollectionAllowedCountries::Sg => "SG",
            ShippingAddressCollectionAllowedCountries::Sh => "SH",
            ShippingAddressCollectionAllowedCountries::Si => "SI",
            ShippingAddressCollectionAllowedCountries::Sj => "SJ",
            ShippingAddressCollectionAllowedCountries::Sk => "SK",
            ShippingAddressCollectionAllowedCountries::Sl => "SL",
            ShippingAddressCollectionAllowedCountries::Sm => "SM",
            ShippingAddressCollectionAllowedCountries::Sn => "SN",
            ShippingAddressCollectionAllowedCountries::So => "SO",
            ShippingAddressCollectionAllowedCountries::Sr => "SR",
            ShippingAddressCollectionAllowedCountries::Ss => "SS",
            ShippingAddressCollectionAllowedCountries::St => "ST",
            ShippingAddressCollectionAllowedCountries::Sv => "SV",
            ShippingAddressCollectionAllowedCountries::Sx => "SX",
            ShippingAddressCollectionAllowedCountries::Sz => "SZ",
            ShippingAddressCollectionAllowedCountries::Ta => "TA",
            ShippingAddressCollectionAllowedCountries::Tc => "TC",
            ShippingAddressCollectionAllowedCountries::Td => "TD",
            ShippingAddressCollectionAllowedCountries::Tf => "TF",
            ShippingAddressCollectionAllowedCountries::Tg => "TG",
            ShippingAddressCollectionAllowedCountries::Th => "TH",
            ShippingAddressCollectionAllowedCountries::Tj => "TJ",
            ShippingAddressCollectionAllowedCountries::Tk => "TK",
            ShippingAddressCollectionAllowedCountries::Tl => "TL",
            ShippingAddressCollectionAllowedCountries::Tm => "TM",
            ShippingAddressCollectionAllowedCountries::Tn => "TN",
            ShippingAddressCollectionAllowedCountries::To => "TO",
            ShippingAddressCollectionAllowedCountries::Tr => "TR",
            ShippingAddressCollectionAllowedCountries::Tt => "TT",
            ShippingAddressCollectionAllowedCountries::Tv => "TV",
            ShippingAddressCollectionAllowedCountries::Tw => "TW",
            ShippingAddressCollectionAllowedCountries::Tz => "TZ",
            ShippingAddressCollectionAllowedCountries::Ua => "UA",
            ShippingAddressCollectionAllowedCountries::Ug => "UG",
            ShippingAddressCollectionAllowedCountries::Us => "US",
            ShippingAddressCollectionAllowedCountries::Uy => "UY",
            ShippingAddressCollectionAllowedCountries::Uz => "UZ",
            ShippingAddressCollectionAllowedCountries::Va => "VA",
            ShippingAddressCollectionAllowedCountries::Vc => "VC",
            ShippingAddressCollectionAllowedCountries::Ve => "VE",
            ShippingAddressCollectionAllowedCountries::Vg => "VG",
            ShippingAddressCollectionAllowedCountries::Vn => "VN",
            ShippingAddressCollectionAllowedCountries::Vu => "VU",
            ShippingAddressCollectionAllowedCountries::Wf => "WF",
            ShippingAddressCollectionAllowedCountries::Ws => "WS",
            ShippingAddressCollectionAllowedCountries::Xk => "XK",
            ShippingAddressCollectionAllowedCountries::Ye => "YE",
            ShippingAddressCollectionAllowedCountries::Yt => "YT",
            ShippingAddressCollectionAllowedCountries::Za => "ZA",
            ShippingAddressCollectionAllowedCountries::Zm => "ZM",
            ShippingAddressCollectionAllowedCountries::Zw => "ZW",
            ShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for ShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
