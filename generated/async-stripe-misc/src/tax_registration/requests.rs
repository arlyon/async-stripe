use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
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
#[serde(rename_all = "snake_case")]
pub enum CreateTaxRegistrationActiveFrom {
    Now,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// Specific options for a registration in the specified `country`.
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAeStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAeType,
}
impl CreateTaxRegistrationCountryOptionsAe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAeStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsAeStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsAeStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAeStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in AL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAl {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAlStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAlType,
}
impl CreateTaxRegistrationCountryOptionsAl {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAlType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAlStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsAlStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsAlStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAlStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAlType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAlType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAlType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAlType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAlType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAlType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAlType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAlType")
        })
    }
}
/// Options for the registration in AM.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAm {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAmType,
}
impl CreateTaxRegistrationCountryOptionsAm {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAmType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsAmType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAmType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAmType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAmType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAmType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAmType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAmType")
        })
    }
}
/// Options for the registration in AO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAo {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAoStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAoType,
}
impl CreateTaxRegistrationCountryOptionsAo {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAoType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAoStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsAoStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsAoStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAoStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAoType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAoType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAoType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAoType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAoType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAoType")
        })
    }
}
/// Options for the registration in AT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAtStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
pub struct CreateTaxRegistrationCountryOptionsAu {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAuStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAuType,
}
impl CreateTaxRegistrationCountryOptionsAu {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAuType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAuStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsAuStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsAuStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAuStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in AW.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAw {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsAwStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAwType,
}
impl CreateTaxRegistrationCountryOptionsAw {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAwType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAwStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsAwStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsAwStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAwStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAwType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsAwType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAwType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAwType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAwType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAwType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAwType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAwType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAwType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAwType")
        })
    }
}
/// Options for the registration in AZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsAz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsAzType,
}
impl CreateTaxRegistrationCountryOptionsAz {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsAzType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsAzType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsAzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsAzType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsAzType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsAzType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsAzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsAzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsAzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsAzType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsAzType")
        })
    }
}
/// Options for the registration in BA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBa {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBaStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBaType,
}
impl CreateTaxRegistrationCountryOptionsBa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBaType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBaStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsBaStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsBaStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBaStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBaType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBaType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBaType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBaType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBaType")
        })
    }
}
/// Options for the registration in BB.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBb {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBbStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBbType,
}
impl CreateTaxRegistrationCountryOptionsBb {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBbType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBbStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsBbStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsBbStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBbStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBbType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBbType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBbType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBbType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBbType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBbType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBbType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBbType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBbType")
        })
    }
}
/// Options for the registration in BD.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBd {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBdStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBdType,
}
impl CreateTaxRegistrationCountryOptionsBd {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBdType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBdStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsBdStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsBdStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBdStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBdType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBdType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBdType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBdType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBdType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBdType")
        })
    }
}
/// Options for the registration in BE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBeStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in BF.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBf {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBfStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBfType,
}
impl CreateTaxRegistrationCountryOptionsBf {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBfType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBfStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsBfStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsBfStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBfStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBfType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBfType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBfType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBfType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBfType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBfType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBfType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBfType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBfType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBfType")
        })
    }
}
/// Options for the registration in BG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBgStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in BH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBh {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBhStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBhType,
}
impl CreateTaxRegistrationCountryOptionsBh {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBhType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBhStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsBhStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsBhStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBhStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBhType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBhType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBhType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBhType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBhType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBhType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBhType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBhType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBhType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBhType")
        })
    }
}
/// Options for the registration in BJ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBj {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBjType,
}
impl CreateTaxRegistrationCountryOptionsBj {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBjType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBjType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsBjType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBjType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBjType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBjType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBjType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBjType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBjType")
        })
    }
}
/// Options for the registration in BS.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsBsStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsBsType,
}
impl CreateTaxRegistrationCountryOptionsBs {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsBsType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBsStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsBsStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsBsStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBsStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsBsType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsBsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsBsType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsBsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsBsType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsBsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsBsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsBsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsBsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsBsType")
        })
    }
}
/// Options for the registration in BY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsBy {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsByType,
}
impl CreateTaxRegistrationCountryOptionsBy {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsByType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsByType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsByType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsByType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsByType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsByType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsByType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsByType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsByType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsByType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsByType")
        })
    }
}
/// Options for the registration in CA.
#[derive(Clone, Debug, serde::Serialize)]
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
/// Options for the registration in CD.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCd {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsCdStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCdType,
}
impl CreateTaxRegistrationCountryOptionsCd {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCdType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCdStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsCdStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsCdStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCdStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCdType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCdType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCdType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCdType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCdType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCdType")
        })
    }
}
/// Options for the registration in CH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCh {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsChStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsChType,
}
impl CreateTaxRegistrationCountryOptionsCh {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsChType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsChStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsChStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsChStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsChStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in CM.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCm {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCmType,
}
impl CreateTaxRegistrationCountryOptionsCm {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCmType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsCmType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCmType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCmType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCmType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCmType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCmType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCmType")
        })
    }
}
/// Options for the registration in CO.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
/// Options for the registration in CR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCr {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCrType,
}
impl CreateTaxRegistrationCountryOptionsCr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCrType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCrType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsCrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCrType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCrType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCrType")
        })
    }
}
/// Options for the registration in CV.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsCv {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsCvType,
}
impl CreateTaxRegistrationCountryOptionsCv {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsCvType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsCvType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsCvType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCvType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsCvType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsCvType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsCvType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsCvType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsCvType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsCvType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsCvType")
        })
    }
}
/// Options for the registration in CY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCyStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsCzStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDeStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsDkStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in EC.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEc {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsEcType,
}
impl CreateTaxRegistrationCountryOptionsEc {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsEcType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEcType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsEcType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEcType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEcType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEcType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEcType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEcType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEcType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsEcType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEcType")
        })
    }
}
/// Options for the registration in EE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEeStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in EG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsEgType,
}
impl CreateTaxRegistrationCountryOptionsEg {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsEgType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEgType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsEgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEgType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEgType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEgType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsEgType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEgType")
        })
    }
}
/// Options for the registration in ES.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEsStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in ET.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEt {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsEtStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsEtType,
}
impl CreateTaxRegistrationCountryOptionsEt {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsEtType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsEtStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsEtStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsEtStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEtStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsEtType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsEtType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsEtType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsEtType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsEtType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsEtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsEtType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsEtType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsEtType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsEtType")
        })
    }
}
/// Options for the registration in FI.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFiStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsFrStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
pub struct CreateTaxRegistrationCountryOptionsGb {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsGbStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsGbType,
}
impl CreateTaxRegistrationCountryOptionsGb {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsGbType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGbStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsGbStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsGbStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsGbStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in GE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsGeType,
}
impl CreateTaxRegistrationCountryOptionsGe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsGeType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGeType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsGeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGeType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGeType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsGeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsGeType")
        })
    }
}
/// Options for the registration in GN.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGn {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsGnStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsGnType,
}
impl CreateTaxRegistrationCountryOptionsGn {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsGnType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsGnStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsGnStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsGnStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsGnStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsGnType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGnType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGnType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsGnType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsGnType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsGnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsGnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsGnType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsGnType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsGnType")
        })
    }
}
/// Options for the registration in GR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsGrStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHrStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsHuStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIeStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in IN.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsInType,
}
impl CreateTaxRegistrationCountryOptionsIn {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsInType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsInType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsInType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsInType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsInType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsInType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsInType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsInType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsInType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsInType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsInType")
        })
    }
}
/// Options for the registration in IS.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsIsStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsIsType,
}
impl CreateTaxRegistrationCountryOptionsIs {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsIsType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsIsStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsIsStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsIsStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsIsStandardPlaceOfSupplyScheme"))
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsItStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
pub struct CreateTaxRegistrationCountryOptionsJp {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsJpStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsJpType,
}
impl CreateTaxRegistrationCountryOptionsJp {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsJpType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsJpStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsJpStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsJpStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsJpStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in KE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsKeType,
}
impl CreateTaxRegistrationCountryOptionsKe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsKeType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsKeType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsKeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsKeType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsKeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsKeType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsKeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsKeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsKeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsKeType")
        })
    }
}
/// Options for the registration in KG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsKgType,
}
impl CreateTaxRegistrationCountryOptionsKg {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsKgType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsKgType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsKgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsKgType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsKgType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsKgType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsKgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsKgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsKgType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsKgType")
        })
    }
}
/// Options for the registration in KH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsKhType,
}
impl CreateTaxRegistrationCountryOptionsKh {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsKhType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsKhType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsKhType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsKhType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsKhType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsKhType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsKhType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKhType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsKhType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsKhType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsKhType")
        })
    }
}
/// Options for the registration in KR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
/// Options for the registration in KZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsKz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsKzType,
}
impl CreateTaxRegistrationCountryOptionsKz {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsKzType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsKzType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsKzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsKzType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsKzType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsKzType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsKzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsKzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsKzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsKzType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsKzType")
        })
    }
}
/// Options for the registration in LA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsLa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsLaType,
}
impl CreateTaxRegistrationCountryOptionsLa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsLaType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsLaType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsLaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLaType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsLaType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsLaType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsLaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsLaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsLaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsLaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsLaType")
        })
    }
}
/// Options for the registration in LT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLtStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLuStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsLvStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in MA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMaType,
}
impl CreateTaxRegistrationCountryOptionsMa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMaType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMaType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsMaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMaType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMaType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMaType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMaType")
        })
    }
}
/// Options for the registration in MD.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMd {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMdType,
}
impl CreateTaxRegistrationCountryOptionsMd {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMdType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMdType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsMdType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMdType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMdType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMdType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMdType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMdType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMdType")
        })
    }
}
/// Options for the registration in ME.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMe {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsMeStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMeType,
}
impl CreateTaxRegistrationCountryOptionsMe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMeType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMeStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsMeStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsMeStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMeStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMeType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMeType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMeType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMeType")
        })
    }
}
/// Options for the registration in MK.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMk {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsMkStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMkType,
}
impl CreateTaxRegistrationCountryOptionsMk {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMkType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMkStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsMkStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsMkStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMkStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMkType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMkType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMkType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMkType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMkType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMkType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMkType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMkType")
        })
    }
}
/// Options for the registration in MR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsMrStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsMrType,
}
impl CreateTaxRegistrationCountryOptionsMr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsMrType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsMrStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsMrStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsMrStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMrStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsMrType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMrType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsMrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsMrType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsMrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsMrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsMrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsMrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsMrType")
        })
    }
}
/// Options for the registration in MT.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsMtStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in NG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNgType,
}
impl CreateTaxRegistrationCountryOptionsNg {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsNgType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNgType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsNgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNgType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNgType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNgType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsNgType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNgType")
        })
    }
}
/// Options for the registration in NL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNlStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
pub struct CreateTaxRegistrationCountryOptionsNo {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsNoStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNoType,
}
impl CreateTaxRegistrationCountryOptionsNo {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsNoType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNoStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsNoStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsNoStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNoStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in NP.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNp {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNpType,
}
impl CreateTaxRegistrationCountryOptionsNp {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsNpType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNpType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsNpType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNpType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNpType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNpType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNpType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsNpType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNpType")
        })
    }
}
/// Options for the registration in NZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNz {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsNzStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsNzType,
}
impl CreateTaxRegistrationCountryOptionsNz {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsNzType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsNzStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsNzStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsNzStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsNzStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in OM.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsOm {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsOmStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsOmType,
}
impl CreateTaxRegistrationCountryOptionsOm {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsOmType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsOmStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsOmStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsOmStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsOmStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsOmType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsOmType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsOmType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsOmType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsOmType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsOmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsOmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsOmType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsOmType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsOmType")
        })
    }
}
/// Options for the registration in PE.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPe {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsPeType,
}
impl CreateTaxRegistrationCountryOptionsPe {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsPeType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPeType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsPeType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPeType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPeType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPeType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsPeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsPeType")
        })
    }
}
/// Options for the registration in PH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsPh {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsPhType,
}
impl CreateTaxRegistrationCountryOptionsPh {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsPhType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsPhType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsPhType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPhType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsPhType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsPhType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsPhType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsPhType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsPhType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsPhType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsPhType")
        })
    }
}
/// Options for the registration in PL.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPlStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsPtStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRoStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in RS.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRs {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsRsStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsRsType,
}
impl CreateTaxRegistrationCountryOptionsRs {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsRsType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRsStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsRsStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsRsStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsRsStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsRsType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsRsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRsType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsRsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsRsType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsRsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsRsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsRsType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsRsType")
        })
    }
}
/// Options for the registration in RU.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsRu {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsRuType,
}
impl CreateTaxRegistrationCountryOptionsRu {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsRuType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsRuType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsRuType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsRuType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsRuType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsRuType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsRuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsRuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsRuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsRuType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsRuType")
        })
    }
}
/// Options for the registration in SA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSeStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
pub struct CreateTaxRegistrationCountryOptionsSg {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSgStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSgType,
}
impl CreateTaxRegistrationCountryOptionsSg {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSgType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSgStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsSgStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsSgStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSgStandardPlaceOfSupplyScheme"))
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSiStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
    InboundGoods,
    SmallSeller,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSkStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
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
            "inbound_goods" => Ok(InboundGoods),
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
/// Options for the registration in SN.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSn {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSnType,
}
impl CreateTaxRegistrationCountryOptionsSn {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSnType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSnType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsSnType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSnType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSnType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSnType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSnType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsSnType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSnType")
        })
    }
}
/// Options for the registration in SR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSr {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsSrStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsSrType,
}
impl CreateTaxRegistrationCountryOptionsSr {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsSrType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsSrStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsSrStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsSrStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSrStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsSrType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsSrType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsSrType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsSrType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsSrType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsSrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsSrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsSrType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsSrType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsSrType")
        })
    }
}
/// Options for the registration in TH.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
/// Options for the registration in TJ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTj {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsTjType,
}
impl CreateTaxRegistrationCountryOptionsTj {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsTjType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsTjType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsTjType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsTjType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsTjType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsTjType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsTjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTjType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsTjType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsTjType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsTjType")
        })
    }
}
/// Options for the registration in TR.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
/// Options for the registration in TW.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTw {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsTwType,
}
impl CreateTaxRegistrationCountryOptionsTw {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsTwType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsTwType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsTwType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsTwType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsTwType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsTwType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsTwType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTwType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsTwType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsTwType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsTwType")
        })
    }
}
/// Options for the registration in TZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsTz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsTzType,
}
impl CreateTaxRegistrationCountryOptionsTz {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsTzType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsTzType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsTzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsTzType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsTzType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsTzType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsTzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsTzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsTzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsTzType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsTzType")
        })
    }
}
/// Options for the registration in UA.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUa {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsUaType,
}
impl CreateTaxRegistrationCountryOptionsUa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsUaType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUaType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsUaType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUaType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUaType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUaType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsUaType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsUaType")
        })
    }
}
/// Options for the registration in UG.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUg {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsUgType,
}
impl CreateTaxRegistrationCountryOptionsUg {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsUgType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUgType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsUgType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUgType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUgType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUgType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUgType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUgType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsUgType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsUgType")
        })
    }
}
/// Options for the registration in US.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUs {
    /// Options for the local amusement tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalAmusementTax>,
    /// Options for the local lease tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<CreateTaxRegistrationCountryOptionsUsLocalLeaseTax>,
    /// Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    pub state: String,
    /// Options for the state sales tax registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_sales_tax: Option<CreateTaxRegistrationCountryOptionsUsStateSalesTax>,
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
            state_sales_tax: None,
            type_: type_.into(),
        }
    }
}
/// Options for the local amusement tax registration.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsLocalAmusementTax {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    /// Supported FIPS codes are: `14000` (Chicago), `02154` (Arlington Heights), `06613` (Bloomington), `10906` (Campton Hills), `21696` (East Dundee), `24582` (Evanston), `45421` (Lynwood), `48892` (Midlothian), `64343` (River Grove), and `68081` (Schiller Park).
    pub jurisdiction: String,
}
impl CreateTaxRegistrationCountryOptionsUsLocalAmusementTax {
    pub fn new(jurisdiction: impl Into<String>) -> Self {
        Self { jurisdiction: jurisdiction.into() }
    }
}
/// Options for the local lease tax registration.
#[derive(Clone, Debug, serde::Serialize)]
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
/// Options for the state sales tax registration.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsStateSalesTax {
    /// Elections for the state sales tax registration.
    pub elections: Vec<CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections>,
}
impl CreateTaxRegistrationCountryOptionsUsStateSalesTax {
    pub fn new(
        elections: impl Into<Vec<CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections>>,
    ) -> Self {
        Self { elections: elections.into() }
    }
}
/// Elections for the state sales tax registration.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections {
    /// A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction.
    /// Supported FIPS codes are: `003` (Allegheny County) and `60000` (Philadelphia City).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    /// The type of the election for the state sales tax registration.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType,
}
impl CreateTaxRegistrationCountryOptionsUsStateSalesTaxElections {
    pub fn new(
        type_: impl Into<CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType>,
    ) -> Self {
        Self { jurisdiction: None, type_: type_.into() }
    }
}
/// The type of the election for the state sales tax registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    LocalUseTax,
    SimplifiedSellersUseTax,
    SingleLocalUseTax,
}
impl CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType::*;
        match self {
            LocalUseTax => "local_use_tax",
            SimplifiedSellersUseTax => "simplified_sellers_use_tax",
            SingleLocalUseTax => "single_local_use_tax",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType::*;
        match s {
            "local_use_tax" => Ok(LocalUseTax),
            "simplified_sellers_use_tax" => Ok(SimplifiedSellersUseTax),
            "single_local_use_tax" => Ok(SingleLocalUseTax),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateTaxRegistrationCountryOptionsUsStateSalesTaxElectionsType",
            )
        })
    }
}
/// Type of registration to be created in the US.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUsType {
    LocalAmusementTax,
    LocalLeaseTax,
    StateCommunicationsTax,
    StateRetailDeliveryFee,
    StateSalesTax,
}
impl CreateTaxRegistrationCountryOptionsUsType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUsType::*;
        match self {
            LocalAmusementTax => "local_amusement_tax",
            LocalLeaseTax => "local_lease_tax",
            StateCommunicationsTax => "state_communications_tax",
            StateRetailDeliveryFee => "state_retail_delivery_fee",
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
            "state_retail_delivery_fee" => Ok(StateRetailDeliveryFee),
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
/// Options for the registration in UY.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUy {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsUyStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsUyType,
}
impl CreateTaxRegistrationCountryOptionsUy {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsUyType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUyStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsUyStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsUyStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsUyStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUyType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsUyType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUyType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUyType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUyType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsUyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsUyType")
        })
    }
}
/// Options for the registration in UZ.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsUz {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsUzType,
}
impl CreateTaxRegistrationCountryOptionsUz {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsUzType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsUzType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsUzType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsUzType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsUzType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsUzType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsUzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsUzType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsUzType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsUzType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsUzType")
        })
    }
}
/// Options for the registration in VN.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
pub struct CreateTaxRegistrationCountryOptionsZa {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsZaStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsZaType,
}
impl CreateTaxRegistrationCountryOptionsZa {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsZaType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZaStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsZaStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsZaStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsZaStandardPlaceOfSupplyScheme"))
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
/// Options for the registration in ZM.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZm {
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsZmType,
}
impl CreateTaxRegistrationCountryOptionsZm {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsZmType>) -> Self {
        Self { type_: type_.into() }
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsZmType {
    Simplified,
}
impl CreateTaxRegistrationCountryOptionsZmType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsZmType::*;
        match self {
            Simplified => "simplified",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsZmType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsZmType::*;
        match s {
            "simplified" => Ok(Simplified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsZmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZmType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsZmType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsZmType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsZmType")
        })
    }
}
/// Options for the registration in ZW.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZw {
    /// Options for the standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<CreateTaxRegistrationCountryOptionsZwStandard>,
    /// Type of registration to be created in `country`.
    #[serde(rename = "type")]
    pub type_: CreateTaxRegistrationCountryOptionsZwType,
}
impl CreateTaxRegistrationCountryOptionsZw {
    pub fn new(type_: impl Into<CreateTaxRegistrationCountryOptionsZwType>) -> Self {
        Self { standard: None, type_: type_.into() }
    }
}
/// Options for the standard registration.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateTaxRegistrationCountryOptionsZwStandard {
    /// Place of supply scheme used in an standard registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_supply_scheme:
        Option<CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme>,
}
impl CreateTaxRegistrationCountryOptionsZwStandard {
    pub fn new() -> Self {
        Self { place_of_supply_scheme: None }
    }
}
impl Default for CreateTaxRegistrationCountryOptionsZwStandard {
    fn default() -> Self {
        Self::new()
    }
}
/// Place of supply scheme used in an standard registration.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
}
impl CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme::*;
        match self {
            InboundGoods => "inbound_goods",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme::*;
        match s {
            "inbound_goods" => Ok(InboundGoods),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsZwStandardPlaceOfSupplyScheme"))
    }
}
/// Type of registration to be created in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateTaxRegistrationCountryOptionsZwType {
    Standard,
}
impl CreateTaxRegistrationCountryOptionsZwType {
    pub fn as_str(self) -> &'static str {
        use CreateTaxRegistrationCountryOptionsZwType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateTaxRegistrationCountryOptionsZwType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateTaxRegistrationCountryOptionsZwType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateTaxRegistrationCountryOptionsZwType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateTaxRegistrationCountryOptionsZwType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateTaxRegistrationCountryOptionsZwType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateTaxRegistrationCountryOptionsZwType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateTaxRegistrationCountryOptionsZwType")
        })
    }
}
/// Creates a new Tax `Registration` object.
#[derive(Clone, Debug, serde::Serialize)]
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
