use stripe::{Client, Response};

impl stripe_core::issuing::card::Card {
    /// Returns a list of Issuing `Card` objects.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn list(
        client: &Client,
        params: ListCard,
    ) -> Response<stripe_types::List<stripe_core::issuing::card::Card>> {
        client.get_query("/issuing/cards", params)
    }
    /// Creates an Issuing `Card` object.
    pub fn create(
        client: &Client,
        params: CreateCard,
    ) -> Response<stripe_core::issuing::card::Card> {
        client.send_form("/issuing/cards", params, http_types::Method::Post)
    }
    /// Retrieves an Issuing `Card` object.
    pub fn retrieve(
        client: &Client,
        card: &stripe_core::card::CardId,
        params: RetrieveCard,
    ) -> Response<stripe_core::issuing::card::Card> {
        client.get_query(&format!("/issuing/cards/{card}", card = card), params)
    }
    /// Updates the specified Issuing `Card` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn update(
        client: &Client,
        card: &stripe_core::card::CardId,
        params: UpdateCard,
    ) -> Response<stripe_core::issuing::card::Card> {
        client.send_form(
            &format!("/issuing/cards/{card}", card = card),
            params,
            http_types::Method::Post,
        )
    }
    /// Updates the shipping status of the specified Issuing `Card` object to `delivered`.
    pub fn deliver_card(
        client: &Client,
        card: &stripe_core::card::CardId,
        params: DeliverCardCard,
    ) -> Response<stripe_core::issuing::card::Card> {
        client.send_form(
            &format!("/test_helpers/issuing/cards/{card}/shipping/deliver", card = card),
            params,
            http_types::Method::Post,
        )
    }
    /// Updates the shipping status of the specified Issuing `Card` object to `shipped`.
    pub fn ship_card(
        client: &Client,
        card: &stripe_core::card::CardId,
        params: ShipCardCard,
    ) -> Response<stripe_core::issuing::card::Card> {
        client.send_form(
            &format!("/test_helpers/issuing/cards/{card}/shipping/ship", card = card),
            params,
            http_types::Method::Post,
        )
    }
    /// Updates the shipping status of the specified Issuing `Card` object to `returned`.
    pub fn return_card(
        client: &Client,
        card: &stripe_core::card::CardId,
        params: ReturnCardCard,
    ) -> Response<stripe_core::issuing::card::Card> {
        client.send_form(
            &format!("/test_helpers/issuing/cards/{card}/shipping/return", card = card),
            params,
            http_types::Method::Post,
        )
    }
    /// Updates the shipping status of the specified Issuing `Card` object to `failure`.
    pub fn fail_card(
        client: &Client,
        card: &stripe_core::card::CardId,
        params: FailCardCard,
    ) -> Response<stripe_core::issuing::card::Card> {
        client.send_form(
            &format!("/test_helpers/issuing/cards/{card}/shipping/fail", card = card),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListCard<'a> {
    /// Only return cards belonging to the Cardholder with the provided ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    /// Only return cards that were issued during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Only return cards that have the given expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    /// Only return cards that have the given expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Only return cards that have the given last four digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<&'a str>,
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
    /// Only return cards that have the given status.
    ///
    /// One of `active`, `inactive`, or `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListCardStatus>,
    /// Only return cards that have the given type.
    ///
    /// One of `virtual` or `physical`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ListCardType>,
}
impl<'a> ListCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return cards that have the given status.
///
/// One of `active`, `inactive`, or `canceled`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ListCardStatus {
    Active,
    Canceled,
    Inactive,
}

impl ListCardStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for ListCardStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "canceled" => Ok(Self::Canceled),
            "inactive" => Ok(Self::Inactive),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListCardStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ListCardStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Only return cards that have the given type.
///
/// One of `virtual` or `physical`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ListCardType {
    Physical,
    Virtual,
}

impl ListCardType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Virtual => "virtual",
        }
    }
}

impl std::str::FromStr for ListCardType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "physical" => Ok(Self::Physical),
            "virtual" => Ok(Self::Virtual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListCardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ListCardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCard<'a> {
    /// The [Cardholder](https://stripe.com/docs/api#issuing_cardholder_object) object with which the card will be associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    /// The currency for the card.
    pub currency: stripe_types::Currency,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The card this is meant to be a replacement for (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<&'a str>,
    /// If `replacement_for` is specified, this should indicate why that card is being replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<CreateCardReplacementReason>,
    /// The address where the card will be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CreateCardShipping<'a>>,
    /// Rules that control spending for this card.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<CreateCardSpendingControls<'a>>,
    /// Whether authorizations can be approved on this card.
    ///
    /// Defaults to `inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateCardStatus>,
    /// The type of card to issue.
    ///
    /// Possible values are `physical` or `virtual`.
    #[serde(rename = "type")]
    pub type_: CreateCardType,
}
impl<'a> CreateCard<'a> {
    pub fn new(currency: stripe_types::Currency, type_: CreateCardType) -> Self {
        Self {
            cardholder: Default::default(),
            currency,
            expand: Default::default(),
            financial_account: Default::default(),
            metadata: Default::default(),
            replacement_for: Default::default(),
            replacement_reason: Default::default(),
            shipping: Default::default(),
            spending_controls: Default::default(),
            status: Default::default(),
            type_,
        }
    }
}
/// If `replacement_for` is specified, this should indicate why that card is being replaced.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl CreateCardReplacementReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Damaged => "damaged",
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for CreateCardReplacementReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "damaged" => Ok(Self::Damaged),
            "expired" => Ok(Self::Expired),
            "lost" => Ok(Self::Lost),
            "stolen" => Ok(Self::Stolen),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardReplacementReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardReplacementReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The address where the card will be shipped.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCardShipping<'a> {
    /// The address that the card is shipped to.
    pub address: CreateCardShippingAddress<'a>,
    /// Customs information for the shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs: Option<CreateCardShippingCustoms<'a>>,
    /// The name printed on the shipping label when shipping the card.
    pub name: &'a str,
    /// Phone number of the recipient of the shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<&'a str>,
    /// Whether a signature is required for card delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_signature: Option<bool>,
    /// Shipment service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<CreateCardShippingService>,
    /// Packaging options.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateCardShippingType>,
}
impl<'a> CreateCardShipping<'a> {
    pub fn new(address: CreateCardShippingAddress<'a>, name: &'a str) -> Self {
        Self {
            address,
            customs: Default::default(),
            name,
            phone_number: Default::default(),
            require_signature: Default::default(),
            service: Default::default(),
            type_: Default::default(),
        }
    }
}
/// The address that the card is shipped to.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCardShippingAddress<'a> {
    /// City, district, suburb, town, or village.
    pub city: &'a str,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: &'a str,
    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: &'a str,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<&'a str>,
    /// ZIP or postal code.
    pub postal_code: &'a str,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
impl<'a> CreateCardShippingAddress<'a> {
    pub fn new(city: &'a str, country: &'a str, line1: &'a str, postal_code: &'a str) -> Self {
        Self {
            city,
            country,
            line1,
            line2: Default::default(),
            postal_code,
            state: Default::default(),
        }
    }
}
/// Customs information for the shipment.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCardShippingCustoms<'a> {
    /// The Economic Operators Registration and Identification (EORI) number to use for Customs.
    ///
    /// Required for bulk shipments to Europe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori_number: Option<&'a str>,
}
impl<'a> CreateCardShippingCustoms<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Shipment service.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardShippingService {
    Express,
    Priority,
    Standard,
}

impl CreateCardShippingService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Express => "express",
            Self::Priority => "priority",
            Self::Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateCardShippingService {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "express" => Ok(Self::Express),
            "priority" => Ok(Self::Priority),
            "standard" => Ok(Self::Standard),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardShippingService {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Packaging options.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardShippingType {
    Bulk,
    Individual,
}

impl CreateCardShippingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bulk => "bulk",
            Self::Individual => "individual",
        }
    }
}

