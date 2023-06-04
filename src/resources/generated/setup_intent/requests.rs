use crate::{Client, Response};

impl crate::setup_intent::SetupIntent {
    /// Creates a SetupIntent object.
    ///
    /// After the SetupIntent is created, attach a payment method and [confirm](https://stripe.com/docs/api/setup_intents/confirm)
    /// to collect any required permissions to charge the payment method later.
    pub fn create(
        client: &Client,
        params: CreateSetupIntent,
    ) -> Response<crate::setup_intent::SetupIntent> {
        client.send_form("/setup_intents", params, http_types::Method::Post)
    }
    /// Returns a list of SetupIntents.
    pub fn list(
        client: &Client,
        params: ListSetupIntent,
    ) -> Response<crate::List<crate::setup_intent::SetupIntent>> {
        client.get_query("/setup_intents", params)
    }
    /// Retrieves the details of a SetupIntent that has previously been created.
    ///
    /// Client-side retrieval using a publishable key is allowed when the `client_secret` is provided in the query string.
    /// When retrieved with a publishable key, only a subset of properties will be returned.
    /// Please refer to the [SetupIntent](https://stripe.com/docs/api#setup_intent_object) object reference for more details.
    pub fn retrieve(
        client: &Client,
        intent: &str,
        params: RetrieveSetupIntent,
    ) -> Response<crate::setup_intent::SetupIntent> {
        client.get_query(&format!("/setup_intents/{intent}", intent = intent), params)
    }
    /// Updates a SetupIntent object.
    pub fn update(
        client: &Client,
        intent: &str,
        params: UpdateSetupIntent,
    ) -> Response<crate::setup_intent::SetupIntent> {
        client.send_form(
            &format!("/setup_intents/{intent}", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Confirm that your customer intends to set up the current or
    /// provided payment method.
    ///
    /// For example, you would confirm a SetupIntent when a customer hits the “Save” button on a payment method management page on your website.  If the selected payment method does not require any additional steps from the customer, the SetupIntent will transition to the `succeeded` status.  Otherwise, it will transition to the `requires_action` status and suggest additional actions via `next_action`.
    /// If setup fails, the SetupIntent will transition to the `requires_payment_method` status.
    pub fn confirm(
        client: &Client,
        intent: &str,
        params: ConfirmSetupIntent,
    ) -> Response<crate::setup_intent::SetupIntent> {
        client.send_form(
            &format!("/setup_intents/{intent}/confirm", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// A SetupIntent object can be canceled when it is in one of these statuses: `requires_payment_method`, `requires_confirmation`, or `requires_action`.
    ///
    /// Once canceled, setup is abandoned and any operations on the SetupIntent will fail with an error.
    pub fn cancel(
        client: &Client,
        intent: &str,
        params: CancelSetupIntent,
    ) -> Response<crate::setup_intent::SetupIntent> {
        client.send_form(
            &format!("/setup_intents/{intent}/cancel", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
    /// Verifies microdeposits on a SetupIntent object.
    pub fn verify_microdeposits(
        client: &Client,
        intent: &str,
        params: VerifyMicrodepositsSetupIntent,
    ) -> Response<crate::setup_intent::SetupIntent> {
        client.send_form(
            &format!("/setup_intents/{intent}/verify_microdeposits", intent = intent),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntent<'a> {
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    /// Set to `true` to attempt to confirm this SetupIntent immediately.
    ///
    /// This parameter defaults to `false`.
    /// If the payment method attached is a card, a return_url may be provided in case additional authentication is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<&'a [CreateSetupIntentFlowDirections]>,
    /// This hash contains details about the Mandate to create.
    ///
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<CreateSetupIntentMandateData<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The Stripe account ID for which this SetupIntent is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
    /// value in the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<CreateSetupIntentPaymentMethodData<'a>>,
    /// Payment-method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateSetupIntentPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) that this SetupIntent is allowed to use.
    /// If this is not provided, defaults to ["card"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [&'a str]>,
    /// The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    ///
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    /// If this hash is populated, this SetupIntent will generate a single_use Mandate on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<CreateSetupIntentSingleUse>,
    /// Indicates how the payment method is intended to be used in the future.
    ///
    /// If not provided, this value defaults to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CreateSetupIntentUsage>,
}
impl<'a> CreateSetupIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates the directions of money movement for which this payment method is intended to be used.
///
/// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
///
/// Include `outbound` if you intend to use the payment method as the destination to send funds to.
/// You can include both if you intend to use the payment method for both purposes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentFlowDirections {
    Inbound,
    Outbound,
}

impl CreateSetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inbound => "inbound",
            Self::Outbound => "outbound",
        }
    }
}

impl AsRef<str> for CreateSetupIntentFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This hash contains details about the Mandate to create.
///
/// This parameter can only be used with [`confirm=true`](https://stripe.com/docs/api/setup_intents/create#create_setup_intent-confirm).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentMandateData<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: CreateSetupIntentMandateDataCustomerAcceptance<'a>,
}
impl<'a> CreateSetupIntentMandateData<'a> {
    pub fn new(customer_acceptance: CreateSetupIntentMandateDataCustomerAcceptance<'a>) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptance<'a> {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<crate::Timestamp>,
    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<CreateSetupIntentMandateDataCustomerAcceptanceOffline>,
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<CreateSetupIntentMandateDataCustomerAcceptanceOnline<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CreateSetupIntentMandateDataCustomerAcceptanceType,
}
impl<'a> CreateSetupIntentMandateDataCustomerAcceptance<'a> {
    pub fn new(type_: CreateSetupIntentMandateDataCustomerAcceptanceType) -> Self {
        Self {
            accepted_at: Default::default(),
            offline: Default::default(),
            online: Default::default(),
            type_,
        }
    }
}
/// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptanceOffline {}
impl CreateSetupIntentMandateDataCustomerAcceptanceOffline {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentMandateDataCustomerAcceptanceOnline<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: &'a str,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: &'a str,
}
impl<'a> CreateSetupIntentMandateDataCustomerAcceptanceOnline<'a> {
    pub fn new(ip_address: &'a str, user_agent: &'a str) -> Self {
        Self { ip_address, user_agent }
    }
}
/// The type of customer acceptance information included with the Mandate.
///
/// One of `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentMandateDataCustomerAcceptanceType {
    Offline,
    Online,
}

impl CreateSetupIntentMandateDataCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentMandateDataCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
/// value in the SetupIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSetupIntentPaymentMethodDataAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<CreateSetupIntentPaymentMethodDataAffirm>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<CreateSetupIntentPaymentMethodDataAfterpayClearpay>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<CreateSetupIntentPaymentMethodDataAlipay>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<CreateSetupIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<CreateSetupIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateSetupIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateSetupIntentPaymentMethodDataBillingDetails<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreateSetupIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<CreateSetupIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateSetupIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<CreateSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<CreateSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<CreateSetupIntentPaymentMethodDataGiropay>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<CreateSetupIntentPaymentMethodDataGrabpay>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<CreateSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<CreateSetupIntentPaymentMethodDataInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<CreateSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateSetupIntentPaymentMethodDataKonbini>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateSetupIntentPaymentMethodDataLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<CreateSetupIntentPaymentMethodDataOxxo>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<CreateSetupIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<CreateSetupIntentPaymentMethodDataPaynow>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<CreateSetupIntentPaymentMethodDataPix>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<CreateSetupIntentPaymentMethodDataPromptpay>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<CreateSetupIntentPaymentMethodDataRadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<CreateSetupIntentPaymentMethodDataSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: CreateSetupIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<CreateSetupIntentPaymentMethodDataWechatPay>,
}
impl<'a> CreateSetupIntentPaymentMethodData<'a> {
    pub fn new(type_: CreateSetupIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            paynow: Default::default(),
            pix: Default::default(),
            promptpay: Default::default(),
            radar_options: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            type_,
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> CreateSetupIntentPaymentMethodDataAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAffirm {}
impl CreateSetupIntentPaymentMethodDataAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAfterpayClearpay {}
impl CreateSetupIntentPaymentMethodDataAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAlipay {}
impl CreateSetupIntentPaymentMethodDataAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> CreateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBancontact {}
impl CreateSetupIntentPaymentMethodDataBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateSetupIntentPaymentMethodDataBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBillingDetailsAddress<'a> {
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
impl<'a> CreateSetupIntentPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBlik {}
impl CreateSetupIntentPaymentMethodDataBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> CreateSetupIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataCustomerBalance {}
impl CreateSetupIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataEpsBank>,
}
impl CreateSetupIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl CreateSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<CreateSetupIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: CreateSetupIntentPaymentMethodDataFpxBank,
}
impl CreateSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: CreateSetupIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// Account holder type for FPX transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl CreateSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataGiropay {}
impl CreateSetupIntentPaymentMethodDataGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataGrabpay {}
impl CreateSetupIntentPaymentMethodDataGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataIdealBank>,
}
impl CreateSetupIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl CreateSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataInteracPresent {}
impl CreateSetupIntentPaymentMethodDataInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<CreateSetupIntentPaymentMethodDataKlarnaDob>,
}
impl CreateSetupIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Customer's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl CreateSetupIntentPaymentMethodDataKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataKonbini {}
impl CreateSetupIntentPaymentMethodDataKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataLink {}
impl CreateSetupIntentPaymentMethodDataLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataOxxo {}
impl CreateSetupIntentPaymentMethodDataOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateSetupIntentPaymentMethodDataP24Bank>,
}
impl CreateSetupIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl CreateSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPaynow {}
impl CreateSetupIntentPaymentMethodDataPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPix {}
impl CreateSetupIntentPaymentMethodDataPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataPromptpay {}
impl CreateSetupIntentPaymentMethodDataPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodDataRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> CreateSetupIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: CreateSetupIntentPaymentMethodDataSofortCountry,
}
impl CreateSetupIntentPaymentMethodDataSofort {
    pub fn new(country: CreateSetupIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl CreateSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
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
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl CreateSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<CreateSetupIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodDataWechatPay {}
impl CreateSetupIntentPaymentMethodDataWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration for this SetupIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSetupIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<CreateSetupIntentPaymentMethodOptionsBlik<'a>>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSetupIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<CreateSetupIntentPaymentMethodOptionsLink<'a>>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<CreateSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> CreateSetupIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
///
/// Must be a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cad => "cad",
            Self::Usd => "usd",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsBlik<'a> {
    /// The 6-digit BLIK code that a customer has generated using their banking application.
    ///
    /// Can only be set on confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsBlik<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    ///
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateSetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// Currency in which future payments will be charged.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<crate::Timestamp>,
    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: crate::Timestamp,
    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        currency: crate::Currency,
        interval: CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: crate::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            currency,
            description: Default::default(),
            end_date: Default::default(),
            interval,
            interval_count: Default::default(),
            reference,
            start_date,
            supported_types: Default::default(),
        }
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Sporadic => "sporadic",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this SetupIntent on.
///
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl CreateSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsLink<'a> {
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}
impl CreateSetupIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {}
impl CreateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for network related functions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Triggers validations to run across the selected networks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this hash is populated, this SetupIntent will generate a single_use Mandate on success.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateSetupIntentSingleUse {
    /// Amount the customer is granting permission to collect later.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
}
impl CreateSetupIntentSingleUse {
    pub fn new(amount: i64, currency: crate::Currency) -> Self {
        Self { amount, currency }
    }
}
/// Indicates how the payment method is intended to be used in the future.
///
/// If not provided, this value defaults to `off_session`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateSetupIntentUsage {
    OffSession,
    OnSession,
}

impl CreateSetupIntentUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreateSetupIntentUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSetupIntentUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListSetupIntent<'a> {
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::RangeQueryTs>,
    /// Only return SetupIntents for the customer specified by this customer ID.
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
    /// Only return SetupIntents associated with the specified payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListSetupIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveSetupIntent<'a> {
    /// The client secret of the SetupIntent.
    ///
    /// Required if a publishable key is used to retrieve the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveSetupIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntent<'a> {
    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    /// ID of the Customer this SetupIntent belongs to, if one exists.
    ///
    /// If present, the SetupIntent's payment method will be attached to the Customer on successful setup.
    ///
    /// Payment methods attached to other Customers cannot be used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<&'a [UpdateSetupIntentFlowDirections]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
    /// value in the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<UpdateSetupIntentPaymentMethodData<'a>>,
    /// Payment-method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateSetupIntentPaymentMethodOptions<'a>>,
    /// The list of payment method types (e.g.
    ///
    /// card) that this SetupIntent is allowed to set up.
    /// If this is not provided, defaults to ["card"].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<&'a [&'a str]>,
}
impl<'a> UpdateSetupIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Indicates the directions of money movement for which this payment method is intended to be used.
///
/// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
///
/// Include `outbound` if you intend to use the payment method as the destination to send funds to.
/// You can include both if you intend to use the payment method for both purposes.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentFlowDirections {
    Inbound,
    Outbound,
}

