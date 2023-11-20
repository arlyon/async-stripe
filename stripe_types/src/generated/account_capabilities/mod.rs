#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<AccountCapabilitiesAcssDebitPayments>,
    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<AccountCapabilitiesAffirmPayments>,
    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<AccountCapabilitiesAfterpayClearpayPayments>,
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<AccountCapabilitiesAuBecsDebitPayments>,
    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<AccountCapabilitiesBacsDebitPayments>,
    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<AccountCapabilitiesBancontactPayments>,
    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<AccountCapabilitiesBankTransferPayments>,
    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<AccountCapabilitiesBlikPayments>,
    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<AccountCapabilitiesBoletoPayments>,
    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<AccountCapabilitiesCardIssuing>,
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<AccountCapabilitiesCardPayments>,
    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<AccountCapabilitiesCartesBancairesPayments>,
    /// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<AccountCapabilitiesCashappPayments>,
    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<AccountCapabilitiesEpsPayments>,
    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<AccountCapabilitiesFpxPayments>,
    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<AccountCapabilitiesGiropayPayments>,
    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<AccountCapabilitiesGrabpayPayments>,
    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<AccountCapabilitiesIdealPayments>,
    /// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<AccountCapabilitiesIndiaInternationalPayments>,
    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<AccountCapabilitiesJcbPayments>,
    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<AccountCapabilitiesKlarnaPayments>,
    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<AccountCapabilitiesKonbiniPayments>,
    /// The status of the legacy payments capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<AccountCapabilitiesLegacyPayments>,
    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<AccountCapabilitiesLinkPayments>,
    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<AccountCapabilitiesOxxoPayments>,
    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<AccountCapabilitiesP24Payments>,
    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<AccountCapabilitiesPaynowPayments>,
    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<AccountCapabilitiesPromptpayPayments>,
    /// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<AccountCapabilitiesRevolutPayPayments>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<AccountCapabilitiesSepaDebitPayments>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<AccountCapabilitiesSofortPayments>,
    /// The status of the tax reporting 1099-K (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<AccountCapabilitiesTaxReportingUs1099K>,
    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<AccountCapabilitiesTaxReportingUs1099Misc>,
    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<AccountCapabilitiesTransfers>,
    /// The status of the banking capability, or whether the account can have bank accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<AccountCapabilitiesTreasury>,
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<AccountCapabilitiesUsBankAccountAchPayments>,
    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<AccountCapabilitiesZipPayments>,
}
/// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesAcssDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesAcssDebitPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesAcssDebitPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesAcssDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesAcssDebitPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesAcssDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesAcssDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesAcssDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesAcssDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesAcssDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesAcssDebitPayments")
        })
    }
}
/// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesAffirmPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesAffirmPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesAffirmPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesAffirmPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesAffirmPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesAffirmPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesAffirmPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesAffirmPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesAffirmPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesAffirmPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesAffirmPayments")
        })
    }
}
/// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesAfterpayClearpayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesAfterpayClearpayPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesAfterpayClearpayPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesAfterpayClearpayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesAfterpayClearpayPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesAfterpayClearpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesAfterpayClearpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesAfterpayClearpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesAfterpayClearpayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesAfterpayClearpayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AccountCapabilitiesAfterpayClearpayPayments",
            )
        })
    }
}
/// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesAuBecsDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesAuBecsDebitPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesAuBecsDebitPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesAuBecsDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesAuBecsDebitPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesAuBecsDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesAuBecsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesAuBecsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesAuBecsDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesAuBecsDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesAuBecsDebitPayments")
        })
    }
}
/// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesBacsDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBacsDebitPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesBacsDebitPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesBacsDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesBacsDebitPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBacsDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBacsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesBacsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesBacsDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesBacsDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesBacsDebitPayments")
        })
    }
}
/// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesBancontactPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBancontactPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesBancontactPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesBancontactPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesBancontactPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBancontactPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBancontactPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesBancontactPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesBancontactPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesBancontactPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesBancontactPayments")
        })
    }
}
/// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesBankTransferPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBankTransferPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesBankTransferPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesBankTransferPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesBankTransferPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBankTransferPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesBankTransferPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesBankTransferPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesBankTransferPayments")
        })
    }
}
/// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesBlikPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBlikPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesBlikPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesBlikPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesBlikPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBlikPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBlikPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesBlikPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesBlikPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesBlikPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesBlikPayments")
        })
    }
}
/// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesBoletoPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesBoletoPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesBoletoPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesBoletoPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesBoletoPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesBoletoPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesBoletoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesBoletoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesBoletoPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesBoletoPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesBoletoPayments")
        })
    }
}
/// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesCardIssuing {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesCardIssuing {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesCardIssuing::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesCardIssuing {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesCardIssuing::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesCardIssuing {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesCardIssuing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesCardIssuing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesCardIssuing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesCardIssuing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesCardIssuing")
        })
    }
}
/// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesCardPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesCardPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesCardPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesCardPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesCardPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesCardPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesCardPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesCardPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesCardPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesCardPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesCardPayments")
        })
    }
}
/// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesCartesBancairesPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesCartesBancairesPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesCartesBancairesPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesCartesBancairesPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesCartesBancairesPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesCartesBancairesPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesCartesBancairesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesCartesBancairesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesCartesBancairesPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesCartesBancairesPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesCartesBancairesPayments")
        })
    }
}
/// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesCashappPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesCashappPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesCashappPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesCashappPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesCashappPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesCashappPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesCashappPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesCashappPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesCashappPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesCashappPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesCashappPayments")
        })
    }
}
/// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesEpsPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesEpsPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesEpsPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesEpsPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesEpsPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesEpsPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesEpsPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesEpsPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesEpsPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesEpsPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesEpsPayments")
        })
    }
}
/// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesFpxPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesFpxPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesFpxPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesFpxPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesFpxPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesFpxPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesFpxPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesFpxPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesFpxPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesFpxPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesFpxPayments")
        })
    }
}
/// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesGiropayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesGiropayPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesGiropayPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesGiropayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesGiropayPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesGiropayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesGiropayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesGiropayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesGiropayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesGiropayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesGiropayPayments")
        })
    }
}
/// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesGrabpayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesGrabpayPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesGrabpayPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesGrabpayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesGrabpayPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesGrabpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesGrabpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesGrabpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesGrabpayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesGrabpayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesGrabpayPayments")
        })
    }
}
/// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesIdealPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesIdealPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesIdealPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesIdealPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesIdealPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesIdealPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesIdealPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesIdealPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesIdealPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesIdealPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesIdealPayments")
        })
    }
}
/// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesIndiaInternationalPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesIndiaInternationalPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesIndiaInternationalPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesIndiaInternationalPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesIndiaInternationalPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesIndiaInternationalPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesIndiaInternationalPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesIndiaInternationalPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesIndiaInternationalPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesIndiaInternationalPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AccountCapabilitiesIndiaInternationalPayments",
            )
        })
    }
}
/// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesJcbPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesJcbPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesJcbPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesJcbPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesJcbPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesJcbPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesJcbPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesJcbPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesJcbPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesJcbPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesJcbPayments")
        })
    }
}
/// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesKlarnaPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesKlarnaPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesKlarnaPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesKlarnaPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesKlarnaPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesKlarnaPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesKlarnaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesKlarnaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesKlarnaPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesKlarnaPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesKlarnaPayments")
        })
    }
}
/// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesKonbiniPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesKonbiniPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesKonbiniPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesKonbiniPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesKonbiniPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesKonbiniPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesKonbiniPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesKonbiniPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesKonbiniPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesKonbiniPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesKonbiniPayments")
        })
    }
}
/// The status of the legacy payments capability of the account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesLegacyPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesLegacyPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesLegacyPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesLegacyPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesLegacyPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesLegacyPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesLegacyPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesLegacyPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesLegacyPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesLegacyPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesLegacyPayments")
        })
    }
}
/// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesLinkPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesLinkPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesLinkPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesLinkPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesLinkPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesLinkPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesLinkPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesLinkPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesLinkPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesLinkPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesLinkPayments")
        })
    }
}
/// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesOxxoPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesOxxoPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesOxxoPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesOxxoPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesOxxoPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesOxxoPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesOxxoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesOxxoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesOxxoPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesOxxoPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesOxxoPayments")
        })
    }
}
/// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesP24Payments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesP24Payments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesP24Payments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesP24Payments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesP24Payments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesP24Payments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesP24Payments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesP24Payments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesP24Payments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesP24Payments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesP24Payments")
        })
    }
}
/// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesPaynowPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesPaynowPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesPaynowPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesPaynowPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesPaynowPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesPaynowPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesPaynowPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesPaynowPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesPaynowPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesPaynowPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesPaynowPayments")
        })
    }
}
/// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesPromptpayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesPromptpayPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesPromptpayPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesPromptpayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesPromptpayPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesPromptpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesPromptpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesPromptpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesPromptpayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesPromptpayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesPromptpayPayments")
        })
    }
}
/// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesRevolutPayPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesRevolutPayPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesRevolutPayPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesRevolutPayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesRevolutPayPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesRevolutPayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesRevolutPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesRevolutPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesRevolutPayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesRevolutPayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesRevolutPayPayments")
        })
    }
}
/// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesSepaDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesSepaDebitPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesSepaDebitPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesSepaDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesSepaDebitPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesSepaDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesSepaDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesSepaDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesSepaDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesSepaDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesSepaDebitPayments")
        })
    }
}
/// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesSofortPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesSofortPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesSofortPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesSofortPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesSofortPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesSofortPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesSofortPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesSofortPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesSofortPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesSofortPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesSofortPayments")
        })
    }
}
/// The status of the tax reporting 1099-K (US) capability of the account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesTaxReportingUs1099K {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesTaxReportingUs1099K {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesTaxReportingUs1099K::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesTaxReportingUs1099K {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesTaxReportingUs1099K::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesTaxReportingUs1099K {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesTaxReportingUs1099K {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesTaxReportingUs1099K {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesTaxReportingUs1099K {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesTaxReportingUs1099K {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesTaxReportingUs1099K")
        })
    }
}
/// The status of the tax reporting 1099-MISC (US) capability of the account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesTaxReportingUs1099Misc {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesTaxReportingUs1099Misc {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesTaxReportingUs1099Misc::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesTaxReportingUs1099Misc {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesTaxReportingUs1099Misc::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesTaxReportingUs1099Misc {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesTaxReportingUs1099Misc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesTaxReportingUs1099Misc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesTaxReportingUs1099Misc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesTaxReportingUs1099Misc {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesTaxReportingUs1099Misc")
        })
    }
}
/// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesTransfers {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesTransfers {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesTransfers::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesTransfers {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesTransfers::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesTransfers {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesTransfers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesTransfers {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountCapabilitiesTransfers"))
    }
}
/// The status of the banking capability, or whether the account can have bank accounts.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesTreasury {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesTreasury {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesTreasury::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesTreasury {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesTreasury::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesTreasury {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesTreasury {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesTreasury {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for AccountCapabilitiesTreasury"))
    }
}
/// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesUsBankAccountAchPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesUsBankAccountAchPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesUsBankAccountAchPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesUsBankAccountAchPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesUsBankAccountAchPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesUsBankAccountAchPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesUsBankAccountAchPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesUsBankAccountAchPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesUsBankAccountAchPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesUsBankAccountAchPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for AccountCapabilitiesUsBankAccountAchPayments",
            )
        })
    }
}
/// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AccountCapabilitiesZipPayments {
    Active,
    Inactive,
    Pending,
}

impl AccountCapabilitiesZipPayments {
    pub fn as_str(self) -> &'static str {
        use AccountCapabilitiesZipPayments::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for AccountCapabilitiesZipPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AccountCapabilitiesZipPayments::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for AccountCapabilitiesZipPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AccountCapabilitiesZipPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for AccountCapabilitiesZipPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for AccountCapabilitiesZipPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountCapabilitiesZipPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for AccountCapabilitiesZipPayments")
        })
    }
}