impl std::str::FromStr for CreateCardShippingType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bulk" => Ok(Self::Bulk),
            "individual" => Ok(Self::Individual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardShippingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardShippingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Rules that control spending for this card.
///
/// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateCardSpendingControls<'a> {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    ///
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<&'a [CreateCardSpendingControlsAllowedCategories]>,
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    ///
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<&'a [CreateCardSpendingControlsBlockedCategories]>,
    /// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<&'a [CreateCardSpendingControlsSpendingLimits<'a>]>,
}
impl<'a> CreateCardSpendingControls<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
///
/// All other categories will be blocked.
/// Cannot be set with `blocked_categories`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardSpendingControlsAllowedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl CreateCardSpendingControlsAllowedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl std::str::FromStr for CreateCardSpendingControlsAllowedCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ac_refrigeration_repair" => Ok(Self::AcRefrigerationRepair),
            "accounting_bookkeeping_services" => Ok(Self::AccountingBookkeepingServices),
            "advertising_services" => Ok(Self::AdvertisingServices),
            "agricultural_cooperative" => Ok(Self::AgriculturalCooperative),
            "airlines_air_carriers" => Ok(Self::AirlinesAirCarriers),
            "airports_flying_fields" => Ok(Self::AirportsFlyingFields),
            "ambulance_services" => Ok(Self::AmbulanceServices),
            "amusement_parks_carnivals" => Ok(Self::AmusementParksCarnivals),
            "antique_reproductions" => Ok(Self::AntiqueReproductions),
            "antique_shops" => Ok(Self::AntiqueShops),
            "aquariums" => Ok(Self::Aquariums),
            "architectural_surveying_services" => Ok(Self::ArchitecturalSurveyingServices),
            "art_dealers_and_galleries" => Ok(Self::ArtDealersAndGalleries),
            "artists_supply_and_craft_shops" => Ok(Self::ArtistsSupplyAndCraftShops),
            "auto_and_home_supply_stores" => Ok(Self::AutoAndHomeSupplyStores),
            "auto_body_repair_shops" => Ok(Self::AutoBodyRepairShops),
            "auto_paint_shops" => Ok(Self::AutoPaintShops),
            "auto_service_shops" => Ok(Self::AutoServiceShops),
            "automated_cash_disburse" => Ok(Self::AutomatedCashDisburse),
            "automated_fuel_dispensers" => Ok(Self::AutomatedFuelDispensers),
            "automobile_associations" => Ok(Self::AutomobileAssociations),
            "automotive_parts_and_accessories_stores" => {
                Ok(Self::AutomotivePartsAndAccessoriesStores)
            }
            "automotive_tire_stores" => Ok(Self::AutomotiveTireStores),
            "bail_and_bond_payments" => Ok(Self::BailAndBondPayments),
            "bakeries" => Ok(Self::Bakeries),
            "bands_orchestras" => Ok(Self::BandsOrchestras),
            "barber_and_beauty_shops" => Ok(Self::BarberAndBeautyShops),
            "betting_casino_gambling" => Ok(Self::BettingCasinoGambling),
            "bicycle_shops" => Ok(Self::BicycleShops),
            "billiard_pool_establishments" => Ok(Self::BilliardPoolEstablishments),
            "boat_dealers" => Ok(Self::BoatDealers),
            "boat_rentals_and_leases" => Ok(Self::BoatRentalsAndLeases),
            "book_stores" => Ok(Self::BookStores),
            "books_periodicals_and_newspapers" => Ok(Self::BooksPeriodicalsAndNewspapers),
            "bowling_alleys" => Ok(Self::BowlingAlleys),
            "bus_lines" => Ok(Self::BusLines),
            "business_secretarial_schools" => Ok(Self::BusinessSecretarialSchools),
            "buying_shopping_services" => Ok(Self::BuyingShoppingServices),
            "cable_satellite_and_other_pay_television_and_radio" => {
                Ok(Self::CableSatelliteAndOtherPayTelevisionAndRadio)
            }
            "camera_and_photographic_supply_stores" => Ok(Self::CameraAndPhotographicSupplyStores),
            "candy_nut_and_confectionery_stores" => Ok(Self::CandyNutAndConfectioneryStores),
            "car_and_truck_dealers_new_used" => Ok(Self::CarAndTruckDealersNewUsed),
            "car_and_truck_dealers_used_only" => Ok(Self::CarAndTruckDealersUsedOnly),
            "car_rental_agencies" => Ok(Self::CarRentalAgencies),
            "car_washes" => Ok(Self::CarWashes),
            "carpentry_services" => Ok(Self::CarpentryServices),
            "carpet_upholstery_cleaning" => Ok(Self::CarpetUpholsteryCleaning),
            "caterers" => Ok(Self::Caterers),
            "charitable_and_social_service_organizations_fundraising" => {
                Ok(Self::CharitableAndSocialServiceOrganizationsFundraising)
            }
            "chemicals_and_allied_products" => Ok(Self::ChemicalsAndAlliedProducts),
            "child_care_services" => Ok(Self::ChildCareServices),
            "childrens_and_infants_wear_stores" => Ok(Self::ChildrensAndInfantsWearStores),
            "chiropodists_podiatrists" => Ok(Self::ChiropodistsPodiatrists),
            "chiropractors" => Ok(Self::Chiropractors),
            "cigar_stores_and_stands" => Ok(Self::CigarStoresAndStands),
            "civic_social_fraternal_associations" => Ok(Self::CivicSocialFraternalAssociations),
            "cleaning_and_maintenance" => Ok(Self::CleaningAndMaintenance),
            "clothing_rental" => Ok(Self::ClothingRental),
            "colleges_universities" => Ok(Self::CollegesUniversities),
            "commercial_equipment" => Ok(Self::CommercialEquipment),
            "commercial_footwear" => Ok(Self::CommercialFootwear),
            "commercial_photography_art_and_graphics" => {
                Ok(Self::CommercialPhotographyArtAndGraphics)
            }
            "commuter_transport_and_ferries" => Ok(Self::CommuterTransportAndFerries),
            "computer_network_services" => Ok(Self::ComputerNetworkServices),
            "computer_programming" => Ok(Self::ComputerProgramming),
            "computer_repair" => Ok(Self::ComputerRepair),
            "computer_software_stores" => Ok(Self::ComputerSoftwareStores),
            "computers_peripherals_and_software" => Ok(Self::ComputersPeripheralsAndSoftware),
            "concrete_work_services" => Ok(Self::ConcreteWorkServices),
            "construction_materials" => Ok(Self::ConstructionMaterials),
            "consulting_public_relations" => Ok(Self::ConsultingPublicRelations),
            "correspondence_schools" => Ok(Self::CorrespondenceSchools),
            "cosmetic_stores" => Ok(Self::CosmeticStores),
            "counseling_services" => Ok(Self::CounselingServices),
            "country_clubs" => Ok(Self::CountryClubs),
            "courier_services" => Ok(Self::CourierServices),
            "court_costs" => Ok(Self::CourtCosts),
            "credit_reporting_agencies" => Ok(Self::CreditReportingAgencies),
            "cruise_lines" => Ok(Self::CruiseLines),
            "dairy_products_stores" => Ok(Self::DairyProductsStores),
            "dance_hall_studios_schools" => Ok(Self::DanceHallStudiosSchools),
            "dating_escort_services" => Ok(Self::DatingEscortServices),
            "dentists_orthodontists" => Ok(Self::DentistsOrthodontists),
            "department_stores" => Ok(Self::DepartmentStores),
            "detective_agencies" => Ok(Self::DetectiveAgencies),
            "digital_goods_applications" => Ok(Self::DigitalGoodsApplications),
            "digital_goods_games" => Ok(Self::DigitalGoodsGames),
            "digital_goods_large_volume" => Ok(Self::DigitalGoodsLargeVolume),
            "digital_goods_media" => Ok(Self::DigitalGoodsMedia),
            "direct_marketing_catalog_merchant" => Ok(Self::DirectMarketingCatalogMerchant),
            "direct_marketing_combination_catalog_and_retail_merchant" => {
                Ok(Self::DirectMarketingCombinationCatalogAndRetailMerchant)
            }
            "direct_marketing_inbound_telemarketing" => {
                Ok(Self::DirectMarketingInboundTelemarketing)
            }
            "direct_marketing_insurance_services" => Ok(Self::DirectMarketingInsuranceServices),
            "direct_marketing_other" => Ok(Self::DirectMarketingOther),
            "direct_marketing_outbound_telemarketing" => {
                Ok(Self::DirectMarketingOutboundTelemarketing)
            }
            "direct_marketing_subscription" => Ok(Self::DirectMarketingSubscription),
            "direct_marketing_travel" => Ok(Self::DirectMarketingTravel),
            "discount_stores" => Ok(Self::DiscountStores),
            "doctors" => Ok(Self::Doctors),
            "door_to_door_sales" => Ok(Self::DoorToDoorSales),
            "drapery_window_covering_and_upholstery_stores" => {
                Ok(Self::DraperyWindowCoveringAndUpholsteryStores)
            }
            "drinking_places" => Ok(Self::DrinkingPlaces),
            "drug_stores_and_pharmacies" => Ok(Self::DrugStoresAndPharmacies),
            "drugs_drug_proprietaries_and_druggist_sundries" => {
                Ok(Self::DrugsDrugProprietariesAndDruggistSundries)
            }
            "dry_cleaners" => Ok(Self::DryCleaners),
            "durable_goods" => Ok(Self::DurableGoods),
            "duty_free_stores" => Ok(Self::DutyFreeStores),
            "eating_places_restaurants" => Ok(Self::EatingPlacesRestaurants),
            "educational_services" => Ok(Self::EducationalServices),
            "electric_razor_stores" => Ok(Self::ElectricRazorStores),
            "electrical_parts_and_equipment" => Ok(Self::ElectricalPartsAndEquipment),
            "electrical_services" => Ok(Self::ElectricalServices),
            "electronics_repair_shops" => Ok(Self::ElectronicsRepairShops),
            "electronics_stores" => Ok(Self::ElectronicsStores),
            "elementary_secondary_schools" => Ok(Self::ElementarySecondarySchools),
            "employment_temp_agencies" => Ok(Self::EmploymentTempAgencies),
            "equipment_rental" => Ok(Self::EquipmentRental),
            "exterminating_services" => Ok(Self::ExterminatingServices),
            "family_clothing_stores" => Ok(Self::FamilyClothingStores),
            "fast_food_restaurants" => Ok(Self::FastFoodRestaurants),
            "financial_institutions" => Ok(Self::FinancialInstitutions),
            "fines_government_administrative_entities" => {
                Ok(Self::FinesGovernmentAdministrativeEntities)
            }
            "fireplace_fireplace_screens_and_accessories_stores" => {
                Ok(Self::FireplaceFireplaceScreensAndAccessoriesStores)
            }
            "floor_covering_stores" => Ok(Self::FloorCoveringStores),
            "florists" => Ok(Self::Florists),
            "florists_supplies_nursery_stock_and_flowers" => {
                Ok(Self::FloristsSuppliesNurseryStockAndFlowers)
            }
            "freezer_and_locker_meat_provisioners" => Ok(Self::FreezerAndLockerMeatProvisioners),
            "fuel_dealers_non_automotive" => Ok(Self::FuelDealersNonAutomotive),
            "funeral_services_crematories" => Ok(Self::FuneralServicesCrematories),
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => {
                Ok(Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances)
            }
            "furniture_repair_refinishing" => Ok(Self::FurnitureRepairRefinishing),
            "furriers_and_fur_shops" => Ok(Self::FurriersAndFurShops),
            "general_services" => Ok(Self::GeneralServices),
            "gift_card_novelty_and_souvenir_shops" => Ok(Self::GiftCardNoveltyAndSouvenirShops),
            "glass_paint_and_wallpaper_stores" => Ok(Self::GlassPaintAndWallpaperStores),
            "glassware_crystal_stores" => Ok(Self::GlasswareCrystalStores),
            "golf_courses_public" => Ok(Self::GolfCoursesPublic),
            "government_services" => Ok(Self::GovernmentServices),
            "grocery_stores_supermarkets" => Ok(Self::GroceryStoresSupermarkets),
            "hardware_equipment_and_supplies" => Ok(Self::HardwareEquipmentAndSupplies),
            "hardware_stores" => Ok(Self::HardwareStores),
            "health_and_beauty_spas" => Ok(Self::HealthAndBeautySpas),
            "hearing_aids_sales_and_supplies" => Ok(Self::HearingAidsSalesAndSupplies),
            "heating_plumbing_a_c" => Ok(Self::HeatingPlumbingAC),
            "hobby_toy_and_game_shops" => Ok(Self::HobbyToyAndGameShops),
            "home_supply_warehouse_stores" => Ok(Self::HomeSupplyWarehouseStores),
            "hospitals" => Ok(Self::Hospitals),
            "hotels_motels_and_resorts" => Ok(Self::HotelsMotelsAndResorts),
            "household_appliance_stores" => Ok(Self::HouseholdApplianceStores),
            "industrial_supplies" => Ok(Self::IndustrialSupplies),
            "information_retrieval_services" => Ok(Self::InformationRetrievalServices),
            "insurance_default" => Ok(Self::InsuranceDefault),
            "insurance_underwriting_premiums" => Ok(Self::InsuranceUnderwritingPremiums),
            "intra_company_purchases" => Ok(Self::IntraCompanyPurchases),
            "jewelry_stores_watches_clocks_and_silverware_stores" => {
                Ok(Self::JewelryStoresWatchesClocksAndSilverwareStores)
            }
            "landscaping_services" => Ok(Self::LandscapingServices),
            "laundries" => Ok(Self::Laundries),
            "laundry_cleaning_services" => Ok(Self::LaundryCleaningServices),
            "legal_services_attorneys" => Ok(Self::LegalServicesAttorneys),
            "luggage_and_leather_goods_stores" => Ok(Self::LuggageAndLeatherGoodsStores),
            "lumber_building_materials_stores" => Ok(Self::LumberBuildingMaterialsStores),
            "manual_cash_disburse" => Ok(Self::ManualCashDisburse),
            "marinas_service_and_supplies" => Ok(Self::MarinasServiceAndSupplies),
            "masonry_stonework_and_plaster" => Ok(Self::MasonryStoneworkAndPlaster),
            "massage_parlors" => Ok(Self::MassageParlors),
            "medical_and_dental_labs" => Ok(Self::MedicalAndDentalLabs),
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => {
                Ok(Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies)
            }
            "medical_services" => Ok(Self::MedicalServices),
            "membership_organizations" => Ok(Self::MembershipOrganizations),
            "mens_and_boys_clothing_and_accessories_stores" => {
                Ok(Self::MensAndBoysClothingAndAccessoriesStores)
            }
            "mens_womens_clothing_stores" => Ok(Self::MensWomensClothingStores),
            "metal_service_centers" => Ok(Self::MetalServiceCenters),
            "miscellaneous" => Ok(Self::Miscellaneous),
            "miscellaneous_apparel_and_accessory_shops" => {
                Ok(Self::MiscellaneousApparelAndAccessoryShops)
            }
            "miscellaneous_auto_dealers" => Ok(Self::MiscellaneousAutoDealers),
            "miscellaneous_business_services" => Ok(Self::MiscellaneousBusinessServices),
            "miscellaneous_food_stores" => Ok(Self::MiscellaneousFoodStores),
            "miscellaneous_general_merchandise" => Ok(Self::MiscellaneousGeneralMerchandise),
            "miscellaneous_general_services" => Ok(Self::MiscellaneousGeneralServices),
            "miscellaneous_home_furnishing_specialty_stores" => {
                Ok(Self::MiscellaneousHomeFurnishingSpecialtyStores)
            }
            "miscellaneous_publishing_and_printing" => Ok(Self::MiscellaneousPublishingAndPrinting),
            "miscellaneous_recreation_services" => Ok(Self::MiscellaneousRecreationServices),
            "miscellaneous_repair_shops" => Ok(Self::MiscellaneousRepairShops),
            "miscellaneous_specialty_retail" => Ok(Self::MiscellaneousSpecialtyRetail),
            "mobile_home_dealers" => Ok(Self::MobileHomeDealers),
            "motion_picture_theaters" => Ok(Self::MotionPictureTheaters),
            "motor_freight_carriers_and_trucking" => Ok(Self::MotorFreightCarriersAndTrucking),
            "motor_homes_dealers" => Ok(Self::MotorHomesDealers),
            "motor_vehicle_supplies_and_new_parts" => Ok(Self::MotorVehicleSuppliesAndNewParts),
            "motorcycle_shops_and_dealers" => Ok(Self::MotorcycleShopsAndDealers),
            "motorcycle_shops_dealers" => Ok(Self::MotorcycleShopsDealers),
            "music_stores_musical_instruments_pianos_and_sheet_music" => {
                Ok(Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic)
            }
            "news_dealers_and_newsstands" => Ok(Self::NewsDealersAndNewsstands),
            "non_fi_money_orders" => Ok(Self::NonFiMoneyOrders),
            "non_fi_stored_value_card_purchase_load" => Ok(Self::NonFiStoredValueCardPurchaseLoad),
            "nondurable_goods" => Ok(Self::NondurableGoods),
            "nurseries_lawn_and_garden_supply_stores" => {
                Ok(Self::NurseriesLawnAndGardenSupplyStores)
            }
            "nursing_personal_care" => Ok(Self::NursingPersonalCare),
            "office_and_commercial_furniture" => Ok(Self::OfficeAndCommercialFurniture),
            "opticians_eyeglasses" => Ok(Self::OpticiansEyeglasses),
            "optometrists_ophthalmologist" => Ok(Self::OptometristsOphthalmologist),
            "orthopedic_goods_prosthetic_devices" => Ok(Self::OrthopedicGoodsProstheticDevices),
            "osteopaths" => Ok(Self::Osteopaths),
            "package_stores_beer_wine_and_liquor" => Ok(Self::PackageStoresBeerWineAndLiquor),
            "paints_varnishes_and_supplies" => Ok(Self::PaintsVarnishesAndSupplies),
            "parking_lots_garages" => Ok(Self::ParkingLotsGarages),
            "passenger_railways" => Ok(Self::PassengerRailways),
            "pawn_shops" => Ok(Self::PawnShops),
            "pet_shops_pet_food_and_supplies" => Ok(Self::PetShopsPetFoodAndSupplies),
            "petroleum_and_petroleum_products" => Ok(Self::PetroleumAndPetroleumProducts),
            "photo_developing" => Ok(Self::PhotoDeveloping),
            "photographic_photocopy_microfilm_equipment_and_supplies" => {
                Ok(Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies)
            }
            "photographic_studios" => Ok(Self::PhotographicStudios),
            "picture_video_production" => Ok(Self::PictureVideoProduction),
            "piece_goods_notions_and_other_dry_goods" => {
                Ok(Self::PieceGoodsNotionsAndOtherDryGoods)
            }
            "plumbing_heating_equipment_and_supplies" => {
                Ok(Self::PlumbingHeatingEquipmentAndSupplies)
            }
            "political_organizations" => Ok(Self::PoliticalOrganizations),
            "postal_services_government_only" => Ok(Self::PostalServicesGovernmentOnly),
            "precious_stones_and_metals_watches_and_jewelry" => {
                Ok(Self::PreciousStonesAndMetalsWatchesAndJewelry)
            }
            "professional_services" => Ok(Self::ProfessionalServices),
            "public_warehousing_and_storage" => Ok(Self::PublicWarehousingAndStorage),
            "quick_copy_repro_and_blueprint" => Ok(Self::QuickCopyReproAndBlueprint),
            "railroads" => Ok(Self::Railroads),
            "real_estate_agents_and_managers_rentals" => {
                Ok(Self::RealEstateAgentsAndManagersRentals)
            }
            "record_stores" => Ok(Self::RecordStores),
            "recreational_vehicle_rentals" => Ok(Self::RecreationalVehicleRentals),
            "religious_goods_stores" => Ok(Self::ReligiousGoodsStores),
            "religious_organizations" => Ok(Self::ReligiousOrganizations),
            "roofing_siding_sheet_metal" => Ok(Self::RoofingSidingSheetMetal),
            "secretarial_support_services" => Ok(Self::SecretarialSupportServices),
            "security_brokers_dealers" => Ok(Self::SecurityBrokersDealers),
            "service_stations" => Ok(Self::ServiceStations),
            "sewing_needlework_fabric_and_piece_goods_stores" => {
                Ok(Self::SewingNeedleworkFabricAndPieceGoodsStores)
            }
            "shoe_repair_hat_cleaning" => Ok(Self::ShoeRepairHatCleaning),
            "shoe_stores" => Ok(Self::ShoeStores),
            "small_appliance_repair" => Ok(Self::SmallApplianceRepair),
            "snowmobile_dealers" => Ok(Self::SnowmobileDealers),
            "special_trade_services" => Ok(Self::SpecialTradeServices),
            "specialty_cleaning" => Ok(Self::SpecialtyCleaning),
            "sporting_goods_stores" => Ok(Self::SportingGoodsStores),
            "sporting_recreation_camps" => Ok(Self::SportingRecreationCamps),
            "sports_and_riding_apparel_stores" => Ok(Self::SportsAndRidingApparelStores),
            "sports_clubs_fields" => Ok(Self::SportsClubsFields),
            "stamp_and_coin_stores" => Ok(Self::StampAndCoinStores),
            "stationary_office_supplies_printing_and_writing_paper" => {
                Ok(Self::StationaryOfficeSuppliesPrintingAndWritingPaper)
            }
            "stationery_stores_office_and_school_supply_stores" => {
                Ok(Self::StationeryStoresOfficeAndSchoolSupplyStores)
            }
            "swimming_pools_sales" => Ok(Self::SwimmingPoolsSales),
            "t_ui_travel_germany" => Ok(Self::TUiTravelGermany),
            "tailors_alterations" => Ok(Self::TailorsAlterations),
            "tax_payments_government_agencies" => Ok(Self::TaxPaymentsGovernmentAgencies),
            "tax_preparation_services" => Ok(Self::TaxPreparationServices),
            "taxicabs_limousines" => Ok(Self::TaxicabsLimousines),
            "telecommunication_equipment_and_telephone_sales" => {
                Ok(Self::TelecommunicationEquipmentAndTelephoneSales)
            }
            "telecommunication_services" => Ok(Self::TelecommunicationServices),
            "telegraph_services" => Ok(Self::TelegraphServices),
            "tent_and_awning_shops" => Ok(Self::TentAndAwningShops),
            "testing_laboratories" => Ok(Self::TestingLaboratories),
            "theatrical_ticket_agencies" => Ok(Self::TheatricalTicketAgencies),
            "timeshares" => Ok(Self::Timeshares),
            "tire_retreading_and_repair" => Ok(Self::TireRetreadingAndRepair),
            "tolls_bridge_fees" => Ok(Self::TollsBridgeFees),
            "tourist_attractions_and_exhibits" => Ok(Self::TouristAttractionsAndExhibits),
            "towing_services" => Ok(Self::TowingServices),
            "trailer_parks_campgrounds" => Ok(Self::TrailerParksCampgrounds),
            "transportation_services" => Ok(Self::TransportationServices),
            "travel_agencies_tour_operators" => Ok(Self::TravelAgenciesTourOperators),
            "truck_stop_iteration" => Ok(Self::TruckStopIteration),
            "truck_utility_trailer_rentals" => Ok(Self::TruckUtilityTrailerRentals),
            "typesetting_plate_making_and_related_services" => {
                Ok(Self::TypesettingPlateMakingAndRelatedServices)
            }
            "typewriter_stores" => Ok(Self::TypewriterStores),
            "u_s_federal_government_agencies_or_departments" => {
                Ok(Self::USFederalGovernmentAgenciesOrDepartments)
            }
            "uniforms_commercial_clothing" => Ok(Self::UniformsCommercialClothing),
            "used_merchandise_and_secondhand_stores" => {
                Ok(Self::UsedMerchandiseAndSecondhandStores)
            }
            "utilities" => Ok(Self::Utilities),
            "variety_stores" => Ok(Self::VarietyStores),
            "veterinary_services" => Ok(Self::VeterinaryServices),
            "video_amusement_game_supplies" => Ok(Self::VideoAmusementGameSupplies),
            "video_game_arcades" => Ok(Self::VideoGameArcades),
            "video_tape_rental_stores" => Ok(Self::VideoTapeRentalStores),
            "vocational_trade_schools" => Ok(Self::VocationalTradeSchools),
            "watch_jewelry_repair" => Ok(Self::WatchJewelryRepair),
            "welding_repair" => Ok(Self::WeldingRepair),
            "wholesale_clubs" => Ok(Self::WholesaleClubs),
            "wig_and_toupee_stores" => Ok(Self::WigAndToupeeStores),
            "wires_money_orders" => Ok(Self::WiresMoneyOrders),
            "womens_accessory_and_specialty_shops" => Ok(Self::WomensAccessoryAndSpecialtyShops),
            "womens_ready_to_wear_stores" => Ok(Self::WomensReadyToWearStores),
            "wrecking_and_salvage_yards" => Ok(Self::WreckingAndSalvageYards),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardSpendingControlsAllowedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardSpendingControlsAllowedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardSpendingControlsAllowedCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
