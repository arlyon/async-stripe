use crate::{Client, Response};

impl crate::reporting::report_run::ReportRun {
    /// Retrieves the details of an existing Report Run.
    pub fn retrieve(
        client: &Client,
        report_run: &str,
        params: RetrieveReportRun,
    ) -> Response<crate::reporting::report_run::ReportRun> {
        client.get_query(
            &format!("/reporting/report_runs/{report_run}", report_run = report_run),
            params,
        )
    }
    /// Creates a new object and begin running the report.
    ///
    /// (Certain report types require a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).).
    pub fn create(
        client: &Client,
        params: CreateReportRun,
    ) -> Response<crate::reporting::report_run::ReportRun> {
        client.send_form("/reporting/report_runs", params, http_types::Method::Post)
    }
    /// Returns a list of Report Runs, with the most recent appearing first.
    pub fn list(
        client: &Client,
        params: ListReportRun,
    ) -> Response<crate::List<crate::reporting::report_run::ReportRun>> {
        client.get_query("/reporting/report_runs", params)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveReportRun<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveReportRun<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateReportRun<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Parameters specifying how the report should be run.
    ///
    /// Different Report Types have different required and optional parameters, listed in the [API Access to Reports](https://stripe.com/docs/reporting/statements/api) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<CreateReportRunParameters<'a>>,
    /// The ID of the [report type](https://stripe.com/docs/reporting/statements/api#report-types) to run, such as `"balance.summary.1"`.
    pub report_type: &'a str,
}
impl<'a> CreateReportRun<'a> {
    pub fn new(report_type: &'a str) -> Self {
        Self { expand: Default::default(), parameters: Default::default(), report_type }
    }
}
/// Parameters specifying how the report should be run.
///
/// Different Report Types have different required and optional parameters, listed in the [API Access to Reports](https://stripe.com/docs/reporting/statements/api) documentation.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateReportRunParameters<'a> {
    /// The set of report columns to include in the report output.
    ///
    /// If omitted, the Report Type is run with its default column set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<&'a [&'a str]>,
    /// Connected account ID to filter for in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account: Option<&'a str>,
    /// Currency of objects to be included in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::Currency>,
    /// Ending timestamp of data to be included in the report run (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_end: Option<crate::Timestamp>,
    /// Starting timestamp of data to be included in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_start: Option<crate::Timestamp>,
    /// Payout ID by which to filter the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<&'a str>,
    /// Category of balance transactions to be included in the report run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_category: Option<CreateReportRunParametersReportingCategory>,
    /// Defaults to `Etc/UTC`.
    ///
    /// The output timezone for all timestamps in the report.
    /// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
    /// Has no effect on `interval_start` or `interval_end`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<CreateReportRunParametersTimezone>,
}
impl<'a> CreateReportRunParameters<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Category of balance transactions to be included in the report run.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateReportRunParametersReportingCategory {
    Advance,
    AdvanceFunding,
    AnticipationRepayment,
    Charge,
    ChargeFailure,
    ConnectCollectionTransfer,
    ConnectReservedFunds,
    Contribution,
    Dispute,
    DisputeReversal,
    Fee,
    FinancingPaydown,
    FinancingPaydownReversal,
    FinancingPayout,
    FinancingPayoutReversal,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    IssuingDispute,
    IssuingTransaction,
    NetworkCost,
    OtherAdjustment,
    PartialCaptureReversal,
    Payout,
    PayoutReversal,
    PlatformEarning,
    PlatformEarningRefund,
    Refund,
    RefundFailure,
    RiskReservedFunds,
    Tax,
    Topup,
    TopupReversal,
    Transfer,
    TransferReversal,
}

impl CreateReportRunParametersReportingCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advance => "advance",
            Self::AdvanceFunding => "advance_funding",
            Self::AnticipationRepayment => "anticipation_repayment",
            Self::Charge => "charge",
            Self::ChargeFailure => "charge_failure",
            Self::ConnectCollectionTransfer => "connect_collection_transfer",
            Self::ConnectReservedFunds => "connect_reserved_funds",
            Self::Contribution => "contribution",
            Self::Dispute => "dispute",
            Self::DisputeReversal => "dispute_reversal",
            Self::Fee => "fee",
            Self::FinancingPaydown => "financing_paydown",
            Self::FinancingPaydownReversal => "financing_paydown_reversal",
            Self::FinancingPayout => "financing_payout",
            Self::FinancingPayoutReversal => "financing_payout_reversal",
            Self::IssuingAuthorizationHold => "issuing_authorization_hold",
            Self::IssuingAuthorizationRelease => "issuing_authorization_release",
            Self::IssuingDispute => "issuing_dispute",
            Self::IssuingTransaction => "issuing_transaction",
            Self::NetworkCost => "network_cost",
            Self::OtherAdjustment => "other_adjustment",
            Self::PartialCaptureReversal => "partial_capture_reversal",
            Self::Payout => "payout",
            Self::PayoutReversal => "payout_reversal",
            Self::PlatformEarning => "platform_earning",
            Self::PlatformEarningRefund => "platform_earning_refund",
            Self::Refund => "refund",
            Self::RefundFailure => "refund_failure",
            Self::RiskReservedFunds => "risk_reserved_funds",
            Self::Tax => "tax",
            Self::Topup => "topup",
            Self::TopupReversal => "topup_reversal",
            Self::Transfer => "transfer",
            Self::TransferReversal => "transfer_reversal",
        }
    }
}

