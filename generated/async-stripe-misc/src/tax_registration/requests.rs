use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTaxRegistrationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListTaxRegistrationBuilder").finish_non_exhaustive()
    }
}
impl ListTaxRegistrationBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None, status: None }
    }
}
/// The status of the Tax Registration.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ListTaxRegistrationStatus {
    Active,
    All,
    Expired,
    Scheduled,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ListTaxRegistrationStatus {
    pub fn as_str(&self) -> &str {
        use ListTaxRegistrationStatus::*;
        match self {
            Active => "active",
            All => "all",
            Expired => "expired",
            Scheduled => "scheduled",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ListTaxRegistrationStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListTaxRegistrationStatus::*;
        match s {
            "active" => Ok(Active),
            "all" => Ok(All),
            "expired" => Ok(Expired),
            "scheduled" => Ok(Scheduled),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ListTaxRegistrationStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ListTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ListTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTaxRegistrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ListTaxRegistrationStatus)).finish_non_exhaustive()
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Returns a list of Tax `Registration` objects.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListTaxRegistration {
    inner: ListTaxRegistrationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListTaxRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListTaxRegistration").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveTaxRegistrationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveTaxRegistrationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveTaxRegistrationBuilder").finish_non_exhaustive()
    }
}
impl RetrieveTaxRegistrationBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a Tax `Registration` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveTaxRegistration {
    inner: RetrieveTaxRegistrationBuilder,
    id: stripe_misc::TaxRegistrationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveTaxRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveTaxRegistration").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateTaxRegistrationBuilder {
    active_from: CreateTaxRegistrationActiveFrom,
    country: String,
    country_options: CreateTaxRegistrationCountryOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationBuilder").finish_non_exhaustive()
    }
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
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateTaxRegistrationActiveFrom {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationActiveFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationActiveFrom").finish_non_exhaustive()
    }
}
/// Specific options for a registration in the specified `country`.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptions {
    /// Options for the registration in AE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae: Option<CreateTaxRegistrationCountryOptionsAe>,
    /// Options for the registration in AL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al: Option<CreateTaxRegistrationCountryOptionsAl>,
    /// Options for the registration in AM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub am: Option<CreateTaxRegistrationCountryOptionsAm>,
    /// Options for the registration in AO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ao: Option<CreateTaxRegistrationCountryOptionsAo>,
    /// Options for the registration in AT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<CreateTaxRegistrationCountryOptionsAt>,
    /// Options for the registration in AU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au: Option<CreateTaxRegistrationCountryOptionsAu>,
    /// Options for the registration in AW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aw: Option<CreateTaxRegistrationCountryOptionsAw>,
    /// Options for the registration in AZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub az: Option<CreateTaxRegistrationCountryOptionsAz>,
    /// Options for the registration in BA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ba: Option<CreateTaxRegistrationCountryOptionsBa>,
    /// Options for the registration in BB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bb: Option<CreateTaxRegistrationCountryOptionsBb>,
    /// Options for the registration in BD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bd: Option<CreateTaxRegistrationCountryOptionsBd>,
    /// Options for the registration in BE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub be: Option<CreateTaxRegistrationCountryOptionsBe>,
    /// Options for the registration in BF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bf: Option<CreateTaxRegistrationCountryOptionsBf>,
    /// Options for the registration in BG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<CreateTaxRegistrationCountryOptionsBg>,
    /// Options for the registration in BH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bh: Option<CreateTaxRegistrationCountryOptionsBh>,
    /// Options for the registration in BJ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bj: Option<CreateTaxRegistrationCountryOptionsBj>,
    /// Options for the registration in BS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bs: Option<CreateTaxRegistrationCountryOptionsBs>,
    /// Options for the registration in BY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<CreateTaxRegistrationCountryOptionsBy>,
    /// Options for the registration in CA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<CreateTaxRegistrationCountryOptionsCa>,
    /// Options for the registration in CD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<CreateTaxRegistrationCountryOptionsCd>,
    /// Options for the registration in CH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<CreateTaxRegistrationCountryOptionsCh>,
    /// Options for the registration in CL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl: Option<CreateTaxRegistrationCountryOptionsCl>,
    /// Options for the registration in CM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cm: Option<CreateTaxRegistrationCountryOptionsCm>,
    /// Options for the registration in CO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co: Option<CreateTaxRegistrationCountryOptionsCo>,
    /// Options for the registration in CR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cr: Option<CreateTaxRegistrationCountryOptionsCr>,
    /// Options for the registration in CV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cv: Option<CreateTaxRegistrationCountryOptionsCv>,
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
    /// Options for the registration in EC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec: Option<CreateTaxRegistrationCountryOptionsEc>,
    /// Options for the registration in EE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ee: Option<CreateTaxRegistrationCountryOptionsEe>,
    /// Options for the registration in EG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eg: Option<CreateTaxRegistrationCountryOptionsEg>,
    /// Options for the registration in ES.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<CreateTaxRegistrationCountryOptionsEs>,
    /// Options for the registration in ET.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub et: Option<CreateTaxRegistrationCountryOptionsEt>,
    /// Options for the registration in FI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fi: Option<CreateTaxRegistrationCountryOptionsFi>,
    /// Options for the registration in FR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<CreateTaxRegistrationCountryOptionsFr>,
    /// Options for the registration in GB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb: Option<CreateTaxRegistrationCountryOptionsGb>,
    /// Options for the registration in GE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ge: Option<CreateTaxRegistrationCountryOptionsGe>,
    /// Options for the registration in GN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gn: Option<CreateTaxRegistrationCountryOptionsGn>,
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
    /// Options for the registration in IN.
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_: Option<CreateTaxRegistrationCountryOptionsIn>,
    /// Options for the registration in IS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is: Option<CreateTaxRegistrationCountryOptionsIs>,
    /// Options for the registration in IT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it: Option<CreateTaxRegistrationCountryOptionsIt>,
    /// Options for the registration in JP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp: Option<CreateTaxRegistrationCountryOptionsJp>,
    /// Options for the registration in KE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ke: Option<CreateTaxRegistrationCountryOptionsKe>,
    /// Options for the registration in KG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kg: Option<CreateTaxRegistrationCountryOptionsKg>,
    /// Options for the registration in KH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kh: Option<CreateTaxRegistrationCountryOptionsKh>,
    /// Options for the registration in KR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<CreateTaxRegistrationCountryOptionsKr>,
    /// Options for the registration in KZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kz: Option<CreateTaxRegistrationCountryOptionsKz>,
    /// Options for the registration in LA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub la: Option<CreateTaxRegistrationCountryOptionsLa>,
    /// Options for the registration in LK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lk: Option<CreateTaxRegistrationCountryOptionsLk>,
    /// Options for the registration in LT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<CreateTaxRegistrationCountryOptionsLt>,
    /// Options for the registration in LU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lu: Option<CreateTaxRegistrationCountryOptionsLu>,
    /// Options for the registration in LV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<CreateTaxRegistrationCountryOptionsLv>,
    /// Options for the registration in MA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ma: Option<CreateTaxRegistrationCountryOptionsMa>,
    /// Options for the registration in MD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md: Option<CreateTaxRegistrationCountryOptionsMd>,
    /// Options for the registration in ME.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub me: Option<CreateTaxRegistrationCountryOptionsMe>,
    /// Options for the registration in MK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mk: Option<CreateTaxRegistrationCountryOptionsMk>,
    /// Options for the registration in MR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mr: Option<CreateTaxRegistrationCountryOptionsMr>,
    /// Options for the registration in MT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mt: Option<CreateTaxRegistrationCountryOptionsMt>,
    /// Options for the registration in MX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx: Option<CreateTaxRegistrationCountryOptionsMx>,
    /// Options for the registration in MY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my: Option<CreateTaxRegistrationCountryOptionsMy>,
    /// Options for the registration in NG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ng: Option<CreateTaxRegistrationCountryOptionsNg>,
    /// Options for the registration in NL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nl: Option<CreateTaxRegistrationCountryOptionsNl>,
    /// Options for the registration in NO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no: Option<CreateTaxRegistrationCountryOptionsNo>,
    /// Options for the registration in NP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub np: Option<CreateTaxRegistrationCountryOptionsNp>,
    /// Options for the registration in NZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz: Option<CreateTaxRegistrationCountryOptionsNz>,
    /// Options for the registration in OM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub om: Option<CreateTaxRegistrationCountryOptionsOm>,
    /// Options for the registration in PE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pe: Option<CreateTaxRegistrationCountryOptionsPe>,
    /// Options for the registration in PH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph: Option<CreateTaxRegistrationCountryOptionsPh>,
    /// Options for the registration in PL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pl: Option<CreateTaxRegistrationCountryOptionsPl>,
    /// Options for the registration in PT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<CreateTaxRegistrationCountryOptionsPt>,
    /// Options for the registration in RO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<CreateTaxRegistrationCountryOptionsRo>,
    /// Options for the registration in RS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rs: Option<CreateTaxRegistrationCountryOptionsRs>,
    /// Options for the registration in RU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru: Option<CreateTaxRegistrationCountryOptionsRu>,
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
    /// Options for the registration in SN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn: Option<CreateTaxRegistrationCountryOptionsSn>,
    /// Options for the registration in SR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sr: Option<CreateTaxRegistrationCountryOptionsSr>,
    /// Options for the registration in TH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th: Option<CreateTaxRegistrationCountryOptionsTh>,
    /// Options for the registration in TJ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tj: Option<CreateTaxRegistrationCountryOptionsTj>,
    /// Options for the registration in TR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<CreateTaxRegistrationCountryOptionsTr>,
    /// Options for the registration in TW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tw: Option<CreateTaxRegistrationCountryOptionsTw>,
    /// Options for the registration in TZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<CreateTaxRegistrationCountryOptionsTz>,
    /// Options for the registration in UA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ua: Option<CreateTaxRegistrationCountryOptionsUa>,
    /// Options for the registration in UG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ug: Option<CreateTaxRegistrationCountryOptionsUg>,
    /// Options for the registration in US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us: Option<CreateTaxRegistrationCountryOptionsUs>,
    /// Options for the registration in UY.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uy: Option<CreateTaxRegistrationCountryOptionsUy>,
    /// Options for the registration in UZ.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uz: Option<CreateTaxRegistrationCountryOptionsUz>,
    /// Options for the registration in VN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vn: Option<CreateTaxRegistrationCountryOptionsVn>,
    /// Options for the registration in ZA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub za: Option<CreateTaxRegistrationCountryOptionsZa>,
    /// Options for the registration in ZM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zm: Option<CreateTaxRegistrationCountryOptionsZm>,
    /// Options for the registration in ZW.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zw: Option<CreateTaxRegistrationCountryOptionsZw>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptions").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptions {
    pub fn new() -> Self {
        Self {
            ae: None,
            al: None,
            am: None,
            ao: None,
            at: None,
            au: None,
            aw: None,
            az: None,
            ba: None,
            bb: None,
            bd: None,
            be: None,
            bf: None,
            bg: None,
            bh: None,
            bj: None,
            bs: None,
            by: None,
            ca: None,
            cd: None,
            ch: None,
            cl: None,
            cm: None,
            co: None,
            cr: None,
            cv: None,
            cy: None,
            cz: None,
            de: None,
            dk: None,
            ec: None,
            ee: None,
            eg: None,
            es: None,
            et: None,
            fi: None,
            fr: None,
            gb: None,
            ge: None,
            gn: None,
            gr: None,
            hr: None,
            hu: None,
            id: None,
            ie: None,
            in_: None,
            is: None,
            it: None,
            jp: None,
            ke: None,
            kg: None,
            kh: None,
            kr: None,
            kz: None,
            la: None,
            lk: None,
            lt: None,
            lu: None,
            lv: None,
            ma: None,
            md: None,
            me: None,
            mk: None,
            mr: None,
            mt: None,
            mx: None,
            my: None,
            ng: None,
            nl: None,
            no: None,
            np: None,
            nz: None,
            om: None,
            pe: None,
            ph: None,
            pl: None,
            pt: None,
            ro: None,
            rs: None,
            ru: None,
            sa: None,
            se: None,
            sg: None,
            si: None,
            sk: None,
            sn: None,
            sr: None,
            th: None,
            tj: None,
            tr: None,
            tw: None,
            tz: None,
            ua: None,
            ug: None,
            us: None,
            uy: None,
            uz: None,
            vn: None,
            za: None,
            zm: None,
            zw: None,
        }
    }
}
impl Default for CreateTaxRegistrationCountryOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Options for the registration in AE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in AL.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAl").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAl {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in AM.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAm {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAm").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAm {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in AO.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAo {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAo").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAo {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in AT.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAt {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAt").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAt {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAtStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAtStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in AU.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAu").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAu {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in AW.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAw {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAw").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAw {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in AZ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsAz").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsAz {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in BA.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBa {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBa").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBa {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in BB.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBb {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBb").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBb {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in BD.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBd {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBd").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBd {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in BE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBe {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBeStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBeStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in BF.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBf {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBf").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBf {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in BG.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBg {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBgStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBg").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBg {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBgStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBgStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBgStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBgStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in BH.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBh {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBh").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBh {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in BJ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBj {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBj").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBj {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in BS.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBs").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBs {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in BY.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBy {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsBy").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsBy {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in CA.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCa {
    /// Options for the provincial tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_standard: Option<CreateTaxRegistrationCountryOptionsCaProvinceStandard>,
    /// Type of registration to be created in Canada.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsCaType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCa").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCa {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsCaType>) -> Self {
        Self { province_standard: None, type_: type_.into() }
    }
}
/// Options for the provincial tax registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCaProvinceStandard {
    /// Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub province: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCaProvinceStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCaProvinceStandard")
            .finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCaProvinceStandard {
    pub fn new(province: impl Into<String>) -> Self {
        Self { province: province.into() }
    }
}
/// Options for the registration in CD.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCd {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCd").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCd {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in CH.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCh {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCh").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCh {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in CL.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCl {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCl").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCl {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in CM.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCm {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCm").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCm {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in CO.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCo {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCo").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCo {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in CR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in CV.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCv {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCv").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCv {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in CY.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCy {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsCyStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCy").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCy {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCyStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCyStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCyStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCyStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in CZ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCz {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsCzStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCz").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCz {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCzStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCzStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsCzStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsCzStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in DE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDe {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsDeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsDe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsDe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDeStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsDeStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsDeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in DK.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDk {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsDkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsDk").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsDk {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsDkStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsDkStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsDkStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsDkStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in EC.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEc {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsEc").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsEc {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in EE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEe {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsEeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsEe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsEe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEeStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsEeStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsEeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in EG.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsEg").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsEg {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in ES.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEs {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsEsStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsEs").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsEs {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEsStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEsStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsEsStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsEsStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in ET.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsEt").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsEt {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in FI.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFi {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsFiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsFi").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsFi {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFiStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFiStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsFiStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsFiStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in FR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFr {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsFrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsFr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsFr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsFrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsFrStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsFrStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsFrStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in GB.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGb {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGb {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsGb").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsGb {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in GE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsGe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsGe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in GN.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGn {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsGn").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsGn {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in GR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGr {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsGrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsGr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsGr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGrStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsGrStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsGrStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in HR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHr {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsHrStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsHr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsHr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHrStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHrStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsHrStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsHrStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in HU.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHu {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsHuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsHu").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsHu {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsHuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsHuStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsHuStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsHuStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in ID.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsId {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsId").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsId {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in IE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIe {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsIeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsIe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsIe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIeStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsIeStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsIeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in IN.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsIn").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsIn {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in IS.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsIs").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsIs {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in IT.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIt {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsItStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsIt").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsIt {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsItStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsItStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsItStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsItStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in JP.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsJp {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsJp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsJp").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsJp {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in KE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsKe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsKe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in KG.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsKg").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsKg {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in KH.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsKh").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsKh {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in KR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsKr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsKr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in KZ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsKz").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsKz {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in LA.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLa").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLa {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in LK.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLk {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLk").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLk {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in LT.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLt {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLt").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLt {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLtStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLtStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in LU.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLu {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLuStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLu").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLu {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLuStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLuStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLuStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLuStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in LV.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLv {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsLvStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLv").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLv {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLvStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLvStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsLvStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsLvStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in MA.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMa").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMa {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in MD.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMd {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMd").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMd {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in ME.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in MK.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMk").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMk {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in MR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in MT.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMt {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsMtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMt").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMt {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMtStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMtStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in MX.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMx {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMx").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMx {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in MY.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMy {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsMy").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsMy {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in NG.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsNg").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsNg {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in NL.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNl {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsNlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsNl").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsNl {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNlStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNlStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsNlStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsNlStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in NO.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNo {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsNo").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsNo {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in NP.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNp {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsNp").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsNp {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in NZ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNz {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsNz").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsNz {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in OM.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsOm {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsOm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsOm").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsOm {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in PE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsPe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsPe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in PH.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsPh").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsPh {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in PL.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPl {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsPlStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsPl").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsPl {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPlStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPlStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsPlStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsPlStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in PT.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPt {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsPtStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsPt").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsPt {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPtStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPtStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsPtStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsPtStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in RO.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRo {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsRoStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsRo").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsRo {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRoStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRoStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsRoStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsRoStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in RS.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsRs").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsRs {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in RU.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRu {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsRu").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsRu {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in SA.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSa").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSa {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in SE.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSe {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSeStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSe").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSe {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSeStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSeStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSeStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSeStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in SG.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSg {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSg").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSg {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in SI.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSi {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSiStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSi {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSi").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSi {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSiStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSiStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSiStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSiStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in SK.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSk {
    /// Options for the IGIC registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igic: Option<Igic>,
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSkStandard>,
    /// Type of registration to be created in an EU country.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAtType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSk").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSk {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAtType>) -> Self {
        Self { igic: None, standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSkStandard {
    /// Place of supply scheme used in an EU standard registration.
    pub place_of_supply_scheme:
        stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSkStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSkStandard").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSkStandard {
    pub fn new(
        place_of_supply_scheme: impl Into<
            stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme,
        >,
    ) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Options for the registration in SN.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSn").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSn {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in SR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsSr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsSr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in TH.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTh {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsTh").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsTh {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in TJ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTj {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsTj").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsTj {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in TR.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsTr").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsTr {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in TW.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTw {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsTw").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsTw {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in TZ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsTz").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsTz {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in UA.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUa").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUa {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in UG.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUg").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUg {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in US.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUs {
    /// Options for the local amusement tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalAmusementTax>,
    /// Options for the local lease tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalLeaseTax>,
    /// Options for the mass transit parking tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass_transit_parking_tax:
        Option<CreateTaxRegistrationCountryOptionsUsMassTransitParkingTax>,
    /// Options for the parking tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parking_tax: Option<CreateTaxRegistrationCountryOptionsUsParkingTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    /// Options for the state sales tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_sales_tax: Option<CreateTaxRegistrationCountryOptionsUsStateSalesTax>,
    /// Type of registration to be created in the US.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsUsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUs").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUs {
    pub fn new(
        state: impl Into<String>,
        type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsUsType>,
    ) -> Self {
        Self {
            local_amusement_tax: None,
            local_lease_tax: None,
            mass_transit_parking_tax: None,
            parking_tax: None,
            state: state.into(),
            state_sales_tax: None,
            type_: type_.into(),
        }
    }
}
/// Options for the local amusement tax registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsLocalAmusementTax {
    /// A jurisdiction code representing the [local jurisdiction](/tax/registering?type=amusement_tax#registration-types).
    pub jurisdiction: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsLocalAmusementTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUsLocalAmusementTax")
            .finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUsLocalAmusementTax {
    pub fn new(jurisdiction: impl Into<String>) -> Self {
        Self { jurisdiction: jurisdiction.into() }
    }
}
/// Options for the local lease tax registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsLocalLeaseTax {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    /// Supported FIPS codes are: `14000` (Chicago).
    pub jurisdiction: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsLocalLeaseTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUsLocalLeaseTax").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUsLocalLeaseTax {
    pub fn new(jurisdiction: impl Into<String>) -> Self {
        Self { jurisdiction: jurisdiction.into() }
    }
}
/// Options for the mass transit parking tax registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsMassTransitParkingTax {
    /// A jurisdiction code representing the [local jurisdiction](/tax/registering?type=mass_transit_parking_tax#registration-types).
    pub jurisdiction: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsMassTransitParkingTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUsMassTransitParkingTax")
            .finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUsMassTransitParkingTax {
    pub fn new(jurisdiction: impl Into<String>) -> Self {
        Self { jurisdiction: jurisdiction.into() }
    }
}
/// Options for the parking tax registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsParkingTax {
    /// A jurisdiction code representing the [local jurisdiction](/tax/registering?type=parking_tax#registration-types).
    pub jurisdiction: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsParkingTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUsParkingTax").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUsParkingTax {
    pub fn new(jurisdiction: impl Into<String>) -> Self {
        Self { jurisdiction: jurisdiction.into() }
    }
}
/// Options for the state sales tax registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsStateSalesTax {
    /// Elections for the state sales tax registration.
    pub elections: Vec<CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsStateSalesTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUsStateSalesTax").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUsStateSalesTax {
    pub fn new(
        elections: impl Into<Vec<CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections>>,
    ) -> Self {
        Self { elections: elections.into() }
    }
}
/// Elections for the state sales tax registration.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    /// Supported FIPS codes are: `003` (Allegheny County) and `60000` (Philadelphia City).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    /// The type of the election for the state sales tax registration.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections")
            .finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections {
    pub fn new(
        type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType>,
    ) -> Self {
        Self { jurisdiction: None, type_: type_.into() }
    }
}
/// Options for the registration in UY.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUy {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUy").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUy {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in UZ.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUz {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsUz").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsUz {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in VN.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsVn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsVn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsVn").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsVn {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in ZA.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZa {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZa {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsZa").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsZa {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the registration in ZM.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZm {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAmType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsZm").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsZm {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Options for the registration in ZW.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZw {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<Standard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: stripe_shared::CreateTaxRegistrationCountryOptionsAeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZw {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistrationCountryOptionsZw").finish_non_exhaustive()
    }
}
impl CreateTaxRegistrationCountryOptionsZw {
    pub fn new(type_: impl Into<stripe_shared::CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Creates a new Tax `Registration` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateTaxRegistration {
    inner: CreateTaxRegistrationBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateTaxRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateTaxRegistration").finish_non_exhaustive()
    }
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateTaxRegistrationBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_from: Option<UpdateTaxRegistrationActiveFrom>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<UpdateTaxRegistrationExpiresAt>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxRegistrationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxRegistrationBuilder").finish_non_exhaustive()
    }
}
impl UpdateTaxRegistrationBuilder {
    fn new() -> Self {
        Self { active_from: None, expand: None, expires_at: None }
    }
}
/// Time at which the registration becomes active.
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateTaxRegistrationActiveFrom {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxRegistrationActiveFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxRegistrationActiveFrom").finish_non_exhaustive()
    }
}
/// If set, the registration stops being active at this time.
/// If not set, the registration will be active indefinitely.
/// It can be either `now` to indicate the current time, or a timestamp measured in seconds since the Unix epoch.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateTaxRegistrationExpiresAt {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxRegistrationExpiresAt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxRegistrationExpiresAt").finish_non_exhaustive()
    }
}
/// Updates an existing Tax `Registration` object.
///
/// A registration cannot be deleted after it has been created.
/// If you wish to end a registration you may do so by setting `expires_at`.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateTaxRegistration {
    inner: UpdateTaxRegistrationBuilder,
    id: stripe_misc::TaxRegistrationId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateTaxRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateTaxRegistration").finish_non_exhaustive()
    }
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

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct Standard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme: Option<stripe_shared::PlaceOfSupplyScheme>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Standard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Standard").finish_non_exhaustive()
    }
}
impl Standard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for Standard {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct Igic {
    /// Place of supply scheme used in an IGIC registration.
    pub place_of_supply_scheme: stripe_shared::PlaceOfSupplyScheme,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Igic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Igic").finish_non_exhaustive()
    }
}
impl Igic {
    pub fn new(place_of_supply_scheme: impl Into<stripe_shared::PlaceOfSupplyScheme>) -> Self {
        Self { place_of_supply_scheme: place_of_supply_scheme.into() }
    }
}
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsAeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsAlType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsAmType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsAoType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsAtIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsAtType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsAuType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsAwType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsAzType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsBaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsBbType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsBdType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsBeIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsBeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsBfType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsBgIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsBgType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsBhType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsBjType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsBsType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsByType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in Canada.
pub type CreateTaxRegistrationCountryOptionsCaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsCaType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsCdType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsChType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsClType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsCmType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsCoType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsCrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsCvType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsCyIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsCyType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsCzIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsCzType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsDeIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsDeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsDkIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsDkType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsEcType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsEeIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsEeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsEgType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsEsIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsEsType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsEtType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsFiIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsFiType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsFrIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsFrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsGbType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsGeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsGnType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsGrIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsGrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsHrIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsHrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsHuIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsHuType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsIdType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsIeIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsIeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsInType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsIsType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsItIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsItType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsJpType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsKeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsKgType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsKhType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsKrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsKzType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsLaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsLkType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsLtIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsLtType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsLuIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsLuType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsLvIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsLvType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsMaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsMdType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsMeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsMkType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsMrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsMtIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsMtType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsMxType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsMyType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsNgType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsNlIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsNlType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsNoType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsNpType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsNzType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsOmType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsPeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsPhType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsPlIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsPlType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsPtIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsPtType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsRoIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsRoType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsRsType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsRuType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsSaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsSeIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsSeType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsSgType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsSiIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsSiType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Place of supply scheme used in an IGIC registration.
pub type CreateTaxRegistrationCountryOptionsSkIgicPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Place of supply scheme used in an EU standard registration.
pub type CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme;
/// Type of registration to be created in an EU country.
pub type CreateTaxRegistrationCountryOptionsSkType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAtType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsSnType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsSrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsThType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsTjType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsTrType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsTwType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsTzType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsUaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsUgType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// The type of the election for the state sales tax registration.
pub type CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType =
    stripe_shared::CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType;