///
/// All other categories will be allowed.
/// Cannot be set with `allowed_categories`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardSpendingControlsBlockedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl CreateCardSpendingControlsBlockedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl std::str::FromStr for CreateCardSpendingControlsBlockedCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ac_refrigeration_repair" => Ok(Self::AcRefrigerationRepair),
            "accounting_bookkeeping_services" => Ok(Self::AccountingBookkeepingServices),
            "advertising_services" => Ok(Self::AdvertisingServices),
            "agricultural_cooperative" => Ok(Self::AgriculturalCooperative),
            "airlines_air_carriers" => Ok(Self::AirlinesAirCarriers),
            "airports_flying_fields" => Ok(Self::AirportsFlyingFields),
            "ambulance_services" => Ok(Self::AmbulanceServices),
            "amusement_parks_carnivals" => Ok(Self::AmusementParksCarnivals),
            "antique_reproductions" => Ok(Self::AntiqueReproductions),
            "antique_shops" => Ok(Self::AntiqueShops),
            "aquariums" => Ok(Self::Aquariums),
            "architectural_surveying_services" => Ok(Self::ArchitecturalSurveyingServices),
            "art_dealers_and_galleries" => Ok(Self::ArtDealersAndGalleries),
            "artists_supply_and_craft_shops" => Ok(Self::ArtistsSupplyAndCraftShops),
            "auto_and_home_supply_stores" => Ok(Self::AutoAndHomeSupplyStores),
            "auto_body_repair_shops" => Ok(Self::AutoBodyRepairShops),
            "auto_paint_shops" => Ok(Self::AutoPaintShops),
            "auto_service_shops" => Ok(Self::AutoServiceShops),
            "automated_cash_disburse" => Ok(Self::AutomatedCashDisburse),
            "automated_fuel_dispensers" => Ok(Self::AutomatedFuelDispensers),
            "automobile_associations" => Ok(Self::AutomobileAssociations),
            "automotive_parts_and_accessories_stores" => {
                Ok(Self::AutomotivePartsAndAccessoriesStores)
            }
            "automotive_tire_stores" => Ok(Self::AutomotiveTireStores),
            "bail_and_bond_payments" => Ok(Self::BailAndBondPayments),
            "bakeries" => Ok(Self::Bakeries),
            "bands_orchestras" => Ok(Self::BandsOrchestras),
            "barber_and_beauty_shops" => Ok(Self::BarberAndBeautyShops),
            "betting_casino_gambling" => Ok(Self::BettingCasinoGambling),
            "bicycle_shops" => Ok(Self::BicycleShops),
            "billiard_pool_establishments" => Ok(Self::BilliardPoolEstablishments),
            "boat_dealers" => Ok(Self::BoatDealers),
            "boat_rentals_and_leases" => Ok(Self::BoatRentalsAndLeases),
            "book_stores" => Ok(Self::BookStores),
            "books_periodicals_and_newspapers" => Ok(Self::BooksPeriodicalsAndNewspapers),
            "bowling_alleys" => Ok(Self::BowlingAlleys),
            "bus_lines" => Ok(Self::BusLines),
            "business_secretarial_schools" => Ok(Self::BusinessSecretarialSchools),
            "buying_shopping_services" => Ok(Self::BuyingShoppingServices),
            "cable_satellite_and_other_pay_television_and_radio" => {
                Ok(Self::CableSatelliteAndOtherPayTelevisionAndRadio)
            }
            "camera_and_photographic_supply_stores" => Ok(Self::CameraAndPhotographicSupplyStores),
            "candy_nut_and_confectionery_stores" => Ok(Self::CandyNutAndConfectioneryStores),
            "car_and_truck_dealers_new_used" => Ok(Self::CarAndTruckDealersNewUsed),
            "car_and_truck_dealers_used_only" => Ok(Self::CarAndTruckDealersUsedOnly),
            "car_rental_agencies" => Ok(Self::CarRentalAgencies),
            "car_washes" => Ok(Self::CarWashes),
            "carpentry_services" => Ok(Self::CarpentryServices),
            "carpet_upholstery_cleaning" => Ok(Self::CarpetUpholsteryCleaning),
            "caterers" => Ok(Self::Caterers),
            "charitable_and_social_service_organizations_fundraising" => {
                Ok(Self::CharitableAndSocialServiceOrganizationsFundraising)
            }
            "chemicals_and_allied_products" => Ok(Self::ChemicalsAndAlliedProducts),
            "child_care_services" => Ok(Self::ChildCareServices),
            "childrens_and_infants_wear_stores" => Ok(Self::ChildrensAndInfantsWearStores),
            "chiropodists_podiatrists" => Ok(Self::ChiropodistsPodiatrists),
            "chiropractors" => Ok(Self::Chiropractors),
            "cigar_stores_and_stands" => Ok(Self::CigarStoresAndStands),
            "civic_social_fraternal_associations" => Ok(Self::CivicSocialFraternalAssociations),
            "cleaning_and_maintenance" => Ok(Self::CleaningAndMaintenance),
            "clothing_rental" => Ok(Self::ClothingRental),
            "colleges_universities" => Ok(Self::CollegesUniversities),
            "commercial_equipment" => Ok(Self::CommercialEquipment),
            "commercial_footwear" => Ok(Self::CommercialFootwear),
            "commercial_photography_art_and_graphics" => {
                Ok(Self::CommercialPhotographyArtAndGraphics)
            }
            "commuter_transport_and_ferries" => Ok(Self::CommuterTransportAndFerries),
            "computer_network_services" => Ok(Self::ComputerNetworkServices),
            "computer_programming" => Ok(Self::ComputerProgramming),
            "computer_repair" => Ok(Self::ComputerRepair),
            "computer_software_stores" => Ok(Self::ComputerSoftwareStores),
            "computers_peripherals_and_software" => Ok(Self::ComputersPeripheralsAndSoftware),
            "concrete_work_services" => Ok(Self::ConcreteWorkServices),
            "construction_materials" => Ok(Self::ConstructionMaterials),
            "consulting_public_relations" => Ok(Self::ConsultingPublicRelations),
            "correspondence_schools" => Ok(Self::CorrespondenceSchools),
            "cosmetic_stores" => Ok(Self::CosmeticStores),
            "counseling_services" => Ok(Self::CounselingServices),
            "country_clubs" => Ok(Self::CountryClubs),
            "courier_services" => Ok(Self::CourierServices),
            "court_costs" => Ok(Self::CourtCosts),
            "credit_reporting_agencies" => Ok(Self::CreditReportingAgencies),
            "cruise_lines" => Ok(Self::CruiseLines),
            "dairy_products_stores" => Ok(Self::DairyProductsStores),
            "dance_hall_studios_schools" => Ok(Self::DanceHallStudiosSchools),
            "dating_escort_services" => Ok(Self::DatingEscortServices),
            "dentists_orthodontists" => Ok(Self::DentistsOrthodontists),
            "department_stores" => Ok(Self::DepartmentStores),
            "detective_agencies" => Ok(Self::DetectiveAgencies),
            "digital_goods_applications" => Ok(Self::DigitalGoodsApplications),
            "digital_goods_games" => Ok(Self::DigitalGoodsGames),
            "digital_goods_large_volume" => Ok(Self::DigitalGoodsLargeVolume),
            "digital_goods_media" => Ok(Self::DigitalGoodsMedia),
            "direct_marketing_catalog_merchant" => Ok(Self::DirectMarketingCatalogMerchant),
            "direct_marketing_combination_catalog_and_retail_merchant" => {
                Ok(Self::DirectMarketingCombinationCatalogAndRetailMerchant)
            }
            "direct_marketing_inbound_telemarketing" => {
                Ok(Self::DirectMarketingInboundTelemarketing)
            }
            "direct_marketing_insurance_services" => Ok(Self::DirectMarketingInsuranceServices),
            "direct_marketing_other" => Ok(Self::DirectMarketingOther),
            "direct_marketing_outbound_telemarketing" => {
                Ok(Self::DirectMarketingOutboundTelemarketing)
            }
            "direct_marketing_subscription" => Ok(Self::DirectMarketingSubscription),
            "direct_marketing_travel" => Ok(Self::DirectMarketingTravel),
            "discount_stores" => Ok(Self::DiscountStores),
            "doctors" => Ok(Self::Doctors),
            "door_to_door_sales" => Ok(Self::DoorToDoorSales),
            "drapery_window_covering_and_upholstery_stores" => {
                Ok(Self::DraperyWindowCoveringAndUpholsteryStores)
            }
            "drinking_places" => Ok(Self::DrinkingPlaces),
            "drug_stores_and_pharmacies" => Ok(Self::DrugStoresAndPharmacies),
            "drugs_drug_proprietaries_and_druggist_sundries" => {
                Ok(Self::DrugsDrugProprietariesAndDruggistSundries)
            }
            "dry_cleaners" => Ok(Self::DryCleaners),
            "durable_goods" => Ok(Self::DurableGoods),
            "duty_free_stores" => Ok(Self::DutyFreeStores),
            "eating_places_restaurants" => Ok(Self::EatingPlacesRestaurants),
            "educational_services" => Ok(Self::EducationalServices),
            "electric_razor_stores" => Ok(Self::ElectricRazorStores),
            "electrical_parts_and_equipment" => Ok(Self::ElectricalPartsAndEquipment),
            "electrical_services" => Ok(Self::ElectricalServices),
            "electronics_repair_shops" => Ok(Self::ElectronicsRepairShops),
            "electronics_stores" => Ok(Self::ElectronicsStores),
            "elementary_secondary_schools" => Ok(Self::ElementarySecondarySchools),
            "employment_temp_agencies" => Ok(Self::EmploymentTempAgencies),
            "equipment_rental" => Ok(Self::EquipmentRental),
            "exterminating_services" => Ok(Self::ExterminatingServices),
            "family_clothing_stores" => Ok(Self::FamilyClothingStores),
            "fast_food_restaurants" => Ok(Self::FastFoodRestaurants),
            "financial_institutions" => Ok(Self::FinancialInstitutions),
            "fines_government_administrative_entities" => {
                Ok(Self::FinesGovernmentAdministrativeEntities)
            }
            "fireplace_fireplace_screens_and_accessories_stores" => {
                Ok(Self::FireplaceFireplaceScreensAndAccessoriesStores)
            }
            "floor_covering_stores" => Ok(Self::FloorCoveringStores),
            "florists" => Ok(Self::Florists),
            "florists_supplies_nursery_stock_and_flowers" => {
                Ok(Self::FloristsSuppliesNurseryStockAndFlowers)
            }
            "freezer_and_locker_meat_provisioners" => Ok(Self::FreezerAndLockerMeatProvisioners),
            "fuel_dealers_non_automotive" => Ok(Self::FuelDealersNonAutomotive),
            "funeral_services_crematories" => Ok(Self::FuneralServicesCrematories),
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => {
                Ok(Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances)
            }
            "furniture_repair_refinishing" => Ok(Self::FurnitureRepairRefinishing),
            "furriers_and_fur_shops" => Ok(Self::FurriersAndFurShops),
            "general_services" => Ok(Self::GeneralServices),
            "gift_card_novelty_and_souvenir_shops" => Ok(Self::GiftCardNoveltyAndSouvenirShops),
            "glass_paint_and_wallpaper_stores" => Ok(Self::GlassPaintAndWallpaperStores),
            "glassware_crystal_stores" => Ok(Self::GlasswareCrystalStores),
            "golf_courses_public" => Ok(Self::GolfCoursesPublic),
            "government_services" => Ok(Self::GovernmentServices),
            "grocery_stores_supermarkets" => Ok(Self::GroceryStoresSupermarkets),
            "hardware_equipment_and_supplies" => Ok(Self::HardwareEquipmentAndSupplies),
            "hardware_stores" => Ok(Self::HardwareStores),
            "health_and_beauty_spas" => Ok(Self::HealthAndBeautySpas),
            "hearing_aids_sales_and_supplies" => Ok(Self::HearingAidsSalesAndSupplies),
            "heating_plumbing_a_c" => Ok(Self::HeatingPlumbingAC),
            "hobby_toy_and_game_shops" => Ok(Self::HobbyToyAndGameShops),
            "home_supply_warehouse_stores" => Ok(Self::HomeSupplyWarehouseStores),
            "hospitals" => Ok(Self::Hospitals),
            "hotels_motels_and_resorts" => Ok(Self::HotelsMotelsAndResorts),
            "household_appliance_stores" => Ok(Self::HouseholdApplianceStores),
            "industrial_supplies" => Ok(Self::IndustrialSupplies),
            "information_retrieval_services" => Ok(Self::InformationRetrievalServices),
            "insurance_default" => Ok(Self::InsuranceDefault),
            "insurance_underwriting_premiums" => Ok(Self::InsuranceUnderwritingPremiums),
            "intra_company_purchases" => Ok(Self::IntraCompanyPurchases),
            "jewelry_stores_watches_clocks_and_silverware_stores" => {
                Ok(Self::JewelryStoresWatchesClocksAndSilverwareStores)
            }
            "landscaping_services" => Ok(Self::LandscapingServices),
            "laundries" => Ok(Self::Laundries),
            "laundry_cleaning_services" => Ok(Self::LaundryCleaningServices),
            "legal_services_attorneys" => Ok(Self::LegalServicesAttorneys),
            "luggage_and_leather_goods_stores" => Ok(Self::LuggageAndLeatherGoodsStores),
            "lumber_building_materials_stores" => Ok(Self::LumberBuildingMaterialsStores),
            "manual_cash_disburse" => Ok(Self::ManualCashDisburse),
            "marinas_service_and_supplies" => Ok(Self::MarinasServiceAndSupplies),
            "masonry_stonework_and_plaster" => Ok(Self::MasonryStoneworkAndPlaster),
            "massage_parlors" => Ok(Self::MassageParlors),
            "medical_and_dental_labs" => Ok(Self::MedicalAndDentalLabs),
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => {
                Ok(Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies)
            }
            "medical_services" => Ok(Self::MedicalServices),
            "membership_organizations" => Ok(Self::MembershipOrganizations),
            "mens_and_boys_clothing_and_accessories_stores" => {
                Ok(Self::MensAndBoysClothingAndAccessoriesStores)
            }
            "mens_womens_clothing_stores" => Ok(Self::MensWomensClothingStores),
            "metal_service_centers" => Ok(Self::MetalServiceCenters),
            "miscellaneous" => Ok(Self::Miscellaneous),
            "miscellaneous_apparel_and_accessory_shops" => {
                Ok(Self::MiscellaneousApparelAndAccessoryShops)
            }
            "miscellaneous_auto_dealers" => Ok(Self::MiscellaneousAutoDealers),
            "miscellaneous_business_services" => Ok(Self::MiscellaneousBusinessServices),
            "miscellaneous_food_stores" => Ok(Self::MiscellaneousFoodStores),
            "miscellaneous_general_merchandise" => Ok(Self::MiscellaneousGeneralMerchandise),
            "miscellaneous_general_services" => Ok(Self::MiscellaneousGeneralServices),
            "miscellaneous_home_furnishing_specialty_stores" => {
                Ok(Self::MiscellaneousHomeFurnishingSpecialtyStores)
            }
            "miscellaneous_publishing_and_printing" => Ok(Self::MiscellaneousPublishingAndPrinting),
            "miscellaneous_recreation_services" => Ok(Self::MiscellaneousRecreationServices),
            "miscellaneous_repair_shops" => Ok(Self::MiscellaneousRepairShops),
            "miscellaneous_specialty_retail" => Ok(Self::MiscellaneousSpecialtyRetail),
            "mobile_home_dealers" => Ok(Self::MobileHomeDealers),
            "motion_picture_theaters" => Ok(Self::MotionPictureTheaters),
            "motor_freight_carriers_and_trucking" => Ok(Self::MotorFreightCarriersAndTrucking),
            "motor_homes_dealers" => Ok(Self::MotorHomesDealers),
            "motor_vehicle_supplies_and_new_parts" => Ok(Self::MotorVehicleSuppliesAndNewParts),
            "motorcycle_shops_and_dealers" => Ok(Self::MotorcycleShopsAndDealers),
            "motorcycle_shops_dealers" => Ok(Self::MotorcycleShopsDealers),
            "music_stores_musical_instruments_pianos_and_sheet_music" => {
                Ok(Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic)
            }
            "news_dealers_and_newsstands" => Ok(Self::NewsDealersAndNewsstands),
            "non_fi_money_orders" => Ok(Self::NonFiMoneyOrders),
            "non_fi_stored_value_card_purchase_load" => Ok(Self::NonFiStoredValueCardPurchaseLoad),
            "nondurable_goods" => Ok(Self::NondurableGoods),
            "nurseries_lawn_and_garden_supply_stores" => {
                Ok(Self::NurseriesLawnAndGardenSupplyStores)
            }
            "nursing_personal_care" => Ok(Self::NursingPersonalCare),
            "office_and_commercial_furniture" => Ok(Self::OfficeAndCommercialFurniture),
            "opticians_eyeglasses" => Ok(Self::OpticiansEyeglasses),
            "optometrists_ophthalmologist" => Ok(Self::OptometristsOphthalmologist),
            "orthopedic_goods_prosthetic_devices" => Ok(Self::OrthopedicGoodsProstheticDevices),
            "osteopaths" => Ok(Self::Osteopaths),
            "package_stores_beer_wine_and_liquor" => Ok(Self::PackageStoresBeerWineAndLiquor),
            "paints_varnishes_and_supplies" => Ok(Self::PaintsVarnishesAndSupplies),
            "parking_lots_garages" => Ok(Self::ParkingLotsGarages),
            "passenger_railways" => Ok(Self::PassengerRailways),
            "pawn_shops" => Ok(Self::PawnShops),
            "pet_shops_pet_food_and_supplies" => Ok(Self::PetShopsPetFoodAndSupplies),
            "petroleum_and_petroleum_products" => Ok(Self::PetroleumAndPetroleumProducts),
            "photo_developing" => Ok(Self::PhotoDeveloping),
            "photographic_photocopy_microfilm_equipment_and_supplies" => {
                Ok(Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies)
            }
            "photographic_studios" => Ok(Self::PhotographicStudios),
            "picture_video_production" => Ok(Self::PictureVideoProduction),
            "piece_goods_notions_and_other_dry_goods" => {
                Ok(Self::PieceGoodsNotionsAndOtherDryGoods)
            }
            "plumbing_heating_equipment_and_supplies" => {
                Ok(Self::PlumbingHeatingEquipmentAndSupplies)
            }
            "political_organizations" => Ok(Self::PoliticalOrganizations),
            "postal_services_government_only" => Ok(Self::PostalServicesGovernmentOnly),
            "precious_stones_and_metals_watches_and_jewelry" => {
                Ok(Self::PreciousStonesAndMetalsWatchesAndJewelry)
            }
            "professional_services" => Ok(Self::ProfessionalServices),
            "public_warehousing_and_storage" => Ok(Self::PublicWarehousingAndStorage),
            "quick_copy_repro_and_blueprint" => Ok(Self::QuickCopyReproAndBlueprint),
            "railroads" => Ok(Self::Railroads),
            "real_estate_agents_and_managers_rentals" => {
                Ok(Self::RealEstateAgentsAndManagersRentals)
            }
            "record_stores" => Ok(Self::RecordStores),
            "recreational_vehicle_rentals" => Ok(Self::RecreationalVehicleRentals),
            "religious_goods_stores" => Ok(Self::ReligiousGoodsStores),
            "religious_organizations" => Ok(Self::ReligiousOrganizations),
            "roofing_siding_sheet_metal" => Ok(Self::RoofingSidingSheetMetal),
            "secretarial_support_services" => Ok(Self::SecretarialSupportServices),
            "security_brokers_dealers" => Ok(Self::SecurityBrokersDealers),
            "service_stations" => Ok(Self::ServiceStations),
            "sewing_needlework_fabric_and_piece_goods_stores" => {
                Ok(Self::SewingNeedleworkFabricAndPieceGoodsStores)
            }
            "shoe_repair_hat_cleaning" => Ok(Self::ShoeRepairHatCleaning),
            "shoe_stores" => Ok(Self::ShoeStores),
            "small_appliance_repair" => Ok(Self::SmallApplianceRepair),
            "snowmobile_dealers" => Ok(Self::SnowmobileDealers),
            "special_trade_services" => Ok(Self::SpecialTradeServices),
            "specialty_cleaning" => Ok(Self::SpecialtyCleaning),
            "sporting_goods_stores" => Ok(Self::SportingGoodsStores),
            "sporting_recreation_camps" => Ok(Self::SportingRecreationCamps),
            "sports_and_riding_apparel_stores" => Ok(Self::SportsAndRidingApparelStores),
            "sports_clubs_fields" => Ok(Self::SportsClubsFields),
            "stamp_and_coin_stores" => Ok(Self::StampAndCoinStores),
            "stationary_office_supplies_printing_and_writing_paper" => {
                Ok(Self::StationaryOfficeSuppliesPrintingAndWritingPaper)
            }
            "stationery_stores_office_and_school_supply_stores" => {
                Ok(Self::StationeryStoresOfficeAndSchoolSupplyStores)
            }
            "swimming_pools_sales" => Ok(Self::SwimmingPoolsSales),
            "t_ui_travel_germany" => Ok(Self::TUiTravelGermany),
            "tailors_alterations" => Ok(Self::TailorsAlterations),
            "tax_payments_government_agencies" => Ok(Self::TaxPaymentsGovernmentAgencies),
            "tax_preparation_services" => Ok(Self::TaxPreparationServices),
            "taxicabs_limousines" => Ok(Self::TaxicabsLimousines),
            "telecommunication_equipment_and_telephone_sales" => {
                Ok(Self::TelecommunicationEquipmentAndTelephoneSales)
            }
            "telecommunication_services" => Ok(Self::TelecommunicationServices),
            "telegraph_services" => Ok(Self::TelegraphServices),
            "tent_and_awning_shops" => Ok(Self::TentAndAwningShops),
            "testing_laboratories" => Ok(Self::TestingLaboratories),
            "theatrical_ticket_agencies" => Ok(Self::TheatricalTicketAgencies),
            "timeshares" => Ok(Self::Timeshares),
            "tire_retreading_and_repair" => Ok(Self::TireRetreadingAndRepair),
            "tolls_bridge_fees" => Ok(Self::TollsBridgeFees),
            "tourist_attractions_and_exhibits" => Ok(Self::TouristAttractionsAndExhibits),
            "towing_services" => Ok(Self::TowingServices),
            "trailer_parks_campgrounds" => Ok(Self::TrailerParksCampgrounds),
            "transportation_services" => Ok(Self::TransportationServices),
            "travel_agencies_tour_operators" => Ok(Self::TravelAgenciesTourOperators),
            "truck_stop_iteration" => Ok(Self::TruckStopIteration),
            "truck_utility_trailer_rentals" => Ok(Self::TruckUtilityTrailerRentals),
            "typesetting_plate_making_and_related_services" => {
                Ok(Self::TypesettingPlateMakingAndRelatedServices)
            }
            "typewriter_stores" => Ok(Self::TypewriterStores),
            "u_s_federal_government_agencies_or_departments" => {
                Ok(Self::USFederalGovernmentAgenciesOrDepartments)
            }
            "uniforms_commercial_clothing" => Ok(Self::UniformsCommercialClothing),
            "used_merchandise_and_secondhand_stores" => {
                Ok(Self::UsedMerchandiseAndSecondhandStores)
            }
            "utilities" => Ok(Self::Utilities),
            "variety_stores" => Ok(Self::VarietyStores),
            "veterinary_services" => Ok(Self::VeterinaryServices),
            "video_amusement_game_supplies" => Ok(Self::VideoAmusementGameSupplies),
            "video_game_arcades" => Ok(Self::VideoGameArcades),
            "video_tape_rental_stores" => Ok(Self::VideoTapeRentalStores),
            "vocational_trade_schools" => Ok(Self::VocationalTradeSchools),
            "watch_jewelry_repair" => Ok(Self::WatchJewelryRepair),
            "welding_repair" => Ok(Self::WeldingRepair),
            "wholesale_clubs" => Ok(Self::WholesaleClubs),
            "wig_and_toupee_stores" => Ok(Self::WigAndToupeeStores),
            "wires_money_orders" => Ok(Self::WiresMoneyOrders),
            "womens_accessory_and_specialty_shops" => Ok(Self::WomensAccessoryAndSpecialtyShops),
            "womens_ready_to_wear_stores" => Ok(Self::WomensReadyToWearStores),
            "wrecking_and_salvage_yards" => Ok(Self::WreckingAndSalvageYards),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardSpendingControlsBlockedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardSpendingControlsBlockedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardSpendingControlsBlockedCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateCardSpendingControlsSpendingLimits<'a> {
    /// Maximum amount allowed to spend per interval.
    pub amount: i64,
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    ///
    /// Omitting this field will apply the limit to all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<&'a [CreateCardSpendingControlsSpendingLimitsCategories]>,
    /// Interval (or event) to which the amount applies.
    pub interval: CreateCardSpendingControlsSpendingLimitsInterval,
}
impl<'a> CreateCardSpendingControlsSpendingLimits<'a> {
    pub fn new(amount: i64, interval: CreateCardSpendingControlsSpendingLimitsInterval) -> Self {
        Self { amount, categories: Default::default(), interval }
    }
}
/// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
///
/// Omitting this field will apply the limit to all categories.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardSpendingControlsSpendingLimitsCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl CreateCardSpendingControlsSpendingLimitsCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl std::str::FromStr for CreateCardSpendingControlsSpendingLimitsCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ac_refrigeration_repair" => Ok(Self::AcRefrigerationRepair),
            "accounting_bookkeeping_services" => Ok(Self::AccountingBookkeepingServices),
            "advertising_services" => Ok(Self::AdvertisingServices),
            "agricultural_cooperative" => Ok(Self::AgriculturalCooperative),
            "airlines_air_carriers" => Ok(Self::AirlinesAirCarriers),
            "airports_flying_fields" => Ok(Self::AirportsFlyingFields),
            "ambulance_services" => Ok(Self::AmbulanceServices),
            "amusement_parks_carnivals" => Ok(Self::AmusementParksCarnivals),
            "antique_reproductions" => Ok(Self::AntiqueReproductions),
            "antique_shops" => Ok(Self::AntiqueShops),
            "aquariums" => Ok(Self::Aquariums),
            "architectural_surveying_services" => Ok(Self::ArchitecturalSurveyingServices),
            "art_dealers_and_galleries" => Ok(Self::ArtDealersAndGalleries),
            "artists_supply_and_craft_shops" => Ok(Self::ArtistsSupplyAndCraftShops),
            "auto_and_home_supply_stores" => Ok(Self::AutoAndHomeSupplyStores),
            "auto_body_repair_shops" => Ok(Self::AutoBodyRepairShops),
            "auto_paint_shops" => Ok(Self::AutoPaintShops),
            "auto_service_shops" => Ok(Self::AutoServiceShops),
            "automated_cash_disburse" => Ok(Self::AutomatedCashDisburse),
            "automated_fuel_dispensers" => Ok(Self::AutomatedFuelDispensers),
            "automobile_associations" => Ok(Self::AutomobileAssociations),
            "automotive_parts_and_accessories_stores" => {
                Ok(Self::AutomotivePartsAndAccessoriesStores)
            }
            "automotive_tire_stores" => Ok(Self::AutomotiveTireStores),
            "bail_and_bond_payments" => Ok(Self::BailAndBondPayments),
            "bakeries" => Ok(Self::Bakeries),
            "bands_orchestras" => Ok(Self::BandsOrchestras),
            "barber_and_beauty_shops" => Ok(Self::BarberAndBeautyShops),
            "betting_casino_gambling" => Ok(Self::BettingCasinoGambling),
            "bicycle_shops" => Ok(Self::BicycleShops),
            "billiard_pool_establishments" => Ok(Self::BilliardPoolEstablishments),
            "boat_dealers" => Ok(Self::BoatDealers),
            "boat_rentals_and_leases" => Ok(Self::BoatRentalsAndLeases),
            "book_stores" => Ok(Self::BookStores),
            "books_periodicals_and_newspapers" => Ok(Self::BooksPeriodicalsAndNewspapers),
            "bowling_alleys" => Ok(Self::BowlingAlleys),
            "bus_lines" => Ok(Self::BusLines),
            "business_secretarial_schools" => Ok(Self::BusinessSecretarialSchools),
            "buying_shopping_services" => Ok(Self::BuyingShoppingServices),
            "cable_satellite_and_other_pay_television_and_radio" => {
                Ok(Self::CableSatelliteAndOtherPayTelevisionAndRadio)
            }
            "camera_and_photographic_supply_stores" => Ok(Self::CameraAndPhotographicSupplyStores),
            "candy_nut_and_confectionery_stores" => Ok(Self::CandyNutAndConfectioneryStores),
            "car_and_truck_dealers_new_used" => Ok(Self::CarAndTruckDealersNewUsed),
            "car_and_truck_dealers_used_only" => Ok(Self::CarAndTruckDealersUsedOnly),
            "car_rental_agencies" => Ok(Self::CarRentalAgencies),
            "car_washes" => Ok(Self::CarWashes),
            "carpentry_services" => Ok(Self::CarpentryServices),
            "carpet_upholstery_cleaning" => Ok(Self::CarpetUpholsteryCleaning),
            "caterers" => Ok(Self::Caterers),
            "charitable_and_social_service_organizations_fundraising" => {
                Ok(Self::CharitableAndSocialServiceOrganizationsFundraising)
            }
            "chemicals_and_allied_products" => Ok(Self::ChemicalsAndAlliedProducts),
            "child_care_services" => Ok(Self::ChildCareServices),
            "childrens_and_infants_wear_stores" => Ok(Self::ChildrensAndInfantsWearStores),
            "chiropodists_podiatrists" => Ok(Self::ChiropodistsPodiatrists),
            "chiropractors" => Ok(Self::Chiropractors),
            "cigar_stores_and_stands" => Ok(Self::CigarStoresAndStands),
            "civic_social_fraternal_associations" => Ok(Self::CivicSocialFraternalAssociations),
            "cleaning_and_maintenance" => Ok(Self::CleaningAndMaintenance),
            "clothing_rental" => Ok(Self::ClothingRental),
            "colleges_universities" => Ok(Self::CollegesUniversities),
            "commercial_equipment" => Ok(Self::CommercialEquipment),
            "commercial_footwear" => Ok(Self::CommercialFootwear),
            "commercial_photography_art_and_graphics" => {
                Ok(Self::CommercialPhotographyArtAndGraphics)
            }
            "commuter_transport_and_ferries" => Ok(Self::CommuterTransportAndFerries),
            "computer_network_services" => Ok(Self::ComputerNetworkServices),
            "computer_programming" => Ok(Self::ComputerProgramming),
            "computer_repair" => Ok(Self::ComputerRepair),
            "computer_software_stores" => Ok(Self::ComputerSoftwareStores),
            "computers_peripherals_and_software" => Ok(Self::ComputersPeripheralsAndSoftware),
            "concrete_work_services" => Ok(Self::ConcreteWorkServices),
            "construction_materials" => Ok(Self::ConstructionMaterials),
            "consulting_public_relations" => Ok(Self::ConsultingPublicRelations),
            "correspondence_schools" => Ok(Self::CorrespondenceSchools),
            "cosmetic_stores" => Ok(Self::CosmeticStores),
            "counseling_services" => Ok(Self::CounselingServices),
            "country_clubs" => Ok(Self::CountryClubs),
            "courier_services" => Ok(Self::CourierServices),
            "court_costs" => Ok(Self::CourtCosts),
            "credit_reporting_agencies" => Ok(Self::CreditReportingAgencies),
            "cruise_lines" => Ok(Self::CruiseLines),
            "dairy_products_stores" => Ok(Self::DairyProductsStores),
            "dance_hall_studios_schools" => Ok(Self::DanceHallStudiosSchools),
            "dating_escort_services" => Ok(Self::DatingEscortServices),
            "dentists_orthodontists" => Ok(Self::DentistsOrthodontists),
            "department_stores" => Ok(Self::DepartmentStores),
            "detective_agencies" => Ok(Self::DetectiveAgencies),
            "digital_goods_applications" => Ok(Self::DigitalGoodsApplications),
            "digital_goods_games" => Ok(Self::DigitalGoodsGames),
            "digital_goods_large_volume" => Ok(Self::DigitalGoodsLargeVolume),
            "digital_goods_media" => Ok(Self::DigitalGoodsMedia),
            "direct_marketing_catalog_merchant" => Ok(Self::DirectMarketingCatalogMerchant),
            "direct_marketing_combination_catalog_and_retail_merchant" => {
                Ok(Self::DirectMarketingCombinationCatalogAndRetailMerchant)
            }
            "direct_marketing_inbound_telemarketing" => {
                Ok(Self::DirectMarketingInboundTelemarketing)
            }
            "direct_marketing_insurance_services" => Ok(Self::DirectMarketingInsuranceServices),
            "direct_marketing_other" => Ok(Self::DirectMarketingOther),
            "direct_marketing_outbound_telemarketing" => {
                Ok(Self::DirectMarketingOutboundTelemarketing)
            }
            "direct_marketing_subscription" => Ok(Self::DirectMarketingSubscription),
            "direct_marketing_travel" => Ok(Self::DirectMarketingTravel),
            "discount_stores" => Ok(Self::DiscountStores),
            "doctors" => Ok(Self::Doctors),
            "door_to_door_sales" => Ok(Self::DoorToDoorSales),
            "drapery_window_covering_and_upholstery_stores" => {
                Ok(Self::DraperyWindowCoveringAndUpholsteryStores)
            }
            "drinking_places" => Ok(Self::DrinkingPlaces),
            "drug_stores_and_pharmacies" => Ok(Self::DrugStoresAndPharmacies),
            "drugs_drug_proprietaries_and_druggist_sundries" => {
                Ok(Self::DrugsDrugProprietariesAndDruggistSundries)
            }
            "dry_cleaners" => Ok(Self::DryCleaners),
            "durable_goods" => Ok(Self::DurableGoods),
            "duty_free_stores" => Ok(Self::DutyFreeStores),
            "eating_places_restaurants" => Ok(Self::EatingPlacesRestaurants),
            "educational_services" => Ok(Self::EducationalServices),
            "electric_razor_stores" => Ok(Self::ElectricRazorStores),
            "electrical_parts_and_equipment" => Ok(Self::ElectricalPartsAndEquipment),
            "electrical_services" => Ok(Self::ElectricalServices),
            "electronics_repair_shops" => Ok(Self::ElectronicsRepairShops),
            "electronics_stores" => Ok(Self::ElectronicsStores),
            "elementary_secondary_schools" => Ok(Self::ElementarySecondarySchools),
            "employment_temp_agencies" => Ok(Self::EmploymentTempAgencies),
            "equipment_rental" => Ok(Self::EquipmentRental),
            "exterminating_services" => Ok(Self::ExterminatingServices),
            "family_clothing_stores" => Ok(Self::FamilyClothingStores),
            "fast_food_restaurants" => Ok(Self::FastFoodRestaurants),
            "financial_institutions" => Ok(Self::FinancialInstitutions),
            "fines_government_administrative_entities" => {
                Ok(Self::FinesGovernmentAdministrativeEntities)
            }
            "fireplace_fireplace_screens_and_accessories_stores" => {
                Ok(Self::FireplaceFireplaceScreensAndAccessoriesStores)
            }
            "floor_covering_stores" => Ok(Self::FloorCoveringStores),
            "florists" => Ok(Self::Florists),
            "florists_supplies_nursery_stock_and_flowers" => {
                Ok(Self::FloristsSuppliesNurseryStockAndFlowers)
            }
            "freezer_and_locker_meat_provisioners" => Ok(Self::FreezerAndLockerMeatProvisioners),
            "fuel_dealers_non_automotive" => Ok(Self::FuelDealersNonAutomotive),
            "funeral_services_crematories" => Ok(Self::FuneralServicesCrematories),
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => {
                Ok(Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances)
            }
            "furniture_repair_refinishing" => Ok(Self::FurnitureRepairRefinishing),
            "furriers_and_fur_shops" => Ok(Self::FurriersAndFurShops),
            "general_services" => Ok(Self::GeneralServices),
            "gift_card_novelty_and_souvenir_shops" => Ok(Self::GiftCardNoveltyAndSouvenirShops),
            "glass_paint_and_wallpaper_stores" => Ok(Self::GlassPaintAndWallpaperStores),
            "glassware_crystal_stores" => Ok(Self::GlasswareCrystalStores),
            "golf_courses_public" => Ok(Self::GolfCoursesPublic),
            "government_services" => Ok(Self::GovernmentServices),
            "grocery_stores_supermarkets" => Ok(Self::GroceryStoresSupermarkets),
            "hardware_equipment_and_supplies" => Ok(Self::HardwareEquipmentAndSupplies),
            "hardware_stores" => Ok(Self::HardwareStores),
            "health_and_beauty_spas" => Ok(Self::HealthAndBeautySpas),
            "hearing_aids_sales_and_supplies" => Ok(Self::HearingAidsSalesAndSupplies),
            "heating_plumbing_a_c" => Ok(Self::HeatingPlumbingAC),
            "hobby_toy_and_game_shops" => Ok(Self::HobbyToyAndGameShops),
            "home_supply_warehouse_stores" => Ok(Self::HomeSupplyWarehouseStores),
            "hospitals" => Ok(Self::Hospitals),
            "hotels_motels_and_resorts" => Ok(Self::HotelsMotelsAndResorts),
            "household_appliance_stores" => Ok(Self::HouseholdApplianceStores),
            "industrial_supplies" => Ok(Self::IndustrialSupplies),
            "information_retrieval_services" => Ok(Self::InformationRetrievalServices),
            "insurance_default" => Ok(Self::InsuranceDefault),
            "insurance_underwriting_premiums" => Ok(Self::InsuranceUnderwritingPremiums),
            "intra_company_purchases" => Ok(Self::IntraCompanyPurchases),
            "jewelry_stores_watches_clocks_and_silverware_stores" => {
                Ok(Self::JewelryStoresWatchesClocksAndSilverwareStores)
            }
            "landscaping_services" => Ok(Self::LandscapingServices),
            "laundries" => Ok(Self::Laundries),
            "laundry_cleaning_services" => Ok(Self::LaundryCleaningServices),
            "legal_services_attorneys" => Ok(Self::LegalServicesAttorneys),
            "luggage_and_leather_goods_stores" => Ok(Self::LuggageAndLeatherGoodsStores),
            "lumber_building_materials_stores" => Ok(Self::LumberBuildingMaterialsStores),
            "manual_cash_disburse" => Ok(Self::ManualCashDisburse),
            "marinas_service_and_supplies" => Ok(Self::MarinasServiceAndSupplies),
            "masonry_stonework_and_plaster" => Ok(Self::MasonryStoneworkAndPlaster),
            "massage_parlors" => Ok(Self::MassageParlors),
            "medical_and_dental_labs" => Ok(Self::MedicalAndDentalLabs),
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => {
                Ok(Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies)
            }
            "medical_services" => Ok(Self::MedicalServices),
            "membership_organizations" => Ok(Self::MembershipOrganizations),
            "mens_and_boys_clothing_and_accessories_stores" => {
                Ok(Self::MensAndBoysClothingAndAccessoriesStores)
            }
            "mens_womens_clothing_stores" => Ok(Self::MensWomensClothingStores),
            "metal_service_centers" => Ok(Self::MetalServiceCenters),
            "miscellaneous" => Ok(Self::Miscellaneous),
            "miscellaneous_apparel_and_accessory_shops" => {
                Ok(Self::MiscellaneousApparelAndAccessoryShops)
            }
            "miscellaneous_auto_dealers" => Ok(Self::MiscellaneousAutoDealers),
            "miscellaneous_business_services" => Ok(Self::MiscellaneousBusinessServices),
            "miscellaneous_food_stores" => Ok(Self::MiscellaneousFoodStores),
            "miscellaneous_general_merchandise" => Ok(Self::MiscellaneousGeneralMerchandise),
            "miscellaneous_general_services" => Ok(Self::MiscellaneousGeneralServices),
            "miscellaneous_home_furnishing_specialty_stores" => {
                Ok(Self::MiscellaneousHomeFurnishingSpecialtyStores)
            }
            "miscellaneous_publishing_and_printing" => Ok(Self::MiscellaneousPublishingAndPrinting),
            "miscellaneous_recreation_services" => Ok(Self::MiscellaneousRecreationServices),
            "miscellaneous_repair_shops" => Ok(Self::MiscellaneousRepairShops),
            "miscellaneous_specialty_retail" => Ok(Self::MiscellaneousSpecialtyRetail),
            "mobile_home_dealers" => Ok(Self::MobileHomeDealers),
            "motion_picture_theaters" => Ok(Self::MotionPictureTheaters),
            "motor_freight_carriers_and_trucking" => Ok(Self::MotorFreightCarriersAndTrucking),
            "motor_homes_dealers" => Ok(Self::MotorHomesDealers),
            "motor_vehicle_supplies_and_new_parts" => Ok(Self::MotorVehicleSuppliesAndNewParts),
            "motorcycle_shops_and_dealers" => Ok(Self::MotorcycleShopsAndDealers),
            "motorcycle_shops_dealers" => Ok(Self::MotorcycleShopsDealers),
            "music_stores_musical_instruments_pianos_and_sheet_music" => {
                Ok(Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic)
            }
            "news_dealers_and_newsstands" => Ok(Self::NewsDealersAndNewsstands),
            "non_fi_money_orders" => Ok(Self::NonFiMoneyOrders),
            "non_fi_stored_value_card_purchase_load" => Ok(Self::NonFiStoredValueCardPurchaseLoad),
            "nondurable_goods" => Ok(Self::NondurableGoods),
            "nurseries_lawn_and_garden_supply_stores" => {
                Ok(Self::NurseriesLawnAndGardenSupplyStores)
            }
            "nursing_personal_care" => Ok(Self::NursingPersonalCare),
            "office_and_commercial_furniture" => Ok(Self::OfficeAndCommercialFurniture),
            "opticians_eyeglasses" => Ok(Self::OpticiansEyeglasses),
            "optometrists_ophthalmologist" => Ok(Self::OptometristsOphthalmologist),
            "orthopedic_goods_prosthetic_devices" => Ok(Self::OrthopedicGoodsProstheticDevices),
            "osteopaths" => Ok(Self::Osteopaths),
            "package_stores_beer_wine_and_liquor" => Ok(Self::PackageStoresBeerWineAndLiquor),
            "paints_varnishes_and_supplies" => Ok(Self::PaintsVarnishesAndSupplies),
            "parking_lots_garages" => Ok(Self::ParkingLotsGarages),
            "passenger_railways" => Ok(Self::PassengerRailways),
            "pawn_shops" => Ok(Self::PawnShops),
            "pet_shops_pet_food_and_supplies" => Ok(Self::PetShopsPetFoodAndSupplies),
            "petroleum_and_petroleum_products" => Ok(Self::PetroleumAndPetroleumProducts),
            "photo_developing" => Ok(Self::PhotoDeveloping),
            "photographic_photocopy_microfilm_equipment_and_supplies" => {
                Ok(Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies)
            }
            "photographic_studios" => Ok(Self::PhotographicStudios),
            "picture_video_production" => Ok(Self::PictureVideoProduction),
            "piece_goods_notions_and_other_dry_goods" => {
                Ok(Self::PieceGoodsNotionsAndOtherDryGoods)
            }
            "plumbing_heating_equipment_and_supplies" => {
                Ok(Self::PlumbingHeatingEquipmentAndSupplies)
            }
            "political_organizations" => Ok(Self::PoliticalOrganizations),
            "postal_services_government_only" => Ok(Self::PostalServicesGovernmentOnly),
            "precious_stones_and_metals_watches_and_jewelry" => {
                Ok(Self::PreciousStonesAndMetalsWatchesAndJewelry)
            }
            "professional_services" => Ok(Self::ProfessionalServices),
            "public_warehousing_and_storage" => Ok(Self::PublicWarehousingAndStorage),
            "quick_copy_repro_and_blueprint" => Ok(Self::QuickCopyReproAndBlueprint),
            "railroads" => Ok(Self::Railroads),
            "real_estate_agents_and_managers_rentals" => {
                Ok(Self::RealEstateAgentsAndManagersRentals)
            }
            "record_stores" => Ok(Self::RecordStores),
            "recreational_vehicle_rentals" => Ok(Self::RecreationalVehicleRentals),
            "religious_goods_stores" => Ok(Self::ReligiousGoodsStores),
            "religious_organizations" => Ok(Self::ReligiousOrganizations),
            "roofing_siding_sheet_metal" => Ok(Self::RoofingSidingSheetMetal),
            "secretarial_support_services" => Ok(Self::SecretarialSupportServices),
            "security_brokers_dealers" => Ok(Self::SecurityBrokersDealers),
            "service_stations" => Ok(Self::ServiceStations),
            "sewing_needlework_fabric_and_piece_goods_stores" => {
                Ok(Self::SewingNeedleworkFabricAndPieceGoodsStores)
            }
            "shoe_repair_hat_cleaning" => Ok(Self::ShoeRepairHatCleaning),
            "shoe_stores" => Ok(Self::ShoeStores),
            "small_appliance_repair" => Ok(Self::SmallApplianceRepair),
            "snowmobile_dealers" => Ok(Self::SnowmobileDealers),
            "special_trade_services" => Ok(Self::SpecialTradeServices),
            "specialty_cleaning" => Ok(Self::SpecialtyCleaning),
            "sporting_goods_stores" => Ok(Self::SportingGoodsStores),
            "sporting_recreation_camps" => Ok(Self::SportingRecreationCamps),
            "sports_and_riding_apparel_stores" => Ok(Self::SportsAndRidingApparelStores),
            "sports_clubs_fields" => Ok(Self::SportsClubsFields),
            "stamp_and_coin_stores" => Ok(Self::StampAndCoinStores),
            "stationary_office_supplies_printing_and_writing_paper" => {
                Ok(Self::StationaryOfficeSuppliesPrintingAndWritingPaper)
            }
            "stationery_stores_office_and_school_supply_stores" => {
                Ok(Self::StationeryStoresOfficeAndSchoolSupplyStores)
            }
            "swimming_pools_sales" => Ok(Self::SwimmingPoolsSales),
            "t_ui_travel_germany" => Ok(Self::TUiTravelGermany),
            "tailors_alterations" => Ok(Self::TailorsAlterations),
            "tax_payments_government_agencies" => Ok(Self::TaxPaymentsGovernmentAgencies),
            "tax_preparation_services" => Ok(Self::TaxPreparationServices),
            "taxicabs_limousines" => Ok(Self::TaxicabsLimousines),
            "telecommunication_equipment_and_telephone_sales" => {
                Ok(Self::TelecommunicationEquipmentAndTelephoneSales)
            }
            "telecommunication_services" => Ok(Self::TelecommunicationServices),
            "telegraph_services" => Ok(Self::TelegraphServices),
            "tent_and_awning_shops" => Ok(Self::TentAndAwningShops),
            "testing_laboratories" => Ok(Self::TestingLaboratories),
            "theatrical_ticket_agencies" => Ok(Self::TheatricalTicketAgencies),
            "timeshares" => Ok(Self::Timeshares),
            "tire_retreading_and_repair" => Ok(Self::TireRetreadingAndRepair),
            "tolls_bridge_fees" => Ok(Self::TollsBridgeFees),
            "tourist_attractions_and_exhibits" => Ok(Self::TouristAttractionsAndExhibits),
            "towing_services" => Ok(Self::TowingServices),
            "trailer_parks_campgrounds" => Ok(Self::TrailerParksCampgrounds),
            "transportation_services" => Ok(Self::TransportationServices),
            "travel_agencies_tour_operators" => Ok(Self::TravelAgenciesTourOperators),
            "truck_stop_iteration" => Ok(Self::TruckStopIteration),
            "truck_utility_trailer_rentals" => Ok(Self::TruckUtilityTrailerRentals),
            "typesetting_plate_making_and_related_services" => {
                Ok(Self::TypesettingPlateMakingAndRelatedServices)
            }
            "typewriter_stores" => Ok(Self::TypewriterStores),
            "u_s_federal_government_agencies_or_departments" => {
                Ok(Self::USFederalGovernmentAgenciesOrDepartments)
            }
            "uniforms_commercial_clothing" => Ok(Self::UniformsCommercialClothing),
            "used_merchandise_and_secondhand_stores" => {
                Ok(Self::UsedMerchandiseAndSecondhandStores)
            }
            "utilities" => Ok(Self::Utilities),
            "variety_stores" => Ok(Self::VarietyStores),
            "veterinary_services" => Ok(Self::VeterinaryServices),
            "video_amusement_game_supplies" => Ok(Self::VideoAmusementGameSupplies),
            "video_game_arcades" => Ok(Self::VideoGameArcades),
            "video_tape_rental_stores" => Ok(Self::VideoTapeRentalStores),
            "vocational_trade_schools" => Ok(Self::VocationalTradeSchools),
            "watch_jewelry_repair" => Ok(Self::WatchJewelryRepair),
            "welding_repair" => Ok(Self::WeldingRepair),
            "wholesale_clubs" => Ok(Self::WholesaleClubs),
            "wig_and_toupee_stores" => Ok(Self::WigAndToupeeStores),
            "wires_money_orders" => Ok(Self::WiresMoneyOrders),
            "womens_accessory_and_specialty_shops" => Ok(Self::WomensAccessoryAndSpecialtyShops),
            "womens_ready_to_wear_stores" => Ok(Self::WomensReadyToWearStores),
            "wrecking_and_salvage_yards" => Ok(Self::WreckingAndSalvageYards),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardSpendingControlsSpendingLimitsCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardSpendingControlsSpendingLimitsCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardSpendingControlsSpendingLimitsCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Interval (or event) to which the amount applies.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardSpendingControlsSpendingLimitsInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