impl UpdateSetupIntentFlowDirections {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inbound => "inbound",
            Self::Outbound => "outbound",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
/// value in the SetupIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSetupIntentPaymentMethodDataAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<UpdateSetupIntentPaymentMethodDataAffirm>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<UpdateSetupIntentPaymentMethodDataAfterpayClearpay>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<UpdateSetupIntentPaymentMethodDataAlipay>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<UpdateSetupIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<UpdateSetupIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdateSetupIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<UpdateSetupIntentPaymentMethodDataBillingDetails<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdateSetupIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<UpdateSetupIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<UpdateSetupIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<UpdateSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<UpdateSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<UpdateSetupIntentPaymentMethodDataGiropay>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<UpdateSetupIntentPaymentMethodDataGrabpay>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<UpdateSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<UpdateSetupIntentPaymentMethodDataInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<UpdateSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<UpdateSetupIntentPaymentMethodDataKonbini>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdateSetupIntentPaymentMethodDataLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<UpdateSetupIntentPaymentMethodDataOxxo>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<UpdateSetupIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<UpdateSetupIntentPaymentMethodDataPaynow>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<UpdateSetupIntentPaymentMethodDataPix>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<UpdateSetupIntentPaymentMethodDataPromptpay>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<UpdateSetupIntentPaymentMethodDataRadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<UpdateSetupIntentPaymentMethodDataSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: UpdateSetupIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<UpdateSetupIntentPaymentMethodDataWechatPay>,
}
impl<'a> UpdateSetupIntentPaymentMethodData<'a> {
    pub fn new(type_: UpdateSetupIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            paynow: Default::default(),
            pix: Default::default(),
            promptpay: Default::default(),
            radar_options: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            type_,
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> UpdateSetupIntentPaymentMethodDataAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAffirm {}
impl UpdateSetupIntentPaymentMethodDataAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAfterpayClearpay {}
impl UpdateSetupIntentPaymentMethodDataAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAlipay {}
impl UpdateSetupIntentPaymentMethodDataAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> UpdateSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBancontact {}
impl UpdateSetupIntentPaymentMethodDataBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateSetupIntentPaymentMethodDataBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBillingDetailsAddress<'a> {
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
impl<'a> UpdateSetupIntentPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBlik {}
impl UpdateSetupIntentPaymentMethodDataBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> UpdateSetupIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataCustomerBalance {}
impl UpdateSetupIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataEpsBank>,
}
impl UpdateSetupIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl UpdateSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<UpdateSetupIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: UpdateSetupIntentPaymentMethodDataFpxBank,
}
impl UpdateSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: UpdateSetupIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// Account holder type for FPX transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl UpdateSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataGiropay {}
impl UpdateSetupIntentPaymentMethodDataGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataGrabpay {}
impl UpdateSetupIntentPaymentMethodDataGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataIdealBank>,
}
impl UpdateSetupIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl UpdateSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataInteracPresent {}
impl UpdateSetupIntentPaymentMethodDataInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<UpdateSetupIntentPaymentMethodDataKlarnaDob>,
}
impl UpdateSetupIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Customer's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl UpdateSetupIntentPaymentMethodDataKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataKonbini {}
impl UpdateSetupIntentPaymentMethodDataKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataLink {}
impl UpdateSetupIntentPaymentMethodDataLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataOxxo {}
impl UpdateSetupIntentPaymentMethodDataOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<UpdateSetupIntentPaymentMethodDataP24Bank>,
}
impl UpdateSetupIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl UpdateSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPaynow {}
impl UpdateSetupIntentPaymentMethodDataPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPix {}
impl UpdateSetupIntentPaymentMethodDataPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataPromptpay {}
impl UpdateSetupIntentPaymentMethodDataPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodDataRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> UpdateSetupIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: UpdateSetupIntentPaymentMethodDataSofortCountry,
}
impl UpdateSetupIntentPaymentMethodDataSofort {
    pub fn new(country: UpdateSetupIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl UpdateSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
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
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl UpdateSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodDataWechatPay {}
impl UpdateSetupIntentPaymentMethodDataWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration for this SetupIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<UpdateSetupIntentPaymentMethodOptionsBlik<'a>>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSetupIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<UpdateSetupIntentPaymentMethodOptionsLink<'a>>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
///
/// Must be a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cad => "cad",
            Self::Usd => "usd",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsBlik<'a> {
    /// The 6-digit BLIK code that a customer has generated using their banking application.
    ///
    /// Can only be set on confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsBlik<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    ///
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<UpdateSetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// Currency in which future payments will be charged.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<crate::Timestamp>,
    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: crate::Timestamp,
    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        currency: crate::Currency,
        interval: UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: crate::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            currency,
            description: Default::default(),
            end_date: Default::default(),
            interval,
            interval_count: Default::default(),
            reference,
            start_date,
            supported_types: Default::default(),
        }
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Sporadic => "sporadic",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this SetupIntent on.
///
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsLink<'a> {
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}
impl UpdateSetupIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {}
impl UpdateSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for network related functions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Triggers validations to run across the selected networks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntent<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// This hash contains details about the Mandate to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<ConfirmSetupIntentMandateData<'a>>,
    /// ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    /// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
    /// value in the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_data: Option<ConfirmSetupIntentPaymentMethodData<'a>>,
    /// Payment-method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<ConfirmSetupIntentPaymentMethodOptions<'a>>,
    /// The URL to redirect your customer back to after they authenticate on the payment method's app or site.
    /// If you'd prefer to redirect to a mobile application, you can alternatively supply an application URI scheme.
    /// This parameter is only used for cards and other redirect-based payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> ConfirmSetupIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// This hash contains details about the Mandate to create.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ConfirmSetupIntentMandateData<'a> {
    SecretKeyParam(ConfirmSetupIntentSecretKeyParam<'a>),
    ClientKeyParam(ConfirmSetupIntentClientKeyParam<'a>),
}
/// This hash contains details about the Mandate to create.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentSecretKeyParam<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a>,
}
impl<'a> ConfirmSetupIntentSecretKeyParam<'a> {
    pub fn new(
        customer_acceptance: ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a>,
    ) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a> {
    /// The time at which the customer accepted the Mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<crate::Timestamp>,
    /// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<ConfirmSetupIntentSecretKeyParamCustomerAcceptanceOffline>,
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<ConfirmSetupIntentSecretKeyParamCustomerAcceptanceOnline<'a>>,
    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType,
}
impl<'a> ConfirmSetupIntentSecretKeyParamCustomerAcceptance<'a> {
    pub fn new(type_: ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType) -> Self {
        Self {
            accepted_at: Default::default(),
            offline: Default::default(),
            online: Default::default(),
            type_,
        }
    }
}
/// If this is a Mandate accepted offline, this hash contains details about the offline acceptance.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentSecretKeyParamCustomerAcceptanceOffline {}
impl ConfirmSetupIntentSecretKeyParamCustomerAcceptanceOffline {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentSecretKeyParamCustomerAcceptanceOnline<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    pub ip_address: &'a str,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    pub user_agent: &'a str,
}
impl<'a> ConfirmSetupIntentSecretKeyParamCustomerAcceptanceOnline<'a> {
    pub fn new(ip_address: &'a str, user_agent: &'a str) -> Self {
        Self { ip_address, user_agent }
    }
}
/// The type of customer acceptance information included with the Mandate.
///
/// One of `online` or `offline`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    Offline,
    Online,
}

