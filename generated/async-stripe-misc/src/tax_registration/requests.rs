use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListTaxRegistrationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<ListTaxRegistrationStatus>,
}
impl ListTaxRegistrationBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None, status: None }
    }
}
/// The status of the Tax Registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListTaxRegistrationStatus {
    Active,
    All,
    Expired,
    Scheduled,
}
impl ListTaxRegistrationStatus {
    pub fn as_str(self) -> &'static str {
        use ListTaxRegistrationStatus::*;
        match self {
            Active => "active",
            All => "all",
            Expired => "expired",
            Scheduled => "scheduled",
        }
    }
}

impl std::str::FromStr for ListTaxRegistrationStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTaxRegistrationStatus::*;
        match s {
            "active" => Ok(Active),
            "all" => Ok(All),
            "expired" => Ok(Expired),
            "scheduled" => Ok(Scheduled),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ListTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListTaxRegistrationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ListTaxRegistrationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ListTaxRegistrationStatus"))
    }
}
/// Returns a list of Tax `Registration` objects.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListTaxRegistration {
    inner: ListTaxRegistrationBuilder,
}
impl ListTaxRegistration {
    /// Construct a new `ListTaxRegistration`.
    pub fn new() -> Self {
        Self { inner: ListTaxRegistrationBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// The status of the Tax Registration.
    pub fn status(mut self, status: impl Into<ListTaxRegistrationStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListTaxRegistration {
    fn default() -> Self {
        Self::new()
    }
}
impl ListTaxRegistration {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_misc::TaxRegistration>> {
        stripe_client_core::ListPaginator::new_list("/tax/registrations", &self.inner)
    }
}

impl StripeRequest for ListTaxRegistration {
    type Output = stripe_types::List<stripe_misc::TaxRegistration>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/tax/registrations").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveTaxRegistrationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveTaxRegistrationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a Tax `Registration` object.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveTaxRegistration {
    inner: RetrieveTaxRegistrationBuilder,
    id: stripe_misc::TaxRegistrationId,
}
impl RetrieveTaxRegistration {
    /// Construct a new `RetrieveTaxRegistration`.
    pub fn new(id: impl Into<stripe_misc::TaxRegistrationId>) -> Self {
        Self { id: id.into(), inner: RetrieveTaxRegistrationBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveTaxRegistration {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveTaxRegistration {
    type Output = stripe_misc::TaxRegistration;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/tax/registrations/{id}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateTaxRegistrationBuilder {
    active_from: CreateTaxRegistrationActiveFrom,
    country: String,
    country_options: CreateTaxRegistrationCountryOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
}
impl CreateTaxRegistrationBuilder {
    fn new(
        active_from: impl Into<CreateTaxRegistrationActiveFrom>,
        country: impl Into<String>,
        country_options: impl Into<CreateTaxRegistrationCountryOptions>,
    ) -> Self {
        Self {
            active_from: active_from.into(),
            country: country.into(),
            country_options: country_options.into(),
            expand: None,
            expires_at: None,
        }
    }
}
/// Time at which the Tax Registration becomes active.
/// It can be either `now` to indicate the current time, or a future timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CreateTaxRegistrationActiveFrom {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Specific options for a registration in the specified `country`.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptions {
    /// Options for the registration in AE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae: Option<CreateTaxRegistrationCountryOptionsAe>,
    /// Options for the registration in AT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<CreateTaxRegistrationCountryOptionsAt>,
    /// Options for the registration in AU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au: Option<CreateTaxRegistrationCountryOptionsAu>,
    /// Options for the registration in BE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub be: Option<CreateTaxRegistrationCountryOptionsBe>,
    /// Options for the registration in BG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<CreateTaxRegistrationCountryOptionsBg>,
    /// Options for the registration in CA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<CreateTaxRegistrationCountryOptionsCa>,
    /// Options for the registration in CH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<CreateTaxRegistrationCountryOptionsCh>,
    /// Options for the registration in CL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl: Option<CreateTaxRegistrationCountryOptionsCl>,
    /// Options for the registration in CO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co: Option<CreateTaxRegistrationCountryOptionsCo>,
    /// Options for the registration in CY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<CreateTaxRegistrationCountryOptionsCy>,
    /// Options for the registration in CZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cz: Option<CreateTaxRegistrationCountryOptionsCz>,
    /// Options for the registration in DE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de: Option<CreateTaxRegistrationCountryOptionsDe>,
    /// Options for the registration in DK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dk: Option<CreateTaxRegistrationCountryOptionsDk>,
    /// Options for the registration in EE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ee: Option<CreateTaxRegistrationCountryOptionsEe>,
    /// Options for the registration in ES.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<CreateTaxRegistrationCountryOptionsEs>,
    /// Options for the registration in FI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fi: Option<CreateTaxRegistrationCountryOptionsFi>,
    /// Options for the registration in FR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<CreateTaxRegistrationCountryOptionsFr>,
    /// Options for the registration in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb: Option<CreateTaxRegistrationCountryOptionsGb>,
    /// Options for the registration in GR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gr: Option<CreateTaxRegistrationCountryOptionsGr>,
    /// Options for the registration in HR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hr: Option<CreateTaxRegistrationCountryOptionsHr>,
    /// Options for the registration in HU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hu: Option<CreateTaxRegistrationCountryOptionsHu>,
    /// Options for the registration in ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<CreateTaxRegistrationCountryOptionsId>,
    /// Options for the registration in IE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ie: Option<CreateTaxRegistrationCountryOptionsIe>,
    /// Options for the registration in IS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is: Option<CreateTaxRegistrationCountryOptionsIs>,
    /// Options for the registration in IT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it: Option<CreateTaxRegistrationCountryOptionsIt>,
    /// Options for the registration in JP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp: Option<CreateTaxRegistrationCountryOptionsJp>,
    /// Options for the registration in KR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<CreateTaxRegistrationCountryOptionsKr>,
    /// Options for the registration in LT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<CreateTaxRegistrationCountryOptionsLt>,
    /// Options for the registration in LU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lu: Option<CreateTaxRegistrationCountryOptionsLu>,
    /// Options for the registration in LV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<CreateTaxRegistrationCountryOptionsLv>,
    /// Options for the registration in MT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mt: Option<CreateTaxRegistrationCountryOptionsMt>,
    /// Options for the registration in MX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx: Option<CreateTaxRegistrationCountryOptionsMx>,
    /// Options for the registration in MY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my: Option<CreateTaxRegistrationCountryOptionsMy>,
    /// Options for the registration in NL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nl: Option<CreateTaxRegistrationCountryOptionsNl>,
    /// Options for the registration in NO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no: Option<CreateTaxRegistrationCountryOptionsNo>,
    /// Options for the registration in NZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz: Option<CreateTaxRegistrationCountryOptionsNz>,
    /// Options for the registration in PL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pl: Option<CreateTaxRegistrationCountryOptionsPl>,
    /// Options for the registration in PT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<CreateTaxRegistrationCountryOptionsPt>,
    /// Options for the registration in RO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<CreateTaxRegistrationCountryOptionsRo>,
    /// Options for the registration in SA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa: Option<CreateTaxRegistrationCountryOptionsSa>,
    /// Options for the registration in SE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se: Option<CreateTaxRegistrationCountryOptionsSe>,
    /// Options for the registration in SG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sg: Option<CreateTaxRegistrationCountryOptionsSg>,
    /// Options for the registration in SI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<CreateTaxRegistrationCountryOptionsSi>,
    /// Options for the registration in SK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sk: Option<CreateTaxRegistrationCountryOptionsSk>,
    /// Options for the registration in TH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th: Option<CreateTaxRegistrationCountryOptionsTh>,
    /// Options for the registration in TR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<CreateTaxRegistrationCountryOptionsTr>,
    /// Options for the registration in US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us: Option<CreateTaxRegistrationCountryOptionsUs>,
    /// Options for the registration in VN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vn: Option<CreateTaxRegistrationCountryOptionsVn>,
    /// Options for the registration in ZA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub za: Option<CreateTaxRegistrationCountryOptionsZa>,
}
impl CreateTaxRegistrationCountryOptions {
    pub fn new() -> Self {
        Self {
            ae: None,
            at: None,
            au: None,
            be: None,
            bg: None,
            ca: None,
            ch: None,
            cl: None,
            co: None,
            cy: None,
            cz: None,
            de: None,
            dk: None,
            ee: None,
            es: None,
            fi: None,
            fr: None,
            gb: None,
            gr: None,
            hr: None,
            hu: None,
            id: None,
            ie: None,
            is: None,
            it: None,
            jp: None,
            kr: None,
            lt: None,
            lu: None,
            lv: None,
            mt: None,
            mx: None,
            my: None,
            nl: None,
            no: None,
            nz: None,
            pl: None,
            pt: None,
            ro: None,
            sa: None,
            se: None,
            sg: None,
            si: None,
            sk: None,
            th: None,
            tr: None,
            us: None,
            vn: None,
            za: None,
        }
    }
}
impl Default for CreateTaxRegistrationCountryOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Options for the registration in AE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsAe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAeType,
}
impl CreateTaxRegistrationCountryOptionsAe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAeType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAeType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAeType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAeType")
        })
    }
}
/// Options for the registration in AT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsAt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAtType,
}
impl CreateTaxRegistrationCountryOptionsAt {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsAtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsAtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAtType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAtType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAtType")
        })
    }
}
/// Options for the registration in AU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsAu {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAuType,
}
impl CreateTaxRegistrationCountryOptionsAu {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAuType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAuType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAuType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAuType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAuType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAuType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAuType")
        })
    }
}
/// Options for the registration in BE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsBe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBeType,
}
impl CreateTaxRegistrationCountryOptionsBe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsBeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsBeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBeType")
        })
    }
}
/// Options for the registration in BG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsBg {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBgStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBgType,
}
impl CreateTaxRegistrationCountryOptionsBg {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBgType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsBgStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsBgStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBgType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBgType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBgType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBgType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBgType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBgType")
        })
    }
}
/// Options for the registration in CA.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCa {
    /// Options for the provincial tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_standard: Option<CreateTaxRegistrationCountryOptionsCaProvinceStandard>,
    /// Type of registration to be created in Canada.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCaType,
}
impl CreateTaxRegistrationCountryOptionsCa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCaType>) -> Self {
        Self { province_standard: None, type_: type_.into() }
    }
}
/// Options for the provincial tax registration.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCaProvinceStandard {
    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: String,
}
impl CreateTaxRegistrationCountryOptionsCaProvinceStandard {
    pub fn new(province: impl Into<String>) -> Self {
        Self { province: province.into() }
    }
}
/// Type of registration to be created in Canada.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCaType {
    ProvinceStandard,
    Simplified,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCaType::*;
        match self {
            ProvinceStandard => "province_standard",
            Simplified => "simplified",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCaType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCaType::*;
        match s {
            "province_standard" => Ok(ProvinceStandard),
            "simplified" => Ok(Simplified),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCaType")
        })
    }
}
/// Options for the registration in CH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsChType,
}
impl CreateTaxRegistrationCountryOptionsCh {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsChType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsChType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsChType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsChType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsChType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsChType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsChType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsChType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsChType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsChType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsChType")
        })
    }
}
/// Options for the registration in CL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCl {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsClType,
}
impl CreateTaxRegistrationCountryOptionsCl {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsClType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsClType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsClType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsClType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsClType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsClType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsClType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsClType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsClType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsClType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsClType")
        })
    }
}
/// Options for the registration in CO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCo {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCoType,
}
impl CreateTaxRegistrationCountryOptionsCo {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCoType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCoType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsCoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCoType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCoType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCoType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCoType")
        })
    }
}
/// Options for the registration in CY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCy {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsCyStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCyType,
}
impl CreateTaxRegistrationCountryOptionsCy {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCyType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCyStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsCyStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCyType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCyType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCyType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCyType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCyType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCyType")
        })
    }
}
/// Options for the registration in CZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCz {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsCzStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCzType,
}
impl CreateTaxRegistrationCountryOptionsCz {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCzType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsCzStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsCzStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCzType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCzType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCzType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCzType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCzType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCzType")
        })
    }
}
/// Options for the registration in DE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsDe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsDeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsDeType,
}
impl CreateTaxRegistrationCountryOptionsDe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsDeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsDeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsDeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsDeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsDeType")
        })
    }
}
/// Options for the registration in DK.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsDk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsDkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsDkType,
}
impl CreateTaxRegistrationCountryOptionsDk {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsDkType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsDkStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsDkStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsDkType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDkType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDkType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsDkType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsDkType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsDkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsDkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsDkType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsDkType")
        })
    }
}
/// Options for the registration in EE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsEe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsEeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsEeType,
}
impl CreateTaxRegistrationCountryOptionsEe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsEeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsEeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsEeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsEeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEeType")
        })
    }
}
/// Options for the registration in ES.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsEs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsEsStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsEsType,
}
impl CreateTaxRegistrationCountryOptionsEs {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsEsType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsEsStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsEsStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEsType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEsType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEsType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsEsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEsType")
        })
    }
}
/// Options for the registration in FI.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsFi {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsFiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsFiType,
}
impl CreateTaxRegistrationCountryOptionsFi {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsFiType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsFiStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsFiStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFiType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFiType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFiType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFiType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFiType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsFiType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsFiType")
        })
    }
}
/// Options for the registration in FR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsFr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsFrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsFrType,
}
impl CreateTaxRegistrationCountryOptionsFr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsFrType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsFrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsFrStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsFrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsFrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsFrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsFrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsFrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsFrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsFrType")
        })
    }
}
/// Options for the registration in GB.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsGb {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsGbType,
}
impl CreateTaxRegistrationCountryOptionsGb {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsGbType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGbType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGbType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGbType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGbType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGbType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGbType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsGbType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsGbType")
        })
    }
}
/// Options for the registration in GR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsGr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsGrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsGrType,
}
impl CreateTaxRegistrationCountryOptionsGr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsGrType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsGrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsGrStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsGrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsGrType")
        })
    }
}
/// Options for the registration in HR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsHr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsHrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsHrType,
}
impl CreateTaxRegistrationCountryOptionsHr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsHrType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsHrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsHrStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHrType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHrType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHrType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsHrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsHrType")
        })
    }
}
/// Options for the registration in HU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsHu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsHuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsHuType,
}
impl CreateTaxRegistrationCountryOptionsHu {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsHuType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsHuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsHuStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsHuType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHuType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsHuType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsHuType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsHuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsHuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsHuType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsHuType")
        })
    }
}
/// Options for the registration in ID.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsId {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsIdType,
}
impl CreateTaxRegistrationCountryOptionsId {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsIdType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIdType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsIdType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIdType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIdType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIdType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsIdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsIdType")
        })
    }
}
/// Options for the registration in IE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsIe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsIeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsIeType,
}
impl CreateTaxRegistrationCountryOptionsIe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsIeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsIeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsIeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsIeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsIeType")
        })
    }
}
/// Options for the registration in IS.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsIs {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsIsType,
}
impl CreateTaxRegistrationCountryOptionsIs {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsIsType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIsType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIsType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIsType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsIsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsIsType")
        })
    }
}
/// Options for the registration in IT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsIt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsItStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsItType,
}
impl CreateTaxRegistrationCountryOptionsIt {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsItType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsItStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsItStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsItType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsItType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsItType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsItType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsItType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsItType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsItType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsItType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsItType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsItType")
        })
    }
}
/// Options for the registration in JP.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsJp {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsJpType,
}
impl CreateTaxRegistrationCountryOptionsJp {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsJpType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsJpType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsJpType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsJpType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsJpType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsJpType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsJpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsJpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsJpType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsJpType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsJpType")
        })
    }
}
/// Options for the registration in KR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsKr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsKrType,
}
impl CreateTaxRegistrationCountryOptionsKr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsKrType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsKrType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsKrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsKrType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsKrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsKrType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsKrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsKrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsKrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsKrType")
        })
    }
}
/// Options for the registration in LT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsLt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsLtType,
}
impl CreateTaxRegistrationCountryOptionsLt {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsLtType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsLtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsLtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLtType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsLtType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsLtType")
        })
    }
}
/// Options for the registration in LU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsLu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsLuType,
}
impl CreateTaxRegistrationCountryOptionsLu {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsLuType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsLuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsLuStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLuType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLuType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLuType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLuType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsLuType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsLuType")
        })
    }
}
/// Options for the registration in LV.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsLv {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLvStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsLvType,
}
impl CreateTaxRegistrationCountryOptionsLv {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsLvType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsLvStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsLvStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLvType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLvType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLvType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLvType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLvType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLvType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLvType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLvType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsLvType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsLvType")
        })
    }
}
/// Options for the registration in MT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsMt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsMtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMtType,
}
impl CreateTaxRegistrationCountryOptionsMt {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMtType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsMtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsMtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMtType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMtType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMtType")
        })
    }
}
/// Options for the registration in MX.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsMx {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMxType,
}
impl CreateTaxRegistrationCountryOptionsMx {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMxType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMxType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsMxType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMxType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMxType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMxType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMxType")
        })
    }
}
/// Options for the registration in MY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsMy {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMyType,
}
impl CreateTaxRegistrationCountryOptionsMy {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMyType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMyType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsMyType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMyType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMyType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMyType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMyType")
        })
    }
}
/// Options for the registration in NL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsNl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsNlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNlType,
}
impl CreateTaxRegistrationCountryOptionsNl {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsNlType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsNlStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsNlStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNlType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNlType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNlType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNlType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNlType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsNlType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNlType")
        })
    }
}
/// Options for the registration in NO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsNo {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNoType,
}
impl CreateTaxRegistrationCountryOptionsNo {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsNoType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNoType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNoType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNoType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNoType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsNoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNoType")
        })
    }
}
/// Options for the registration in NZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsNz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNzType,
}
impl CreateTaxRegistrationCountryOptionsNz {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsNzType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNzType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNzType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNzType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNzType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsNzType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNzType")
        })
    }
}
/// Options for the registration in PL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsPl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsPlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsPlType,
}
impl CreateTaxRegistrationCountryOptionsPl {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsPlType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsPlStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsPlStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPlType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPlType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPlType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPlType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPlType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsPlType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsPlType")
        })
    }
}
/// Options for the registration in PT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsPt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsPtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsPtType,
}
impl CreateTaxRegistrationCountryOptionsPt {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsPtType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsPtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsPtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPtType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPtType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPtType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsPtType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsPtType")
        })
    }
}
/// Options for the registration in RO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsRo {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsRoStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsRoType,
}
impl CreateTaxRegistrationCountryOptionsRo {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsRoType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsRoStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsRoStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsRoType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsRoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRoType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsRoType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsRoType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsRoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsRoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsRoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsRoType")
        })
    }
}
/// Options for the registration in SA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSaType,
}
impl CreateTaxRegistrationCountryOptionsSa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSaType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSaType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsSaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSaType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSaType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSaType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsSaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSaType")
        })
    }
}
/// Options for the registration in SE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSeType,
}
impl CreateTaxRegistrationCountryOptionsSe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsSeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSeType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSeType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsSeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSeType")
        })
    }
}
/// Options for the registration in SG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSgType,
}
impl CreateTaxRegistrationCountryOptionsSg {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSgType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSgType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSgType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSgType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSgType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsSgType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSgType")
        })
    }
}
/// Options for the registration in SI.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSi {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSiType,
}
impl CreateTaxRegistrationCountryOptionsSi {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSiType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSiStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsSiStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSiType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSiType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSiType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSiType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSiType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSiType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSiType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsSiType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSiType")
        })
    }
}
/// Options for the registration in SK.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSkType,
}
impl CreateTaxRegistrationCountryOptionsSk {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSkType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsSkStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme: CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme,
}
impl CreateTaxRegistrationCountryOptionsSkStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an EU standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme::*;
        match self {
            SmallSeller => "small_seller",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme::*;
        match s {
            "small_seller" => Ok(SmallSeller),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in an EU country.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSkType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSkType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSkType::*;
        match self {
            Ioss => "ioss",
            OssNonUnion => "oss_non_union",
            OssUnion => "oss_union",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSkType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSkType::*;
        match s {
            "ioss" => Ok(Ioss),
            "oss_non_union" => Ok(OssNonUnion),
            "oss_union" => Ok(OssUnion),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsSkType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSkType")
        })
    }
}
/// Options for the registration in TH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsTh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsThType,
}
impl CreateTaxRegistrationCountryOptionsTh {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsThType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsThType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsThType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsThType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsThType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsThType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsThType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsThType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsThType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsThType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsThType")
        })
    }
}
/// Options for the registration in TR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsTr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsTrType,
}
impl CreateTaxRegistrationCountryOptionsTr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsTrType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsTrType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsTrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsTrType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsTrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsTrType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsTrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsTrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsTrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsTrType")
        })
    }
}
/// Options for the registration in US.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsUs {
    /// Options for the local amusement tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalAmusementTax>,
    /// Options for the local lease tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalLeaseTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    /// Type of registration to be created in the US.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsUsType,
}
impl CreateTaxRegistrationCountryOptionsUs {
    pub fn new(
        state: impl Into<String>,
        type_: impl Into<CreateTaxRegistrationCountryOptionsUsType>,
    ) -> Self {
        Self {
            local_amusement_tax: None,
            local_lease_tax: None,
            state: state.into(),
            type_: type_.into(),
        }
    }
}
/// Options for the local amusement tax registration.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsUsLocalAmusementTax {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    /// Supported FIPS codes are: `14000` (Chicago), `06613` (Bloomington), `21696` (East Dundee), `24582` (Evanston), and `68081` (Schiller Park).
    pub jurisdiction: String,
}
impl CreateTaxRegistrationCountryOptionsUsLocalAmusementTax {
    pub fn new(jurisdiction: impl Into<String>) -> Self {
        Self { jurisdiction: jurisdiction.into() }
    }
}
/// Options for the local lease tax registration.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsUsLocalLeaseTax {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    /// Supported FIPS codes are: `14000` (Chicago).
    pub jurisdiction: String,
}
impl CreateTaxRegistrationCountryOptionsUsLocalLeaseTax {
    pub fn new(jurisdiction: impl Into<String>) -> Self {
        Self { jurisdiction: jurisdiction.into() }
    }
}
/// Type of registration to be created in the US.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUsType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateSalesTax,
}
impl CreateTaxRegistrationCountryOptionsUsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUsType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateSalesTax => "state_sales_tax",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUsType::*;
        match s {
            "local_amusement_tax" => Ok(LocalAmusementTax),
            "local_lease_tax" => Ok(LocalLeaseTax),
            "state_communications_tax" => Ok(StateCommunicationsTax),
            "state_sales_tax" => Ok(StateSalesTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsUsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsUsType")
        })
    }
}
/// Options for the registration in VN.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsVn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsVnType,
}
impl CreateTaxRegistrationCountryOptionsVn {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsVnType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsVnType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsVnType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsVnType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsVnType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsVnType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsVnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsVnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsVnType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsVnType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsVnType")
        })
    }
}
/// Options for the registration in ZA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistrationCountryOptionsZa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsZaType,
}
impl CreateTaxRegistrationCountryOptionsZa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsZaType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsZaType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsZaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsZaType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsZaType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsZaType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsZaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsZaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsZaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsZaType")
        })
    }
}
/// Creates a new Tax `Registration` object.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateTaxRegistration {
    inner: CreateTaxRegistrationBuilder,
}
impl CreateTaxRegistration {
    /// Construct a new `CreateTaxRegistration`.
    pub fn new(
        active_from: impl Into<CreateTaxRegistrationActiveFrom>,
        country: impl Into<String>,
        country_options: impl Into<CreateTaxRegistrationCountryOptions>,
    ) -> Self {
        Self {
            inner: CreateTaxRegistrationBuilder::new(
                active_from.into(),
                country.into(),
                country_options.into(),
            ),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// If set, the Tax Registration stops being active at this time.
    /// If not set, the Tax Registration will be active indefinitely.
    /// Timestamp measured in seconds since the Unix epoch.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
}
impl CreateTaxRegistration {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateTaxRegistration {
    type Output = stripe_misc::TaxRegistration;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/tax/registrations").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdateTaxRegistrationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_from: Option<UpdateTaxRegistrationActiveFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<UpdateTaxRegistrationExpiresAt>,
}
impl UpdateTaxRegistrationBuilder {
    fn new() -> Self {
        Self { active_from: None, expand: None, expires_at: None }
    }
}
/// Time at which the registration becomes active.
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum UpdateTaxRegistrationActiveFrom {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// If set, the registration stops being active at this time.
/// If not set, the registration will be active indefinitely.
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum UpdateTaxRegistrationExpiresAt {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Updates an existing Tax `Registration` object.
///
/// A registration cannot be deleted after it has been created.
/// If you wish to end a registration you may do so by setting `expires_at`.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateTaxRegistration {
    inner: UpdateTaxRegistrationBuilder,
    id: stripe_misc::TaxRegistrationId,
}
impl UpdateTaxRegistration {
    /// Construct a new `UpdateTaxRegistration`.
    pub fn new(id: impl Into<stripe_misc::TaxRegistrationId>) -> Self {
        Self { id: id.into(), inner: UpdateTaxRegistrationBuilder::new() }
    }
    /// Time at which the registration becomes active.
    /// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
    pub fn active_from(mut self, active_from: impl Into<UpdateTaxRegistrationActiveFrom>) -> Self {
        self.inner.active_from = Some(active_from.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// If set, the registration stops being active at this time.
    /// If not set, the registration will be active indefinitely.
    /// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
    pub fn expires_at(mut self, expires_at: impl Into<UpdateTaxRegistrationExpiresAt>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
}
impl UpdateTaxRegistration {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateTaxRegistration {
    type Output = stripe_misc::TaxRegistration;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/tax/registrations/{id}"))
            .form(&self.inner)
    }
}