impl AsRef<str> for CreateReportRunParametersReportingCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateReportRunParametersReportingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Defaults to `Etc/UTC`.
///
/// The output timezone for all timestamps in the report.
/// A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones).
/// Has no effect on `interval_start` or `interval_end`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateReportRunParametersTimezone {
    #[serde(rename = "Africa/Abidjan")]
    AfricaAbidjan,
    #[serde(rename = "Africa/Accra")]
    AfricaAccra,
    #[serde(rename = "Africa/Addis_Ababa")]
    AfricaAddisAbaba,
    #[serde(rename = "Africa/Algiers")]
    AfricaAlgiers,
    #[serde(rename = "Africa/Asmara")]
    AfricaAsmara,
    #[serde(rename = "Africa/Asmera")]
    AfricaAsmera,
    #[serde(rename = "Africa/Bamako")]
    AfricaBamako,
    #[serde(rename = "Africa/Bangui")]
    AfricaBangui,
    #[serde(rename = "Africa/Banjul")]
    AfricaBanjul,
    #[serde(rename = "Africa/Bissau")]
    AfricaBissau,
    #[serde(rename = "Africa/Blantyre")]
    AfricaBlantyre,
    #[serde(rename = "Africa/Brazzaville")]
    AfricaBrazzaville,
    #[serde(rename = "Africa/Bujumbura")]
    AfricaBujumbura,
    #[serde(rename = "Africa/Cairo")]
    AfricaCairo,
    #[serde(rename = "Africa/Casablanca")]
    AfricaCasablanca,
    #[serde(rename = "Africa/Ceuta")]
    AfricaCeuta,
    #[serde(rename = "Africa/Conakry")]
    AfricaConakry,
    #[serde(rename = "Africa/Dakar")]
    AfricaDakar,
    #[serde(rename = "Africa/Dar_es_Salaam")]
    AfricaDarEsSalaam,
    #[serde(rename = "Africa/Djibouti")]
    AfricaDjibouti,
    #[serde(rename = "Africa/Douala")]
    AfricaDouala,
    #[serde(rename = "Africa/El_Aaiun")]
    AfricaElAaiun,
    #[serde(rename = "Africa/Freetown")]
    AfricaFreetown,
    #[serde(rename = "Africa/Gaborone")]
    AfricaGaborone,
    #[serde(rename = "Africa/Harare")]
    AfricaHarare,
    #[serde(rename = "Africa/Johannesburg")]
    AfricaJohannesburg,
    #[serde(rename = "Africa/Juba")]
    AfricaJuba,
    #[serde(rename = "Africa/Kampala")]
    AfricaKampala,
    #[serde(rename = "Africa/Khartoum")]
    AfricaKhartoum,
    #[serde(rename = "Africa/Kigali")]
    AfricaKigali,
    #[serde(rename = "Africa/Kinshasa")]
    AfricaKinshasa,
    #[serde(rename = "Africa/Lagos")]
    AfricaLagos,
    #[serde(rename = "Africa/Libreville")]
    AfricaLibreville,
    #[serde(rename = "Africa/Lome")]
    AfricaLome,
    #[serde(rename = "Africa/Luanda")]
    AfricaLuanda,
    #[serde(rename = "Africa/Lubumbashi")]
    AfricaLubumbashi,
    #[serde(rename = "Africa/Lusaka")]
    AfricaLusaka,
    #[serde(rename = "Africa/Malabo")]
    AfricaMalabo,
    #[serde(rename = "Africa/Maputo")]
    AfricaMaputo,
    #[serde(rename = "Africa/Maseru")]
    AfricaMaseru,
    #[serde(rename = "Africa/Mbabane")]
    AfricaMbabane,
    #[serde(rename = "Africa/Mogadishu")]
    AfricaMogadishu,
    #[serde(rename = "Africa/Monrovia")]
    AfricaMonrovia,
    #[serde(rename = "Africa/Nairobi")]
    AfricaNairobi,
    #[serde(rename = "Africa/Ndjamena")]
    AfricaNdjamena,
    #[serde(rename = "Africa/Niamey")]
    AfricaNiamey,
    #[serde(rename = "Africa/Nouakchott")]
    AfricaNouakchott,
    #[serde(rename = "Africa/Ouagadougou")]
    AfricaOuagadougou,
    #[serde(rename = "Africa/Porto-Novo")]
    AfricaPortoMinusNovo,
    #[serde(rename = "Africa/Sao_Tome")]
    AfricaSaoTome,
    #[serde(rename = "Africa/Timbuktu")]
    AfricaTimbuktu,
    #[serde(rename = "Africa/Tripoli")]
    AfricaTripoli,
    #[serde(rename = "Africa/Tunis")]
    AfricaTunis,
    #[serde(rename = "Africa/Windhoek")]
    AfricaWindhoek,
    #[serde(rename = "America/Adak")]
    AmericaAdak,
    #[serde(rename = "America/Anchorage")]
    AmericaAnchorage,
    #[serde(rename = "America/Anguilla")]
    AmericaAnguilla,
    #[serde(rename = "America/Antigua")]
    AmericaAntigua,
    #[serde(rename = "America/Araguaina")]
    AmericaAraguaina,
    #[serde(rename = "America/Argentina/Buenos_Aires")]
    AmericaArgentinaBuenosAires,
    #[serde(rename = "America/Argentina/Catamarca")]
    AmericaArgentinaCatamarca,
    #[serde(rename = "America/Argentina/ComodRivadavia")]
    AmericaArgentinaComodRivadavia,
    #[serde(rename = "America/Argentina/Cordoba")]
    AmericaArgentinaCordoba,
    #[serde(rename = "America/Argentina/Jujuy")]
    AmericaArgentinaJujuy,
    #[serde(rename = "America/Argentina/La_Rioja")]
    AmericaArgentinaLaRioja,
    #[serde(rename = "America/Argentina/Mendoza")]
    AmericaArgentinaMendoza,
    #[serde(rename = "America/Argentina/Rio_Gallegos")]
    AmericaArgentinaRioGallegos,
    #[serde(rename = "America/Argentina/Salta")]
    AmericaArgentinaSalta,
    #[serde(rename = "America/Argentina/San_Juan")]
    AmericaArgentinaSanJuan,
    #[serde(rename = "America/Argentina/San_Luis")]
    AmericaArgentinaSanLuis,
    #[serde(rename = "America/Argentina/Tucuman")]
    AmericaArgentinaTucuman,
    #[serde(rename = "America/Argentina/Ushuaia")]
    AmericaArgentinaUshuaia,
    #[serde(rename = "America/Aruba")]
    AmericaAruba,
    #[serde(rename = "America/Asuncion")]
    AmericaAsuncion,
    #[serde(rename = "America/Atikokan")]
    AmericaAtikokan,
    #[serde(rename = "America/Atka")]
    AmericaAtka,
    #[serde(rename = "America/Bahia")]
    AmericaBahia,
    #[serde(rename = "America/Bahia_Banderas")]
    AmericaBahiaBanderas,
    #[serde(rename = "America/Barbados")]
    AmericaBarbados,
    #[serde(rename = "America/Belem")]
    AmericaBelem,
    #[serde(rename = "America/Belize")]
    AmericaBelize,
    #[serde(rename = "America/Blanc-Sablon")]
    AmericaBlancMinusSablon,
    #[serde(rename = "America/Boa_Vista")]
    AmericaBoaVista,
    #[serde(rename = "America/Bogota")]
    AmericaBogota,
    #[serde(rename = "America/Boise")]
    AmericaBoise,
    #[serde(rename = "America/Buenos_Aires")]
    AmericaBuenosAires,
    #[serde(rename = "America/Cambridge_Bay")]
    AmericaCambridgeBay,
    #[serde(rename = "America/Campo_Grande")]
    AmericaCampoGrande,
    #[serde(rename = "America/Cancun")]
    AmericaCancun,
    #[serde(rename = "America/Caracas")]
    AmericaCaracas,
    #[serde(rename = "America/Catamarca")]
    AmericaCatamarca,
    #[serde(rename = "America/Cayenne")]
    AmericaCayenne,
    #[serde(rename = "America/Cayman")]
    AmericaCayman,
    #[serde(rename = "America/Chicago")]
    AmericaChicago,
    #[serde(rename = "America/Chihuahua")]
    AmericaChihuahua,
    #[serde(rename = "America/Coral_Harbour")]
    AmericaCoralHarbour,
    #[serde(rename = "America/Cordoba")]
    AmericaCordoba,
    #[serde(rename = "America/Costa_Rica")]
    AmericaCostaRica,
    #[serde(rename = "America/Creston")]
    AmericaCreston,
    #[serde(rename = "America/Cuiaba")]
    AmericaCuiaba,
    #[serde(rename = "America/Curacao")]
    AmericaCuracao,
    #[serde(rename = "America/Danmarkshavn")]
    AmericaDanmarkshavn,
    #[serde(rename = "America/Dawson")]
    AmericaDawson,
    #[serde(rename = "America/Dawson_Creek")]
    AmericaDawsonCreek,
    #[serde(rename = "America/Denver")]
    AmericaDenver,
    #[serde(rename = "America/Detroit")]
    AmericaDetroit,
    #[serde(rename = "America/Dominica")]
    AmericaDominica,
    #[serde(rename = "America/Edmonton")]
    AmericaEdmonton,
    #[serde(rename = "America/Eirunepe")]
    AmericaEirunepe,
    #[serde(rename = "America/El_Salvador")]
    AmericaElSalvador,
    #[serde(rename = "America/Ensenada")]
    AmericaEnsenada,
    #[serde(rename = "America/Fort_Nelson")]
    AmericaFortNelson,
    #[serde(rename = "America/Fort_Wayne")]
    AmericaFortWayne,
    #[serde(rename = "America/Fortaleza")]
    AmericaFortaleza,
    #[serde(rename = "America/Glace_Bay")]
    AmericaGlaceBay,
    #[serde(rename = "America/Godthab")]
    AmericaGodthab,
    #[serde(rename = "America/Goose_Bay")]
    AmericaGooseBay,
    #[serde(rename = "America/Grand_Turk")]
    AmericaGrandTurk,
    #[serde(rename = "America/Grenada")]
    AmericaGrenada,
    #[serde(rename = "America/Guadeloupe")]
    AmericaGuadeloupe,
    #[serde(rename = "America/Guatemala")]
    AmericaGuatemala,
    #[serde(rename = "America/Guayaquil")]
    AmericaGuayaquil,
    #[serde(rename = "America/Guyana")]
    AmericaGuyana,
    #[serde(rename = "America/Halifax")]
    AmericaHalifax,
    #[serde(rename = "America/Havana")]
    AmericaHavana,
    #[serde(rename = "America/Hermosillo")]
    AmericaHermosillo,
    #[serde(rename = "America/Indiana/Indianapolis")]
    AmericaIndianaIndianapolis,
    #[serde(rename = "America/Indiana/Knox")]
    AmericaIndianaKnox,
    #[serde(rename = "America/Indiana/Marengo")]
    AmericaIndianaMarengo,
    #[serde(rename = "America/Indiana/Petersburg")]
    AmericaIndianaPetersburg,
    #[serde(rename = "America/Indiana/Tell_City")]
    AmericaIndianaTellCity,
    #[serde(rename = "America/Indiana/Vevay")]
    AmericaIndianaVevay,
    #[serde(rename = "America/Indiana/Vincennes")]
    AmericaIndianaVincennes,
    #[serde(rename = "America/Indiana/Winamac")]
    AmericaIndianaWinamac,
    #[serde(rename = "America/Indianapolis")]
    AmericaIndianapolis,
    #[serde(rename = "America/Inuvik")]
    AmericaInuvik,
    #[serde(rename = "America/Iqaluit")]
    AmericaIqaluit,
    #[serde(rename = "America/Jamaica")]
    AmericaJamaica,
    #[serde(rename = "America/Jujuy")]
    AmericaJujuy,
    #[serde(rename = "America/Juneau")]
    AmericaJuneau,
    #[serde(rename = "America/Kentucky/Louisville")]
    AmericaKentuckyLouisville,
    #[serde(rename = "America/Kentucky/Monticello")]
    AmericaKentuckyMonticello,
    #[serde(rename = "America/Knox_IN")]
    AmericaKnoxIn,
    #[serde(rename = "America/Kralendijk")]
    AmericaKralendijk,
    #[serde(rename = "America/La_Paz")]
    AmericaLaPaz,
    #[serde(rename = "America/Lima")]
    AmericaLima,
    #[serde(rename = "America/Los_Angeles")]
    AmericaLosAngeles,
    #[serde(rename = "America/Louisville")]
    AmericaLouisville,
    #[serde(rename = "America/Lower_Princes")]
    AmericaLowerPrinces,
    #[serde(rename = "America/Maceio")]
    AmericaMaceio,
    #[serde(rename = "America/Managua")]
    AmericaManagua,
    #[serde(rename = "America/Manaus")]
    AmericaManaus,
    #[serde(rename = "America/Marigot")]
    AmericaMarigot,
    #[serde(rename = "America/Martinique")]
    AmericaMartinique,
    #[serde(rename = "America/Matamoros")]
    AmericaMatamoros,
    #[serde(rename = "America/Mazatlan")]
    AmericaMazatlan,
    #[serde(rename = "America/Mendoza")]
    AmericaMendoza,
    #[serde(rename = "America/Menominee")]
    AmericaMenominee,
    #[serde(rename = "America/Merida")]
    AmericaMerida,
    #[serde(rename = "America/Metlakatla")]
    AmericaMetlakatla,
    #[serde(rename = "America/Mexico_City")]
    AmericaMexicoCity,
    #[serde(rename = "America/Miquelon")]
    AmericaMiquelon,
    #[serde(rename = "America/Moncton")]
    AmericaMoncton,
    #[serde(rename = "America/Monterrey")]
    AmericaMonterrey,
    #[serde(rename = "America/Montevideo")]
    AmericaMontevideo,
    #[serde(rename = "America/Montreal")]
    AmericaMontreal,
    #[serde(rename = "America/Montserrat")]
    AmericaMontserrat,
    #[serde(rename = "America/Nassau")]
    AmericaNassau,
    #[serde(rename = "America/New_York")]
    AmericaNewYork,
    #[serde(rename = "America/Nipigon")]
    AmericaNipigon,
    #[serde(rename = "America/Nome")]
    AmericaNome,
    #[serde(rename = "America/Noronha")]
    AmericaNoronha,
    #[serde(rename = "America/North_Dakota/Beulah")]
    AmericaNorthDakotaBeulah,
    #[serde(rename = "America/North_Dakota/Center")]
    AmericaNorthDakotaCenter,
    #[serde(rename = "America/North_Dakota/New_Salem")]
    AmericaNorthDakotaNewSalem,
    #[serde(rename = "America/Nuuk")]
    AmericaNuuk,
    #[serde(rename = "America/Ojinaga")]
    AmericaOjinaga,
    #[serde(rename = "America/Panama")]
    AmericaPanama,
    #[serde(rename = "America/Pangnirtung")]
    AmericaPangnirtung,
    #[serde(rename = "America/Paramaribo")]
    AmericaParamaribo,
    #[serde(rename = "America/Phoenix")]
    AmericaPhoenix,
    #[serde(rename = "America/Port-au-Prince")]
    AmericaPortMinusauMinusPrince,
    #[serde(rename = "America/Port_of_Spain")]
    AmericaPortOfSpain,
    #[serde(rename = "America/Porto_Acre")]
    AmericaPortoAcre,
    #[serde(rename = "America/Porto_Velho")]
    AmericaPortoVelho,
    #[serde(rename = "America/Puerto_Rico")]
    AmericaPuertoRico,
    #[serde(rename = "America/Punta_Arenas")]
    AmericaPuntaArenas,
    #[serde(rename = "America/Rainy_River")]
    AmericaRainyRiver,
    #[serde(rename = "America/Rankin_Inlet")]
    AmericaRankinInlet,
    #[serde(rename = "America/Recife")]
    AmericaRecife,
    #[serde(rename = "America/Regina")]
    AmericaRegina,
    #[serde(rename = "America/Resolute")]
    AmericaResolute,
    #[serde(rename = "America/Rio_Branco")]
    AmericaRioBranco,
    #[serde(rename = "America/Rosario")]
    AmericaRosario,
    #[serde(rename = "America/Santa_Isabel")]
    AmericaSantaIsabel,
    #[serde(rename = "America/Santarem")]
    AmericaSantarem,
    #[serde(rename = "America/Santiago")]
    AmericaSantiago,
    #[serde(rename = "America/Santo_Domingo")]
    AmericaSantoDomingo,
    #[serde(rename = "America/Sao_Paulo")]
    AmericaSaoPaulo,
    #[serde(rename = "America/Scoresbysund")]
    AmericaScoresbysund,
    #[serde(rename = "America/Shiprock")]
    AmericaShiprock,
    #[serde(rename = "America/Sitka")]
    AmericaSitka,
    #[serde(rename = "America/St_Barthelemy")]
    AmericaStBarthelemy,
    #[serde(rename = "America/St_Johns")]
    AmericaStJohns,
    #[serde(rename = "America/St_Kitts")]
    AmericaStKitts,
    #[serde(rename = "America/St_Lucia")]
    AmericaStLucia,
    #[serde(rename = "America/St_Thomas")]
    AmericaStThomas,
    #[serde(rename = "America/St_Vincent")]
    AmericaStVincent,
    #[serde(rename = "America/Swift_Current")]
    AmericaSwiftCurrent,
    #[serde(rename = "America/Tegucigalpa")]
    AmericaTegucigalpa,
    #[serde(rename = "America/Thule")]
    AmericaThule,
    #[serde(rename = "America/Thunder_Bay")]
    AmericaThunderBay,
    #[serde(rename = "America/Tijuana")]
    AmericaTijuana,
    #[serde(rename = "America/Toronto")]
    AmericaToronto,
    #[serde(rename = "America/Tortola")]
    AmericaTortola,
    #[serde(rename = "America/Vancouver")]
    AmericaVancouver,
    #[serde(rename = "America/Virgin")]
    AmericaVirgin,
    #[serde(rename = "America/Whitehorse")]
    AmericaWhitehorse,
    #[serde(rename = "America/Winnipeg")]
    AmericaWinnipeg,
    #[serde(rename = "America/Yakutat")]
    AmericaYakutat,
    #[serde(rename = "America/Yellowknife")]
    AmericaYellowknife,
    #[serde(rename = "Antarctica/Casey")]
    AntarcticaCasey,
    #[serde(rename = "Antarctica/Davis")]
    AntarcticaDavis,
    #[serde(rename = "Antarctica/DumontDUrville")]
    AntarcticaDumontDUrville,
    #[serde(rename = "Antarctica/Macquarie")]
    AntarcticaMacquarie,
    #[serde(rename = "Antarctica/Mawson")]
    AntarcticaMawson,
    #[serde(rename = "Antarctica/McMurdo")]
    AntarcticaMcMurdo,
    #[serde(rename = "Antarctica/Palmer")]
    AntarcticaPalmer,
    #[serde(rename = "Antarctica/Rothera")]
    AntarcticaRothera,
    #[serde(rename = "Antarctica/South_Pole")]
    AntarcticaSouthPole,
    #[serde(rename = "Antarctica/Syowa")]
    AntarcticaSyowa,
    #[serde(rename = "Antarctica/Troll")]
    AntarcticaTroll,
    #[serde(rename = "Antarctica/Vostok")]
    AntarcticaVostok,
    #[serde(rename = "Arctic/Longyearbyen")]
    ArcticLongyearbyen,
    #[serde(rename = "Asia/Aden")]
    AsiaAden,
    #[serde(rename = "Asia/Almaty")]
    AsiaAlmaty,
    #[serde(rename = "Asia/Amman")]
    AsiaAmman,
    #[serde(rename = "Asia/Anadyr")]
    AsiaAnadyr,
    #[serde(rename = "Asia/Aqtau")]
    AsiaAqtau,
    #[serde(rename = "Asia/Aqtobe")]
    AsiaAqtobe,
    #[serde(rename = "Asia/Ashgabat")]
    AsiaAshgabat,
    #[serde(rename = "Asia/Ashkhabad")]
    AsiaAshkhabad,
    #[serde(rename = "Asia/Atyrau")]
    AsiaAtyrau,
    #[serde(rename = "Asia/Baghdad")]
    AsiaBaghdad,
    #[serde(rename = "Asia/Bahrain")]
    AsiaBahrain,
    #[serde(rename = "Asia/Baku")]
    AsiaBaku,
    #[serde(rename = "Asia/Bangkok")]
    AsiaBangkok,
    #[serde(rename = "Asia/Barnaul")]
    AsiaBarnaul,
    #[serde(rename = "Asia/Beirut")]
    AsiaBeirut,
    #[serde(rename = "Asia/Bishkek")]
    AsiaBishkek,
    #[serde(rename = "Asia/Brunei")]
    AsiaBrunei,
    #[serde(rename = "Asia/Calcutta")]
    AsiaCalcutta,
    #[serde(rename = "Asia/Chita")]
    AsiaChita,
    #[serde(rename = "Asia/Choibalsan")]
    AsiaChoibalsan,
    #[serde(rename = "Asia/Chongqing")]
    AsiaChongqing,
    #[serde(rename = "Asia/Chungking")]
    AsiaChungking,
    #[serde(rename = "Asia/Colombo")]
    AsiaColombo,
    #[serde(rename = "Asia/Dacca")]
    AsiaDacca,
    #[serde(rename = "Asia/Damascus")]
    AsiaDamascus,
    #[serde(rename = "Asia/Dhaka")]
    AsiaDhaka,
    #[serde(rename = "Asia/Dili")]
    AsiaDili,
    #[serde(rename = "Asia/Dubai")]
    AsiaDubai,
    #[serde(rename = "Asia/Dushanbe")]
    AsiaDushanbe,
    #[serde(rename = "Asia/Famagusta")]
    AsiaFamagusta,
    #[serde(rename = "Asia/Gaza")]
    AsiaGaza,
    #[serde(rename = "Asia/Harbin")]
    AsiaHarbin,
    #[serde(rename = "Asia/Hebron")]
    AsiaHebron,
    #[serde(rename = "Asia/Ho_Chi_Minh")]
    AsiaHoChiMinh,
    #[serde(rename = "Asia/Hong_Kong")]
    AsiaHongKong,
    #[serde(rename = "Asia/Hovd")]
    AsiaHovd,
    #[serde(rename = "Asia/Irkutsk")]
    AsiaIrkutsk,
    #[serde(rename = "Asia/Istanbul")]
    AsiaIstanbul,
    #[serde(rename = "Asia/Jakarta")]
    AsiaJakarta,
    #[serde(rename = "Asia/Jayapura")]
    AsiaJayapura,
    #[serde(rename = "Asia/Jerusalem")]
    AsiaJerusalem,
    #[serde(rename = "Asia/Kabul")]
    AsiaKabul,
    #[serde(rename = "Asia/Kamchatka")]
    AsiaKamchatka,
    #[serde(rename = "Asia/Karachi")]
    AsiaKarachi,
    #[serde(rename = "Asia/Kashgar")]
    AsiaKashgar,
    #[serde(rename = "Asia/Kathmandu")]
    AsiaKathmandu,
    #[serde(rename = "Asia/Katmandu")]
    AsiaKatmandu,
    #[serde(rename = "Asia/Khandyga")]
    AsiaKhandyga,
    #[serde(rename = "Asia/Kolkata")]
    AsiaKolkata,
    #[serde(rename = "Asia/Krasnoyarsk")]
    AsiaKrasnoyarsk,
    #[serde(rename = "Asia/Kuala_Lumpur")]
    AsiaKualaLumpur,
    #[serde(rename = "Asia/Kuching")]
    AsiaKuching,
    #[serde(rename = "Asia/Kuwait")]
    AsiaKuwait,
    #[serde(rename = "Asia/Macao")]
    AsiaMacao,
    #[serde(rename = "Asia/Macau")]
    AsiaMacau,
    #[serde(rename = "Asia/Magadan")]
    AsiaMagadan,
    #[serde(rename = "Asia/Makassar")]
    AsiaMakassar,
    #[serde(rename = "Asia/Manila")]
    AsiaManila,
    #[serde(rename = "Asia/Muscat")]
    AsiaMuscat,
    #[serde(rename = "Asia/Nicosia")]
    AsiaNicosia,
    #[serde(rename = "Asia/Novokuznetsk")]
    AsiaNovokuznetsk,
    #[serde(rename = "Asia/Novosibirsk")]
    AsiaNovosibirsk,
    #[serde(rename = "Asia/Omsk")]
    AsiaOmsk,
    #[serde(rename = "Asia/Oral")]
    AsiaOral,
    #[serde(rename = "Asia/Phnom_Penh")]
    AsiaPhnomPenh,
    #[serde(rename = "Asia/Pontianak")]
    AsiaPontianak,
    #[serde(rename = "Asia/Pyongyang")]
    AsiaPyongyang,
    #[serde(rename = "Asia/Qatar")]
    AsiaQatar,
    #[serde(rename = "Asia/Qostanay")]
    AsiaQostanay,
    #[serde(rename = "Asia/Qyzylorda")]
    AsiaQyzylorda,
    #[serde(rename = "Asia/Rangoon")]
    AsiaRangoon,
    #[serde(rename = "Asia/Riyadh")]
    AsiaRiyadh,
    #[serde(rename = "Asia/Saigon")]
    AsiaSaigon,
    #[serde(rename = "Asia/Sakhalin")]
    AsiaSakhalin,
    #[serde(rename = "Asia/Samarkand")]
    AsiaSamarkand,
    #[serde(rename = "Asia/Seoul")]
    AsiaSeoul,
    #[serde(rename = "Asia/Shanghai")]
    AsiaShanghai,
    #[serde(rename = "Asia/Singapore")]
    AsiaSingapore,
    #[serde(rename = "Asia/Srednekolymsk")]
    AsiaSrednekolymsk,
    #[serde(rename = "Asia/Taipei")]
    AsiaTaipei,
    #[serde(rename = "Asia/Tashkent")]
    AsiaTashkent,
    #[serde(rename = "Asia/Tbilisi")]
    AsiaTbilisi,
    #[serde(rename = "Asia/Tehran")]
    AsiaTehran,
    #[serde(rename = "Asia/Tel_Aviv")]
    AsiaTelAviv,
    #[serde(rename = "Asia/Thimbu")]
    AsiaThimbu,
    #[serde(rename = "Asia/Thimphu")]
    AsiaThimphu,
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
    #[serde(rename = "Asia/Tomsk")]
    AsiaTomsk,
    #[serde(rename = "Asia/Ujung_Pandang")]
    AsiaUjungPandang,
    #[serde(rename = "Asia/Ulaanbaatar")]
    AsiaUlaanbaatar,
    #[serde(rename = "Asia/Ulan_Bator")]
    AsiaUlanBator,
    #[serde(rename = "Asia/Urumqi")]
    AsiaUrumqi,
    #[serde(rename = "Asia/Ust-Nera")]
    AsiaUstMinusNera,
    #[serde(rename = "Asia/Vientiane")]
    AsiaVientiane,
    #[serde(rename = "Asia/Vladivostok")]
    AsiaVladivostok,
    #[serde(rename = "Asia/Yakutsk")]
    AsiaYakutsk,
    #[serde(rename = "Asia/Yangon")]
    AsiaYangon,
    #[serde(rename = "Asia/Yekaterinburg")]
    AsiaYekaterinburg,
    #[serde(rename = "Asia/Yerevan")]
    AsiaYerevan,
    #[serde(rename = "Atlantic/Azores")]
    AtlanticAzores,
    #[serde(rename = "Atlantic/Bermuda")]
    AtlanticBermuda,
    #[serde(rename = "Atlantic/Canary")]
    AtlanticCanary,
    #[serde(rename = "Atlantic/Cape_Verde")]
    AtlanticCapeVerde,
    #[serde(rename = "Atlantic/Faeroe")]
    AtlanticFaeroe,
    #[serde(rename = "Atlantic/Faroe")]
    AtlanticFaroe,
    #[serde(rename = "Atlantic/Jan_Mayen")]
    AtlanticJanMayen,
    #[serde(rename = "Atlantic/Madeira")]
    AtlanticMadeira,
    #[serde(rename = "Atlantic/Reykjavik")]
    AtlanticReykjavik,
    #[serde(rename = "Atlantic/South_Georgia")]
    AtlanticSouthGeorgia,
    #[serde(rename = "Atlantic/St_Helena")]
    AtlanticStHelena,
    #[serde(rename = "Atlantic/Stanley")]
    AtlanticStanley,
    #[serde(rename = "Australia/ACT")]
    AustraliaAct,
    #[serde(rename = "Australia/Adelaide")]
    AustraliaAdelaide,
    #[serde(rename = "Australia/Brisbane")]
    AustraliaBrisbane,
    #[serde(rename = "Australia/Broken_Hill")]
    AustraliaBrokenHill,
    #[serde(rename = "Australia/Canberra")]
    AustraliaCanberra,
    #[serde(rename = "Australia/Currie")]
    AustraliaCurrie,
    #[serde(rename = "Australia/Darwin")]
    AustraliaDarwin,
    #[serde(rename = "Australia/Eucla")]
    AustraliaEucla,
    #[serde(rename = "Australia/Hobart")]
    AustraliaHobart,
    #[serde(rename = "Australia/LHI")]
    AustraliaLhi,
    #[serde(rename = "Australia/Lindeman")]
    AustraliaLindeman,
    #[serde(rename = "Australia/Lord_Howe")]
    AustraliaLordHowe,
    #[serde(rename = "Australia/Melbourne")]
    AustraliaMelbourne,
    #[serde(rename = "Australia/NSW")]
    AustraliaNsw,
    #[serde(rename = "Australia/North")]
    AustraliaNorth,
    #[serde(rename = "Australia/Perth")]
    AustraliaPerth,
    #[serde(rename = "Australia/Queensland")]
    AustraliaQueensland,
    #[serde(rename = "Australia/South")]
    AustraliaSouth,
    #[serde(rename = "Australia/Sydney")]
    AustraliaSydney,
    #[serde(rename = "Australia/Tasmania")]
    AustraliaTasmania,
    #[serde(rename = "Australia/Victoria")]
    AustraliaVictoria,
    #[serde(rename = "Australia/West")]
    AustraliaWest,
    #[serde(rename = "Australia/Yancowinna")]
    AustraliaYancowinna,
    #[serde(rename = "Brazil/Acre")]
    BrazilAcre,
    #[serde(rename = "Brazil/DeNoronha")]
    BrazilDeNoronha,
    #[serde(rename = "Brazil/East")]
    BrazilEast,
    #[serde(rename = "Brazil/West")]
    BrazilWest,
    #[serde(rename = "CET")]
    Cet,
    #[serde(rename = "CST6CDT")]
    Cst6cdt,
    #[serde(rename = "Canada/Atlantic")]
    CanadaAtlantic,
    #[serde(rename = "Canada/Central")]
    CanadaCentral,
    #[serde(rename = "Canada/Eastern")]
    CanadaEastern,
    #[serde(rename = "Canada/Mountain")]
    CanadaMountain,
    #[serde(rename = "Canada/Newfoundland")]
    CanadaNewfoundland,
    #[serde(rename = "Canada/Pacific")]
    CanadaPacific,
    #[serde(rename = "Canada/Saskatchewan")]
    CanadaSaskatchewan,
    #[serde(rename = "Canada/Yukon")]
    CanadaYukon,
    #[serde(rename = "Chile/Continental")]
    ChileContinental,
    #[serde(rename = "Chile/EasterIsland")]
    ChileEasterIsland,
    #[serde(rename = "Cuba")]
    Cuba,
    #[serde(rename = "EET")]
    Eet,
    #[serde(rename = "EST")]
    Est,
    #[serde(rename = "EST5EDT")]
    Est5edt,
    #[serde(rename = "Egypt")]
    Egypt,
    #[serde(rename = "Eire")]
    Eire,
    #[serde(rename = "Etc/GMT")]
    EtcGmt,
    #[serde(rename = "Etc/GMT+0")]
    EtcGmtPlus0,
    #[serde(rename = "Etc/GMT+1")]
    EtcGmtPlus1,
    #[serde(rename = "Etc/GMT+10")]
    EtcGmtPlus10,
    #[serde(rename = "Etc/GMT+11")]
    EtcGmtPlus11,
    #[serde(rename = "Etc/GMT+12")]
    EtcGmtPlus12,
    #[serde(rename = "Etc/GMT+2")]
    EtcGmtPlus2,
    #[serde(rename = "Etc/GMT+3")]
    EtcGmtPlus3,
    #[serde(rename = "Etc/GMT+4")]
    EtcGmtPlus4,
    #[serde(rename = "Etc/GMT+5")]
    EtcGmtPlus5,
    #[serde(rename = "Etc/GMT+6")]
    EtcGmtPlus6,
    #[serde(rename = "Etc/GMT+7")]
    EtcGmtPlus7,
    #[serde(rename = "Etc/GMT+8")]
    EtcGmtPlus8,
    #[serde(rename = "Etc/GMT+9")]
    EtcGmtPlus9,
    #[serde(rename = "Etc/GMT-0")]
    EtcGmtMinus0,
    #[serde(rename = "Etc/GMT-1")]
    EtcGmtMinus1,
    #[serde(rename = "Etc/GMT-10")]
    EtcGmtMinus10,
    #[serde(rename = "Etc/GMT-11")]
    EtcGmtMinus11,
    #[serde(rename = "Etc/GMT-12")]
    EtcGmtMinus12,
    #[serde(rename = "Etc/GMT-13")]
    EtcGmtMinus13,
    #[serde(rename = "Etc/GMT-14")]
    EtcGmtMinus14,
    #[serde(rename = "Etc/GMT-2")]
    EtcGmtMinus2,
    #[serde(rename = "Etc/GMT-3")]
    EtcGmtMinus3,
    #[serde(rename = "Etc/GMT-4")]
    EtcGmtMinus4,
    #[serde(rename = "Etc/GMT-5")]
    EtcGmtMinus5,
    #[serde(rename = "Etc/GMT-6")]
    EtcGmtMinus6,
    #[serde(rename = "Etc/GMT-7")]
    EtcGmtMinus7,
    #[serde(rename = "Etc/GMT-8")]
    EtcGmtMinus8,
    #[serde(rename = "Etc/GMT-9")]
    EtcGmtMinus9,
    #[serde(rename = "Etc/GMT0")]
    EtcGmt0,
    #[serde(rename = "Etc/Greenwich")]
    EtcGreenwich,
    #[serde(rename = "Etc/UCT")]
    EtcUct,
    #[serde(rename = "Etc/UTC")]
    EtcUtc,
    #[serde(rename = "Etc/Universal")]
    EtcUniversal,
    #[serde(rename = "Etc/Zulu")]
    EtcZulu,
    #[serde(rename = "Europe/Amsterdam")]
    EuropeAmsterdam,
    #[serde(rename = "Europe/Andorra")]
    EuropeAndorra,
    #[serde(rename = "Europe/Astrakhan")]
    EuropeAstrakhan,
    #[serde(rename = "Europe/Athens")]
    EuropeAthens,
    #[serde(rename = "Europe/Belfast")]
    EuropeBelfast,
    #[serde(rename = "Europe/Belgrade")]
    EuropeBelgrade,
    #[serde(rename = "Europe/Berlin")]
    EuropeBerlin,
    #[serde(rename = "Europe/Bratislava")]
    EuropeBratislava,
    #[serde(rename = "Europe/Brussels")]
    EuropeBrussels,
    #[serde(rename = "Europe/Bucharest")]
    EuropeBucharest,
    #[serde(rename = "Europe/Budapest")]
    EuropeBudapest,
    #[serde(rename = "Europe/Busingen")]
    EuropeBusingen,
    #[serde(rename = "Europe/Chisinau")]
    EuropeChisinau,
    #[serde(rename = "Europe/Copenhagen")]
    EuropeCopenhagen,
    #[serde(rename = "Europe/Dublin")]
    EuropeDublin,
    #[serde(rename = "Europe/Gibraltar")]
    EuropeGibraltar,
    #[serde(rename = "Europe/Guernsey")]
    EuropeGuernsey,
    #[serde(rename = "Europe/Helsinki")]
    EuropeHelsinki,
    #[serde(rename = "Europe/Isle_of_Man")]
    EuropeIsleOfMan,
    #[serde(rename = "Europe/Istanbul")]
    EuropeIstanbul,
    #[serde(rename = "Europe/Jersey")]
    EuropeJersey,
    #[serde(rename = "Europe/Kaliningrad")]
    EuropeKaliningrad,
    #[serde(rename = "Europe/Kiev")]
    EuropeKiev,
    #[serde(rename = "Europe/Kirov")]
    EuropeKirov,
    #[serde(rename = "Europe/Kyiv")]
    EuropeKyiv,
    #[serde(rename = "Europe/Lisbon")]
    EuropeLisbon,
    #[serde(rename = "Europe/Ljubljana")]
    EuropeLjubljana,
    #[serde(rename = "Europe/London")]
    EuropeLondon,
    #[serde(rename = "Europe/Luxembourg")]
    EuropeLuxembourg,
    #[serde(rename = "Europe/Madrid")]
    EuropeMadrid,
    #[serde(rename = "Europe/Malta")]
    EuropeMalta,
    #[serde(rename = "Europe/Mariehamn")]
    EuropeMariehamn,
    #[serde(rename = "Europe/Minsk")]
    EuropeMinsk,
    #[serde(rename = "Europe/Monaco")]
    EuropeMonaco,
    #[serde(rename = "Europe/Moscow")]
    EuropeMoscow,
    #[serde(rename = "Europe/Nicosia")]
    EuropeNicosia,
    #[serde(rename = "Europe/Oslo")]
    EuropeOslo,
    #[serde(rename = "Europe/Paris")]
    EuropeParis,
    #[serde(rename = "Europe/Podgorica")]
    EuropePodgorica,
    #[serde(rename = "Europe/Prague")]
    EuropePrague,
    #[serde(rename = "Europe/Riga")]
    EuropeRiga,
    #[serde(rename = "Europe/Rome")]
    EuropeRome,
    #[serde(rename = "Europe/Samara")]
    EuropeSamara,
    #[serde(rename = "Europe/San_Marino")]
    EuropeSanMarino,
    #[serde(rename = "Europe/Sarajevo")]
    EuropeSarajevo,
    #[serde(rename = "Europe/Saratov")]
    EuropeSaratov,
    #[serde(rename = "Europe/Simferopol")]
    EuropeSimferopol,
    #[serde(rename = "Europe/Skopje")]
    EuropeSkopje,
    #[serde(rename = "Europe/Sofia")]
    EuropeSofia,
    #[serde(rename = "Europe/Stockholm")]
    EuropeStockholm,
    #[serde(rename = "Europe/Tallinn")]
    EuropeTallinn,
    #[serde(rename = "Europe/Tirane")]
    EuropeTirane,
    #[serde(rename = "Europe/Tiraspol")]
    EuropeTiraspol,
    #[serde(rename = "Europe/Ulyanovsk")]
    EuropeUlyanovsk,
    #[serde(rename = "Europe/Uzhgorod")]
    EuropeUzhgorod,
    #[serde(rename = "Europe/Vaduz")]
    EuropeVaduz,
    #[serde(rename = "Europe/Vatican")]
    EuropeVatican,
    #[serde(rename = "Europe/Vienna")]
    EuropeVienna,
    #[serde(rename = "Europe/Vilnius")]
    EuropeVilnius,
    #[serde(rename = "Europe/Volgograd")]
    EuropeVolgograd,
    #[serde(rename = "Europe/Warsaw")]
    EuropeWarsaw,
    #[serde(rename = "Europe/Zagreb")]
    EuropeZagreb,
    #[serde(rename = "Europe/Zaporozhye")]
    EuropeZaporozhye,
    #[serde(rename = "Europe/Zurich")]
    EuropeZurich,
    #[serde(rename = "Factory")]
    Factory,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GB-Eire")]
    GbMinusEire,
    #[serde(rename = "GMT")]
    Gmt,
    #[serde(rename = "GMT+0")]
    GmtPlus0,
    #[serde(rename = "GMT-0")]
    GmtMinus0,
    #[serde(rename = "GMT0")]
    Gmt0,
    #[serde(rename = "Greenwich")]
    Greenwich,
    #[serde(rename = "HST")]
    Hst,
    #[serde(rename = "Hongkong")]
    Hongkong,
    #[serde(rename = "Iceland")]
    Iceland,
    #[serde(rename = "Indian/Antananarivo")]
    IndianAntananarivo,
    #[serde(rename = "Indian/Chagos")]
    IndianChagos,
    #[serde(rename = "Indian/Christmas")]
    IndianChristmas,
    #[serde(rename = "Indian/Cocos")]
    IndianCocos,
    #[serde(rename = "Indian/Comoro")]
    IndianComoro,
    #[serde(rename = "Indian/Kerguelen")]
    IndianKerguelen,
    #[serde(rename = "Indian/Mahe")]
    IndianMahe,
    #[serde(rename = "Indian/Maldives")]
    IndianMaldives,
    #[serde(rename = "Indian/Mauritius")]
    IndianMauritius,
    #[serde(rename = "Indian/Mayotte")]
    IndianMayotte,
    #[serde(rename = "Indian/Reunion")]
    IndianReunion,
    #[serde(rename = "Iran")]
    Iran,
    #[serde(rename = "Israel")]
    Israel,
    #[serde(rename = "Jamaica")]
    Jamaica,
    #[serde(rename = "Japan")]
    Japan,
    #[serde(rename = "Kwajalein")]
    Kwajalein,
    #[serde(rename = "Libya")]
    Libya,
    #[serde(rename = "MET")]
    Met,
    #[serde(rename = "MST")]
    Mst,
    #[serde(rename = "MST7MDT")]
    Mst7mdt,
    #[serde(rename = "Mexico/BajaNorte")]
    MexicoBajaNorte,
    #[serde(rename = "Mexico/BajaSur")]
    MexicoBajaSur,
    #[serde(rename = "Mexico/General")]
    MexicoGeneral,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "NZ-CHAT")]
    NzMinusChat,
    #[serde(rename = "Navajo")]
    Navajo,
    #[serde(rename = "PRC")]
    Prc,
    #[serde(rename = "PST8PDT")]
    Pst8pdt,
    #[serde(rename = "Pacific/Apia")]
    PacificApia,
    #[serde(rename = "Pacific/Auckland")]
    PacificAuckland,
    #[serde(rename = "Pacific/Bougainville")]
    PacificBougainville,
    #[serde(rename = "Pacific/Chatham")]
    PacificChatham,
    #[serde(rename = "Pacific/Chuuk")]
    PacificChuuk,
    #[serde(rename = "Pacific/Easter")]
    PacificEaster,
    #[serde(rename = "Pacific/Efate")]
    PacificEfate,
    #[serde(rename = "Pacific/Enderbury")]
    PacificEnderbury,
    #[serde(rename = "Pacific/Fakaofo")]
    PacificFakaofo,
    #[serde(rename = "Pacific/Fiji")]
    PacificFiji,
    #[serde(rename = "Pacific/Funafuti")]
    PacificFunafuti,
    #[serde(rename = "Pacific/Galapagos")]
    PacificGalapagos,
    #[serde(rename = "Pacific/Gambier")]
    PacificGambier,
    #[serde(rename = "Pacific/Guadalcanal")]
    PacificGuadalcanal,
    #[serde(rename = "Pacific/Guam")]
    PacificGuam,
    #[serde(rename = "Pacific/Honolulu")]
    PacificHonolulu,
    #[serde(rename = "Pacific/Johnston")]
    PacificJohnston,
    #[serde(rename = "Pacific/Kanton")]
    PacificKanton,
    #[serde(rename = "Pacific/Kiritimati")]
    PacificKiritimati,
    #[serde(rename = "Pacific/Kosrae")]
    PacificKosrae,
    #[serde(rename = "Pacific/Kwajalein")]
    PacificKwajalein,
    #[serde(rename = "Pacific/Majuro")]
    PacificMajuro,
    #[serde(rename = "Pacific/Marquesas")]
    PacificMarquesas,
    #[serde(rename = "Pacific/Midway")]
    PacificMidway,
    #[serde(rename = "Pacific/Nauru")]
    PacificNauru,
    #[serde(rename = "Pacific/Niue")]
    PacificNiue,
    #[serde(rename = "Pacific/Norfolk")]
    PacificNorfolk,
    #[serde(rename = "Pacific/Noumea")]
    PacificNoumea,
    #[serde(rename = "Pacific/Pago_Pago")]
    PacificPagoPago,
    #[serde(rename = "Pacific/Palau")]
    PacificPalau,
    #[serde(rename = "Pacific/Pitcairn")]
    PacificPitcairn,
    #[serde(rename = "Pacific/Pohnpei")]
    PacificPohnpei,
    #[serde(rename = "Pacific/Ponape")]
    PacificPonape,
    #[serde(rename = "Pacific/Port_Moresby")]
    PacificPortMoresby,
    #[serde(rename = "Pacific/Rarotonga")]
    PacificRarotonga,
    #[serde(rename = "Pacific/Saipan")]
    PacificSaipan,
    #[serde(rename = "Pacific/Samoa")]
    PacificSamoa,
    #[serde(rename = "Pacific/Tahiti")]
    PacificTahiti,
    #[serde(rename = "Pacific/Tarawa")]
    PacificTarawa,
    #[serde(rename = "Pacific/Tongatapu")]
    PacificTongatapu,
    #[serde(rename = "Pacific/Truk")]
    PacificTruk,
    #[serde(rename = "Pacific/Wake")]
    PacificWake,
    #[serde(rename = "Pacific/Wallis")]
    PacificWallis,
    #[serde(rename = "Pacific/Yap")]
    PacificYap,
    #[serde(rename = "Poland")]
    Poland,
    #[serde(rename = "Portugal")]
    Portugal,
    #[serde(rename = "ROC")]
    Roc,
    #[serde(rename = "ROK")]
    Rok,
    #[serde(rename = "Singapore")]
    Singapore,
    #[serde(rename = "Turkey")]
    Turkey,
    #[serde(rename = "UCT")]
    Uct,
    #[serde(rename = "US/Alaska")]
    UsAlaska,
    #[serde(rename = "US/Aleutian")]
    UsAleutian,
    #[serde(rename = "US/Arizona")]
    UsArizona,
    #[serde(rename = "US/Central")]
    UsCentral,
    #[serde(rename = "US/East-Indiana")]
    UsEastMinusIndiana,
    #[serde(rename = "US/Eastern")]
    UsEastern,
    #[serde(rename = "US/Hawaii")]
    UsHawaii,
    #[serde(rename = "US/Indiana-Starke")]
    UsIndianaMinusStarke,
    #[serde(rename = "US/Michigan")]
    UsMichigan,
    #[serde(rename = "US/Mountain")]
    UsMountain,
    #[serde(rename = "US/Pacific")]
    UsPacific,
    #[serde(rename = "US/Pacific-New")]
    UsPacificMinusNew,
    #[serde(rename = "US/Samoa")]
    UsSamoa,
    #[serde(rename = "UTC")]
    Utc,
    #[serde(rename = "Universal")]
    Universal,
    #[serde(rename = "W-SU")]
    WMinusSu,
    #[serde(rename = "WET")]
    Wet,
    #[serde(rename = "Zulu")]
    Zulu,
}