impl ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentSecretKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// This hash contains details about the Mandate to create.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParam<'a> {
    /// This hash contains details about the customer acceptance of the Mandate.
    pub customer_acceptance: ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a>,
}
impl<'a> ConfirmSetupIntentClientKeyParam<'a> {
    pub fn new(
        customer_acceptance: ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a>,
    ) -> Self {
        Self { customer_acceptance }
    }
}
/// This hash contains details about the customer acceptance of the Mandate.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a> {
    /// If this is a Mandate accepted online, this hash contains details about the online acceptance.
    pub online: ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a>,
    /// The type of customer acceptance information included with the Mandate.
    #[serde(rename = "type")]
    pub type_: ConfirmSetupIntentClientKeyParamCustomerAcceptanceType,
}
impl<'a> ConfirmSetupIntentClientKeyParamCustomerAcceptance<'a> {
    pub fn new(
        online: ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a>,
        type_: ConfirmSetupIntentClientKeyParamCustomerAcceptanceType,
    ) -> Self {
        Self { online, type_ }
    }
}
/// If this is a Mandate accepted online, this hash contains details about the online acceptance.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a> {
    /// The IP address from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<&'a str>,
    /// The user agent of the browser from which the Mandate was accepted by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentClientKeyParamCustomerAcceptanceOnline<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of customer acceptance information included with the Mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    Online,
}

