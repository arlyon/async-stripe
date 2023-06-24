#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Capabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<CapabilitiesAcssDebitPayments>,
    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<CapabilitiesAffirmPayments>,
    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<CapabilitiesAfterpayClearpayPayments>,
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<CapabilitiesAuBecsDebitPayments>,
    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<CapabilitiesBacsDebitPayments>,
    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<CapabilitiesBancontactPayments>,
    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<CapabilitiesBankTransferPayments>,
    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<CapabilitiesBlikPayments>,
    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<CapabilitiesBoletoPayments>,
    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CapabilitiesCardIssuing>,
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CapabilitiesCardPayments>,
    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<CapabilitiesCartesBancairesPayments>,
    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<CapabilitiesEpsPayments>,
    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<CapabilitiesFpxPayments>,
    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<CapabilitiesGiropayPayments>,
    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<CapabilitiesGrabpayPayments>,
    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<CapabilitiesIdealPayments>,
    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<CapabilitiesJcbPayments>,
    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<CapabilitiesKlarnaPayments>,
    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<CapabilitiesKonbiniPayments>,
    /// The status of the legacy payments capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CapabilitiesLegacyPayments>,
    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<CapabilitiesLinkPayments>,
    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<CapabilitiesOxxoPayments>,
    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<CapabilitiesP24Payments>,
    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<CapabilitiesPaynowPayments>,
    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<CapabilitiesPromptpayPayments>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<CapabilitiesSepaDebitPayments>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<CapabilitiesSofortPayments>,
    /// The status of the tax reporting 1099-K (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<CapabilitiesTaxReportingUs1099K>,
    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<CapabilitiesTaxReportingUs1099Misc>,
    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CapabilitiesTransfers>,
    /// The status of the banking capability, or whether the account can have bank accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CapabilitiesTreasury>,
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<CapabilitiesUsBankAccountAchPayments>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Capabilities {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesAcssDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesAcssDebitPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesAcssDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesAcssDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesAcssDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesAcssDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesAcssDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesAcssDebitPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesAcssDebitPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesAcssDebitPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesAcssDebitPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesAffirmPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesAffirmPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesAffirmPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesAffirmPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesAffirmPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesAffirmPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesAffirmPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesAffirmPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesAffirmPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesAffirmPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesAffirmPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesAfterpayClearpayPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesAfterpayClearpayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesAfterpayClearpayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesAfterpayClearpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesAfterpayClearpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesAfterpayClearpayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesAfterpayClearpayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesAfterpayClearpayPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesAfterpayClearpayPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesAfterpayClearpayPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesAfterpayClearpayPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesAuBecsDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesAuBecsDebitPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesAuBecsDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesAuBecsDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesAuBecsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesAuBecsDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesAuBecsDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesAuBecsDebitPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesAuBecsDebitPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesAuBecsDebitPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesAuBecsDebitPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesBacsDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesBacsDebitPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesBacsDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesBacsDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesBacsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesBacsDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesBacsDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesBacsDebitPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesBacsDebitPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesBacsDebitPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesBacsDebitPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesBancontactPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesBancontactPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesBancontactPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesBancontactPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesBancontactPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesBancontactPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesBancontactPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesBancontactPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesBancontactPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesBancontactPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesBancontactPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesBankTransferPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesBankTransferPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesBankTransferPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesBankTransferPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesBankTransferPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesBankTransferPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesBankTransferPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesBankTransferPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesBankTransferPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesBankTransferPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesBlikPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesBlikPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesBlikPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesBlikPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesBlikPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesBlikPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesBlikPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesBlikPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesBlikPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesBlikPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesBlikPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesBoletoPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesBoletoPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesBoletoPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesBoletoPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesBoletoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesBoletoPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesBoletoPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesBoletoPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesBoletoPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesBoletoPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesBoletoPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesCardIssuing {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesCardIssuing {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesCardIssuing {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesCardIssuing {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesCardIssuing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesCardIssuing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesCardIssuing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesCardIssuing"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesCardIssuing {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesCardIssuing> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesCardIssuing::from_str(s)?);
        Ok(())
    }
}
/// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesCardPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesCardPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesCardPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesCardPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesCardPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesCardPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesCardPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesCardPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesCardPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesCardPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesCardPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesCartesBancairesPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesCartesBancairesPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesCartesBancairesPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesCartesBancairesPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesCartesBancairesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesCartesBancairesPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesCartesBancairesPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesCartesBancairesPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesCartesBancairesPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesCartesBancairesPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesCartesBancairesPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesEpsPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesEpsPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesEpsPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesEpsPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesEpsPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesEpsPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesEpsPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesEpsPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesEpsPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesEpsPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesEpsPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesFpxPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesFpxPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesFpxPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesFpxPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesFpxPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesFpxPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesFpxPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesFpxPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesFpxPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesFpxPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesFpxPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesGiropayPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesGiropayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesGiropayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesGiropayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesGiropayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesGiropayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesGiropayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesGiropayPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesGiropayPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesGiropayPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesGiropayPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesGrabpayPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesGrabpayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesGrabpayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesGrabpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesGrabpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesGrabpayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesGrabpayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesGrabpayPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesGrabpayPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesGrabpayPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesGrabpayPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesIdealPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesIdealPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesIdealPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesIdealPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesIdealPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesIdealPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesIdealPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesIdealPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesIdealPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesIdealPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesIdealPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesJcbPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesJcbPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesJcbPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesJcbPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesJcbPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesJcbPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesJcbPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesJcbPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesJcbPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesJcbPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesJcbPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesKlarnaPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesKlarnaPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesKlarnaPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesKlarnaPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesKlarnaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesKlarnaPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesKlarnaPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesKlarnaPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesKlarnaPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesKlarnaPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesKlarnaPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesKonbiniPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesKonbiniPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesKonbiniPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesKonbiniPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesKonbiniPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesKonbiniPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesKonbiniPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesKonbiniPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesKonbiniPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesKonbiniPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesKonbiniPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the legacy payments capability of the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesLegacyPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesLegacyPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesLegacyPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesLegacyPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesLegacyPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesLegacyPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesLegacyPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesLegacyPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesLegacyPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesLegacyPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesLegacyPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesLinkPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesLinkPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesLinkPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesLinkPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesLinkPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesLinkPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesLinkPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesLinkPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesLinkPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesLinkPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesLinkPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesOxxoPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesOxxoPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesOxxoPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesOxxoPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesOxxoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesOxxoPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesOxxoPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesOxxoPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesOxxoPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesOxxoPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesOxxoPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesP24Payments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesP24Payments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesP24Payments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesP24Payments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesP24Payments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesP24Payments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesP24Payments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesP24Payments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesP24Payments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesP24Payments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesP24Payments::from_str(s)?);
        Ok(())
    }
}
/// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesPaynowPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesPaynowPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesPaynowPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesPaynowPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesPaynowPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesPaynowPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesPaynowPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesPaynowPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesPaynowPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesPaynowPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesPaynowPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesPromptpayPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesPromptpayPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesPromptpayPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesPromptpayPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesPromptpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesPromptpayPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesPromptpayPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesPromptpayPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesPromptpayPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesPromptpayPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesPromptpayPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesSepaDebitPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesSepaDebitPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesSepaDebitPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesSepaDebitPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesSepaDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesSepaDebitPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesSepaDebitPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesSepaDebitPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesSepaDebitPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesSepaDebitPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesSepaDebitPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesSofortPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesSofortPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesSofortPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesSofortPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesSofortPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesSofortPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesSofortPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesSofortPayments"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesSofortPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesSofortPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesSofortPayments::from_str(s)?);
        Ok(())
    }
}
/// The status of the tax reporting 1099-K (US) capability of the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesTaxReportingUs1099K {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesTaxReportingUs1099K {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesTaxReportingUs1099K {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesTaxReportingUs1099K {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesTaxReportingUs1099K {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesTaxReportingUs1099K {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesTaxReportingUs1099K {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesTaxReportingUs1099K")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesTaxReportingUs1099K {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesTaxReportingUs1099K> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesTaxReportingUs1099K::from_str(s)?);
        Ok(())
    }
}
/// The status of the tax reporting 1099-MISC (US) capability of the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesTaxReportingUs1099Misc {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesTaxReportingUs1099Misc {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesTaxReportingUs1099Misc {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesTaxReportingUs1099Misc {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesTaxReportingUs1099Misc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesTaxReportingUs1099Misc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesTaxReportingUs1099Misc {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesTaxReportingUs1099Misc")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesTaxReportingUs1099Misc {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesTaxReportingUs1099Misc> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesTaxReportingUs1099Misc::from_str(s)?);
        Ok(())
    }
}
/// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesTransfers {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesTransfers {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesTransfers {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesTransfers {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesTransfers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesTransfers {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesTransfers"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesTransfers {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesTransfers> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesTransfers::from_str(s)?);
        Ok(())
    }
}
/// The status of the banking capability, or whether the account can have bank accounts.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesTreasury {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesTreasury {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesTreasury {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesTreasury {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesTreasury {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesTreasury {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CapabilitiesTreasury"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesTreasury {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesTreasury> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesTreasury::from_str(s)?);
        Ok(())
    }
}
/// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CapabilitiesUsBankAccountAchPayments {
    Active,
    Inactive,
    Pending,
}

impl CapabilitiesUsBankAccountAchPayments {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl std::str::FromStr for CapabilitiesUsBankAccountAchPayments {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CapabilitiesUsBankAccountAchPayments {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilitiesUsBankAccountAchPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CapabilitiesUsBankAccountAchPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CapabilitiesUsBankAccountAchPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CapabilitiesUsBankAccountAchPayments")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CapabilitiesUsBankAccountAchPayments {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<CapabilitiesUsBankAccountAchPayments> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CapabilitiesUsBankAccountAchPayments::from_str(s)?);
        Ok(())
    }
}