/// Type of registration to be created in the US.
pub type CreateTaxRegistrationCountryOptionsUsType =
    stripe_shared::CreateTaxRegistrationCountryOptionsUsType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsUyType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsUzType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsVnType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsZaType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsZmType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAmType;
/// Place of supply scheme used in an standard registration.
pub type CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme =
    stripe_shared::PlaceOfSupplyScheme;
/// Type of registration to be created in `country`.
pub type CreateTaxRegistrationCountryOptionsZwType =
    stripe_shared::CreateTaxRegistrationCountryOptionsAeType;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsAeStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsAlStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsAoStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsAtIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsAuStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsAwStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsBaStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsBbStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsBdStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsBeIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsBfStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsBgIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsBhStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsBsStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsCdStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsChStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsCyIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsCzIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsDeIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsDkIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsEeIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsEsIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsEtStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsFiIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsFrIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsGbStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsGnStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsGrIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsHrIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsHuIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsIeIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsIsStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsItIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsJpStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsLtIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsLuIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsLvIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsMeStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsMkStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsMrStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsMtIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsNlIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsNoStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsNzStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsOmStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsPlIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsPtIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsRoIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsRsStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsSeIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsSgStandard = Standard;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsSiIgic = Igic;
/// Options for the IGIC registration.
pub type CreateTaxRegistrationCountryOptionsSkIgic = Igic;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsSrStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsUyStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsZaStandard = Standard;
/// Options for the standard registration.
pub type CreateTaxRegistrationCountryOptionsZwStandard = Standard;
