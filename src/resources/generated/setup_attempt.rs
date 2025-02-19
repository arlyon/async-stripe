// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{SetupAttemptId, SetupIntentId};
use crate::params::{Expand, Expandable, List, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{
    Account, ApiErrors, Application, Customer, Mandate, PaymentMethod,
    PaymentMethodDetailsCardWalletApplePay, PaymentMethodDetailsCardWalletGooglePay, SetupIntent,
};
use serde::{Deserialize, Serialize};

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
    /// Returns a list of SetupAttempts that associate with a provided SetupIntent.
    pub fn list(client: &Client, params: &ListSetupAttempts<'_>) -> Response<List<SetupAttempt>> {
        client.get_query("/setup_attempts", params)
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
    pub boleto: Option<SetupAttemptPaymentMethodDetailsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SetupAttemptPaymentMethodDetailsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<SetupAttemptPaymentMethodDetailsCardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<SetupAttemptPaymentMethodDetailsCashapp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<SetupAttemptPaymentMethodDetailsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<SetupAttemptPaymentMethodDetailsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<SetupAttemptPaymentMethodDetailsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<SetupAttemptPaymentMethodDetailsPaypal>,

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
pub struct SetupAttemptPaymentMethodDetailsBoleto {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCard {
    /// Card brand.
    ///
    /// Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub brand: Option<String>,

    /// Check results by Card networks on Card address and CVC at the time of authorization.
    pub checks: Option<SetupAttemptPaymentMethodDetailsCardChecks>,

    /// Two-letter ISO code representing the country of the card.
    ///
    /// You could use this attribute to get a sense of the international breakdown of cards you've collected.
    pub country: Option<String>,

    /// A high-level description of the type of cards issued in this range.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: Option<i64>,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: Option<i64>,

    /// Uniquely identifies this particular card number.
    ///
    /// You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example.
    /// For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.  *As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Card funding type.
    ///
    /// Can be `credit`, `debit`, `prepaid`, or `unknown`.
    pub funding: Option<String>,

    /// Issuer identification number of the card.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,

    /// The name of the card's issuing bank.
    ///
    /// (For internal use only and not typically available in standard API requests.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The last four digits of the card.
    pub last4: Option<String>,

    /// Identifies which network this charge was processed on.
    ///
    /// Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    pub network: Option<String>,

    /// Populated if this authorization used 3D Secure authentication.
    pub three_d_secure: Option<ThreeDSecureDetails>,

    /// If this Card is part of a card wallet, this contains the details of the card wallet.
    pub wallet: Option<SetupAttemptPaymentMethodDetailsCardWallet>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCardChecks {
    /// If a address line1 was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check: Option<String>,

    /// If a address postal code was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check: Option<String>,

    /// If a CVC was provided, results of the check, one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCardPresent {
    /// The ID of the Card PaymentMethod which was generated by this SetupAttempt.
    pub generated_card: Option<Expandable<PaymentMethod>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCardWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<PaymentMethodDetailsCardWalletApplePay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<PaymentMethodDetailsCardWalletGooglePay>,

    /// The type of the card wallet, one of `apple_pay`, `google_pay`, or `link`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: SetupAttemptPaymentMethodDetailsCardWalletType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCashapp {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SetupAttemptPaymentMethodDetailsIdeal {
    /// The customer's bank.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `n26`, `nn`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, `van_lanschot`, or `yoursafe`.
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
pub struct SetupAttemptPaymentMethodDetailsPaypal {}

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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThreeDSecureDetails {
    /// For authenticated transactions: how the customer was authenticated by
    /// the issuing bank.
    pub authentication_flow: Option<ThreeDSecureDetailsAuthenticationFlow>,

    /// The Electronic Commerce Indicator (ECI).
    ///
    /// A protocol-level field indicating what degree of authentication was performed.
    pub electronic_commerce_indicator: Option<ThreeDSecureDetailsElectronicCommerceIndicator>,

    /// Indicates the outcome of 3D Secure authentication.
    pub result: Option<ThreeDSecureDetailsResult>,

    /// Additional information about why 3D Secure succeeded or failed based
    /// on the `result`.
    pub result_reason: Option<ThreeDSecureDetailsResultReason>,

    /// The 3D Secure 1 XID or 3D Secure 2 Directory Server Transaction ID
    /// (dsTransId) for this payment.
    pub transaction_id: Option<String>,

    /// The version of 3D Secure that was used.
    pub version: Option<ThreeDSecureDetailsVersion>,
}

/// The parameters for `SetupAttempt::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSetupAttempts<'a> {
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp or a dictionary with a number of different query options.
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

/// An enum representing the possible values of an `SetupAttemptPaymentMethodDetailsCardWallet`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SetupAttemptPaymentMethodDetailsCardWalletType {
    ApplePay,
    GooglePay,
    Link,
}

impl SetupAttemptPaymentMethodDetailsCardWalletType {
    pub fn as_str(self) -> &'static str {
        match self {
            SetupAttemptPaymentMethodDetailsCardWalletType::ApplePay => "apple_pay",
            SetupAttemptPaymentMethodDetailsCardWalletType::GooglePay => "google_pay",
            SetupAttemptPaymentMethodDetailsCardWalletType::Link => "link",
        }
    }
}

impl AsRef<str> for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn default() -> Self {
        Self::ApplePay
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
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
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
            SetupAttemptPaymentMethodDetailsIdealBank::N26 => "n26",
            SetupAttemptPaymentMethodDetailsIdealBank::Nn => "nn",
            SetupAttemptPaymentMethodDetailsIdealBank::Rabobank => "rabobank",
            SetupAttemptPaymentMethodDetailsIdealBank::Regiobank => "regiobank",
            SetupAttemptPaymentMethodDetailsIdealBank::Revolut => "revolut",
            SetupAttemptPaymentMethodDetailsIdealBank::SnsBank => "sns_bank",
            SetupAttemptPaymentMethodDetailsIdealBank::TriodosBank => "triodos_bank",
            SetupAttemptPaymentMethodDetailsIdealBank::VanLanschot => "van_lanschot",
            SetupAttemptPaymentMethodDetailsIdealBank::Yoursafe => "yoursafe",
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
    #[serde(rename = "BITSNL2A")]
    Bitsnl2a,
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
    #[serde(rename = "NNBANL2G")]
    Nnbanl2g,
    #[serde(rename = "NTSBDEB1")]
    Ntsbdeb1,
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
    #[serde(rename = "REVOIE23")]
    Revoie23,
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
            SetupAttemptPaymentMethodDetailsIdealBic::Bitsnl2a => "BITSNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Bunqnl2a => "BUNQNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Fvlbnl22 => "FVLBNL22",
            SetupAttemptPaymentMethodDetailsIdealBic::Handnl2a => "HANDNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Ingbnl2a => "INGBNL2A",
            SetupAttemptPaymentMethodDetailsIdealBic::Knabnl2h => "KNABNL2H",
            SetupAttemptPaymentMethodDetailsIdealBic::Moyonl21 => "MOYONL21",
            SetupAttemptPaymentMethodDetailsIdealBic::Nnbanl2g => "NNBANL2G",
            SetupAttemptPaymentMethodDetailsIdealBic::Ntsbdeb1 => "NTSBDEB1",
            SetupAttemptPaymentMethodDetailsIdealBic::Rabonl2u => "RABONL2U",
            SetupAttemptPaymentMethodDetailsIdealBic::Rbrbnl21 => "RBRBNL21",
            SetupAttemptPaymentMethodDetailsIdealBic::Revoie23 => "REVOIE23",
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

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `authentication_flow` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
}

impl ThreeDSecureDetailsAuthenticationFlow {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsAuthenticationFlow::Challenge => "challenge",
            ThreeDSecureDetailsAuthenticationFlow::Frictionless => "frictionless",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsAuthenticationFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsAuthenticationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsAuthenticationFlow {
    fn default() -> Self {
        Self::Challenge
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `electronic_commerce_indicator` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsElectronicCommerceIndicator {
    #[serde(rename = "01")]
    V01,
    #[serde(rename = "02")]
    V02,
    #[serde(rename = "05")]
    V05,
    #[serde(rename = "06")]
    V06,
    #[serde(rename = "07")]
    V07,
}

impl ThreeDSecureDetailsElectronicCommerceIndicator {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsElectronicCommerceIndicator::V01 => "01",
            ThreeDSecureDetailsElectronicCommerceIndicator::V02 => "02",
            ThreeDSecureDetailsElectronicCommerceIndicator::V05 => "05",
            ThreeDSecureDetailsElectronicCommerceIndicator::V06 => "06",
            ThreeDSecureDetailsElectronicCommerceIndicator::V07 => "07",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn default() -> Self {
        Self::V01
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `result` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResult {
    AttemptAcknowledged,
    Authenticated,
    Exempted,
    Failed,
    NotSupported,
    ProcessingError,
}

impl ThreeDSecureDetailsResult {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsResult::AttemptAcknowledged => "attempt_acknowledged",
            ThreeDSecureDetailsResult::Authenticated => "authenticated",
            ThreeDSecureDetailsResult::Exempted => "exempted",
            ThreeDSecureDetailsResult::Failed => "failed",
            ThreeDSecureDetailsResult::NotSupported => "not_supported",
            ThreeDSecureDetailsResult::ProcessingError => "processing_error",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsResult {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsResult {
    fn default() -> Self {
        Self::AttemptAcknowledged
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `result_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
}

impl ThreeDSecureDetailsResultReason {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsResultReason::Abandoned => "abandoned",
            ThreeDSecureDetailsResultReason::Bypassed => "bypassed",
            ThreeDSecureDetailsResultReason::Canceled => "canceled",
            ThreeDSecureDetailsResultReason::CardNotEnrolled => "card_not_enrolled",
            ThreeDSecureDetailsResultReason::NetworkNotSupported => "network_not_supported",
            ThreeDSecureDetailsResultReason::ProtocolError => "protocol_error",
            ThreeDSecureDetailsResultReason::Rejected => "rejected",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsResultReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsResultReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsResultReason {
    fn default() -> Self {
        Self::Abandoned
    }
}

/// An enum representing the possible values of an `ThreeDSecureDetails`'s `version` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureDetailsVersion {
    #[serde(rename = "1.0.2")]
    V1_0_2,
    #[serde(rename = "2.1.0")]
    V2_1_0,
    #[serde(rename = "2.2.0")]
    V2_2_0,
}

impl ThreeDSecureDetailsVersion {
    pub fn as_str(self) -> &'static str {
        match self {
            ThreeDSecureDetailsVersion::V1_0_2 => "1.0.2",
            ThreeDSecureDetailsVersion::V2_1_0 => "2.1.0",
            ThreeDSecureDetailsVersion::V2_2_0 => "2.2.0",
        }
    }
}

impl AsRef<str> for ThreeDSecureDetailsVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ThreeDSecureDetailsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ThreeDSecureDetailsVersion {
    fn default() -> Self {
        Self::V1_0_2
    }
}
