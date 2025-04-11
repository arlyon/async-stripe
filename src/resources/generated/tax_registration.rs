// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{TaxRegistrationId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TaxProductRegistrationsResourceTaxRegistration".
///
/// For more details see <https://stripe.com/docs/api/tax/registrations/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxRegistration {
    /// Unique identifier for the object.
    pub id: TaxRegistrationId,

    /// Time at which the registration becomes active.
    ///
    /// Measured in seconds since the Unix epoch.
    pub active_from: Timestamp,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    pub country_options: TaxProductRegistrationsResourceCountryOptions,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// If set, the registration stops being active at this time.
    ///
    /// If not set, the registration will be active indefinitely.
    /// Measured in seconds since the Unix epoch.
    pub expires_at: Option<Timestamp>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The status of the registration.
    ///
    /// This field is present for convenience and can be deduced from `active_from` and `expires_at`.
    pub status: TaxRegistrationStatus,
}

impl Object for TaxRegistration {
    type Id = TaxRegistrationId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "tax.registration"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptions {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub al: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub am: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ao: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ba: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bb: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub be: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bh: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<TaxProductRegistrationsResourceCountryOptionsCanada>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub co: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cr: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cz: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub de: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dk: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ee: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fi: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gn: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hu: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ie: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub it: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kz: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lu: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ma: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub md: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub me: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mk: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mr: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub my: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ng: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nl: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub np: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub om: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pe: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pl: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rs: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub se: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sg: Option<TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sk: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sr: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub th: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tj: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ug: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us: Option<TaxProductRegistrationsResourceCountryOptionsUnitedStates>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uy: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uz: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vn: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub za: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zm: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zw: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsCanada {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_standard: Option<TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,

    /// Type of registration in Canada.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsCanadaType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {

    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefault {

    /// Type of registration in `country`.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsDefaultType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods {

    /// Type of registration in `country`.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoodsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsEurope {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<TaxProductRegistrationsResourceCountryOptionsEuStandard>,

    /// Type of registration in an EU country.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsEuropeType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsEuStandard {

    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsSimplified {

    /// Type of registration in `country`.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsSimplifiedType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,

    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_sales_tax: Option<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax>,

    /// Type of registration in the US.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsUnitedStatesType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {

    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    pub jurisdiction: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax {

    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    pub jurisdiction: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax {

    /// Elections for the state sales tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elections: Option<Vec<TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {

    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,

    /// The type of the election for the state sales tax registration.
    #[serde(rename = "type")]
    pub type_: TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType,
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsCanada`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsCanadaType {
    ProvinceStandard,
    Simplified,
    Standard,
}

impl TaxProductRegistrationsResourceCountryOptionsCanadaType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsCanadaType::ProvinceStandard => "province_standard",
            TaxProductRegistrationsResourceCountryOptionsCanadaType::Simplified => "simplified",
            TaxProductRegistrationsResourceCountryOptionsCanadaType::Standard => "standard",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn default() -> Self {
        Self::ProvinceStandard
    }
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoodsType {
    Standard,
}

impl TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoodsType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoodsType::Standard => "standard",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoodsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoodsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoodsType {
    fn default() -> Self {
        Self::Standard
    }
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsDefault`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultType {
    Standard,
}

impl TaxProductRegistrationsResourceCountryOptionsDefaultType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsDefaultType::Standard => "standard",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn default() -> Self {
        Self::Standard
    }
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsEuStandard`'s `place_of_supply_scheme` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}

impl TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::SmallSeller => "small_seller",
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::Standard => "standard",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    fn default() -> Self {
        Self::SmallSeller
    }
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsEurope`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsEuropeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl TaxProductRegistrationsResourceCountryOptionsEuropeType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsEuropeType::Ioss => "ioss",
            TaxProductRegistrationsResourceCountryOptionsEuropeType::OssNonUnion => "oss_non_union",
            TaxProductRegistrationsResourceCountryOptionsEuropeType::OssUnion => "oss_union",
            TaxProductRegistrationsResourceCountryOptionsEuropeType::Standard => "standard",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn default() -> Self {
        Self::Ioss
    }
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsSimplified`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    Simplified,
}

impl TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsSimplifiedType::Simplified => "simplified",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn default() -> Self {
        Self::Simplified
    }
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsUnitedStates`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateRetailDeliveryFee,
    StateSalesTax,
}

impl TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LocalAmusementTax => "local_amusement_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LocalLeaseTax => "local_lease_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateCommunicationsTax => "state_communications_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateRetailDeliveryFee => "state_retail_delivery_fee",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateSalesTax => "state_sales_tax",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn default() -> Self {
        Self::LocalAmusementTax
    }
}

/// An enum representing the possible values of an `TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    LocalUseTax,
    SimplifiedSellersUseTax,
    SingleLocalUseTax,
}

impl TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::LocalUseTax => "local_use_tax",
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::SimplifiedSellersUseTax => "simplified_sellers_use_tax",
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::SingleLocalUseTax => "single_local_use_tax",
        }
    }
}

impl AsRef<str> for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn default() -> Self {
        Self::LocalUseTax
    }
}

/// An enum representing the possible values of an `TaxRegistration`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxRegistrationStatus {
    Active,
    Expired,
    Scheduled,
}

impl TaxRegistrationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxRegistrationStatus::Active => "active",
            TaxRegistrationStatus::Expired => "expired",
            TaxRegistrationStatus::Scheduled => "scheduled",
        }
    }
}

impl AsRef<str> for TaxRegistrationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxRegistrationStatus {
    fn default() -> Self {
        Self::Active
    }
}