impl CreateReportRunParametersTimezone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AfricaAbidjan => "Africa/Abidjan",
            Self::AfricaAccra => "Africa/Accra",
            Self::AfricaAddisAbaba => "Africa/Addis_Ababa",
            Self::AfricaAlgiers => "Africa/Algiers",
            Self::AfricaAsmara => "Africa/Asmara",
            Self::AfricaAsmera => "Africa/Asmera",
            Self::AfricaBamako => "Africa/Bamako",
            Self::AfricaBangui => "Africa/Bangui",
            Self::AfricaBanjul => "Africa/Banjul",
            Self::AfricaBissau => "Africa/Bissau",
            Self::AfricaBlantyre => "Africa/Blantyre",
            Self::AfricaBrazzaville => "Africa/Brazzaville",
            Self::AfricaBujumbura => "Africa/Bujumbura",
            Self::AfricaCairo => "Africa/Cairo",
            Self::AfricaCasablanca => "Africa/Casablanca",
            Self::AfricaCeuta => "Africa/Ceuta",
            Self::AfricaConakry => "Africa/Conakry",
            Self::AfricaDakar => "Africa/Dakar",
            Self::AfricaDarEsSalaam => "Africa/Dar_es_Salaam",
            Self::AfricaDjibouti => "Africa/Djibouti",
            Self::AfricaDouala => "Africa/Douala",
            Self::AfricaElAaiun => "Africa/El_Aaiun",
            Self::AfricaFreetown => "Africa/Freetown",
            Self::AfricaGaborone => "Africa/Gaborone",
            Self::AfricaHarare => "Africa/Harare",
            Self::AfricaJohannesburg => "Africa/Johannesburg",
            Self::AfricaJuba => "Africa/Juba",
            Self::AfricaKampala => "Africa/Kampala",
            Self::AfricaKhartoum => "Africa/Khartoum",
            Self::AfricaKigali => "Africa/Kigali",
            Self::AfricaKinshasa => "Africa/Kinshasa",
            Self::AfricaLagos => "Africa/Lagos",
            Self::AfricaLibreville => "Africa/Libreville",
            Self::AfricaLome => "Africa/Lome",
            Self::AfricaLuanda => "Africa/Luanda",
            Self::AfricaLubumbashi => "Africa/Lubumbashi",
            Self::AfricaLusaka => "Africa/Lusaka",
            Self::AfricaMalabo => "Africa/Malabo",
            Self::AfricaMaputo => "Africa/Maputo",
            Self::AfricaMaseru => "Africa/Maseru",
            Self::AfricaMbabane => "Africa/Mbabane",
            Self::AfricaMogadishu => "Africa/Mogadishu",
            Self::AfricaMonrovia => "Africa/Monrovia",
            Self::AfricaNairobi => "Africa/Nairobi",
            Self::AfricaNdjamena => "Africa/Ndjamena",
            Self::AfricaNiamey => "Africa/Niamey",
            Self::AfricaNouakchott => "Africa/Nouakchott",
            Self::AfricaOuagadougou => "Africa/Ouagadougou",
            Self::AfricaPortoMinusNovo => "Africa/Porto-Novo",
            Self::AfricaSaoTome => "Africa/Sao_Tome",
            Self::AfricaTimbuktu => "Africa/Timbuktu",
            Self::AfricaTripoli => "Africa/Tripoli",
            Self::AfricaTunis => "Africa/Tunis",
            Self::AfricaWindhoek => "Africa/Windhoek",
            Self::AmericaAdak => "America/Adak",
            Self::AmericaAnchorage => "America/Anchorage",
            Self::AmericaAnguilla => "America/Anguilla",
            Self::AmericaAntigua => "America/Antigua",
            Self::AmericaAraguaina => "America/Araguaina",
            Self::AmericaArgentinaBuenosAires => "America/Argentina/Buenos_Aires",
            Self::AmericaArgentinaCatamarca => "America/Argentina/Catamarca",
            Self::AmericaArgentinaComodRivadavia => "America/Argentina/ComodRivadavia",
            Self::AmericaArgentinaCordoba => "America/Argentina/Cordoba",
            Self::AmericaArgentinaJujuy => "America/Argentina/Jujuy",
            Self::AmericaArgentinaLaRioja => "America/Argentina/La_Rioja",
            Self::AmericaArgentinaMendoza => "America/Argentina/Mendoza",
            Self::AmericaArgentinaRioGallegos => "America/Argentina/Rio_Gallegos",
            Self::AmericaArgentinaSalta => "America/Argentina/Salta",
            Self::AmericaArgentinaSanJuan => "America/Argentina/San_Juan",
            Self::AmericaArgentinaSanLuis => "America/Argentina/San_Luis",
            Self::AmericaArgentinaTucuman => "America/Argentina/Tucuman",
            Self::AmericaArgentinaUshuaia => "America/Argentina/Ushuaia",
            Self::AmericaAruba => "America/Aruba",
            Self::AmericaAsuncion => "America/Asuncion",
            Self::AmericaAtikokan => "America/Atikokan",
            Self::AmericaAtka => "America/Atka",
            Self::AmericaBahia => "America/Bahia",
            Self::AmericaBahiaBanderas => "America/Bahia_Banderas",
            Self::AmericaBarbados => "America/Barbados",
            Self::AmericaBelem => "America/Belem",
            Self::AmericaBelize => "America/Belize",
            Self::AmericaBlancMinusSablon => "America/Blanc-Sablon",
            Self::AmericaBoaVista => "America/Boa_Vista",
            Self::AmericaBogota => "America/Bogota",
            Self::AmericaBoise => "America/Boise",
            Self::AmericaBuenosAires => "America/Buenos_Aires",
            Self::AmericaCambridgeBay => "America/Cambridge_Bay",
            Self::AmericaCampoGrande => "America/Campo_Grande",
            Self::AmericaCancun => "America/Cancun",
            Self::AmericaCaracas => "America/Caracas",
            Self::AmericaCatamarca => "America/Catamarca",
            Self::AmericaCayenne => "America/Cayenne",
            Self::AmericaCayman => "America/Cayman",
            Self::AmericaChicago => "America/Chicago",
            Self::AmericaChihuahua => "America/Chihuahua",
            Self::AmericaCoralHarbour => "America/Coral_Harbour",
            Self::AmericaCordoba => "America/Cordoba",
            Self::AmericaCostaRica => "America/Costa_Rica",
            Self::AmericaCreston => "America/Creston",
            Self::AmericaCuiaba => "America/Cuiaba",
            Self::AmericaCuracao => "America/Curacao",
            Self::AmericaDanmarkshavn => "America/Danmarkshavn",
            Self::AmericaDawson => "America/Dawson",
            Self::AmericaDawsonCreek => "America/Dawson_Creek",
            Self::AmericaDenver => "America/Denver",
            Self::AmericaDetroit => "America/Detroit",
            Self::AmericaDominica => "America/Dominica",
            Self::AmericaEdmonton => "America/Edmonton",
            Self::AmericaEirunepe => "America/Eirunepe",
            Self::AmericaElSalvador => "America/El_Salvador",
            Self::AmericaEnsenada => "America/Ensenada",
            Self::AmericaFortNelson => "America/Fort_Nelson",
            Self::AmericaFortWayne => "America/Fort_Wayne",
            Self::AmericaFortaleza => "America/Fortaleza",
            Self::AmericaGlaceBay => "America/Glace_Bay",
            Self::AmericaGodthab => "America/Godthab",
            Self::AmericaGooseBay => "America/Goose_Bay",
            Self::AmericaGrandTurk => "America/Grand_Turk",
            Self::AmericaGrenada => "America/Grenada",
            Self::AmericaGuadeloupe => "America/Guadeloupe",
            Self::AmericaGuatemala => "America/Guatemala",
            Self::AmericaGuayaquil => "America/Guayaquil",
            Self::AmericaGuyana => "America/Guyana",
            Self::AmericaHalifax => "America/Halifax",
            Self::AmericaHavana => "America/Havana",
            Self::AmericaHermosillo => "America/Hermosillo",
            Self::AmericaIndianaIndianapolis => "America/Indiana/Indianapolis",
            Self::AmericaIndianaKnox => "America/Indiana/Knox",
            Self::AmericaIndianaMarengo => "America/Indiana/Marengo",
            Self::AmericaIndianaPetersburg => "America/Indiana/Petersburg",
            Self::AmericaIndianaTellCity => "America/Indiana/Tell_City",
            Self::AmericaIndianaVevay => "America/Indiana/Vevay",
            Self::AmericaIndianaVincennes => "America/Indiana/Vincennes",
            Self::AmericaIndianaWinamac => "America/Indiana/Winamac",
            Self::AmericaIndianapolis => "America/Indianapolis",
            Self::AmericaInuvik => "America/Inuvik",
            Self::AmericaIqaluit => "America/Iqaluit",
            Self::AmericaJamaica => "America/Jamaica",
            Self::AmericaJujuy => "America/Jujuy",
            Self::AmericaJuneau => "America/Juneau",
            Self::AmericaKentuckyLouisville => "America/Kentucky/Louisville",
            Self::AmericaKentuckyMonticello => "America/Kentucky/Monticello",
            Self::AmericaKnoxIn => "America/Knox_IN",
            Self::AmericaKralendijk => "America/Kralendijk",
            Self::AmericaLaPaz => "America/La_Paz",
            Self::AmericaLima => "America/Lima",
            Self::AmericaLosAngeles => "America/Los_Angeles",
            Self::AmericaLouisville => "America/Louisville",
            Self::AmericaLowerPrinces => "America/Lower_Princes",
            Self::AmericaMaceio => "America/Maceio",
            Self::AmericaManagua => "America/Managua",
            Self::AmericaManaus => "America/Manaus",
            Self::AmericaMarigot => "America/Marigot",
            Self::AmericaMartinique => "America/Martinique",
            Self::AmericaMatamoros => "America/Matamoros",
            Self::AmericaMazatlan => "America/Mazatlan",
            Self::AmericaMendoza => "America/Mendoza",
            Self::AmericaMenominee => "America/Menominee",
            Self::AmericaMerida => "America/Merida",
            Self::AmericaMetlakatla => "America/Metlakatla",
            Self::AmericaMexicoCity => "America/Mexico_City",
            Self::AmericaMiquelon => "America/Miquelon",
            Self::AmericaMoncton => "America/Moncton",
            Self::AmericaMonterrey => "America/Monterrey",
            Self::AmericaMontevideo => "America/Montevideo",
            Self::AmericaMontreal => "America/Montreal",
            Self::AmericaMontserrat => "America/Montserrat",
            Self::AmericaNassau => "America/Nassau",
            Self::AmericaNewYork => "America/New_York",
            Self::AmericaNipigon => "America/Nipigon",
            Self::AmericaNome => "America/Nome",
            Self::AmericaNoronha => "America/Noronha",
            Self::AmericaNorthDakotaBeulah => "America/North_Dakota/Beulah",
            Self::AmericaNorthDakotaCenter => "America/North_Dakota/Center",
            Self::AmericaNorthDakotaNewSalem => "America/North_Dakota/New_Salem",
            Self::AmericaNuuk => "America/Nuuk",
            Self::AmericaOjinaga => "America/Ojinaga",
            Self::AmericaPanama => "America/Panama",
            Self::AmericaPangnirtung => "America/Pangnirtung",
            Self::AmericaParamaribo => "America/Paramaribo",
            Self::AmericaPhoenix => "America/Phoenix",
            Self::AmericaPortMinusauMinusPrince => "America/Port-au-Prince",
            Self::AmericaPortOfSpain => "America/Port_of_Spain",
            Self::AmericaPortoAcre => "America/Porto_Acre",
            Self::AmericaPortoVelho => "America/Porto_Velho",
            Self::AmericaPuertoRico => "America/Puerto_Rico",
            Self::AmericaPuntaArenas => "America/Punta_Arenas",
            Self::AmericaRainyRiver => "America/Rainy_River",
            Self::AmericaRankinInlet => "America/Rankin_Inlet",
            Self::AmericaRecife => "America/Recife",
            Self::AmericaRegina => "America/Regina",
            Self::AmericaResolute => "America/Resolute",
            Self::AmericaRioBranco => "America/Rio_Branco",
            Self::AmericaRosario => "America/Rosario",
            Self::AmericaSantaIsabel => "America/Santa_Isabel",
            Self::AmericaSantarem => "America/Santarem",
            Self::AmericaSantiago => "America/Santiago",
            Self::AmericaSantoDomingo => "America/Santo_Domingo",
            Self::AmericaSaoPaulo => "America/Sao_Paulo",
            Self::AmericaScoresbysund => "America/Scoresbysund",
            Self::AmericaShiprock => "America/Shiprock",
            Self::AmericaSitka => "America/Sitka",
            Self::AmericaStBarthelemy => "America/St_Barthelemy",
            Self::AmericaStJohns => "America/St_Johns",
            Self::AmericaStKitts => "America/St_Kitts",
            Self::AmericaStLucia => "America/St_Lucia",
            Self::AmericaStThomas => "America/St_Thomas",
            Self::AmericaStVincent => "America/St_Vincent",
            Self::AmericaSwiftCurrent => "America/Swift_Current",
            Self::AmericaTegucigalpa => "America/Tegucigalpa",
            Self::AmericaThule => "America/Thule",
            Self::AmericaThunderBay => "America/Thunder_Bay",
            Self::AmericaTijuana => "America/Tijuana",
            Self::AmericaToronto => "America/Toronto",
            Self::AmericaTortola => "America/Tortola",
            Self::AmericaVancouver => "America/Vancouver",
            Self::AmericaVirgin => "America/Virgin",
            Self::AmericaWhitehorse => "America/Whitehorse",
            Self::AmericaWinnipeg => "America/Winnipeg",
            Self::AmericaYakutat => "America/Yakutat",
            Self::AmericaYellowknife => "America/Yellowknife",
            Self::AntarcticaCasey => "Antarctica/Casey",
            Self::AntarcticaDavis => "Antarctica/Davis",
            Self::AntarcticaDumontDUrville => "Antarctica/DumontDUrville",
            Self::AntarcticaMacquarie => "Antarctica/Macquarie",
            Self::AntarcticaMawson => "Antarctica/Mawson",
            Self::AntarcticaMcMurdo => "Antarctica/McMurdo",
            Self::AntarcticaPalmer => "Antarctica/Palmer",
            Self::AntarcticaRothera => "Antarctica/Rothera",
            Self::AntarcticaSouthPole => "Antarctica/South_Pole",
            Self::AntarcticaSyowa => "Antarctica/Syowa",
            Self::AntarcticaTroll => "Antarctica/Troll",
            Self::AntarcticaVostok => "Antarctica/Vostok",
            Self::ArcticLongyearbyen => "Arctic/Longyearbyen",
            Self::AsiaAden => "Asia/Aden",
            Self::AsiaAlmaty => "Asia/Almaty",
            Self::AsiaAmman => "Asia/Amman",
            Self::AsiaAnadyr => "Asia/Anadyr",
            Self::AsiaAqtau => "Asia/Aqtau",
            Self::AsiaAqtobe => "Asia/Aqtobe",
            Self::AsiaAshgabat => "Asia/Ashgabat",
            Self::AsiaAshkhabad => "Asia/Ashkhabad",
            Self::AsiaAtyrau => "Asia/Atyrau",
            Self::AsiaBaghdad => "Asia/Baghdad",
            Self::AsiaBahrain => "Asia/Bahrain",
            Self::AsiaBaku => "Asia/Baku",
            Self::AsiaBangkok => "Asia/Bangkok",
            Self::AsiaBarnaul => "Asia/Barnaul",
            Self::AsiaBeirut => "Asia/Beirut",
            Self::AsiaBishkek => "Asia/Bishkek",
            Self::AsiaBrunei => "Asia/Brunei",
            Self::AsiaCalcutta => "Asia/Calcutta",
            Self::AsiaChita => "Asia/Chita",
            Self::AsiaChoibalsan => "Asia/Choibalsan",
            Self::AsiaChongqing => "Asia/Chongqing",
            Self::AsiaChungking => "Asia/Chungking",
            Self::AsiaColombo => "Asia/Colombo",
            Self::AsiaDacca => "Asia/Dacca",
            Self::AsiaDamascus => "Asia/Damascus",
            Self::AsiaDhaka => "Asia/Dhaka",
            Self::AsiaDili => "Asia/Dili",
            Self::AsiaDubai => "Asia/Dubai",
            Self::AsiaDushanbe => "Asia/Dushanbe",
            Self::AsiaFamagusta => "Asia/Famagusta",
            Self::AsiaGaza => "Asia/Gaza",
            Self::AsiaHarbin => "Asia/Harbin",
            Self::AsiaHebron => "Asia/Hebron",
            Self::AsiaHoChiMinh => "Asia/Ho_Chi_Minh",
            Self::AsiaHongKong => "Asia/Hong_Kong",
            Self::AsiaHovd => "Asia/Hovd",
            Self::AsiaIrkutsk => "Asia/Irkutsk",
            Self::AsiaIstanbul => "Asia/Istanbul",
            Self::AsiaJakarta => "Asia/Jakarta",
            Self::AsiaJayapura => "Asia/Jayapura",
            Self::AsiaJerusalem => "Asia/Jerusalem",
            Self::AsiaKabul => "Asia/Kabul",
            Self::AsiaKamchatka => "Asia/Kamchatka",
            Self::AsiaKarachi => "Asia/Karachi",
            Self::AsiaKashgar => "Asia/Kashgar",
            Self::AsiaKathmandu => "Asia/Kathmandu",
            Self::AsiaKatmandu => "Asia/Katmandu",
            Self::AsiaKhandyga => "Asia/Khandyga",
            Self::AsiaKolkata => "Asia/Kolkata",
            Self::AsiaKrasnoyarsk => "Asia/Krasnoyarsk",
            Self::AsiaKualaLumpur => "Asia/Kuala_Lumpur",
            Self::AsiaKuching => "Asia/Kuching",
            Self::AsiaKuwait => "Asia/Kuwait",
            Self::AsiaMacao => "Asia/Macao",
            Self::AsiaMacau => "Asia/Macau",
            Self::AsiaMagadan => "Asia/Magadan",
            Self::AsiaMakassar => "Asia/Makassar",
            Self::AsiaManila => "Asia/Manila",
            Self::AsiaMuscat => "Asia/Muscat",
            Self::AsiaNicosia => "Asia/Nicosia",
            Self::AsiaNovokuznetsk => "Asia/Novokuznetsk",
            Self::AsiaNovosibirsk => "Asia/Novosibirsk",
            Self::AsiaOmsk => "Asia/Omsk",
            Self::AsiaOral => "Asia/Oral",
            Self::AsiaPhnomPenh => "Asia/Phnom_Penh",
            Self::AsiaPontianak => "Asia/Pontianak",
            Self::AsiaPyongyang => "Asia/Pyongyang",
            Self::AsiaQatar => "Asia/Qatar",
            Self::AsiaQostanay => "Asia/Qostanay",
            Self::AsiaQyzylorda => "Asia/Qyzylorda",
            Self::AsiaRangoon => "Asia/Rangoon",
            Self::AsiaRiyadh => "Asia/Riyadh",
            Self::AsiaSaigon => "Asia/Saigon",
            Self::AsiaSakhalin => "Asia/Sakhalin",
            Self::AsiaSamarkand => "Asia/Samarkand",
            Self::AsiaSeoul => "Asia/Seoul",
            Self::AsiaShanghai => "Asia/Shanghai",
            Self::AsiaSingapore => "Asia/Singapore",
            Self::AsiaSrednekolymsk => "Asia/Srednekolymsk",
            Self::AsiaTaipei => "Asia/Taipei",
            Self::AsiaTashkent => "Asia/Tashkent",
            Self::AsiaTbilisi => "Asia/Tbilisi",
            Self::AsiaTehran => "Asia/Tehran",
            Self::AsiaTelAviv => "Asia/Tel_Aviv",
            Self::AsiaThimbu => "Asia/Thimbu",
            Self::AsiaThimphu => "Asia/Thimphu",
            Self::AsiaTokyo => "Asia/Tokyo",
            Self::AsiaTomsk => "Asia/Tomsk",
            Self::AsiaUjungPandang => "Asia/Ujung_Pandang",
            Self::AsiaUlaanbaatar => "Asia/Ulaanbaatar",
            Self::AsiaUlanBator => "Asia/Ulan_Bator",
            Self::AsiaUrumqi => "Asia/Urumqi",
            Self::AsiaUstMinusNera => "Asia/Ust-Nera",
            Self::AsiaVientiane => "Asia/Vientiane",
            Self::AsiaVladivostok => "Asia/Vladivostok",
            Self::AsiaYakutsk => "Asia/Yakutsk",
            Self::AsiaYangon => "Asia/Yangon",
            Self::AsiaYekaterinburg => "Asia/Yekaterinburg",
            Self::AsiaYerevan => "Asia/Yerevan",
            Self::AtlanticAzores => "Atlantic/Azores",
            Self::AtlanticBermuda => "Atlantic/Bermuda",
            Self::AtlanticCanary => "Atlantic/Canary",
            Self::AtlanticCapeVerde => "Atlantic/Cape_Verde",
            Self::AtlanticFaeroe => "Atlantic/Faeroe",
            Self::AtlanticFaroe => "Atlantic/Faroe",
            Self::AtlanticJanMayen => "Atlantic/Jan_Mayen",
            Self::AtlanticMadeira => "Atlantic/Madeira",
            Self::AtlanticReykjavik => "Atlantic/Reykjavik",
            Self::AtlanticSouthGeorgia => "Atlantic/South_Georgia",
            Self::AtlanticStHelena => "Atlantic/St_Helena",
            Self::AtlanticStanley => "Atlantic/Stanley",
            Self::AustraliaAct => "Australia/ACT",
            Self::AustraliaAdelaide => "Australia/Adelaide",
            Self::AustraliaBrisbane => "Australia/Brisbane",
            Self::AustraliaBrokenHill => "Australia/Broken_Hill",
            Self::AustraliaCanberra => "Australia/Canberra",
            Self::AustraliaCurrie => "Australia/Currie",
            Self::AustraliaDarwin => "Australia/Darwin",
            Self::AustraliaEucla => "Australia/Eucla",
            Self::AustraliaHobart => "Australia/Hobart",
            Self::AustraliaLhi => "Australia/LHI",
            Self::AustraliaLindeman => "Australia/Lindeman",
            Self::AustraliaLordHowe => "Australia/Lord_Howe",
            Self::AustraliaMelbourne => "Australia/Melbourne",
            Self::AustraliaNsw => "Australia/NSW",
            Self::AustraliaNorth => "Australia/North",
            Self::AustraliaPerth => "Australia/Perth",
            Self::AustraliaQueensland => "Australia/Queensland",
            Self::AustraliaSouth => "Australia/South",
            Self::AustraliaSydney => "Australia/Sydney",
            Self::AustraliaTasmania => "Australia/Tasmania",
            Self::AustraliaVictoria => "Australia/Victoria",
            Self::AustraliaWest => "Australia/West",
            Self::AustraliaYancowinna => "Australia/Yancowinna",
            Self::BrazilAcre => "Brazil/Acre",
            Self::BrazilDeNoronha => "Brazil/DeNoronha",
            Self::BrazilEast => "Brazil/East",
            Self::BrazilWest => "Brazil/West",
            Self::Cet => "CET",
            Self::Cst6cdt => "CST6CDT",
            Self::CanadaAtlantic => "Canada/Atlantic",
            Self::CanadaCentral => "Canada/Central",
            Self::CanadaEastern => "Canada/Eastern",
            Self::CanadaMountain => "Canada/Mountain",
            Self::CanadaNewfoundland => "Canada/Newfoundland",
            Self::CanadaPacific => "Canada/Pacific",
            Self::CanadaSaskatchewan => "Canada/Saskatchewan",
            Self::CanadaYukon => "Canada/Yukon",
            Self::ChileContinental => "Chile/Continental",
            Self::ChileEasterIsland => "Chile/EasterIsland",
            Self::Cuba => "Cuba",
            Self::Eet => "EET",
            Self::Est => "EST",
            Self::Est5edt => "EST5EDT",
            Self::Egypt => "Egypt",
            Self::Eire => "Eire",
            Self::EtcGmt => "Etc/GMT",
            Self::EtcGmtPlus0 => "Etc/GMT+0",
            Self::EtcGmtPlus1 => "Etc/GMT+1",
            Self::EtcGmtPlus10 => "Etc/GMT+10",
            Self::EtcGmtPlus11 => "Etc/GMT+11",
            Self::EtcGmtPlus12 => "Etc/GMT+12",
            Self::EtcGmtPlus2 => "Etc/GMT+2",
            Self::EtcGmtPlus3 => "Etc/GMT+3",
            Self::EtcGmtPlus4 => "Etc/GMT+4",
            Self::EtcGmtPlus5 => "Etc/GMT+5",
            Self::EtcGmtPlus6 => "Etc/GMT+6",
            Self::EtcGmtPlus7 => "Etc/GMT+7",
            Self::EtcGmtPlus8 => "Etc/GMT+8",
            Self::EtcGmtPlus9 => "Etc/GMT+9",
            Self::EtcGmtMinus0 => "Etc/GMT-0",
            Self::EtcGmtMinus1 => "Etc/GMT-1",
            Self::EtcGmtMinus10 => "Etc/GMT-10",
            Self::EtcGmtMinus11 => "Etc/GMT-11",
            Self::EtcGmtMinus12 => "Etc/GMT-12",
            Self::EtcGmtMinus13 => "Etc/GMT-13",
            Self::EtcGmtMinus14 => "Etc/GMT-14",
            Self::EtcGmtMinus2 => "Etc/GMT-2",
            Self::EtcGmtMinus3 => "Etc/GMT-3",
            Self::EtcGmtMinus4 => "Etc/GMT-4",
            Self::EtcGmtMinus5 => "Etc/GMT-5",
            Self::EtcGmtMinus6 => "Etc/GMT-6",
            Self::EtcGmtMinus7 => "Etc/GMT-7",
            Self::EtcGmtMinus8 => "Etc/GMT-8",
            Self::EtcGmtMinus9 => "Etc/GMT-9",
            Self::EtcGmt0 => "Etc/GMT0",
            Self::EtcGreenwich => "Etc/Greenwich",
            Self::EtcUct => "Etc/UCT",
            Self::EtcUtc => "Etc/UTC",
            Self::EtcUniversal => "Etc/Universal",
            Self::EtcZulu => "Etc/Zulu",
            Self::EuropeAmsterdam => "Europe/Amsterdam",
            Self::EuropeAndorra => "Europe/Andorra",
            Self::EuropeAstrakhan => "Europe/Astrakhan",
            Self::EuropeAthens => "Europe/Athens",
            Self::EuropeBelfast => "Europe/Belfast",
            Self::EuropeBelgrade => "Europe/Belgrade",
            Self::EuropeBerlin => "Europe/Berlin",
            Self::EuropeBratislava => "Europe/Bratislava",
            Self::EuropeBrussels => "Europe/Brussels",
            Self::EuropeBucharest => "Europe/Bucharest",
            Self::EuropeBudapest => "Europe/Budapest",
            Self::EuropeBusingen => "Europe/Busingen",
            Self::EuropeChisinau => "Europe/Chisinau",
            Self::EuropeCopenhagen => "Europe/Copenhagen",
            Self::EuropeDublin => "Europe/Dublin",
            Self::EuropeGibraltar => "Europe/Gibraltar",
            Self::EuropeGuernsey => "Europe/Guernsey",
            Self::EuropeHelsinki => "Europe/Helsinki",
            Self::EuropeIsleOfMan => "Europe/Isle_of_Man",
            Self::EuropeIstanbul => "Europe/Istanbul",
            Self::EuropeJersey => "Europe/Jersey",
            Self::EuropeKaliningrad => "Europe/Kaliningrad",
            Self::EuropeKiev => "Europe/Kiev",
            Self::EuropeKirov => "Europe/Kirov",
            Self::EuropeKyiv => "Europe/Kyiv",
            Self::EuropeLisbon => "Europe/Lisbon",
            Self::EuropeLjubljana => "Europe/Ljubljana",
            Self::EuropeLondon => "Europe/London",
            Self::EuropeLuxembourg => "Europe/Luxembourg",
            Self::EuropeMadrid => "Europe/Madrid",
            Self::EuropeMalta => "Europe/Malta",
            Self::EuropeMariehamn => "Europe/Mariehamn",
            Self::EuropeMinsk => "Europe/Minsk",
            Self::EuropeMonaco => "Europe/Monaco",
            Self::EuropeMoscow => "Europe/Moscow",
            Self::EuropeNicosia => "Europe/Nicosia",
            Self::EuropeOslo => "Europe/Oslo",
            Self::EuropeParis => "Europe/Paris",
            Self::EuropePodgorica => "Europe/Podgorica",
            Self::EuropePrague => "Europe/Prague",
            Self::EuropeRiga => "Europe/Riga",
            Self::EuropeRome => "Europe/Rome",
            Self::EuropeSamara => "Europe/Samara",
            Self::EuropeSanMarino => "Europe/San_Marino",
            Self::EuropeSarajevo => "Europe/Sarajevo",
            Self::EuropeSaratov => "Europe/Saratov",
            Self::EuropeSimferopol => "Europe/Simferopol",
            Self::EuropeSkopje => "Europe/Skopje",
            Self::EuropeSofia => "Europe/Sofia",
            Self::EuropeStockholm => "Europe/Stockholm",
            Self::EuropeTallinn => "Europe/Tallinn",
            Self::EuropeTirane => "Europe/Tirane",
            Self::EuropeTiraspol => "Europe/Tiraspol",
            Self::EuropeUlyanovsk => "Europe/Ulyanovsk",
            Self::EuropeUzhgorod => "Europe/Uzhgorod",
            Self::EuropeVaduz => "Europe/Vaduz",
            Self::EuropeVatican => "Europe/Vatican",
            Self::EuropeVienna => "Europe/Vienna",
            Self::EuropeVilnius => "Europe/Vilnius",
            Self::EuropeVolgograd => "Europe/Volgograd",
            Self::EuropeWarsaw => "Europe/Warsaw",
            Self::EuropeZagreb => "Europe/Zagreb",
            Self::EuropeZaporozhye => "Europe/Zaporozhye",
            Self::EuropeZurich => "Europe/Zurich",
            Self::Factory => "Factory",
            Self::Gb => "GB",
            Self::GbMinusEire => "GB-Eire",
            Self::Gmt => "GMT",
            Self::GmtPlus0 => "GMT+0",
            Self::GmtMinus0 => "GMT-0",
            Self::Gmt0 => "GMT0",
            Self::Greenwich => "Greenwich",
            Self::Hst => "HST",
            Self::Hongkong => "Hongkong",
            Self::Iceland => "Iceland",
            Self::IndianAntananarivo => "Indian/Antananarivo",
            Self::IndianChagos => "Indian/Chagos",
            Self::IndianChristmas => "Indian/Christmas",
            Self::IndianCocos => "Indian/Cocos",
            Self::IndianComoro => "Indian/Comoro",
            Self::IndianKerguelen => "Indian/Kerguelen",
            Self::IndianMahe => "Indian/Mahe",
            Self::IndianMaldives => "Indian/Maldives",
            Self::IndianMauritius => "Indian/Mauritius",
            Self::IndianMayotte => "Indian/Mayotte",
            Self::IndianReunion => "Indian/Reunion",
            Self::Iran => "Iran",
            Self::Israel => "Israel",
            Self::Jamaica => "Jamaica",
            Self::Japan => "Japan",
            Self::Kwajalein => "Kwajalein",
            Self::Libya => "Libya",
            Self::Met => "MET",
            Self::Mst => "MST",
            Self::Mst7mdt => "MST7MDT",
            Self::MexicoBajaNorte => "Mexico/BajaNorte",
            Self::MexicoBajaSur => "Mexico/BajaSur",
            Self::MexicoGeneral => "Mexico/General",
            Self::Nz => "NZ",
            Self::NzMinusChat => "NZ-CHAT",
            Self::Navajo => "Navajo",
            Self::Prc => "PRC",
            Self::Pst8pdt => "PST8PDT",
            Self::PacificApia => "Pacific/Apia",
            Self::PacificAuckland => "Pacific/Auckland",
            Self::PacificBougainville => "Pacific/Bougainville",
            Self::PacificChatham => "Pacific/Chatham",
            Self::PacificChuuk => "Pacific/Chuuk",
            Self::PacificEaster => "Pacific/Easter",
            Self::PacificEfate => "Pacific/Efate",
            Self::PacificEnderbury => "Pacific/Enderbury",
            Self::PacificFakaofo => "Pacific/Fakaofo",
            Self::PacificFiji => "Pacific/Fiji",
            Self::PacificFunafuti => "Pacific/Funafuti",
            Self::PacificGalapagos => "Pacific/Galapagos",
            Self::PacificGambier => "Pacific/Gambier",
            Self::PacificGuadalcanal => "Pacific/Guadalcanal",
            Self::PacificGuam => "Pacific/Guam",
            Self::PacificHonolulu => "Pacific/Honolulu",
            Self::PacificJohnston => "Pacific/Johnston",
            Self::PacificKanton => "Pacific/Kanton",
            Self::PacificKiritimati => "Pacific/Kiritimati",
            Self::PacificKosrae => "Pacific/Kosrae",
            Self::PacificKwajalein => "Pacific/Kwajalein",
            Self::PacificMajuro => "Pacific/Majuro",
            Self::PacificMarquesas => "Pacific/Marquesas",
            Self::PacificMidway => "Pacific/Midway",
            Self::PacificNauru => "Pacific/Nauru",
            Self::PacificNiue => "Pacific/Niue",
            Self::PacificNorfolk => "Pacific/Norfolk",
            Self::PacificNoumea => "Pacific/Noumea",
            Self::PacificPagoPago => "Pacific/Pago_Pago",
            Self::PacificPalau => "Pacific/Palau",
            Self::PacificPitcairn => "Pacific/Pitcairn",
            Self::PacificPohnpei => "Pacific/Pohnpei",
            Self::PacificPonape => "Pacific/Ponape",
            Self::PacificPortMoresby => "Pacific/Port_Moresby",
            Self::PacificRarotonga => "Pacific/Rarotonga",
            Self::PacificSaipan => "Pacific/Saipan",
            Self::PacificSamoa => "Pacific/Samoa",
            Self::PacificTahiti => "Pacific/Tahiti",
            Self::PacificTarawa => "Pacific/Tarawa",
            Self::PacificTongatapu => "Pacific/Tongatapu",
            Self::PacificTruk => "Pacific/Truk",
            Self::PacificWake => "Pacific/Wake",
            Self::PacificWallis => "Pacific/Wallis",
            Self::PacificYap => "Pacific/Yap",
            Self::Poland => "Poland",
            Self::Portugal => "Portugal",
            Self::Roc => "ROC",
            Self::Rok => "ROK",
            Self::Singapore => "Singapore",
            Self::Turkey => "Turkey",
            Self::Uct => "UCT",
            Self::UsAlaska => "US/Alaska",
            Self::UsAleutian => "US/Aleutian",
            Self::UsArizona => "US/Arizona",
            Self::UsCentral => "US/Central",
            Self::UsEastMinusIndiana => "US/East-Indiana",
            Self::UsEastern => "US/Eastern",
            Self::UsHawaii => "US/Hawaii",
            Self::UsIndianaMinusStarke => "US/Indiana-Starke",
            Self::UsMichigan => "US/Michigan",
            Self::UsMountain => "US/Mountain",
            Self::UsPacific => "US/Pacific",
            Self::UsPacificMinusNew => "US/Pacific-New",
            Self::UsSamoa => "US/Samoa",
            Self::Utc => "UTC",
            Self::Universal => "Universal",
            Self::WMinusSu => "W-SU",
            Self::Wet => "WET",
            Self::Zulu => "Zulu",
        }
    }
}

impl AsRef<str> for CreateReportRunParametersTimezone {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateReportRunParametersTimezone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListReportRun<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
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
    pub starting_after: Option<String>,
}
impl<'a> ListReportRun<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