impl CreateCardSpendingControlsSpendingLimitsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllTime => "all_time",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::PerAuthorization => "per_authorization",
            Self::Weekly => "weekly",
            Self::Yearly => "yearly",
        }
    }
}

impl std::str::FromStr for CreateCardSpendingControlsSpendingLimitsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all_time" => Ok(Self::AllTime),
            "daily" => Ok(Self::Daily),
            "monthly" => Ok(Self::Monthly),
            "per_authorization" => Ok(Self::PerAuthorization),
            "weekly" => Ok(Self::Weekly),
            "yearly" => Ok(Self::Yearly),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardSpendingControlsSpendingLimitsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardSpendingControlsSpendingLimitsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardSpendingControlsSpendingLimitsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Whether authorizations can be approved on this card.
///
/// Defaults to `inactive`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardStatus {
    Active,
    Inactive,
}

impl CreateCardStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for CreateCardStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The type of card to issue.
///
/// Possible values are `physical` or `virtual`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateCardType {
    Physical,
    Virtual,
}

impl CreateCardType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Virtual => "virtual",
        }
    }
}

impl std::str::FromStr for CreateCardType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "physical" => Ok(Self::Physical),
            "virtual" => Ok(Self::Virtual),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateCardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateCardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveCard<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCard<'a> {
    /// Reason why the `status` of this card is `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<UpdateCardCancellationReason>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The desired new PIN for this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<UpdateCardPin<'a>>,
    /// Rules that control spending for this card.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<UpdateCardSpendingControls<'a>>,
    /// Dictates whether authorizations can be approved on this card.
    ///
    /// If this card is being canceled because it was lost or stolen, this information should be provided as `cancellation_reason`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateCardStatus>,
}
impl<'a> UpdateCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Reason why the `status` of this card is `canceled`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateCardCancellationReason {
    Lost,
    Stolen,
}

