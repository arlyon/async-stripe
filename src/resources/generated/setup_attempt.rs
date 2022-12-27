// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{SetupAttemptId, SetupIntentId};
use crate::params::{Expand, Expandable, List, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{
    Account, ApiErrors, Application, Customer, Mandate, PaymentMethod, SetupIntent,
    ThreeDSecureDetails,
};

/// The resource representing a Stripe "PaymentFlowsSetupIntentSetupAttempt".
///
/// For more details see <https://stripe.com/docs/api/setup_attempts/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttempt {
    /// Unique identifier for the object.
    pub id: SetupAttemptId,

    /// The value of [application](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-application) on the SetupIntent at the time of this confirmation.
    pub application: Option<Expandable<Application>>,

    /// If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.
    ///
    /// It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers.
    ///
    /// It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The value of [customer](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-customer) on the SetupIntent at the time of this confirmation.
    pub customer: Option<Expandable<Customer>>,

    /// Indicates the directions of money movement for which this payment method is intended to be used.
    ///
    /// Include `inbound` if you intend to use the payment method as the origin to pull funds from.
    ///
    /// Include `outbound` if you intend to use the payment method as the destination to send funds to.
    /// You can include both if you intend to use the payment method for both purposes.
    pub flow_directions: Option<Vec<SetupAttemptFlowDirections>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The value of [on_behalf_of](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-on_behalf_of) on the SetupIntent at the time of this confirmation.
    pub on_behalf_of: Option<Expandable<Account>>,

    /// ID of the payment method used with this SetupAttempt.
    pub payment_method: Expandable<PaymentMethod>,

    pub payment_method_details: SetupAttemptPaymentMethodDetails,

    /// The error encountered during this attempt to confirm the SetupIntent, if any.
    pub setup_error: Option<Box<ApiErrors>>,

    /// ID of the SetupIntent that this attempt belongs to.
    pub setup_intent: Expandable<SetupIntent>,

    /// Status of this SetupAttempt, one of `requires_confirmation`, `requires_action`, `processing`, `succeeded`, `failed`, or `abandoned`.
    pub status: String,

    /// The value of [usage](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-usage) on the SetupIntent at the time of this confirmation, one of `off_session` or `on_session`.
    pub usage: String,
}

impl SetupAttempt {
    /// Returns a list of SetupAttempts associated with a provided SetupIntent.
    pub fn list(client: &Client, params: &ListSetupAttempts<'_>) -> Response<List<SetupAttempt>> {
        client.get_query("/setup_attempts", &params)
    }
}

impl Object for SetupAttempt {
    type Id = SetupAttemptId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "setup_attempt"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<SetupAttemptPaymentMethodDetailsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<SetupAttemptPaymentMethodDetailsAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<SetupAttemptPaymentMethodDetailsBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<SetupAttemptPaymentMethodDetailsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<SetupAttemptPaymentMethodDetailsBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<SetupAttemptPaymentMethodDetailsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SetupAttemptPaymentMethodDetailsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<SetupAttemptPaymentMethodDetailsCardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<SetupAttemptPaymentMethodDetailsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<SetupAttemptPaymentMethodDetailsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupAttemptPaymentMethodDetailsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SetupAttemptPaymentMethodDetailsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<SetupAttemptPaymentMethodDetailsSofort>,

    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<SetupAttemptPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsAcssDebit {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsAuBecsDebit {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsBacsDebit {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsBancontact {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<Expandable<PaymentMethod>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<Expandable<Mandate>>,

    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,

    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`.
    pub preferred_language: Option<SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Bancontact directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsBlik {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsBoleto {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCard {
    /// Populated if this authorization used 3D Secure authentication.
    pub three_d_secure: Option<ThreeDSecureDetails>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCardPresent {
    /// The ID of the Card PaymentMethod which was generated by this SetupAttempt.
    pub generated_card: Option<Expandable<PaymentMethod>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsIdeal {
    /// The customer's bank.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    pub bank: Option<SetupAttemptPaymentMethodDetailsIdealBank>,

    /// The Bank Identifier Code of the customer's bank.
    pub bic: Option<SetupAttemptPaymentMethodDetailsIdealBic>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<Expandable<PaymentMethod>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<Expandable<Mandate>>,

    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by iDEAL directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsKlarna {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsLink {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsSepaDebit {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsSofort {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,

    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,

    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,

    /// The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit: Option<Expandable<PaymentMethod>>,

    /// The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt.
    pub generated_sepa_debit_mandate: Option<Expandable<Mandate>>,

    /// Last four characters of the IBAN.
    pub iban_last4: Option<String>,

    /// Preferred language of the Sofort authorization page that the customer is redirected to.
    /// Can be one of `en`, `de`, `fr`, or `nl`.
    pub preferred_language: Option<SetupAttemptPaymentMethodDetailsSofortPreferredLanguage>,

    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Sofort directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsUsBankAccount {}

/// The parameters for `SetupAttempt::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSetupAttempts<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SetupAttemptId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return SetupAttempts created by the SetupIntent specified by
    /// this ID.
    pub setup_intent: SetupIntentId,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SetupAttemptId>,
}

impl<'a> ListSetupAttempts<'a> {
    pub fn new(setup_intent: SetupIntentId) -> Self {
        ListSetupAttempts {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            setup_intent,
            starting_after: Default::default(),
        }
    }
}

impl Paginable for ListSetupAttempts<'_> {
    type O = SetupAttempt;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}

/// An enum representing the possible values of an `SetupAttempt`'s `flow_directions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptFlowDirections {
    Inbound,
    Outbound,
}

impl SetupAttemptFlowDirections {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupAttemptFlowDirections::Inbound => "inbound",
            SetupAttemptFlowDirections::Outbound => "outbound",
        }
    }
}

