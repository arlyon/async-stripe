#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListTaxProductRegistrationsResourceTaxRegistration<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// The status of the Tax Registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTaxProductRegistrationsResourceTaxRegistrationStatus>,
}
impl<'a> ListTaxProductRegistrationsResourceTaxRegistration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The status of the Tax Registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTaxProductRegistrationsResourceTaxRegistrationStatus {
    Active,
    All,
    Expired,
    Scheduled,
}

impl ListTaxProductRegistrationsResourceTaxRegistrationStatus {
    pub fn as_str(self) -> &'static str {
        use ListTaxProductRegistrationsResourceTaxRegistrationStatus::*;
        match self {
            Active => "active",
            All => "all",
            Expired => "expired",
            Scheduled => "scheduled",
        }
    }
}

impl std::str::FromStr for ListTaxProductRegistrationsResourceTaxRegistrationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTaxProductRegistrationsResourceTaxRegistrationStatus::*;
        match s {
            "active" => Ok(Active),
            "all" => Ok(All),
            "expired" => Ok(Expired),
            "scheduled" => Ok(Scheduled),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListTaxProductRegistrationsResourceTaxRegistrationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListTaxProductRegistrationsResourceTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTaxProductRegistrationsResourceTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTaxProductRegistrationsResourceTaxRegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListTaxProductRegistrationsResourceTaxRegistration<'a> {
    /// Returns a list of Tax `Registration` objects.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<
        stripe_types::List<stripe_misc::TaxProductRegistrationsResourceTaxRegistration>,
    > {
        client.get_query("/tax/registrations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_misc::TaxProductRegistrationsResourceTaxRegistration> {
        stripe::ListPaginator::from_params("/tax/registrations", self)
    }
}
impl<'a> stripe::PaginationParams for ListTaxProductRegistrationsResourceTaxRegistration<'a> {}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistration<'a> {
    /// Time at which the Tax Registration becomes active.
    ///
    /// It can be either `now` to indicate the current time, or a future timestamp measured in seconds since the Unix epoch.
    pub active_from: CreateTaxProductRegistrationsResourceTaxRegistrationActiveFrom,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: &'a str,
    /// Specific options for a registration in the specified `country`.
    pub country_options: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptions<'a>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If set, the Tax Registration stops being active at this time.
    ///
    /// If not set, the Tax Registration will be active indefinitely.
    /// Timestamp measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<stripe_types::Timestamp>,
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistration<'a> {
    pub fn new(
        active_from: CreateTaxProductRegistrationsResourceTaxRegistrationActiveFrom,
        country: &'a str,
        country_options: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptions<'a>,
    ) -> Self {
        Self {
            active_from,
            country,
            country_options,
            expand: Default::default(),
            expires_at: Default::default(),
        }
    }
}
/// Time at which the Tax Registration becomes active.
///
/// It can be either `now` to indicate the current time, or a future timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationActiveFrom {
    Now,
    Timestamp(stripe_types::Timestamp),
}
/// Specific options for a registration in the specified `country`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptions<'a> {
    /// Options for the registration in AE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAe>,
    /// Options for the registration in AT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAt>,
    /// Options for the registration in AU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAu>,
    /// Options for the registration in BE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub be: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBe>,
    /// Options for the registration in BG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBg>,
    /// Options for the registration in CA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCa<'a>>,
    /// Options for the registration in CH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCh>,
    /// Options for the registration in CL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCl>,
    /// Options for the registration in CO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCo>,
    /// Options for the registration in CY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCy>,
    /// Options for the registration in CZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cz: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCz>,
    /// Options for the registration in DE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDe>,
    /// Options for the registration in DK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dk: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDk>,
    /// Options for the registration in EE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ee: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEe>,
    /// Options for the registration in ES.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEs>,
    /// Options for the registration in FI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fi: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFi>,
    /// Options for the registration in FR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFr>,
    /// Options for the registration in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGb>,
    /// Options for the registration in GR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gr: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGr>,
    /// Options for the registration in HR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hr: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHr>,
    /// Options for the registration in HU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hu: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHu>,
    /// Options for the registration in ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsId>,
    /// Options for the registration in IE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ie: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIe>,
    /// Options for the registration in IS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIs>,
    /// Options for the registration in IT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIt>,
    /// Options for the registration in JP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJp>,
    /// Options for the registration in KR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKr>,
    /// Options for the registration in LT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLt>,
    /// Options for the registration in LU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lu: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLu>,
    /// Options for the registration in LV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLv>,
    /// Options for the registration in MT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mt: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMt>,
    /// Options for the registration in MX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMx>,
    /// Options for the registration in MY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMy>,
    /// Options for the registration in NL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nl: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNl>,
    /// Options for the registration in NO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNo>,
    /// Options for the registration in NZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNz>,
    /// Options for the registration in PL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pl: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPl>,
    /// Options for the registration in PT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPt>,
    /// Options for the registration in RO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRo>,
    /// Options for the registration in SA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSa>,
    /// Options for the registration in SE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSe>,
    /// Options for the registration in SG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sg: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSg>,
    /// Options for the registration in SI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSi>,
    /// Options for the registration in SK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sk: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSk>,
    /// Options for the registration in TH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTh>,
    /// Options for the registration in TR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTr>,
    /// Options for the registration in US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUs<'a>>,
    /// Options for the registration in VN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vn: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVn>,
    /// Options for the registration in ZA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub za: Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZa>,
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Options for the registration in AE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAe {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in AT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAt {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in AU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAu {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAu {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsAuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in BE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBe {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in BG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBg {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBg {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsBgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCa<'a> {
    /// Options for the provincial tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_standard: Option<
        CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaProvinceStandard<'a>,
    >,
    /// Type of registration to be created in Canada.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType,
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCa<'a> {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType,
    ) -> Self {
        Self { province_standard: Default::default(), type_ }
    }
}
/// Options for the provincial tax registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaProvinceStandard<'a>
{
    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: &'a str,
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaProvinceStandard<'a> {
    pub fn new(province: &'a str) -> Self {
        Self { province }
    }
}
/// Type of registration to be created in Canada.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType {
    ProvinceStandard,
    Simplified,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType::*;
        match self {
            ProvinceStandard => "province_standard",
            Simplified => "simplified",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType::*;
        match s {
            "province_standard" => Ok(ProvinceStandard),
            "simplified" => Ok(Simplified),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCh {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsChType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCl {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCl {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsClType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCo {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCo {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCy {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCy {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in CZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCz {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCz {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsCzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in DE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDe {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in DK.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDk {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsDkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in EE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEe {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in ES.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEs {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsEsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in FI.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFi {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFi {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in FR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFr {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsFrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in GB.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGb {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGb {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGbType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in GR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGr {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsGrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in HR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHr {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in HU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHu {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsHuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in ID.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsId {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsId {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in IE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIe {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in IS.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIs {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIs {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in IT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsIt {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsItType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in JP.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJp {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJp {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsJpType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in KR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKr {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsKrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in LT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLt {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in LU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLu {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in LV.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLv {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLv {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsLvType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in MT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMt {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in MX.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMx {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMx {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in MY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMy {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMy {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsMyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in NL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNl {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in NO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNo {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNo {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in NZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNz {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsNzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in PL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPl {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in PT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPt {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsPtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in RO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRo {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRo {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsRoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSa {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSe {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSg {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SI.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSi {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSi {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in SK.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard:
        Option<CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSk {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType,
    ) -> Self {
        Self { standard: Default::default(), type_ }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandard {
    /// Place of supply scheme used in an EU standard registration.
pub place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme,

}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandard {
    pub fn new(
        place_of_supply_scheme: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme,
    ) -> Self {
        Self { place_of_supply_scheme }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme
{
    SmallSeller,
    Standard,
}

impl
    CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme
{
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme::*;
        match s {
    "small_seller" => Ok(SmallSeller),
"standard" => Ok(Standard),
_ => Err(())

        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsSkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in TH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTh {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsThType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in TR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTr {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsTrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in US.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUs<'a> {
    /// Options for the local amusement tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<
        CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsLocalAmusementTax<'a>,
    >,
    /// Options for the local lease tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<
        CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsLocalLeaseTax<'a>,
    >,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: &'a str,
    /// Type of registration to be created in the US.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType,
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUs<'a> {
    pub fn new(
        state: &'a str,
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType,
    ) -> Self {
        Self {
            local_amusement_tax: Default::default(),
            local_lease_tax: Default::default(),
            state,
            type_,
        }
    }
}
/// Options for the local amusement tax registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsLocalAmusementTax<'a>
{
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    ///
    /// Supported FIPS codes are: `14000` (Chicago), `06613` (Bloomington), `21696` (East Dundee), `24582` (Evanston), and `68081` (Schiller Park).
    pub jurisdiction: &'a str,
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsLocalAmusementTax<'a> {
    pub fn new(jurisdiction: &'a str) -> Self {
        Self { jurisdiction }
    }
}
/// Options for the local lease tax registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsLocalLeaseTax<'a> {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    ///
    /// Supported FIPS codes are: `14000` (Chicago).
    pub jurisdiction: &'a str,
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsLocalLeaseTax<'a> {
    pub fn new(jurisdiction: &'a str) -> Self {
        Self { jurisdiction }
    }
}
/// Type of registration to be created in the US.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateSalesTax,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateSalesTax => "state_sales_tax",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_sales_tax" => Ok(StateSalesTax),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsUsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in VN.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVn {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType {
    Simplified,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsVnType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Options for the registration in ZA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType,
}
impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZa {
    pub fn new(
        type_: CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType,
    ) -> Self {
        Self { type_ }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType {
    Standard,
}

impl CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxProductRegistrationsResourceTaxRegistrationCountryOptionsZaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateTaxProductRegistrationsResourceTaxRegistration<'a> {
    /// Creates a new Tax `Registration` object.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_misc::TaxProductRegistrationsResourceTaxRegistration> {
        client.send_form("/tax/registrations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateTaxProductRegistrationsResourceTaxRegistration<'a> {
    /// Time at which the registration becomes active.
    ///
    /// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_from: Option<UpdateTaxProductRegistrationsResourceTaxRegistrationActiveFrom>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If set, the registration stops being active at this time.
    ///
    /// If not set, the registration will be active indefinitely.
    /// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<UpdateTaxProductRegistrationsResourceTaxRegistrationExpiresAt>,
}
impl<'a> UpdateTaxProductRegistrationsResourceTaxRegistration<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Time at which the registration becomes active.
///
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateTaxProductRegistrationsResourceTaxRegistrationActiveFrom {
    Now,
    Timestamp(stripe_types::Timestamp),
}
/// If set, the registration stops being active at this time.
///
/// If not set, the registration will be active indefinitely.
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum UpdateTaxProductRegistrationsResourceTaxRegistrationExpiresAt {
    Now,
    Timestamp(stripe_types::Timestamp),
}
impl<'a> UpdateTaxProductRegistrationsResourceTaxRegistration<'a> {
    /// Updates an existing Tax `Registration` object.
    ///
    /// A registration cannot be deleted after it has been created.
    ///
    /// If you wish to end a registration you may do so by setting `expires_at`.
    pub fn send(
        &self,
        client: &stripe::Client,
        id: &stripe_misc::tax_product_registrations_resource_tax_registration::TaxRegistrationId,
    ) -> stripe::Response<stripe_misc::TaxProductRegistrationsResourceTaxRegistration> {
        client.send_form(&format!("/tax/registrations/{id}"), self, http_types::Method::Post)
    }
}