impl UpdateCardCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lost => "lost",
            Self::Stolen => "stolen",
        }
    }
}

impl std::str::FromStr for UpdateCardCancellationReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lost" => Ok(Self::Lost),
            "stolen" => Ok(Self::Stolen),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateCardCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateCardCancellationReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The desired new PIN for this card.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCardPin<'a> {
    /// The card's desired new PIN, encrypted under Stripe's public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_number: Option<&'a str>,
}
impl<'a> UpdateCardPin<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Rules that control spending for this card.
///
/// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateCardSpendingControls<'a> {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    ///
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<&'a [UpdateCardSpendingControlsAllowedCategories]>,
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    ///
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<&'a [UpdateCardSpendingControlsBlockedCategories]>,
    /// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<&'a [UpdateCardSpendingControlsSpendingLimits<'a>]>,
}
impl<'a> UpdateCardSpendingControls<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
///
/// All other categories will be blocked.
/// Cannot be set with `blocked_categories`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateCardSpendingControlsAllowedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl UpdateCardSpendingControlsAllowedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl std::str::FromStr for UpdateCardSpendingControlsAllowedCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ac_refrigeration_repair" => Ok(Self::AcRefrigerationRepair),
            "accounting_bookkeeping_services" => Ok(Self::AccountingBookkeepingServices),
            "advertising_services" => Ok(Self::AdvertisingServices),
            "agricultural_cooperative" => Ok(Self::AgriculturalCooperative),
            "airlines_air_carriers" => Ok(Self::AirlinesAirCarriers),
            "airports_flying_fields" => Ok(Self::AirportsFlyingFields),
            "ambulance_services" => Ok(Self::AmbulanceServices),
            "amusement_parks_carnivals" => Ok(Self::AmusementParksCarnivals),
            "antique_reproductions" => Ok(Self::AntiqueReproductions),
            "antique_shops" => Ok(Self::AntiqueShops),
            "aquariums" => Ok(Self::Aquariums),
            "architectural_surveying_services" => Ok(Self::ArchitecturalSurveyingServices),
            "art_dealers_and_galleries" => Ok(Self::ArtDealersAndGalleries),
            "artists_supply_and_craft_shops" => Ok(Self::ArtistsSupplyAndCraftShops),
            "auto_and_home_supply_stores" => Ok(Self::AutoAndHomeSupplyStores),
            "auto_body_repair_shops" => Ok(Self::AutoBodyRepairShops),
            "auto_paint_shops" => Ok(Self::AutoPaintShops),
            "auto_service_shops" => Ok(Self::AutoServiceShops),
            "automated_cash_disburse" => Ok(Self::AutomatedCashDisburse),
            "automated_fuel_dispensers" => Ok(Self::AutomatedFuelDispensers),
            "automobile_associations" => Ok(Self::AutomobileAssociations),
            "automotive_parts_and_accessories_stores" => {
                Ok(Self::AutomotivePartsAndAccessoriesStores)
            }
            "automotive_tire_stores" => Ok(Self::AutomotiveTireStores),
            "bail_and_bond_payments" => Ok(Self::BailAndBondPayments),
            "bakeries" => Ok(Self::Bakeries),
            "bands_orchestras" => Ok(Self::BandsOrchestras),
            "barber_and_beauty_shops" => Ok(Self::BarberAndBeautyShops),
            "betting_casino_gambling" => Ok(Self::BettingCasinoGambling),
            "bicycle_shops" => Ok(Self::BicycleShops),
            "billiard_pool_establishments" => Ok(Self::BilliardPoolEstablishments),
            "boat_dealers" => Ok(Self::BoatDealers),
            "boat_rentals_and_leases" => Ok(Self::BoatRentalsAndLeases),
            "book_stores" => Ok(Self::BookStores),
            "books_periodicals_and_newspapers" => Ok(Self::BooksPeriodicalsAndNewspapers),
            "bowling_alleys" => Ok(Self::BowlingAlleys),
            "bus_lines" => Ok(Self::BusLines),
            "business_secretarial_schools" => Ok(Self::BusinessSecretarialSchools),
            "buying_shopping_services" => Ok(Self::BuyingShoppingServices),
            "cable_satellite_and_other_pay_television_and_radio" => {
                Ok(Self::CableSatelliteAndOtherPayTelevisionAndRadio)
            }
            "camera_and_photographic_supply_stores" => Ok(Self::CameraAndPhotographicSupplyStores),
            "candy_nut_and_confectionery_stores" => Ok(Self::CandyNutAndConfectioneryStores),
            "car_and_truck_dealers_new_used" => Ok(Self::CarAndTruckDealersNewUsed),
            "car_and_truck_dealers_used_only" => Ok(Self::CarAndTruckDealersUsedOnly),
            "car_rental_agencies" => Ok(Self::CarRentalAgencies),
            "car_washes" => Ok(Self::CarWashes),
            "carpentry_services" => Ok(Self::CarpentryServices),
            "carpet_upholstery_cleaning" => Ok(Self::CarpetUpholsteryCleaning),
            "caterers" => Ok(Self::Caterers),
            "charitable_and_social_service_organizations_fundraising" => {
                Ok(Self::CharitableAndSocialServiceOrganizationsFundraising)
            }
            "chemicals_and_allied_products" => Ok(Self::ChemicalsAndAlliedProducts),
            "child_care_services" => Ok(Self::ChildCareServices),
            "childrens_and_infants_wear_stores" => Ok(Self::ChildrensAndInfantsWearStores),
            "chiropodists_podiatrists" => Ok(Self::ChiropodistsPodiatrists),
            "chiropractors" => Ok(Self::Chiropractors),
            "cigar_stores_and_stands" => Ok(Self::CigarStoresAndStands),
            "civic_social_fraternal_associations" => Ok(Self::CivicSocialFraternalAssociations),
            "cleaning_and_maintenance" => Ok(Self::CleaningAndMaintenance),
            "clothing_rental" => Ok(Self::ClothingRental),
            "colleges_universities" => Ok(Self::CollegesUniversities),
            "commercial_equipment" => Ok(Self::CommercialEquipment),
            "commercial_footwear" => Ok(Self::CommercialFootwear),
            "commercial_photography_art_and_graphics" => {
                Ok(Self::CommercialPhotographyArtAndGraphics)
            }
            "commuter_transport_and_ferries" => Ok(Self::CommuterTransportAndFerries),
            "computer_network_services" => Ok(Self::ComputerNetworkServices),
            "computer_programming" => Ok(Self::ComputerProgramming),
            "computer_repair" => Ok(Self::ComputerRepair),
            "computer_software_stores" => Ok(Self::ComputerSoftwareStores),
            "computers_peripherals_and_software" => Ok(Self::ComputersPeripheralsAndSoftware),
            "concrete_work_services" => Ok(Self::ConcreteWorkServices),
            "construction_materials" => Ok(Self::ConstructionMaterials),
            "consulting_public_relations" => Ok(Self::ConsultingPublicRelations),
            "correspondence_schools" => Ok(Self::CorrespondenceSchools),
            "cosmetic_stores" => Ok(Self::CosmeticStores),
            "counseling_services" => Ok(Self::CounselingServices),
            "country_clubs" => Ok(Self::CountryClubs),
            "courier_services" => Ok(Self::CourierServices),
            "court_costs" => Ok(Self::CourtCosts),
            "credit_reporting_agencies" => Ok(Self::CreditReportingAgencies),
            "cruise_lines" => Ok(Self::CruiseLines),
            "dairy_products_stores" => Ok(Self::DairyProductsStores),
            "dance_hall_studios_schools" => Ok(Self::DanceHallStudiosSchools),
            "dating_escort_services" => Ok(Self::DatingEscortServices),
            "dentists_orthodontists" => Ok(Self::DentistsOrthodontists),
            "department_stores" => Ok(Self::DepartmentStores),
            "detective_agencies" => Ok(Self::DetectiveAgencies),
            "digital_goods_applications" => Ok(Self::DigitalGoodsApplications),
            "digital_goods_games" => Ok(Self::DigitalGoodsGames),
            "digital_goods_large_volume" => Ok(Self::DigitalGoodsLargeVolume),
            "digital_goods_media" => Ok(Self::DigitalGoodsMedia),
            "direct_marketing_catalog_merchant" => Ok(Self::DirectMarketingCatalogMerchant),
            "direct_marketing_combination_catalog_and_retail_merchant" => {
                Ok(Self::DirectMarketingCombinationCatalogAndRetailMerchant)
            }
            "direct_marketing_inbound_telemarketing" => {
                Ok(Self::DirectMarketingInboundTelemarketing)
            }
            "direct_marketing_insurance_services" => Ok(Self::DirectMarketingInsuranceServices),
            "direct_marketing_other" => Ok(Self::DirectMarketingOther),
            "direct_marketing_outbound_telemarketing" => {
                Ok(Self::DirectMarketingOutboundTelemarketing)
            }
            "direct_marketing_subscription" => Ok(Self::DirectMarketingSubscription),
            "direct_marketing_travel" => Ok(Self::DirectMarketingTravel),
            "discount_stores" => Ok(Self::DiscountStores),
            "doctors" => Ok(Self::Doctors),
            "door_to_door_sales" => Ok(Self::DoorToDoorSales),
            "drapery_window_covering_and_upholstery_stores" => {
                Ok(Self::DraperyWindowCoveringAndUpholsteryStores)
            }
            "drinking_places" => Ok(Self::DrinkingPlaces),
            "drug_stores_and_pharmacies" => Ok(Self::DrugStoresAndPharmacies),
            "drugs_drug_proprietaries_and_druggist_sundries" => {
                Ok(Self::DrugsDrugProprietariesAndDruggistSundries)
            }
            "dry_cleaners" => Ok(Self::DryCleaners),
            "durable_goods" => Ok(Self::DurableGoods),
            "duty_free_stores" => Ok(Self::DutyFreeStores),
            "eating_places_restaurants" => Ok(Self::EatingPlacesRestaurants),
            "educational_services" => Ok(Self::EducationalServices),
            "electric_razor_stores" => Ok(Self::ElectricRazorStores),
            "electrical_parts_and_equipment" => Ok(Self::ElectricalPartsAndEquipment),
            "electrical_services" => Ok(Self::ElectricalServices),
            "electronics_repair_shops" => Ok(Self::ElectronicsRepairShops),
            "electronics_stores" => Ok(Self::ElectronicsStores),
            "elementary_secondary_schools" => Ok(Self::ElementarySecondarySchools),
            "employment_temp_agencies" => Ok(Self::EmploymentTempAgencies),
            "equipment_rental" => Ok(Self::EquipmentRental),
            "exterminating_services" => Ok(Self::ExterminatingServices),
            "family_clothing_stores" => Ok(Self::FamilyClothingStores),
            "fast_food_restaurants" => Ok(Self::FastFoodRestaurants),
            "financial_institutions" => Ok(Self::FinancialInstitutions),
            "fines_government_administrative_entities" => {
                Ok(Self::FinesGovernmentAdministrativeEntities)
            }
            "fireplace_fireplace_screens_and_accessories_stores" => {
                Ok(Self::FireplaceFireplaceScreensAndAccessoriesStores)
            }
            "floor_covering_stores" => Ok(Self::FloorCoveringStores),
            "florists" => Ok(Self::Florists),
            "florists_supplies_nursery_stock_and_flowers" => {
                Ok(Self::FloristsSuppliesNurseryStockAndFlowers)
            }
            "freezer_and_locker_meat_provisioners" => Ok(Self::FreezerAndLockerMeatProvisioners),
            "fuel_dealers_non_automotive" => Ok(Self::FuelDealersNonAutomotive),
            "funeral_services_crematories" => Ok(Self::FuneralServicesCrematories),
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => {
                Ok(Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances)
            }
            "furniture_repair_refinishing" => Ok(Self::FurnitureRepairRefinishing),
            "furriers_and_fur_shops" => Ok(Self::FurriersAndFurShops),
            "general_services" => Ok(Self::GeneralServices),
            "gift_card_novelty_and_souvenir_shops" => Ok(Self::GiftCardNoveltyAndSouvenirShops),
            "glass_paint_and_wallpaper_stores" => Ok(Self::GlassPaintAndWallpaperStores),
            "glassware_crystal_stores" => Ok(Self::GlasswareCrystalStores),
            "golf_courses_public" => Ok(Self::GolfCoursesPublic),
            "government_services" => Ok(Self::GovernmentServices),
            "grocery_stores_supermarkets" => Ok(Self::GroceryStoresSupermarkets),
            "hardware_equipment_and_supplies" => Ok(Self::HardwareEquipmentAndSupplies),
            "hardware_stores" => Ok(Self::HardwareStores),
            "health_and_beauty_spas" => Ok(Self::HealthAndBeautySpas),
            "hearing_aids_sales_and_supplies" => Ok(Self::HearingAidsSalesAndSupplies),
            "heating_plumbing_a_c" => Ok(Self::HeatingPlumbingAC),
            "hobby_toy_and_game_shops" => Ok(Self::HobbyToyAndGameShops),
            "home_supply_warehouse_stores" => Ok(Self::HomeSupplyWarehouseStores),
            "hospitals" => Ok(Self::Hospitals),
            "hotels_motels_and_resorts" => Ok(Self::HotelsMotelsAndResorts),
            "household_appliance_stores" => Ok(Self::HouseholdApplianceStores),
            "industrial_supplies" => Ok(Self::IndustrialSupplies),
            "information_retrieval_services" => Ok(Self::InformationRetrievalServices),
            "insurance_default" => Ok(Self::InsuranceDefault),
            "insurance_underwriting_premiums" => Ok(Self::InsuranceUnderwritingPremiums),
            "intra_company_purchases" => Ok(Self::IntraCompanyPurchases),
            "jewelry_stores_watches_clocks_and_silverware_stores" => {
                Ok(Self::JewelryStoresWatchesClocksAndSilverwareStores)
            }
            "landscaping_services" => Ok(Self::LandscapingServices),
            "laundries" => Ok(Self::Laundries),
            "laundry_cleaning_services" => Ok(Self::LaundryCleaningServices),
            "legal_services_attorneys" => Ok(Self::LegalServicesAttorneys),
            "luggage_and_leather_goods_stores" => Ok(Self::LuggageAndLeatherGoodsStores),
            "lumber_building_materials_stores" => Ok(Self::LumberBuildingMaterialsStores),
            "manual_cash_disburse" => Ok(Self::ManualCashDisburse),
            "marinas_service_and_supplies" => Ok(Self::MarinasServiceAndSupplies),
            "masonry_stonework_and_plaster" => Ok(Self::MasonryStoneworkAndPlaster),
            "massage_parlors" => Ok(Self::MassageParlors),
            "medical_and_dental_labs" => Ok(Self::MedicalAndDentalLabs),
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => {
                Ok(Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies)
            }
            "medical_services" => Ok(Self::MedicalServices),
            "membership_organizations" => Ok(Self::MembershipOrganizations),
            "mens_and_boys_clothing_and_accessories_stores" => {
                Ok(Self::MensAndBoysClothingAndAccessoriesStores)
            }
            "mens_womens_clothing_stores" => Ok(Self::MensWomensClothingStores),
            "metal_service_centers" => Ok(Self::MetalServiceCenters),
            "miscellaneous" => Ok(Self::Miscellaneous),
            "miscellaneous_apparel_and_accessory_shops" => {
                Ok(Self::MiscellaneousApparelAndAccessoryShops)
            }
            "miscellaneous_auto_dealers" => Ok(Self::MiscellaneousAutoDealers),
            "miscellaneous_business_services" => Ok(Self::MiscellaneousBusinessServices),
            "miscellaneous_food_stores" => Ok(Self::MiscellaneousFoodStores),
            "miscellaneous_general_merchandise" => Ok(Self::MiscellaneousGeneralMerchandise),
            "miscellaneous_general_services" => Ok(Self::MiscellaneousGeneralServices),
            "miscellaneous_home_furnishing_specialty_stores" => {
                Ok(Self::MiscellaneousHomeFurnishingSpecialtyStores)
            }
            "miscellaneous_publishing_and_printing" => Ok(Self::MiscellaneousPublishingAndPrinting),
            "miscellaneous_recreation_services" => Ok(Self::MiscellaneousRecreationServices),
            "miscellaneous_repair_shops" => Ok(Self::MiscellaneousRepairShops),
            "miscellaneous_specialty_retail" => Ok(Self::MiscellaneousSpecialtyRetail),
            "mobile_home_dealers" => Ok(Self::MobileHomeDealers),
            "motion_picture_theaters" => Ok(Self::MotionPictureTheaters),
            "motor_freight_carriers_and_trucking" => Ok(Self::MotorFreightCarriersAndTrucking),
            "motor_homes_dealers" => Ok(Self::MotorHomesDealers),
            "motor_vehicle_supplies_and_new_parts" => Ok(Self::MotorVehicleSuppliesAndNewParts),
            "motorcycle_shops_and_dealers" => Ok(Self::MotorcycleShopsAndDealers),
            "motorcycle_shops_dealers" => Ok(Self::MotorcycleShopsDealers),
            "music_stores_musical_instruments_pianos_and_sheet_music" => {
                Ok(Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic)
            }
            "news_dealers_and_newsstands" => Ok(Self::NewsDealersAndNewsstands),
            "non_fi_money_orders" => Ok(Self::NonFiMoneyOrders),
            "non_fi_stored_value_card_purchase_load" => Ok(Self::NonFiStoredValueCardPurchaseLoad),
            "nondurable_goods" => Ok(Self::NondurableGoods),
            "nurseries_lawn_and_garden_supply_stores" => {
                Ok(Self::NurseriesLawnAndGardenSupplyStores)
            }
            "nursing_personal_care" => Ok(Self::NursingPersonalCare),
            "office_and_commercial_furniture" => Ok(Self::OfficeAndCommercialFurniture),
            "opticians_eyeglasses" => Ok(Self::OpticiansEyeglasses),
            "optometrists_ophthalmologist" => Ok(Self::OptometristsOphthalmologist),
            "orthopedic_goods_prosthetic_devices" => Ok(Self::OrthopedicGoodsProstheticDevices),
            "osteopaths" => Ok(Self::Osteopaths),
            "package_stores_beer_wine_and_liquor" => Ok(Self::PackageStoresBeerWineAndLiquor),
            "paints_varnishes_and_supplies" => Ok(Self::PaintsVarnishesAndSupplies),
            "parking_lots_garages" => Ok(Self::ParkingLotsGarages),
            "passenger_railways" => Ok(Self::PassengerRailways),
            "pawn_shops" => Ok(Self::PawnShops),
            "pet_shops_pet_food_and_supplies" => Ok(Self::PetShopsPetFoodAndSupplies),
            "petroleum_and_petroleum_products" => Ok(Self::PetroleumAndPetroleumProducts),
            "photo_developing" => Ok(Self::PhotoDeveloping),
            "photographic_photocopy_microfilm_equipment_and_supplies" => {
                Ok(Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies)
            }
            "photographic_studios" => Ok(Self::PhotographicStudios),
            "picture_video_production" => Ok(Self::PictureVideoProduction),
            "piece_goods_notions_and_other_dry_goods" => {
                Ok(Self::PieceGoodsNotionsAndOtherDryGoods)
            }
            "plumbing_heating_equipment_and_supplies" => {
                Ok(Self::PlumbingHeatingEquipmentAndSupplies)
            }
            "political_organizations" => Ok(Self::PoliticalOrganizations),
            "postal_services_government_only" => Ok(Self::PostalServicesGovernmentOnly),
            "precious_stones_and_metals_watches_and_jewelry" => {
                Ok(Self::PreciousStonesAndMetalsWatchesAndJewelry)
            }
            "professional_services" => Ok(Self::ProfessionalServices),
            "public_warehousing_and_storage" => Ok(Self::PublicWarehousingAndStorage),
            "quick_copy_repro_and_blueprint" => Ok(Self::QuickCopyReproAndBlueprint),
            "railroads" => Ok(Self::Railroads),
            "real_estate_agents_and_managers_rentals" => {
                Ok(Self::RealEstateAgentsAndManagersRentals)
            }
            "record_stores" => Ok(Self::RecordStores),
            "recreational_vehicle_rentals" => Ok(Self::RecreationalVehicleRentals),
            "religious_goods_stores" => Ok(Self::ReligiousGoodsStores),
            "religious_organizations" => Ok(Self::ReligiousOrganizations),
            "roofing_siding_sheet_metal" => Ok(Self::RoofingSidingSheetMetal),
            "secretarial_support_services" => Ok(Self::SecretarialSupportServices),
            "security_brokers_dealers" => Ok(Self::SecurityBrokersDealers),
            "service_stations" => Ok(Self::ServiceStations),
            "sewing_needlework_fabric_and_piece_goods_stores" => {
                Ok(Self::SewingNeedleworkFabricAndPieceGoodsStores)
            }
            "shoe_repair_hat_cleaning" => Ok(Self::ShoeRepairHatCleaning),
            "shoe_stores" => Ok(Self::ShoeStores),
            "small_appliance_repair" => Ok(Self::SmallApplianceRepair),
            "snowmobile_dealers" => Ok(Self::SnowmobileDealers),
            "special_trade_services" => Ok(Self::SpecialTradeServices),
            "specialty_cleaning" => Ok(Self::SpecialtyCleaning),
            "sporting_goods_stores" => Ok(Self::SportingGoodsStores),
            "sporting_recreation_camps" => Ok(Self::SportingRecreationCamps),
            "sports_and_riding_apparel_stores" => Ok(Self::SportsAndRidingApparelStores),
            "sports_clubs_fields" => Ok(Self::SportsClubsFields),
            "stamp_and_coin_stores" => Ok(Self::StampAndCoinStores),
            "stationary_office_supplies_printing_and_writing_paper" => {
                Ok(Self::StationaryOfficeSuppliesPrintingAndWritingPaper)
            }
            "stationery_stores_office_and_school_supply_stores" => {
                Ok(Self::StationeryStoresOfficeAndSchoolSupplyStores)
            }
            "swimming_pools_sales" => Ok(Self::SwimmingPoolsSales),
            "t_ui_travel_germany" => Ok(Self::TUiTravelGermany),
            "tailors_alterations" => Ok(Self::TailorsAlterations),
            "tax_payments_government_agencies" => Ok(Self::TaxPaymentsGovernmentAgencies),
            "tax_preparation_services" => Ok(Self::TaxPreparationServices),
            "taxicabs_limousines" => Ok(Self::TaxicabsLimousines),
            "telecommunication_equipment_and_telephone_sales" => {
                Ok(Self::TelecommunicationEquipmentAndTelephoneSales)
            }
            "telecommunication_services" => Ok(Self::TelecommunicationServices),
            "telegraph_services" => Ok(Self::TelegraphServices),
            "tent_and_awning_shops" => Ok(Self::TentAndAwningShops),
            "testing_laboratories" => Ok(Self::TestingLaboratories),
            "theatrical_ticket_agencies" => Ok(Self::TheatricalTicketAgencies),
            "timeshares" => Ok(Self::Timeshares),
            "tire_retreading_and_repair" => Ok(Self::TireRetreadingAndRepair),
            "tolls_bridge_fees" => Ok(Self::TollsBridgeFees),
            "tourist_attractions_and_exhibits" => Ok(Self::TouristAttractionsAndExhibits),
            "towing_services" => Ok(Self::TowingServices),
            "trailer_parks_campgrounds" => Ok(Self::TrailerParksCampgrounds),
            "transportation_services" => Ok(Self::TransportationServices),
            "travel_agencies_tour_operators" => Ok(Self::TravelAgenciesTourOperators),
            "truck_stop_iteration" => Ok(Self::TruckStopIteration),
            "truck_utility_trailer_rentals" => Ok(Self::TruckUtilityTrailerRentals),
            "typesetting_plate_making_and_related_services" => {
                Ok(Self::TypesettingPlateMakingAndRelatedServices)
            }
            "typewriter_stores" => Ok(Self::TypewriterStores),
            "u_s_federal_government_agencies_or_departments" => {
                Ok(Self::USFederalGovernmentAgenciesOrDepartments)
            }
            "uniforms_commercial_clothing" => Ok(Self::UniformsCommercialClothing),
            "used_merchandise_and_secondhand_stores" => {
                Ok(Self::UsedMerchandiseAndSecondhandStores)
            }
            "utilities" => Ok(Self::Utilities),
            "variety_stores" => Ok(Self::VarietyStores),
            "veterinary_services" => Ok(Self::VeterinaryServices),
            "video_amusement_game_supplies" => Ok(Self::VideoAmusementGameSupplies),
            "video_game_arcades" => Ok(Self::VideoGameArcades),
            "video_tape_rental_stores" => Ok(Self::VideoTapeRentalStores),
            "vocational_trade_schools" => Ok(Self::VocationalTradeSchools),
            "watch_jewelry_repair" => Ok(Self::WatchJewelryRepair),
            "welding_repair" => Ok(Self::WeldingRepair),
            "wholesale_clubs" => Ok(Self::WholesaleClubs),
            "wig_and_toupee_stores" => Ok(Self::WigAndToupeeStores),
            "wires_money_orders" => Ok(Self::WiresMoneyOrders),
            "womens_accessory_and_specialty_shops" => Ok(Self::WomensAccessoryAndSpecialtyShops),
            "womens_ready_to_wear_stores" => Ok(Self::WomensReadyToWearStores),
            "wrecking_and_salvage_yards" => Ok(Self::WreckingAndSalvageYards),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateCardSpendingControlsAllowedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCardSpendingControlsAllowedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateCardSpendingControlsAllowedCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