impl AsRef<str> for SetupAttemptFlowDirections {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptFlowDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupAttemptFlowDirections {
    fn default() -> Self {
        Self::Inbound
    }
}

/// An enum representing the possible values of an `SetupAttemptPaymentMethodDetailsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage::De => "de",
            SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage::En => "en",
            SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage::Fr => "fr",
            SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str> for SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `SetupAttemptPaymentMethodDetailsIdeal`'s `bank` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsIdealBank {
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

impl SetupAttemptPaymentMethodDetailsIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupAttemptPaymentMethodDetailsIdealBank::AbnAmro => "abn_amro",
            SetupAttemptPaymentMethodDetailsIdealBank::AsnBank => "asn_bank",
            SetupAttemptPaymentMethodDetailsIdealBank::Bunq => "bunq",
            SetupAttemptPaymentMethodDetailsIdealBank::Handelsbanken => "handelsbanken",
            SetupAttemptPaymentMethodDetailsIdealBank::Ing => "ing",
            SetupAttemptPaymentMethodDetailsIdealBank::Knab => "knab",
            SetupAttemptPaymentMethodDetailsIdealBank::Moneyou => "moneyou",
            SetupAttemptPaymentMethodDetailsIdealBank::Rabobank => "rabobank",
            SetupAttemptPaymentMethodDetailsIdealBank::Regiobank => "regiobank",
            SetupAttemptPaymentMethodDetailsIdealBank::Revolut => "revolut",
            SetupAttemptPaymentMethodDetailsIdealBank::SnsBank => "sns_bank",
            SetupAttemptPaymentMethodDetailsIdealBank::TriodosBank => "triodos_bank",
            SetupAttemptPaymentMethodDetailsIdealBank::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for SetupAttemptPaymentMethodDetailsIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptPaymentMethodDetailsIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupAttemptPaymentMethodDetailsIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}

/// An enum representing the possible values of an `SetupAttemptPaymentMethodDetailsIdeal`'s `bic` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsIdealBic {
    #[serde(rename = "ABNANL2A")]
    Abnanl2a,
    #[serde(rename = "ASNBNL21")]
    Asnbnl21,
    #[serde(rename = "BUNQNL2A")]
    Bunqnl2a,
    #[serde(rename = "FVLBNL22")]
    Fvlbnl22,
    #[serde(rename = "HANDNL2A")]
    Handnl2a,
    #[serde(rename = "INGBNL2A")]
    Ingbnl2a,
    #[serde(rename = "KNABNL2H")]
    Knabnl2h,
    #[serde(rename = "MOYONL21")]
    Moyonl21,
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
    #[serde(rename = "REVOLT21")]
    Revolt21,
    #[serde(rename = "SNSBNL2A")]
    Snsbnl2a,
    #[serde(rename = "TRIONL2U")]
    Trionl2u,
}

impl SetupAttemptPaymentMethodDetailsIdealBic {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupAttemptPaymentMethodDetailsIdealBic::Abnanl2a => "ABNANL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Asnbnl21 => "ASNBNL21",
            SetupAttemptPaymentMethodDetailsIdealBic::Bunqnl2a => "BUNQNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Fvlbnl22 => "FVLBNL22",
            SetupAttemptPaymentMethodDetailsIdealBic::Handnl2a => "HANDNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Ingbnl2a => "INGBNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Knabnl2h => "KNABNL2H",
            SetupAttemptPaymentMethodDetailsIdealBic::Moyonl21 => "MOYONL21",
            SetupAttemptPaymentMethodDetailsIdealBic::Rabonl2u => "RABONL2U",
            SetupAttemptPaymentMethodDetailsIdealBic::Rbrbnl21 => "RBRBNL21",
            SetupAttemptPaymentMethodDetailsIdealBic::Revolt21 => "REVOLT21",
            SetupAttemptPaymentMethodDetailsIdealBic::Snsbnl2a => "SNSBNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Trionl2u => "TRIONL2U",
        }
    }
}

impl AsRef<str> for SetupAttemptPaymentMethodDetailsIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptPaymentMethodDetailsIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupAttemptPaymentMethodDetailsIdealBic {
    fn default() -> Self {
        Self::Abnanl2a
    }
}

/// An enum representing the possible values of an `SetupAttemptPaymentMethodDetailsSofort`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupAttemptPaymentMethodDetailsSofortPreferredLanguage::De => "de",
            SetupAttemptPaymentMethodDetailsSofortPreferredLanguage::En => "en",
            SetupAttemptPaymentMethodDetailsSofortPreferredLanguage::Fr => "fr",
            SetupAttemptPaymentMethodDetailsSofortPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str> for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupAttemptPaymentMethodDetailsSofortPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}
