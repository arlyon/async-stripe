#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<Status>,
    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<Status>,
    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<Status>,
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<Status>,
    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<Status>,
    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<Status>,
    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<Status>,
    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<Status>,
    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<Status>,
    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<Status>,
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<Status>,
    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<Status>,
    /// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<Status>,
    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<Status>,
    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<Status>,
    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<Status>,
    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<Status>,
    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<Status>,
    /// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<Status>,
    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<Status>,
    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<Status>,
    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<Status>,
    /// The status of the legacy payments capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<Status>,
    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<Status>,
    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<Status>,
    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<Status>,
    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<Status>,
    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<Status>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<Status>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<Status>,
    /// The status of the tax reporting 1099-K (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<Status>,
    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<Status>,
    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<Status>,
    /// The status of the banking capability, or whether the account can have bank accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<Status>,
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<Status>,
    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<Status>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        use Status::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Pending => "pending",
        }
    }
}

impl std::str::FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Status::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "pending" => Ok(Pending),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Status {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Status {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for Status"))
    }
}