///
/// All other categories will be allowed.
/// Cannot be set with `allowed_categories`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateCardSpendingControlsBlockedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl UpdateCardSpendingControlsBlockedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl std::str::FromStr for UpdateCardSpendingControlsBlockedCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ac_refrigeration_repair" => Ok(Self::AcRefrigerationRepair),
            "accounting_bookkeeping_services" => Ok(Self::AccountingBookkeepingServices),
            "advertising_services" => Ok(Self::AdvertisingServices),
            "agricultural_cooperative" => Ok(Self::AgriculturalCooperative),
            "airlines_air_carriers" => Ok(Self::AirlinesAirCarriers),
            "airports_flying_fields" => Ok(Self::AirportsFlyingFields),
            "ambulance_services" => Ok(Self::AmbulanceServices),
            "amusement_parks_carnivals" => Ok(Self::AmusementParksCarnivals),
            "antique_reproductions" => Ok(Self::AntiqueReproductions),
            "antique_shops" => Ok(Self::AntiqueShops),
            "aquariums" => Ok(Self::Aquariums),
            "architectural_surveying_services" => Ok(Self::ArchitecturalSurveyingServices),
            "art_dealers_and_galleries" => Ok(Self::ArtDealersAndGalleries),
            "artists_supply_and_craft_shops" => Ok(Self::ArtistsSupplyAndCraftShops),
            "auto_and_home_supply_stores" => Ok(Self::AutoAndHomeSupplyStores),
            "auto_body_repair_shops" => Ok(Self::AutoBodyRepairShops),
            "auto_paint_shops" => Ok(Self::AutoPaintShops),
            "auto_service_shops" => Ok(Self::AutoServiceShops),
            "automated_cash_disburse" => Ok(Self::AutomatedCashDisburse),
            "automated_fuel_dispensers" => Ok(Self::AutomatedFuelDispensers),
            "automobile_associations" => Ok(Self::AutomobileAssociations),
            "automotive_parts_and_accessories_stores" => {
                Ok(Self::AutomotivePartsAndAccessoriesStores)
            }
            "automotive_tire_stores" => Ok(Self::AutomotiveTireStores),
            "bail_and_bond_payments" => Ok(Self::BailAndBondPayments),
            "bakeries" => Ok(Self::Bakeries),
            "bands_orchestras" => Ok(Self::BandsOrchestras),
            "barber_and_beauty_shops" => Ok(Self::BarberAndBeautyShops),
            "betting_casino_gambling" => Ok(Self::BettingCasinoGambling),
            "bicycle_shops" => Ok(Self::BicycleShops),
            "billiard_pool_establishments" => Ok(Self::BilliardPoolEstablishments),
            "boat_dealers" => Ok(Self::BoatDealers),
            "boat_rentals_and_leases" => Ok(Self::BoatRentalsAndLeases),
            "book_stores" => Ok(Self::BookStores),
            "books_periodicals_and_newspapers" => Ok(Self::BooksPeriodicalsAndNewspapers),
            "bowling_alleys" => Ok(Self::BowlingAlleys),
            "bus_lines" => Ok(Self::BusLines),
            "business_secretarial_schools" => Ok(Self::BusinessSecretarialSchools),
            "buying_shopping_services" => Ok(Self::BuyingShoppingServices),
            "cable_satellite_and_other_pay_television_and_radio" => {
                Ok(Self::CableSatelliteAndOtherPayTelevisionAndRadio)
            }
            "camera_and_photographic_supply_stores" => Ok(Self::CameraAndPhotographicSupplyStores),
            "candy_nut_and_confectionery_stores" => Ok(Self::CandyNutAndConfectioneryStores),
            "car_and_truck_dealers_new_used" => Ok(Self::CarAndTruckDealersNewUsed),
            "car_and_truck_dealers_used_only" => Ok(Self::CarAndTruckDealersUsedOnly),
            "car_rental_agencies" => Ok(Self::CarRentalAgencies),
            "car_washes" => Ok(Self::CarWashes),
            "carpentry_services" => Ok(Self::CarpentryServices),
            "carpet_upholstery_cleaning" => Ok(Self::CarpetUpholsteryCleaning),
            "caterers" => Ok(Self::Caterers),
            "charitable_and_social_service_organizations_fundraising" => {
                Ok(Self::CharitableAndSocialServiceOrganizationsFundraising)
            }
            "chemicals_and_allied_products" => Ok(Self::ChemicalsAndAlliedProducts),
            "child_care_services" => Ok(Self::ChildCareServices),
            "childrens_and_infants_wear_stores" => Ok(Self::ChildrensAndInfantsWearStores),
            "chiropodists_podiatrists" => Ok(Self::ChiropodistsPodiatrists),
            "chiropractors" => Ok(Self::Chiropractors),
            "cigar_stores_and_stands" => Ok(Self::CigarStoresAndStands),
            "civic_social_fraternal_associations" => Ok(Self::CivicSocialFraternalAssociations),
            "cleaning_and_maintenance" => Ok(Self::CleaningAndMaintenance),
            "clothing_rental" => Ok(Self::ClothingRental),
            "colleges_universities" => Ok(Self::CollegesUniversities),
            "commercial_equipment" => Ok(Self::CommercialEquipment),
            "commercial_footwear" => Ok(Self::CommercialFootwear),
            "commercial_photography_art_and_graphics" => {
                Ok(Self::CommercialPhotographyArtAndGraphics)
            }
            "commuter_transport_and_ferries" => Ok(Self::CommuterTransportAndFerries),
            "computer_network_services" => Ok(Self::ComputerNetworkServices),
            "computer_programming" => Ok(Self::ComputerProgramming),
            "computer_repair" => Ok(Self::ComputerRepair),
            "computer_software_stores" => Ok(Self::ComputerSoftwareStores),
            "computers_peripherals_and_software" => Ok(Self::ComputersPeripheralsAndSoftware),
            "concrete_work_services" => Ok(Self::ConcreteWorkServices),
            "construction_materials" => Ok(Self::ConstructionMaterials),
            "consulting_public_relations" => Ok(Self::ConsultingPublicRelations),
            "correspondence_schools" => Ok(Self::CorrespondenceSchools),
            "cosmetic_stores" => Ok(Self::CosmeticStores),
            "counseling_services" => Ok(Self::CounselingServices),
            "country_clubs" => Ok(Self::CountryClubs),
            "courier_services" => Ok(Self::CourierServices),
            "court_costs" => Ok(Self::CourtCosts),
            "credit_reporting_agencies" => Ok(Self::CreditReportingAgencies),
            "cruise_lines" => Ok(Self::CruiseLines),
            "dairy_products_stores" => Ok(Self::DairyProductsStores),
            "dance_hall_studios_schools" => Ok(Self::DanceHallStudiosSchools),
            "dating_escort_services" => Ok(Self::DatingEscortServices),
            "dentists_orthodontists" => Ok(Self::DentistsOrthodontists),
            "department_stores" => Ok(Self::DepartmentStores),
            "detective_agencies" => Ok(Self::DetectiveAgencies),
            "digital_goods_applications" => Ok(Self::DigitalGoodsApplications),
            "digital_goods_games" => Ok(Self::DigitalGoodsGames),
            "digital_goods_large_volume" => Ok(Self::DigitalGoodsLargeVolume),
            "digital_goods_media" => Ok(Self::DigitalGoodsMedia),
            "direct_marketing_catalog_merchant" => Ok(Self::DirectMarketingCatalogMerchant),
            "direct_marketing_combination_catalog_and_retail_merchant" => {
                Ok(Self::DirectMarketingCombinationCatalogAndRetailMerchant)
            }
            "direct_marketing_inbound_telemarketing" => {
                Ok(Self::DirectMarketingInboundTelemarketing)
            }
            "direct_marketing_insurance_services" => Ok(Self::DirectMarketingInsuranceServices),
            "direct_marketing_other" => Ok(Self::DirectMarketingOther),
            "direct_marketing_outbound_telemarketing" => {
                Ok(Self::DirectMarketingOutboundTelemarketing)
            }
            "direct_marketing_subscription" => Ok(Self::DirectMarketingSubscription),
            "direct_marketing_travel" => Ok(Self::DirectMarketingTravel),
            "discount_stores" => Ok(Self::DiscountStores),
            "doctors" => Ok(Self::Doctors),
            "door_to_door_sales" => Ok(Self::DoorToDoorSales),
            "drapery_window_covering_and_upholstery_stores" => {
                Ok(Self::DraperyWindowCoveringAndUpholsteryStores)
            }
            "drinking_places" => Ok(Self::DrinkingPlaces),
            "drug_stores_and_pharmacies" => Ok(Self::DrugStoresAndPharmacies),
            "drugs_drug_proprietaries_and_druggist_sundries" => {
                Ok(Self::DrugsDrugProprietariesAndDruggistSundries)
            }
            "dry_cleaners" => Ok(Self::DryCleaners),
            "durable_goods" => Ok(Self::DurableGoods),
            "duty_free_stores" => Ok(Self::DutyFreeStores),
            "eating_places_restaurants" => Ok(Self::EatingPlacesRestaurants),
            "educational_services" => Ok(Self::EducationalServices),
            "electric_razor_stores" => Ok(Self::ElectricRazorStores),
            "electrical_parts_and_equipment" => Ok(Self::ElectricalPartsAndEquipment),
            "electrical_services" => Ok(Self::ElectricalServices),
            "electronics_repair_shops" => Ok(Self::ElectronicsRepairShops),
            "electronics_stores" => Ok(Self::ElectronicsStores),
            "elementary_secondary_schools" => Ok(Self::ElementarySecondarySchools),
            "employment_temp_agencies" => Ok(Self::EmploymentTempAgencies),
            "equipment_rental" => Ok(Self::EquipmentRental),
            "exterminating_services" => Ok(Self::ExterminatingServices),
            "family_clothing_stores" => Ok(Self::FamilyClothingStores),
            "fast_food_restaurants" => Ok(Self::FastFoodRestaurants),
            "financial_institutions" => Ok(Self::FinancialInstitutions),
            "fines_government_administrative_entities" => {
                Ok(Self::FinesGovernmentAdministrativeEntities)
            }
            "fireplace_fireplace_screens_and_accessories_stores" => {
                Ok(Self::FireplaceFireplaceScreensAndAccessoriesStores)
            }
            "floor_covering_stores" => Ok(Self::FloorCoveringStores),
            "florists" => Ok(Self::Florists),
            "florists_supplies_nursery_stock_and_flowers" => {
                Ok(Self::FloristsSuppliesNurseryStockAndFlowers)
            }
            "freezer_and_locker_meat_provisioners" => Ok(Self::FreezerAndLockerMeatProvisioners),
            "fuel_dealers_non_automotive" => Ok(Self::FuelDealersNonAutomotive),
            "funeral_services_crematories" => Ok(Self::FuneralServicesCrematories),
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => {
                Ok(Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances)
            }
            "furniture_repair_refinishing" => Ok(Self::FurnitureRepairRefinishing),
            "furriers_and_fur_shops" => Ok(Self::FurriersAndFurShops),
            "general_services" => Ok(Self::GeneralServices),
            "gift_card_novelty_and_souvenir_shops" => Ok(Self::GiftCardNoveltyAndSouvenirShops),
            "glass_paint_and_wallpaper_stores" => Ok(Self::GlassPaintAndWallpaperStores),
            "glassware_crystal_stores" => Ok(Self::GlasswareCrystalStores),
            "golf_courses_public" => Ok(Self::GolfCoursesPublic),
            "government_services" => Ok(Self::GovernmentServices),
            "grocery_stores_supermarkets" => Ok(Self::GroceryStoresSupermarkets),
            "hardware_equipment_and_supplies" => Ok(Self::HardwareEquipmentAndSupplies),
            "hardware_stores" => Ok(Self::HardwareStores),
            "health_and_beauty_spas" => Ok(Self::HealthAndBeautySpas),
            "hearing_aids_sales_and_supplies" => Ok(Self::HearingAidsSalesAndSupplies),
            "heating_plumbing_a_c" => Ok(Self::HeatingPlumbingAC),
            "hobby_toy_and_game_shops" => Ok(Self::HobbyToyAndGameShops),
            "home_supply_warehouse_stores" => Ok(Self::HomeSupplyWarehouseStores),
            "hospitals" => Ok(Self::Hospitals),
            "hotels_motels_and_resorts" => Ok(Self::HotelsMotelsAndResorts),
            "household_appliance_stores" => Ok(Self::HouseholdApplianceStores),
            "industrial_supplies" => Ok(Self::IndustrialSupplies),
            "information_retrieval_services" => Ok(Self::InformationRetrievalServices),
            "insurance_default" => Ok(Self::InsuranceDefault),
            "insurance_underwriting_premiums" => Ok(Self::InsuranceUnderwritingPremiums),
            "intra_company_purchases" => Ok(Self::IntraCompanyPurchases),
            "jewelry_stores_watches_clocks_and_silverware_stores" => {
                Ok(Self::JewelryStoresWatchesClocksAndSilverwareStores)
            }
            "landscaping_services" => Ok(Self::LandscapingServices),
            "laundries" => Ok(Self::Laundries),
            "laundry_cleaning_services" => Ok(Self::LaundryCleaningServices),
            "legal_services_attorneys" => Ok(Self::LegalServicesAttorneys),
            "luggage_and_leather_goods_stores" => Ok(Self::LuggageAndLeatherGoodsStores),
            "lumber_building_materials_stores" => Ok(Self::LumberBuildingMaterialsStores),
            "manual_cash_disburse" => Ok(Self::ManualCashDisburse),
            "marinas_service_and_supplies" => Ok(Self::MarinasServiceAndSupplies),
            "masonry_stonework_and_plaster" => Ok(Self::MasonryStoneworkAndPlaster),
            "massage_parlors" => Ok(Self::MassageParlors),
            "medical_and_dental_labs" => Ok(Self::MedicalAndDentalLabs),
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => {
                Ok(Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies)
            }
            "medical_services" => Ok(Self::MedicalServices),
            "membership_organizations" => Ok(Self::MembershipOrganizations),
            "mens_and_boys_clothing_and_accessories_stores" => {
                Ok(Self::MensAndBoysClothingAndAccessoriesStores)
            }
            "mens_womens_clothing_stores" => Ok(Self::MensWomensClothingStores),
            "metal_service_centers" => Ok(Self::MetalServiceCenters),
            "miscellaneous" => Ok(Self::Miscellaneous),
            "miscellaneous_apparel_and_accessory_shops" => {
                Ok(Self::MiscellaneousApparelAndAccessoryShops)
            }
            "miscellaneous_auto_dealers" => Ok(Self::MiscellaneousAutoDealers),
            "miscellaneous_business_services" => Ok(Self::MiscellaneousBusinessServices),
            "miscellaneous_food_stores" => Ok(Self::MiscellaneousFoodStores),
            "miscellaneous_general_merchandise" => Ok(Self::MiscellaneousGeneralMerchandise),
            "miscellaneous_general_services" => Ok(Self::MiscellaneousGeneralServices),
            "miscellaneous_home_furnishing_specialty_stores" => {
                Ok(Self::MiscellaneousHomeFurnishingSpecialtyStores)
            }
            "miscellaneous_publishing_and_printing" => Ok(Self::MiscellaneousPublishingAndPrinting),
            "miscellaneous_recreation_services" => Ok(Self::MiscellaneousRecreationServices),
            "miscellaneous_repair_shops" => Ok(Self::MiscellaneousRepairShops),
            "miscellaneous_specialty_retail" => Ok(Self::MiscellaneousSpecialtyRetail),
            "mobile_home_dealers" => Ok(Self::MobileHomeDealers),
            "motion_picture_theaters" => Ok(Self::MotionPictureTheaters),
            "motor_freight_carriers_and_trucking" => Ok(Self::MotorFreightCarriersAndTrucking),
            "motor_homes_dealers" => Ok(Self::MotorHomesDealers),
            "motor_vehicle_supplies_and_new_parts" => Ok(Self::MotorVehicleSuppliesAndNewParts),
            "motorcycle_shops_and_dealers" => Ok(Self::MotorcycleShopsAndDealers),
            "motorcycle_shops_dealers" => Ok(Self::MotorcycleShopsDealers),
            "music_stores_musical_instruments_pianos_and_sheet_music" => {
                Ok(Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic)
            }
            "news_dealers_and_newsstands" => Ok(Self::NewsDealersAndNewsstands),
            "non_fi_money_orders" => Ok(Self::NonFiMoneyOrders),
            "non_fi_stored_value_card_purchase_load" => Ok(Self::NonFiStoredValueCardPurchaseLoad),
            "nondurable_goods" => Ok(Self::NondurableGoods),
            "nurseries_lawn_and_garden_supply_stores" => {
                Ok(Self::NurseriesLawnAndGardenSupplyStores)
            }
            "nursing_personal_care" => Ok(Self::NursingPersonalCare),
            "office_and_commercial_furniture" => Ok(Self::OfficeAndCommercialFurniture),
            "opticians_eyeglasses" => Ok(Self::OpticiansEyeglasses),
            "optometrists_ophthalmologist" => Ok(Self::OptometristsOphthalmologist),
            "orthopedic_goods_prosthetic_devices" => Ok(Self::OrthopedicGoodsProstheticDevices),
            "osteopaths" => Ok(Self::Osteopaths),
            "package_stores_beer_wine_and_liquor" => Ok(Self::PackageStoresBeerWineAndLiquor),
            "paints_varnishes_and_supplies" => Ok(Self::PaintsVarnishesAndSupplies),
            "parking_lots_garages" => Ok(Self::ParkingLotsGarages),
            "passenger_railways" => Ok(Self::PassengerRailways),
            "pawn_shops" => Ok(Self::PawnShops),
            "pet_shops_pet_food_and_supplies" => Ok(Self::PetShopsPetFoodAndSupplies),
            "petroleum_and_petroleum_products" => Ok(Self::PetroleumAndPetroleumProducts),
            "photo_developing" => Ok(Self::PhotoDeveloping),
            "photographic_photocopy_microfilm_equipment_and_supplies" => {
                Ok(Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies)
            }
            "photographic_studios" => Ok(Self::PhotographicStudios),
            "picture_video_production" => Ok(Self::PictureVideoProduction),
            "piece_goods_notions_and_other_dry_goods" => {
                Ok(Self::PieceGoodsNotionsAndOtherDryGoods)
            }
            "plumbing_heating_equipment_and_supplies" => {
                Ok(Self::PlumbingHeatingEquipmentAndSupplies)
            }
            "political_organizations" => Ok(Self::PoliticalOrganizations),
            "postal_services_government_only" => Ok(Self::PostalServicesGovernmentOnly),
            "precious_stones_and_metals_watches_and_jewelry" => {
                Ok(Self::PreciousStonesAndMetalsWatchesAndJewelry)
            }
            "professional_services" => Ok(Self::ProfessionalServices),
            "public_warehousing_and_storage" => Ok(Self::PublicWarehousingAndStorage),
            "quick_copy_repro_and_blueprint" => Ok(Self::QuickCopyReproAndBlueprint),
            "railroads" => Ok(Self::Railroads),
            "real_estate_agents_and_managers_rentals" => {
                Ok(Self::RealEstateAgentsAndManagersRentals)
            }
            "record_stores" => Ok(Self::RecordStores),
            "recreational_vehicle_rentals" => Ok(Self::RecreationalVehicleRentals),
            "religious_goods_stores" => Ok(Self::ReligiousGoodsStores),
            "religious_organizations" => Ok(Self::ReligiousOrganizations),
            "roofing_siding_sheet_metal" => Ok(Self::RoofingSidingSheetMetal),
            "secretarial_support_services" => Ok(Self::SecretarialSupportServices),
            "security_brokers_dealers" => Ok(Self::SecurityBrokersDealers),
            "service_stations" => Ok(Self::ServiceStations),
            "sewing_needlework_fabric_and_piece_goods_stores" => {
                Ok(Self::SewingNeedleworkFabricAndPieceGoodsStores)
            }
            "shoe_repair_hat_cleaning" => Ok(Self::ShoeRepairHatCleaning),
            "shoe_stores" => Ok(Self::ShoeStores),
            "small_appliance_repair" => Ok(Self::SmallApplianceRepair),
            "snowmobile_dealers" => Ok(Self::SnowmobileDealers),
            "special_trade_services" => Ok(Self::SpecialTradeServices),
            "specialty_cleaning" => Ok(Self::SpecialtyCleaning),
            "sporting_goods_stores" => Ok(Self::SportingGoodsStores),
            "sporting_recreation_camps" => Ok(Self::SportingRecreationCamps),
            "sports_and_riding_apparel_stores" => Ok(Self::SportsAndRidingApparelStores),
            "sports_clubs_fields" => Ok(Self::SportsClubsFields),
            "stamp_and_coin_stores" => Ok(Self::StampAndCoinStores),
            "stationary_office_supplies_printing_and_writing_paper" => {
                Ok(Self::StationaryOfficeSuppliesPrintingAndWritingPaper)
            }
            "stationery_stores_office_and_school_supply_stores" => {
                Ok(Self::StationeryStoresOfficeAndSchoolSupplyStores)
            }
            "swimming_pools_sales" => Ok(Self::SwimmingPoolsSales),
            "t_ui_travel_germany" => Ok(Self::TUiTravelGermany),
            "tailors_alterations" => Ok(Self::TailorsAlterations),
            "tax_payments_government_agencies" => Ok(Self::TaxPaymentsGovernmentAgencies),
            "tax_preparation_services" => Ok(Self::TaxPreparationServices),
            "taxicabs_limousines" => Ok(Self::TaxicabsLimousines),
            "telecommunication_equipment_and_telephone_sales" => {
                Ok(Self::TelecommunicationEquipmentAndTelephoneSales)
            }
            "telecommunication_services" => Ok(Self::TelecommunicationServices),
            "telegraph_services" => Ok(Self::TelegraphServices),
            "tent_and_awning_shops" => Ok(Self::TentAndAwningShops),
            "testing_laboratories" => Ok(Self::TestingLaboratories),
            "theatrical_ticket_agencies" => Ok(Self::TheatricalTicketAgencies),
            "timeshares" => Ok(Self::Timeshares),
            "tire_retreading_and_repair" => Ok(Self::TireRetreadingAndRepair),
            "tolls_bridge_fees" => Ok(Self::TollsBridgeFees),
            "tourist_attractions_and_exhibits" => Ok(Self::TouristAttractionsAndExhibits),
            "towing_services" => Ok(Self::TowingServices),
            "trailer_parks_campgrounds" => Ok(Self::TrailerParksCampgrounds),
            "transportation_services" => Ok(Self::TransportationServices),
            "travel_agencies_tour_operators" => Ok(Self::TravelAgenciesTourOperators),
            "truck_stop_iteration" => Ok(Self::TruckStopIteration),
            "truck_utility_trailer_rentals" => Ok(Self::TruckUtilityTrailerRentals),
            "typesetting_plate_making_and_related_services" => {
                Ok(Self::TypesettingPlateMakingAndRelatedServices)
            }
            "typewriter_stores" => Ok(Self::TypewriterStores),
            "u_s_federal_government_agencies_or_departments" => {
                Ok(Self::USFederalGovernmentAgenciesOrDepartments)
            }
            "uniforms_commercial_clothing" => Ok(Self::UniformsCommercialClothing),
            "used_merchandise_and_secondhand_stores" => {
                Ok(Self::UsedMerchandiseAndSecondhandStores)
            }
            "utilities" => Ok(Self::Utilities),
            "variety_stores" => Ok(Self::VarietyStores),
            "veterinary_services" => Ok(Self::VeterinaryServices),
            "video_amusement_game_supplies" => Ok(Self::VideoAmusementGameSupplies),
            "video_game_arcades" => Ok(Self::VideoGameArcades),
            "video_tape_rental_stores" => Ok(Self::VideoTapeRentalStores),
            "vocational_trade_schools" => Ok(Self::VocationalTradeSchools),
            "watch_jewelry_repair" => Ok(Self::WatchJewelryRepair),
            "welding_repair" => Ok(Self::WeldingRepair),
            "wholesale_clubs" => Ok(Self::WholesaleClubs),
            "wig_and_toupee_stores" => Ok(Self::WigAndToupeeStores),
            "wires_money_orders" => Ok(Self::WiresMoneyOrders),
            "womens_accessory_and_specialty_shops" => Ok(Self::WomensAccessoryAndSpecialtyShops),
            "womens_ready_to_wear_stores" => Ok(Self::WomensReadyToWearStores),
            "wrecking_and_salvage_yards" => Ok(Self::WreckingAndSalvageYards),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateCardSpendingControlsBlockedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCardSpendingControlsBlockedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateCardSpendingControlsBlockedCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateCardSpendingControlsSpendingLimits<'a> {
    /// Maximum amount allowed to spend per interval.
    pub amount: i64,
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    ///
    /// Omitting this field will apply the limit to all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<&'a [UpdateCardSpendingControlsSpendingLimitsCategories]>,
    /// Interval (or event) to which the amount applies.
    pub interval: UpdateCardSpendingControlsSpendingLimitsInterval,
}
impl<'a> UpdateCardSpendingControlsSpendingLimits<'a> {
    pub fn new(amount: i64, interval: UpdateCardSpendingControlsSpendingLimitsInterval) -> Self {
        Self { amount, categories: Default::default(), interval }
    }
}
/// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
///
/// Omitting this field will apply the limit to all categories.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateCardSpendingControlsSpendingLimitsCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl UpdateCardSpendingControlsSpendingLimitsCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl std::str::FromStr for UpdateCardSpendingControlsSpendingLimitsCategories {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ac_refrigeration_repair" => Ok(Self::AcRefrigerationRepair),
            "accounting_bookkeeping_services" => Ok(Self::AccountingBookkeepingServices),
            "advertising_services" => Ok(Self::AdvertisingServices),
            "agricultural_cooperative" => Ok(Self::AgriculturalCooperative),
            "airlines_air_carriers" => Ok(Self::AirlinesAirCarriers),
            "airports_flying_fields" => Ok(Self::AirportsFlyingFields),
            "ambulance_services" => Ok(Self::AmbulanceServices),
            "amusement_parks_carnivals" => Ok(Self::AmusementParksCarnivals),
            "antique_reproductions" => Ok(Self::AntiqueReproductions),
            "antique_shops" => Ok(Self::AntiqueShops),
            "aquariums" => Ok(Self::Aquariums),
            "architectural_surveying_services" => Ok(Self::ArchitecturalSurveyingServices),
            "art_dealers_and_galleries" => Ok(Self::ArtDealersAndGalleries),
            "artists_supply_and_craft_shops" => Ok(Self::ArtistsSupplyAndCraftShops),
            "auto_and_home_supply_stores" => Ok(Self::AutoAndHomeSupplyStores),
            "auto_body_repair_shops" => Ok(Self::AutoBodyRepairShops),
            "auto_paint_shops" => Ok(Self::AutoPaintShops),
            "auto_service_shops" => Ok(Self::AutoServiceShops),
            "automated_cash_disburse" => Ok(Self::AutomatedCashDisburse),
            "automated_fuel_dispensers" => Ok(Self::AutomatedFuelDispensers),
            "automobile_associations" => Ok(Self::AutomobileAssociations),
            "automotive_parts_and_accessories_stores" => {
                Ok(Self::AutomotivePartsAndAccessoriesStores)
            }
            "automotive_tire_stores" => Ok(Self::AutomotiveTireStores),
            "bail_and_bond_payments" => Ok(Self::BailAndBondPayments),
            "bakeries" => Ok(Self::Bakeries),
            "bands_orchestras" => Ok(Self::BandsOrchestras),
            "barber_and_beauty_shops" => Ok(Self::BarberAndBeautyShops),
            "betting_casino_gambling" => Ok(Self::BettingCasinoGambling),
            "bicycle_shops" => Ok(Self::BicycleShops),
            "billiard_pool_establishments" => Ok(Self::BilliardPoolEstablishments),
            "boat_dealers" => Ok(Self::BoatDealers),
            "boat_rentals_and_leases" => Ok(Self::BoatRentalsAndLeases),
            "book_stores" => Ok(Self::BookStores),
            "books_periodicals_and_newspapers" => Ok(Self::BooksPeriodicalsAndNewspapers),
            "bowling_alleys" => Ok(Self::BowlingAlleys),
            "bus_lines" => Ok(Self::BusLines),
            "business_secretarial_schools" => Ok(Self::BusinessSecretarialSchools),
            "buying_shopping_services" => Ok(Self::BuyingShoppingServices),
            "cable_satellite_and_other_pay_television_and_radio" => {
                Ok(Self::CableSatelliteAndOtherPayTelevisionAndRadio)
            }
            "camera_and_photographic_supply_stores" => Ok(Self::CameraAndPhotographicSupplyStores),
            "candy_nut_and_confectionery_stores" => Ok(Self::CandyNutAndConfectioneryStores),
            "car_and_truck_dealers_new_used" => Ok(Self::CarAndTruckDealersNewUsed),
            "car_and_truck_dealers_used_only" => Ok(Self::CarAndTruckDealersUsedOnly),
            "car_rental_agencies" => Ok(Self::CarRentalAgencies),
            "car_washes" => Ok(Self::CarWashes),
            "carpentry_services" => Ok(Self::CarpentryServices),
            "carpet_upholstery_cleaning" => Ok(Self::CarpetUpholsteryCleaning),
            "caterers" => Ok(Self::Caterers),
            "charitable_and_social_service_organizations_fundraising" => {
                Ok(Self::CharitableAndSocialServiceOrganizationsFundraising)
            }
            "chemicals_and_allied_products" => Ok(Self::ChemicalsAndAlliedProducts),
            "child_care_services" => Ok(Self::ChildCareServices),
            "childrens_and_infants_wear_stores" => Ok(Self::ChildrensAndInfantsWearStores),
            "chiropodists_podiatrists" => Ok(Self::ChiropodistsPodiatrists),
            "chiropractors" => Ok(Self::Chiropractors),
            "cigar_stores_and_stands" => Ok(Self::CigarStoresAndStands),
            "civic_social_fraternal_associations" => Ok(Self::CivicSocialFraternalAssociations),
            "cleaning_and_maintenance" => Ok(Self::CleaningAndMaintenance),
            "clothing_rental" => Ok(Self::ClothingRental),
            "colleges_universities" => Ok(Self::CollegesUniversities),
            "commercial_equipment" => Ok(Self::CommercialEquipment),
            "commercial_footwear" => Ok(Self::CommercialFootwear),
            "commercial_photography_art_and_graphics" => {
                Ok(Self::CommercialPhotographyArtAndGraphics)
            }
            "commuter_transport_and_ferries" => Ok(Self::CommuterTransportAndFerries),
            "computer_network_services" => Ok(Self::ComputerNetworkServices),
            "computer_programming" => Ok(Self::ComputerProgramming),
            "computer_repair" => Ok(Self::ComputerRepair),
            "computer_software_stores" => Ok(Self::ComputerSoftwareStores),
            "computers_peripherals_and_software" => Ok(Self::ComputersPeripheralsAndSoftware),
            "concrete_work_services" => Ok(Self::ConcreteWorkServices),
            "construction_materials" => Ok(Self::ConstructionMaterials),
            "consulting_public_relations" => Ok(Self::ConsultingPublicRelations),
            "correspondence_schools" => Ok(Self::CorrespondenceSchools),
            "cosmetic_stores" => Ok(Self::CosmeticStores),
            "counseling_services" => Ok(Self::CounselingServices),
            "country_clubs" => Ok(Self::CountryClubs),
            "courier_services" => Ok(Self::CourierServices),
            "court_costs" => Ok(Self::CourtCosts),
            "credit_reporting_agencies" => Ok(Self::CreditReportingAgencies),
            "cruise_lines" => Ok(Self::CruiseLines),
            "dairy_products_stores" => Ok(Self::DairyProductsStores),
            "dance_hall_studios_schools" => Ok(Self::DanceHallStudiosSchools),
            "dating_escort_services" => Ok(Self::DatingEscortServices),
            "dentists_orthodontists" => Ok(Self::DentistsOrthodontists),
            "department_stores" => Ok(Self::DepartmentStores),
            "detective_agencies" => Ok(Self::DetectiveAgencies),
            "digital_goods_applications" => Ok(Self::DigitalGoodsApplications),
            "digital_goods_games" => Ok(Self::DigitalGoodsGames),
            "digital_goods_large_volume" => Ok(Self::DigitalGoodsLargeVolume),
            "digital_goods_media" => Ok(Self::DigitalGoodsMedia),
            "direct_marketing_catalog_merchant" => Ok(Self::DirectMarketingCatalogMerchant),
            "direct_marketing_combination_catalog_and_retail_merchant" => {
                Ok(Self::DirectMarketingCombinationCatalogAndRetailMerchant)
            }
            "direct_marketing_inbound_telemarketing" => {
                Ok(Self::DirectMarketingInboundTelemarketing)
            }
            "direct_marketing_insurance_services" => Ok(Self::DirectMarketingInsuranceServices),
            "direct_marketing_other" => Ok(Self::DirectMarketingOther),
            "direct_marketing_outbound_telemarketing" => {
                Ok(Self::DirectMarketingOutboundTelemarketing)
            }
            "direct_marketing_subscription" => Ok(Self::DirectMarketingSubscription),
            "direct_marketing_travel" => Ok(Self::DirectMarketingTravel),
            "discount_stores" => Ok(Self::DiscountStores),
            "doctors" => Ok(Self::Doctors),
            "door_to_door_sales" => Ok(Self::DoorToDoorSales),
            "drapery_window_covering_and_upholstery_stores" => {
                Ok(Self::DraperyWindowCoveringAndUpholsteryStores)
            }
            "drinking_places" => Ok(Self::DrinkingPlaces),
            "drug_stores_and_pharmacies" => Ok(Self::DrugStoresAndPharmacies),
            "drugs_drug_proprietaries_and_druggist_sundries" => {
                Ok(Self::DrugsDrugProprietariesAndDruggistSundries)
            }
            "dry_cleaners" => Ok(Self::DryCleaners),
            "durable_goods" => Ok(Self::DurableGoods),
            "duty_free_stores" => Ok(Self::DutyFreeStores),
            "eating_places_restaurants" => Ok(Self::EatingPlacesRestaurants),
            "educational_services" => Ok(Self::EducationalServices),
            "electric_razor_stores" => Ok(Self::ElectricRazorStores),
            "electrical_parts_and_equipment" => Ok(Self::ElectricalPartsAndEquipment),
            "electrical_services" => Ok(Self::ElectricalServices),
            "electronics_repair_shops" => Ok(Self::ElectronicsRepairShops),
            "electronics_stores" => Ok(Self::ElectronicsStores),
            "elementary_secondary_schools" => Ok(Self::ElementarySecondarySchools),
            "employment_temp_agencies" => Ok(Self::EmploymentTempAgencies),
            "equipment_rental" => Ok(Self::EquipmentRental),
            "exterminating_services" => Ok(Self::ExterminatingServices),
            "family_clothing_stores" => Ok(Self::FamilyClothingStores),
            "fast_food_restaurants" => Ok(Self::FastFoodRestaurants),
            "financial_institutions" => Ok(Self::FinancialInstitutions),
            "fines_government_administrative_entities" => {
                Ok(Self::FinesGovernmentAdministrativeEntities)
            }
            "fireplace_fireplace_screens_and_accessories_stores" => {
                Ok(Self::FireplaceFireplaceScreensAndAccessoriesStores)
            }
            "floor_covering_stores" => Ok(Self::FloorCoveringStores),
            "florists" => Ok(Self::Florists),
            "florists_supplies_nursery_stock_and_flowers" => {
                Ok(Self::FloristsSuppliesNurseryStockAndFlowers)
            }
            "freezer_and_locker_meat_provisioners" => Ok(Self::FreezerAndLockerMeatProvisioners),
            "fuel_dealers_non_automotive" => Ok(Self::FuelDealersNonAutomotive),
            "funeral_services_crematories" => Ok(Self::FuneralServicesCrematories),
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => {
                Ok(Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances)
            }
            "furniture_repair_refinishing" => Ok(Self::FurnitureRepairRefinishing),
            "furriers_and_fur_shops" => Ok(Self::FurriersAndFurShops),
            "general_services" => Ok(Self::GeneralServices),
            "gift_card_novelty_and_souvenir_shops" => Ok(Self::GiftCardNoveltyAndSouvenirShops),
            "glass_paint_and_wallpaper_stores" => Ok(Self::GlassPaintAndWallpaperStores),
            "glassware_crystal_stores" => Ok(Self::GlasswareCrystalStores),
            "golf_courses_public" => Ok(Self::GolfCoursesPublic),
            "government_services" => Ok(Self::GovernmentServices),
            "grocery_stores_supermarkets" => Ok(Self::GroceryStoresSupermarkets),
            "hardware_equipment_and_supplies" => Ok(Self::HardwareEquipmentAndSupplies),
            "hardware_stores" => Ok(Self::HardwareStores),
            "health_and_beauty_spas" => Ok(Self::HealthAndBeautySpas),
            "hearing_aids_sales_and_supplies" => Ok(Self::HearingAidsSalesAndSupplies),
            "heating_plumbing_a_c" => Ok(Self::HeatingPlumbingAC),
            "hobby_toy_and_game_shops" => Ok(Self::HobbyToyAndGameShops),
            "home_supply_warehouse_stores" => Ok(Self::HomeSupplyWarehouseStores),
            "hospitals" => Ok(Self::Hospitals),
            "hotels_motels_and_resorts" => Ok(Self::HotelsMotelsAndResorts),
            "household_appliance_stores" => Ok(Self::HouseholdApplianceStores),
            "industrial_supplies" => Ok(Self::IndustrialSupplies),
            "information_retrieval_services" => Ok(Self::InformationRetrievalServices),
            "insurance_default" => Ok(Self::InsuranceDefault),
            "insurance_underwriting_premiums" => Ok(Self::InsuranceUnderwritingPremiums),
            "intra_company_purchases" => Ok(Self::IntraCompanyPurchases),
            "jewelry_stores_watches_clocks_and_silverware_stores" => {
                Ok(Self::JewelryStoresWatchesClocksAndSilverwareStores)
            }
            "landscaping_services" => Ok(Self::LandscapingServices),
            "laundries" => Ok(Self::Laundries),
            "laundry_cleaning_services" => Ok(Self::LaundryCleaningServices),
            "legal_services_attorneys" => Ok(Self::LegalServicesAttorneys),
            "luggage_and_leather_goods_stores" => Ok(Self::LuggageAndLeatherGoodsStores),
            "lumber_building_materials_stores" => Ok(Self::LumberBuildingMaterialsStores),
            "manual_cash_disburse" => Ok(Self::ManualCashDisburse),
            "marinas_service_and_supplies" => Ok(Self::MarinasServiceAndSupplies),
            "masonry_stonework_and_plaster" => Ok(Self::MasonryStoneworkAndPlaster),
            "massage_parlors" => Ok(Self::MassageParlors),
            "medical_and_dental_labs" => Ok(Self::MedicalAndDentalLabs),
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => {
                Ok(Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies)
            }
            "medical_services" => Ok(Self::MedicalServices),
            "membership_organizations" => Ok(Self::MembershipOrganizations),
            "mens_and_boys_clothing_and_accessories_stores" => {
                Ok(Self::MensAndBoysClothingAndAccessoriesStores)
            }
            "mens_womens_clothing_stores" => Ok(Self::MensWomensClothingStores),
            "metal_service_centers" => Ok(Self::MetalServiceCenters),
            "miscellaneous" => Ok(Self::Miscellaneous),
            "miscellaneous_apparel_and_accessory_shops" => {
                Ok(Self::MiscellaneousApparelAndAccessoryShops)
            }
            "miscellaneous_auto_dealers" => Ok(Self::MiscellaneousAutoDealers),
            "miscellaneous_business_services" => Ok(Self::MiscellaneousBusinessServices),
            "miscellaneous_food_stores" => Ok(Self::MiscellaneousFoodStores),
            "miscellaneous_general_merchandise" => Ok(Self::MiscellaneousGeneralMerchandise),
            "miscellaneous_general_services" => Ok(Self::MiscellaneousGeneralServices),
            "miscellaneous_home_furnishing_specialty_stores" => {
                Ok(Self::MiscellaneousHomeFurnishingSpecialtyStores)
            }
            "miscellaneous_publishing_and_printing" => Ok(Self::MiscellaneousPublishingAndPrinting),
            "miscellaneous_recreation_services" => Ok(Self::MiscellaneousRecreationServices),
            "miscellaneous_repair_shops" => Ok(Self::MiscellaneousRepairShops),
            "miscellaneous_specialty_retail" => Ok(Self::MiscellaneousSpecialtyRetail),
            "mobile_home_dealers" => Ok(Self::MobileHomeDealers),
            "motion_picture_theaters" => Ok(Self::MotionPictureTheaters),
            "motor_freight_carriers_and_trucking" => Ok(Self::MotorFreightCarriersAndTrucking),
            "motor_homes_dealers" => Ok(Self::MotorHomesDealers),
            "motor_vehicle_supplies_and_new_parts" => Ok(Self::MotorVehicleSuppliesAndNewParts),
            "motorcycle_shops_and_dealers" => Ok(Self::MotorcycleShopsAndDealers),
            "motorcycle_shops_dealers" => Ok(Self::MotorcycleShopsDealers),
            "music_stores_musical_instruments_pianos_and_sheet_music" => {
                Ok(Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic)
            }
            "news_dealers_and_newsstands" => Ok(Self::NewsDealersAndNewsstands),
            "non_fi_money_orders" => Ok(Self::NonFiMoneyOrders),
            "non_fi_stored_value_card_purchase_load" => Ok(Self::NonFiStoredValueCardPurchaseLoad),
            "nondurable_goods" => Ok(Self::NondurableGoods),
            "nurseries_lawn_and_garden_supply_stores" => {
                Ok(Self::NurseriesLawnAndGardenSupplyStores)
            }
            "nursing_personal_care" => Ok(Self::NursingPersonalCare),
            "office_and_commercial_furniture" => Ok(Self::OfficeAndCommercialFurniture),
            "opticians_eyeglasses" => Ok(Self::OpticiansEyeglasses),
            "optometrists_ophthalmologist" => Ok(Self::OptometristsOphthalmologist),
            "orthopedic_goods_prosthetic_devices" => Ok(Self::OrthopedicGoodsProstheticDevices),
            "osteopaths" => Ok(Self::Osteopaths),
            "package_stores_beer_wine_and_liquor" => Ok(Self::PackageStoresBeerWineAndLiquor),
            "paints_varnishes_and_supplies" => Ok(Self::PaintsVarnishesAndSupplies),
            "parking_lots_garages" => Ok(Self::ParkingLotsGarages),
            "passenger_railways" => Ok(Self::PassengerRailways),
            "pawn_shops" => Ok(Self::PawnShops),
            "pet_shops_pet_food_and_supplies" => Ok(Self::PetShopsPetFoodAndSupplies),
            "petroleum_and_petroleum_products" => Ok(Self::PetroleumAndPetroleumProducts),
            "photo_developing" => Ok(Self::PhotoDeveloping),
            "photographic_photocopy_microfilm_equipment_and_supplies" => {
                Ok(Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies)
            }
            "photographic_studios" => Ok(Self::PhotographicStudios),
            "picture_video_production" => Ok(Self::PictureVideoProduction),
            "piece_goods_notions_and_other_dry_goods" => {
                Ok(Self::PieceGoodsNotionsAndOtherDryGoods)
            }
            "plumbing_heating_equipment_and_supplies" => {
                Ok(Self::PlumbingHeatingEquipmentAndSupplies)
            }
            "political_organizations" => Ok(Self::PoliticalOrganizations),
            "postal_services_government_only" => Ok(Self::PostalServicesGovernmentOnly),
            "precious_stones_and_metals_watches_and_jewelry" => {
                Ok(Self::PreciousStonesAndMetalsWatchesAndJewelry)
            }
            "professional_services" => Ok(Self::ProfessionalServices),
            "public_warehousing_and_storage" => Ok(Self::PublicWarehousingAndStorage),
            "quick_copy_repro_and_blueprint" => Ok(Self::QuickCopyReproAndBlueprint),
            "railroads" => Ok(Self::Railroads),
            "real_estate_agents_and_managers_rentals" => {
                Ok(Self::RealEstateAgentsAndManagersRentals)
            }
            "record_stores" => Ok(Self::RecordStores),
            "recreational_vehicle_rentals" => Ok(Self::RecreationalVehicleRentals),
            "religious_goods_stores" => Ok(Self::ReligiousGoodsStores),
            "religious_organizations" => Ok(Self::ReligiousOrganizations),
            "roofing_siding_sheet_metal" => Ok(Self::RoofingSidingSheetMetal),
            "secretarial_support_services" => Ok(Self::SecretarialSupportServices),
            "security_brokers_dealers" => Ok(Self::SecurityBrokersDealers),
            "service_stations" => Ok(Self::ServiceStations),
            "sewing_needlework_fabric_and_piece_goods_stores" => {
                Ok(Self::SewingNeedleworkFabricAndPieceGoodsStores)
            }
            "shoe_repair_hat_cleaning" => Ok(Self::ShoeRepairHatCleaning),
            "shoe_stores" => Ok(Self::ShoeStores),
            "small_appliance_repair" => Ok(Self::SmallApplianceRepair),
            "snowmobile_dealers" => Ok(Self::SnowmobileDealers),
            "special_trade_services" => Ok(Self::SpecialTradeServices),
            "specialty_cleaning" => Ok(Self::SpecialtyCleaning),
            "sporting_goods_stores" => Ok(Self::SportingGoodsStores),
            "sporting_recreation_camps" => Ok(Self::SportingRecreationCamps),
            "sports_and_riding_apparel_stores" => Ok(Self::SportsAndRidingApparelStores),
            "sports_clubs_fields" => Ok(Self::SportsClubsFields),
            "stamp_and_coin_stores" => Ok(Self::StampAndCoinStores),
            "stationary_office_supplies_printing_and_writing_paper" => {
                Ok(Self::StationaryOfficeSuppliesPrintingAndWritingPaper)
            }
            "stationery_stores_office_and_school_supply_stores" => {
                Ok(Self::StationeryStoresOfficeAndSchoolSupplyStores)
            }
            "swimming_pools_sales" => Ok(Self::SwimmingPoolsSales),
            "t_ui_travel_germany" => Ok(Self::TUiTravelGermany),
            "tailors_alterations" => Ok(Self::TailorsAlterations),
            "tax_payments_government_agencies" => Ok(Self::TaxPaymentsGovernmentAgencies),
            "tax_preparation_services" => Ok(Self::TaxPreparationServices),
            "taxicabs_limousines" => Ok(Self::TaxicabsLimousines),
            "telecommunication_equipment_and_telephone_sales" => {
                Ok(Self::TelecommunicationEquipmentAndTelephoneSales)
            }
            "telecommunication_services" => Ok(Self::TelecommunicationServices),
            "telegraph_services" => Ok(Self::TelegraphServices),
            "tent_and_awning_shops" => Ok(Self::TentAndAwningShops),
            "testing_laboratories" => Ok(Self::TestingLaboratories),
            "theatrical_ticket_agencies" => Ok(Self::TheatricalTicketAgencies),
            "timeshares" => Ok(Self::Timeshares),
            "tire_retreading_and_repair" => Ok(Self::TireRetreadingAndRepair),
            "tolls_bridge_fees" => Ok(Self::TollsBridgeFees),
            "tourist_attractions_and_exhibits" => Ok(Self::TouristAttractionsAndExhibits),
            "towing_services" => Ok(Self::TowingServices),
            "trailer_parks_campgrounds" => Ok(Self::TrailerParksCampgrounds),
            "transportation_services" => Ok(Self::TransportationServices),
            "travel_agencies_tour_operators" => Ok(Self::TravelAgenciesTourOperators),
            "truck_stop_iteration" => Ok(Self::TruckStopIteration),
            "truck_utility_trailer_rentals" => Ok(Self::TruckUtilityTrailerRentals),
            "typesetting_plate_making_and_related_services" => {
                Ok(Self::TypesettingPlateMakingAndRelatedServices)
            }
            "typewriter_stores" => Ok(Self::TypewriterStores),
            "u_s_federal_government_agencies_or_departments" => {
                Ok(Self::USFederalGovernmentAgenciesOrDepartments)
            }
            "uniforms_commercial_clothing" => Ok(Self::UniformsCommercialClothing),
            "used_merchandise_and_secondhand_stores" => {
                Ok(Self::UsedMerchandiseAndSecondhandStores)
            }
            "utilities" => Ok(Self::Utilities),
            "variety_stores" => Ok(Self::VarietyStores),
            "veterinary_services" => Ok(Self::VeterinaryServices),
            "video_amusement_game_supplies" => Ok(Self::VideoAmusementGameSupplies),
            "video_game_arcades" => Ok(Self::VideoGameArcades),
            "video_tape_rental_stores" => Ok(Self::VideoTapeRentalStores),
            "vocational_trade_schools" => Ok(Self::VocationalTradeSchools),
            "watch_jewelry_repair" => Ok(Self::WatchJewelryRepair),
            "welding_repair" => Ok(Self::WeldingRepair),
            "wholesale_clubs" => Ok(Self::WholesaleClubs),
            "wig_and_toupee_stores" => Ok(Self::WigAndToupeeStores),
            "wires_money_orders" => Ok(Self::WiresMoneyOrders),
            "womens_accessory_and_specialty_shops" => Ok(Self::WomensAccessoryAndSpecialtyShops),
            "womens_ready_to_wear_stores" => Ok(Self::WomensReadyToWearStores),
            "wrecking_and_salvage_yards" => Ok(Self::WreckingAndSalvageYards),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateCardSpendingControlsSpendingLimitsCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCardSpendingControlsSpendingLimitsCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateCardSpendingControlsSpendingLimitsCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Interval (or event) to which the amount applies.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateCardSpendingControlsSpendingLimitsInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