impl ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentClientKeyParamCustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// When included, this hash creates a PaymentMethod that is set as the [`payment_method`](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-payment_method)
/// value in the SetupIntent.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodData<'a> {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<ConfirmSetupIntentPaymentMethodDataAcssDebit<'a>>,
    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<ConfirmSetupIntentPaymentMethodDataAffirm>,
    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<ConfirmSetupIntentPaymentMethodDataAfterpayClearpay>,
    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<ConfirmSetupIntentPaymentMethodDataAlipay>,
    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<ConfirmSetupIntentPaymentMethodDataAuBecsDebit<'a>>,
    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<ConfirmSetupIntentPaymentMethodDataBacsDebit<'a>>,
    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<ConfirmSetupIntentPaymentMethodDataBancontact>,
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<ConfirmSetupIntentPaymentMethodDataBillingDetails<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<ConfirmSetupIntentPaymentMethodDataBlik>,
    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<ConfirmSetupIntentPaymentMethodDataBoleto<'a>>,
    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<ConfirmSetupIntentPaymentMethodDataCustomerBalance>,
    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<ConfirmSetupIntentPaymentMethodDataEps>,
    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<ConfirmSetupIntentPaymentMethodDataFpx>,
    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<ConfirmSetupIntentPaymentMethodDataGiropay>,
    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<ConfirmSetupIntentPaymentMethodDataGrabpay>,
    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<ConfirmSetupIntentPaymentMethodDataIdeal>,
    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<ConfirmSetupIntentPaymentMethodDataInteracPresent>,
    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<ConfirmSetupIntentPaymentMethodDataKlarna>,
    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<ConfirmSetupIntentPaymentMethodDataKonbini>,
    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<ConfirmSetupIntentPaymentMethodDataLink>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<ConfirmSetupIntentPaymentMethodDataOxxo>,
    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<ConfirmSetupIntentPaymentMethodDataP24>,
    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<ConfirmSetupIntentPaymentMethodDataPaynow>,
    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<ConfirmSetupIntentPaymentMethodDataPix>,
    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<ConfirmSetupIntentPaymentMethodDataPromptpay>,
    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<ConfirmSetupIntentPaymentMethodDataRadarOptions<'a>>,
    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmSetupIntentPaymentMethodDataSepaDebit<'a>>,
    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<ConfirmSetupIntentPaymentMethodDataSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: ConfirmSetupIntentPaymentMethodDataType,
    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmSetupIntentPaymentMethodDataUsBankAccount<'a>>,
    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<ConfirmSetupIntentPaymentMethodDataWechatPay>,
}
impl<'a> ConfirmSetupIntentPaymentMethodData<'a> {
    pub fn new(type_: ConfirmSetupIntentPaymentMethodDataType) -> Self {
        Self {
            acss_debit: Default::default(),
            affirm: Default::default(),
            afterpay_clearpay: Default::default(),
            alipay: Default::default(),
            au_becs_debit: Default::default(),
            bacs_debit: Default::default(),
            bancontact: Default::default(),
            billing_details: Default::default(),
            blik: Default::default(),
            boleto: Default::default(),
            customer_balance: Default::default(),
            eps: Default::default(),
            fpx: Default::default(),
            giropay: Default::default(),
            grabpay: Default::default(),
            ideal: Default::default(),
            interac_present: Default::default(),
            klarna: Default::default(),
            konbini: Default::default(),
            link: Default::default(),
            metadata: Default::default(),
            oxxo: Default::default(),
            p24: Default::default(),
            paynow: Default::default(),
            pix: Default::default(),
            promptpay: Default::default(),
            radar_options: Default::default(),
            sepa_debit: Default::default(),
            sofort: Default::default(),
            type_,
            us_bank_account: Default::default(),
            wechat_pay: Default::default(),
        }
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataAcssDebit<'a> {
    /// Customer's bank account number.
    pub account_number: &'a str,
    /// Institution number of the customer's bank.
    pub institution_number: &'a str,
    /// Transit number of the customer's bank.
    pub transit_number: &'a str,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataAcssDebit<'a> {
    pub fn new(
        account_number: &'a str,
        institution_number: &'a str,
        transit_number: &'a str,
    ) -> Self {
        Self { account_number, institution_number, transit_number }
    }
}
/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataAffirm {}
impl ConfirmSetupIntentPaymentMethodDataAffirm {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataAfterpayClearpay {}
impl ConfirmSetupIntentPaymentMethodDataAfterpayClearpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataAlipay {}
impl ConfirmSetupIntentPaymentMethodDataAlipay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    /// The account number for the bank account.
    pub account_number: &'a str,
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: &'a str,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataAuBecsDebit<'a> {
    pub fn new(account_number: &'a str, bsb_number: &'a str) -> Self {
        Self { account_number, bsb_number }
    }
}
/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBacsDebit<'a> {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataBacsDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBancontact {}
impl ConfirmSetupIntentPaymentMethodDataBancontact {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBillingDetails<'a> {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ConfirmSetupIntentPaymentMethodDataBillingDetailsAddress<'a>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataBillingDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Billing address.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBillingDetailsAddress<'a> {
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
impl<'a> ConfirmSetupIntentPaymentMethodDataBillingDetailsAddress<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBlik {}
impl ConfirmSetupIntentPaymentMethodDataBlik {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataBoleto<'a> {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: &'a str,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataBoleto<'a> {
    pub fn new(tax_id: &'a str) -> Self {
        Self { tax_id }
    }
}
/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataCustomerBalance {}
impl ConfirmSetupIntentPaymentMethodDataCustomerBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmSetupIntentPaymentMethodDataEpsBank>,
}
impl ConfirmSetupIntentPaymentMethodDataEps {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl ConfirmSetupIntentPaymentMethodDataEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType>,
    /// The customer's bank.
    pub bank: ConfirmSetupIntentPaymentMethodDataFpxBank,
}
impl ConfirmSetupIntentPaymentMethodDataFpx {
    pub fn new(bank: ConfirmSetupIntentPaymentMethodDataFpxBank) -> Self {
        Self { account_holder_type: Default::default(), bank }
    }
}
/// Account holder type for FPX transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
}

impl ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl ConfirmSetupIntentPaymentMethodDataFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataGiropay {}
impl ConfirmSetupIntentPaymentMethodDataGiropay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataGrabpay {}
impl ConfirmSetupIntentPaymentMethodDataGrabpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmSetupIntentPaymentMethodDataIdealBank>,
}
impl ConfirmSetupIntentPaymentMethodDataIdeal {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl ConfirmSetupIntentPaymentMethodDataIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataInteracPresent {}
impl ConfirmSetupIntentPaymentMethodDataInteracPresent {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<ConfirmSetupIntentPaymentMethodDataKlarnaDob>,
}
impl ConfirmSetupIntentPaymentMethodDataKlarna {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Customer's date of birth.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl ConfirmSetupIntentPaymentMethodDataKlarnaDob {
    pub fn new(day: i64, month: i64, year: i64) -> Self {
        Self { day, month, year }
    }
}
/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataKonbini {}
impl ConfirmSetupIntentPaymentMethodDataKonbini {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataLink {}
impl ConfirmSetupIntentPaymentMethodDataLink {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataOxxo {}
impl ConfirmSetupIntentPaymentMethodDataOxxo {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<ConfirmSetupIntentPaymentMethodDataP24Bank>,
}
impl ConfirmSetupIntentPaymentMethodDataP24 {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The customer's bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl ConfirmSetupIntentPaymentMethodDataP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataPaynow {}
impl ConfirmSetupIntentPaymentMethodDataPaynow {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataPix {}
impl ConfirmSetupIntentPaymentMethodDataPix {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataPromptpay {}
impl ConfirmSetupIntentPaymentMethodDataPromptpay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataRadarOptions<'a> {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataRadarOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataSepaDebit<'a> {
    /// IBAN of the bank account.
    pub iban: &'a str,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataSepaDebit<'a> {
    pub fn new(iban: &'a str) -> Self {
        Self { iban }
    }
}
/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: ConfirmSetupIntentPaymentMethodDataSofortCountry,
}
impl ConfirmSetupIntentPaymentMethodDataSofort {
    pub fn new(country: ConfirmSetupIntentPaymentMethodDataSofortCountry) -> Self {
        Self { country }
    }
}
/// Two-letter ISO code representing the country the bank account is located in.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl ConfirmSetupIntentPaymentMethodDataSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
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
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl ConfirmSetupIntentPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataUsBankAccount<'a> {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType>,
    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType>,
    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<&'a str>,
    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodDataUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Account holder type: individual or company.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Account type: checkings or savings.
///
/// Defaults to checking if omitted.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodDataUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodDataWechatPay {}
impl ConfirmSetupIntentPaymentMethodDataWechatPay {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Payment-method-specific configuration for this SetupIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptions<'a> {
    /// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebit<'a>>,
    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<ConfirmSetupIntentPaymentMethodOptionsBlik<'a>>,
    /// Configuration for any card setup attempted on this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<ConfirmSetupIntentPaymentMethodOptionsCard<'a>>,
    /// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<ConfirmSetupIntentPaymentMethodOptionsLink<'a>>,
    /// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<ConfirmSetupIntentPaymentMethodOptionsSepaDebit>,
    /// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccount<'a>>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsAcssDebit<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
///
/// Must be a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
}

impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cad => "cad",
            Self::Usd => "usd",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<&'a str>,
    /// List of Stripe products where this mandate can be selected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for:
        Option<&'a [ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor]>,
    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<&'a str>,
    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// List of Stripe products where this mandate can be selected automatically.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    Invoice,
    Subscription,
}

impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
            Self::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsDefaultFor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Payment schedule for the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Transaction type of the mandate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsBlik<'a> {
    /// The 6-digit BLIK code that a customer has generated using their banking application.
    ///
    /// Can only be set on confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsBlik<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration for any card setup attempted on this SetupIntent.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCard<'a> {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsCardMandateOptions<'a>>,
    /// When specified, this parameter signals that a card has been collected
    /// as MOTO (Mail Order Telephone Order) and thus out of scope for SCA.
    ///
    /// This parameter can only be provided during confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    /// Selected network to process this SetupIntent on.
    ///
    /// Depends on the available networks of the card attached to the SetupIntent.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<ConfirmSetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Permitted values include: `automatic` or `any`.
    /// If not provided, defaults to `automatic`.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Configuration options for setting up an eMandate for cards issued in India.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    /// Amount to be charged for future payments.
    pub amount: i64,
    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
    /// Currency in which future payments will be charged.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// End date of the mandate or subscription.
    ///
    /// If not provided, the mandate will be active until canceled.
    /// If provided, end date should be after start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<crate::Timestamp>,
    /// Specifies payment frequency.
    ///
    /// One of `day`, `week`, `month`, `year`, or `sporadic`.
    pub interval: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
    /// The number of intervals between payments.
    ///
    /// For example, `interval=month` and `interval_count=3` indicates one payment every three months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    /// This parameter is optional when `interval=sporadic`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
    /// Unique identifier for the mandate or subscription.
    pub reference: &'a str,
    /// Start date of the mandate or subscription.
    ///
    /// Start date should not be lesser than yesterday.
    pub start_date: crate::Timestamp,
    /// Specifies the type of mandates supported.
    ///
    /// Possible values are `india`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_types:
        Option<&'a [ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes]>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsCardMandateOptions<'a> {
    pub fn new(
        amount: i64,
        amount_type: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType,
        currency: crate::Currency,
        interval: ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval,
        reference: &'a str,
        start_date: crate::Timestamp,
    ) -> Self {
        Self {
            amount,
            amount_type,
            currency,
            description: Default::default(),
            end_date: Default::default(),
            interval,
            interval_count: Default::default(),
            reference,
            start_date,
            supported_types: Default::default(),
        }
    }
}
/// One of `fixed` or `maximum`.
///
/// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
/// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies payment frequency.
///
/// One of `day`, `week`, `month`, `year`, or `sporadic`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
}

impl ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Sporadic => "sporadic",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies the type of mandates supported.
///
/// Possible values are `india`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    India,
}

impl ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::India => "india",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Selected network to process this SetupIntent on.
///
/// Depends on the available networks of the card attached to the SetupIntent.
/// Can be only set confirm-time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Amex => "amex",
            Self::CartesBancaires => "cartes_bancaires",
            Self::Diners => "diners",
            Self::Discover => "discover",
            Self::Interac => "interac",
            Self::Jcb => "jcb",
            Self::Mastercard => "mastercard",
            Self::Unionpay => "unionpay",
            Self::Unknown => "unknown",
            Self::Visa => "visa",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
///
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Permitted values include: `automatic` or `any`.
/// If not provided, defaults to `automatic`.
/// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsLink<'a> {
    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsLink<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions>,
}
impl ConfirmSetupIntentPaymentMethodOptionsSepaDebit {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Mandate creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {}
impl ConfirmSetupIntentPaymentMethodOptionsSepaDebitMandateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method options.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a>>,
    /// Additional fields for network related functions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a>>,
    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsUsBankAccount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional fields for Financial Connections Session creation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<
        &'a [ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions],
    >,
    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnections<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The list of permissions to request.
///
/// If this parameter is passed, the `payment_method` permission must be included.
/// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Additional fields for network related functions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    /// Triggers validations to run across the selected networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested:
        Option<&'a [ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested]>,
}
impl<'a> ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworks<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Triggers validations to run across the selected networks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    Ach,
    UsDomesticWire,
}

impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountNetworksRequested {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Verification method for the intent.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConfirmSetupIntentPaymentMethodOptionsUsBankAccountVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelSetupIntent<'a> {
    /// Reason for canceling this SetupIntent.
    ///
    /// Possible values are `abandoned`, `requested_by_customer`, or `duplicate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<CancelSetupIntentCancellationReason>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelSetupIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Reason for canceling this SetupIntent.
///
/// Possible values are `abandoned`, `requested_by_customer`, or `duplicate`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CancelSetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
}

impl CancelSetupIntentCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abandoned => "abandoned",
            Self::Duplicate => "duplicate",
            Self::RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl AsRef<str> for CancelSetupIntentCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CancelSetupIntentCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct VerifyMicrodepositsSetupIntent<'a> {
    /// Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<&'a [i64]>,
    /// A six-character code starting with SM present in the microdeposit sent to the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor_code: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> VerifyMicrodepositsSetupIntent<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