impl UpdateCardSpendingControlsSpendingLimitsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllTime => "all_time",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::PerAuthorization => "per_authorization",
            Self::Weekly => "weekly",
            Self::Yearly => "yearly",
        }
    }
}

impl std::str::FromStr for UpdateCardSpendingControlsSpendingLimitsInterval {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all_time" => Ok(Self::AllTime),
            "daily" => Ok(Self::Daily),
            "monthly" => Ok(Self::Monthly),
            "per_authorization" => Ok(Self::PerAuthorization),
            "weekly" => Ok(Self::Weekly),
            "yearly" => Ok(Self::Yearly),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateCardSpendingControlsSpendingLimitsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCardSpendingControlsSpendingLimitsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateCardSpendingControlsSpendingLimitsInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Dictates whether authorizations can be approved on this card.
///
/// If this card is being canceled because it was lost or stolen, this information should be provided as `cancellation_reason`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateCardStatus {
    Active,
    Canceled,
    Inactive,
}

impl UpdateCardStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Inactive => "inactive",
        }
    }
}

impl std::str::FromStr for UpdateCardStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "canceled" => Ok(Self::Canceled),
            "inactive" => Ok(Self::Inactive),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateCardStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateCardStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeliverCardCard<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> DeliverCardCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ShipCardCard<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ShipCardCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReturnCardCard<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ReturnCardCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FailCardCard<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> FailCardCard<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
