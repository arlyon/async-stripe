// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingCardholderId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Address, Currency, File, MerchantCategory};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingCardholder".
///
/// For more details see <https://stripe.com/docs/api/issuing/cardholders/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholder {
    /// Unique identifier for the object.
    pub id: IssuingCardholderId,

    pub billing: IssuingCardholderAddress,

    /// Additional information about a `company` cardholder.
    pub company: Option<IssuingCardholderCompany>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The cardholder's email address.
    pub email: Option<String>,

    /// Additional information about an `individual` cardholder.
    pub individual: Option<IssuingCardholderIndividual>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The cardholder's name.
    ///
    /// This will be printed on cards issued to them.
    pub name: String,

    /// The cardholder's phone number.
    ///
    /// This is required for all cardholders who will be creating EU cards.
    /// See the [3D Secure documentation](https://stripe.com/docs/issuing/3d-secure#when-is-3d-secure-applied) for more details.
    pub phone_number: Option<String>,

    /// The cardholder’s preferred locales (languages), ordered by preference.
    ///
    /// Locales can be `de`, `en`, `es`, `fr`, or `it`.  This changes the language of the [3D Secure flow](https://stripe.com/docs/issuing/3d-secure) and one-time password messages sent to the cardholder.
    pub preferred_locales: Option<Vec<IssuingCardholderPreferredLocales>>,

    pub requirements: IssuingCardholderRequirements,

    /// Rules that control spending across this cardholder's cards.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    pub spending_controls: Option<IssuingCardholderAuthorizationControls>,

    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub status: IssuingCardholderStatus,

    /// One of `individual` or `company`.
    ///
    /// See [Choose a cardholder type](https://stripe.com/docs/issuing/other/choose-cardholder) for more details.
    #[serde(rename = "type")]
    pub type_: IssuingCardholderType,
}

impl Object for IssuingCardholder {
    type Id = IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.cardholder"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderAddress {
    pub address: Address,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderAuthorizationControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    ///
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    pub allowed_categories: Option<Vec<MerchantCategory>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    ///
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    pub blocked_categories: Option<Vec<MerchantCategory>>,

    /// Limit spending with amount-based rules that apply across this cardholder's cards.
    pub spending_limits: Option<Vec<IssuingCardholderSpendingLimit>>,

    /// Currency of the amounts within `spending_limits`.
    pub spending_limits_currency: Option<Currency>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderCompany {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderIndividual {
    /// Information related to the card_issuing program for this cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<IssuingCardholderCardIssuing>,

    /// The date of birth of this cardholder.
    pub dob: Option<IssuingCardholderIndividualDob>,

    /// The first name of this cardholder.
    ///
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub first_name: Option<String>,

    /// The last name of this cardholder.
    ///
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    pub last_name: Option<String>,

    /// Government-issued ID document for this cardholder.
    pub verification: Option<IssuingCardholderVerification>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderCardIssuing {
    /// Information about cardholder acceptance of Celtic [Authorized User Terms](https://stripe.com/docs/issuing/cards#accept-authorized-user-terms).
    ///
    /// Required for cards backed by a Celtic program.
    pub user_terms_acceptance: Option<IssuingCardholderUserTermsAcceptance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderIndividualDob {
    /// The day of birth, between 1 and 31.
    pub day: Option<i64>,

    /// The month of birth, between 1 and 12.
    pub month: Option<i64>,

    /// The four-digit year of birth.
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderRequirements {
    /// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    pub disabled_reason: Option<IssuingCardholderRequirementsDisabledReason>,

    /// Array of fields that need to be collected in order to verify and re-enable the cardholder.
    pub past_due: Option<Vec<IssuingCardholderRequirementsPastDue>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderSpendingLimit {
    /// Maximum amount allowed to spend per interval.
    ///
    /// This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    ///
    /// Omitting this field will apply the limit to all categories.
    pub categories: Option<Vec<IssuingCardholderSpendingLimitCategories>>,

    /// Interval (or event) to which the amount applies.
    pub interval: IssuingCardholderSpendingLimitInterval,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderUserTermsAcceptance {
    /// The Unix timestamp marking when the cardholder accepted the Authorized User Terms.
    pub date: Option<Timestamp>,

    /// The IP address from which the cardholder accepted the Authorized User Terms.
    pub ip: Option<String>,

    /// The user agent of the browser from which the cardholder accepted the Authorized User Terms.
    pub user_agent: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderVerification {
    /// An identifying document, either a passport or local ID card.
    pub document: Option<IssuingCardholderIdDocument>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingCardholderIdDocument {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub back: Option<Expandable<File>>,

    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    pub front: Option<Expandable<File>>,
}

/// An enum representing the possible values of an `IssuingCardholder`'s `preferred_locales` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderPreferredLocales {
    De,
    En,
    Es,
    Fr,
    It,
}

impl IssuingCardholderPreferredLocales {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderPreferredLocales::De => "de",
            IssuingCardholderPreferredLocales::En => "en",
            IssuingCardholderPreferredLocales::Es => "es",
            IssuingCardholderPreferredLocales::Fr => "fr",
            IssuingCardholderPreferredLocales::It => "it",
        }
    }
}

impl AsRef<str> for IssuingCardholderPreferredLocales {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderPreferredLocales {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingCardholderPreferredLocales {
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `IssuingCardholderRequirements`'s `disabled_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    #[serde(rename = "rejected.listed")]
    RejectedListed,
    #[serde(rename = "requirements.past_due")]
    RequirementsPastDue,
    UnderReview,
}

impl IssuingCardholderRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderRequirementsDisabledReason::Listed => "listed",
            IssuingCardholderRequirementsDisabledReason::RejectedListed => "rejected.listed",
            IssuingCardholderRequirementsDisabledReason::RequirementsPastDue => {
                "requirements.past_due"
            }
            IssuingCardholderRequirementsDisabledReason::UnderReview => "under_review",
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingCardholderRequirementsDisabledReason {
    fn default() -> Self {
        Self::Listed
    }
}

/// An enum representing the possible values of an `IssuingCardholderRequirements`'s `past_due` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsPastDue {
    #[serde(rename = "company.tax_id")]
    CompanyTaxId,
    #[serde(rename = "individual.card_issuing.user_terms_acceptance.date")]
    IndividualCardIssuingUserTermsAcceptanceDate,
    #[serde(rename = "individual.card_issuing.user_terms_acceptance.ip")]
    IndividualCardIssuingUserTermsAcceptanceIp,
    #[serde(rename = "individual.dob.day")]
    IndividualDobDay,
    #[serde(rename = "individual.dob.month")]
    IndividualDobMonth,
    #[serde(rename = "individual.dob.year")]
    IndividualDobYear,
    #[serde(rename = "individual.first_name")]
    IndividualFirstName,
    #[serde(rename = "individual.last_name")]
    IndividualLastName,
    #[serde(rename = "individual.verification.document")]
    IndividualVerificationDocument,
}

impl IssuingCardholderRequirementsPastDue {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderRequirementsPastDue::CompanyTaxId => "company.tax_id",
            IssuingCardholderRequirementsPastDue::IndividualCardIssuingUserTermsAcceptanceDate => {
                "individual.card_issuing.user_terms_acceptance.date"
            }
            IssuingCardholderRequirementsPastDue::IndividualCardIssuingUserTermsAcceptanceIp => {
                "individual.card_issuing.user_terms_acceptance.ip"
            }
            IssuingCardholderRequirementsPastDue::IndividualDobDay => "individual.dob.day",
            IssuingCardholderRequirementsPastDue::IndividualDobMonth => "individual.dob.month",
            IssuingCardholderRequirementsPastDue::IndividualDobYear => "individual.dob.year",
            IssuingCardholderRequirementsPastDue::IndividualFirstName => "individual.first_name",
            IssuingCardholderRequirementsPastDue::IndividualLastName => "individual.last_name",
            IssuingCardholderRequirementsPastDue::IndividualVerificationDocument => {
                "individual.verification.document"
            }
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsPastDue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingCardholderRequirementsPastDue {
    fn default() -> Self {
        Self::CompanyTaxId
    }
}

/// An enum representing the possible values of an `IssuingCardholderSpendingLimit`'s `categories` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderSpendingLimitCategories {
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
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
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
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
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
    Marketplaces,
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

impl IssuingCardholderSpendingLimitCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderSpendingLimitCategories::AcRefrigerationRepair => "ac_refrigeration_repair",
            IssuingCardholderSpendingLimitCategories::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            IssuingCardholderSpendingLimitCategories::AdvertisingServices => "advertising_services",
            IssuingCardholderSpendingLimitCategories::AgriculturalCooperative => "agricultural_cooperative",
            IssuingCardholderSpendingLimitCategories::AirlinesAirCarriers => "airlines_air_carriers",
            IssuingCardholderSpendingLimitCategories::AirportsFlyingFields => "airports_flying_fields",
            IssuingCardholderSpendingLimitCategories::AmbulanceServices => "ambulance_services",
            IssuingCardholderSpendingLimitCategories::AmusementParksCarnivals => "amusement_parks_carnivals",
            IssuingCardholderSpendingLimitCategories::AntiqueReproductions => "antique_reproductions",
            IssuingCardholderSpendingLimitCategories::AntiqueShops => "antique_shops",
            IssuingCardholderSpendingLimitCategories::Aquariums => "aquariums",
            IssuingCardholderSpendingLimitCategories::ArchitecturalSurveyingServices => "architectural_surveying_services",
            IssuingCardholderSpendingLimitCategories::ArtDealersAndGalleries => "art_dealers_and_galleries",
            IssuingCardholderSpendingLimitCategories::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            IssuingCardholderSpendingLimitCategories::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            IssuingCardholderSpendingLimitCategories::AutoBodyRepairShops => "auto_body_repair_shops",
            IssuingCardholderSpendingLimitCategories::AutoPaintShops => "auto_paint_shops",
            IssuingCardholderSpendingLimitCategories::AutoServiceShops => "auto_service_shops",
            IssuingCardholderSpendingLimitCategories::AutomatedCashDisburse => "automated_cash_disburse",
            IssuingCardholderSpendingLimitCategories::AutomatedFuelDispensers => "automated_fuel_dispensers",
            IssuingCardholderSpendingLimitCategories::AutomobileAssociations => "automobile_associations",
            IssuingCardholderSpendingLimitCategories::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            IssuingCardholderSpendingLimitCategories::AutomotiveTireStores => "automotive_tire_stores",
            IssuingCardholderSpendingLimitCategories::BailAndBondPayments => "bail_and_bond_payments",
            IssuingCardholderSpendingLimitCategories::Bakeries => "bakeries",
            IssuingCardholderSpendingLimitCategories::BandsOrchestras => "bands_orchestras",
            IssuingCardholderSpendingLimitCategories::BarberAndBeautyShops => "barber_and_beauty_shops",
            IssuingCardholderSpendingLimitCategories::BettingCasinoGambling => "betting_casino_gambling",
            IssuingCardholderSpendingLimitCategories::BicycleShops => "bicycle_shops",
            IssuingCardholderSpendingLimitCategories::BilliardPoolEstablishments => "billiard_pool_establishments",
            IssuingCardholderSpendingLimitCategories::BoatDealers => "boat_dealers",
            IssuingCardholderSpendingLimitCategories::BoatRentalsAndLeases => "boat_rentals_and_leases",
            IssuingCardholderSpendingLimitCategories::BookStores => "book_stores",
            IssuingCardholderSpendingLimitCategories::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            IssuingCardholderSpendingLimitCategories::BowlingAlleys => "bowling_alleys",
            IssuingCardholderSpendingLimitCategories::BusLines => "bus_lines",
            IssuingCardholderSpendingLimitCategories::BusinessSecretarialSchools => "business_secretarial_schools",
            IssuingCardholderSpendingLimitCategories::BuyingShoppingServices => "buying_shopping_services",
            IssuingCardholderSpendingLimitCategories::CableSatelliteAndOtherPayTelevisionAndRadio => "cable_satellite_and_other_pay_television_and_radio",
            IssuingCardholderSpendingLimitCategories::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            IssuingCardholderSpendingLimitCategories::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            IssuingCardholderSpendingLimitCategories::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            IssuingCardholderSpendingLimitCategories::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            IssuingCardholderSpendingLimitCategories::CarRentalAgencies => "car_rental_agencies",
            IssuingCardholderSpendingLimitCategories::CarWashes => "car_washes",
            IssuingCardholderSpendingLimitCategories::CarpentryServices => "carpentry_services",
            IssuingCardholderSpendingLimitCategories::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            IssuingCardholderSpendingLimitCategories::Caterers => "caterers",
            IssuingCardholderSpendingLimitCategories::CharitableAndSocialServiceOrganizationsFundraising => "charitable_and_social_service_organizations_fundraising",
            IssuingCardholderSpendingLimitCategories::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            IssuingCardholderSpendingLimitCategories::ChildCareServices => "child_care_services",
            IssuingCardholderSpendingLimitCategories::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            IssuingCardholderSpendingLimitCategories::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            IssuingCardholderSpendingLimitCategories::Chiropractors => "chiropractors",
            IssuingCardholderSpendingLimitCategories::CigarStoresAndStands => "cigar_stores_and_stands",
            IssuingCardholderSpendingLimitCategories::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            IssuingCardholderSpendingLimitCategories::CleaningAndMaintenance => "cleaning_and_maintenance",
            IssuingCardholderSpendingLimitCategories::ClothingRental => "clothing_rental",
            IssuingCardholderSpendingLimitCategories::CollegesUniversities => "colleges_universities",
            IssuingCardholderSpendingLimitCategories::CommercialEquipment => "commercial_equipment",
            IssuingCardholderSpendingLimitCategories::CommercialFootwear => "commercial_footwear",
            IssuingCardholderSpendingLimitCategories::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            IssuingCardholderSpendingLimitCategories::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            IssuingCardholderSpendingLimitCategories::ComputerNetworkServices => "computer_network_services",
            IssuingCardholderSpendingLimitCategories::ComputerProgramming => "computer_programming",
            IssuingCardholderSpendingLimitCategories::ComputerRepair => "computer_repair",
            IssuingCardholderSpendingLimitCategories::ComputerSoftwareStores => "computer_software_stores",
            IssuingCardholderSpendingLimitCategories::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            IssuingCardholderSpendingLimitCategories::ConcreteWorkServices => "concrete_work_services",
            IssuingCardholderSpendingLimitCategories::ConstructionMaterials => "construction_materials",
            IssuingCardholderSpendingLimitCategories::ConsultingPublicRelations => "consulting_public_relations",
            IssuingCardholderSpendingLimitCategories::CorrespondenceSchools => "correspondence_schools",
            IssuingCardholderSpendingLimitCategories::CosmeticStores => "cosmetic_stores",
            IssuingCardholderSpendingLimitCategories::CounselingServices => "counseling_services",
            IssuingCardholderSpendingLimitCategories::CountryClubs => "country_clubs",
            IssuingCardholderSpendingLimitCategories::CourierServices => "courier_services",
            IssuingCardholderSpendingLimitCategories::CourtCosts => "court_costs",
            IssuingCardholderSpendingLimitCategories::CreditReportingAgencies => "credit_reporting_agencies",
            IssuingCardholderSpendingLimitCategories::CruiseLines => "cruise_lines",
            IssuingCardholderSpendingLimitCategories::DairyProductsStores => "dairy_products_stores",
            IssuingCardholderSpendingLimitCategories::DanceHallStudiosSchools => "dance_hall_studios_schools",
            IssuingCardholderSpendingLimitCategories::DatingEscortServices => "dating_escort_services",
            IssuingCardholderSpendingLimitCategories::DentistsOrthodontists => "dentists_orthodontists",
            IssuingCardholderSpendingLimitCategories::DepartmentStores => "department_stores",
            IssuingCardholderSpendingLimitCategories::DetectiveAgencies => "detective_agencies",
            IssuingCardholderSpendingLimitCategories::DigitalGoodsApplications => "digital_goods_applications",
            IssuingCardholderSpendingLimitCategories::DigitalGoodsGames => "digital_goods_games",
            IssuingCardholderSpendingLimitCategories::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            IssuingCardholderSpendingLimitCategories::DigitalGoodsMedia => "digital_goods_media",
            IssuingCardholderSpendingLimitCategories::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            IssuingCardholderSpendingLimitCategories::DirectMarketingCombinationCatalogAndRetailMerchant => "direct_marketing_combination_catalog_and_retail_merchant",
            IssuingCardholderSpendingLimitCategories::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            IssuingCardholderSpendingLimitCategories::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            IssuingCardholderSpendingLimitCategories::DirectMarketingOther => "direct_marketing_other",
            IssuingCardholderSpendingLimitCategories::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            IssuingCardholderSpendingLimitCategories::DirectMarketingSubscription => "direct_marketing_subscription",
            IssuingCardholderSpendingLimitCategories::DirectMarketingTravel => "direct_marketing_travel",
            IssuingCardholderSpendingLimitCategories::DiscountStores => "discount_stores",
            IssuingCardholderSpendingLimitCategories::Doctors => "doctors",
            IssuingCardholderSpendingLimitCategories::DoorToDoorSales => "door_to_door_sales",
            IssuingCardholderSpendingLimitCategories::DraperyWindowCoveringAndUpholsteryStores => "drapery_window_covering_and_upholstery_stores",
            IssuingCardholderSpendingLimitCategories::DrinkingPlaces => "drinking_places",
            IssuingCardholderSpendingLimitCategories::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            IssuingCardholderSpendingLimitCategories::DrugsDrugProprietariesAndDruggistSundries => "drugs_drug_proprietaries_and_druggist_sundries",
            IssuingCardholderSpendingLimitCategories::DryCleaners => "dry_cleaners",
            IssuingCardholderSpendingLimitCategories::DurableGoods => "durable_goods",
            IssuingCardholderSpendingLimitCategories::DutyFreeStores => "duty_free_stores",
            IssuingCardholderSpendingLimitCategories::EatingPlacesRestaurants => "eating_places_restaurants",
            IssuingCardholderSpendingLimitCategories::EducationalServices => "educational_services",
            IssuingCardholderSpendingLimitCategories::ElectricRazorStores => "electric_razor_stores",
            IssuingCardholderSpendingLimitCategories::ElectricVehicleCharging => "electric_vehicle_charging",
            IssuingCardholderSpendingLimitCategories::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            IssuingCardholderSpendingLimitCategories::ElectricalServices => "electrical_services",
            IssuingCardholderSpendingLimitCategories::ElectronicsRepairShops => "electronics_repair_shops",
            IssuingCardholderSpendingLimitCategories::ElectronicsStores => "electronics_stores",
            IssuingCardholderSpendingLimitCategories::ElementarySecondarySchools => "elementary_secondary_schools",
            IssuingCardholderSpendingLimitCategories::EmergencyServicesGcasVisaUseOnly => "emergency_services_gcas_visa_use_only",
            IssuingCardholderSpendingLimitCategories::EmploymentTempAgencies => "employment_temp_agencies",
            IssuingCardholderSpendingLimitCategories::EquipmentRental => "equipment_rental",
            IssuingCardholderSpendingLimitCategories::ExterminatingServices => "exterminating_services",
            IssuingCardholderSpendingLimitCategories::FamilyClothingStores => "family_clothing_stores",
            IssuingCardholderSpendingLimitCategories::FastFoodRestaurants => "fast_food_restaurants",
            IssuingCardholderSpendingLimitCategories::FinancialInstitutions => "financial_institutions",
            IssuingCardholderSpendingLimitCategories::FinesGovernmentAdministrativeEntities => "fines_government_administrative_entities",
            IssuingCardholderSpendingLimitCategories::FireplaceFireplaceScreensAndAccessoriesStores => "fireplace_fireplace_screens_and_accessories_stores",
            IssuingCardholderSpendingLimitCategories::FloorCoveringStores => "floor_covering_stores",
            IssuingCardholderSpendingLimitCategories::Florists => "florists",
            IssuingCardholderSpendingLimitCategories::FloristsSuppliesNurseryStockAndFlowers => "florists_supplies_nursery_stock_and_flowers",
            IssuingCardholderSpendingLimitCategories::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            IssuingCardholderSpendingLimitCategories::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            IssuingCardholderSpendingLimitCategories::FuneralServicesCrematories => "funeral_services_crematories",
            IssuingCardholderSpendingLimitCategories::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => "furniture_home_furnishings_and_equipment_stores_except_appliances",
            IssuingCardholderSpendingLimitCategories::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            IssuingCardholderSpendingLimitCategories::FurriersAndFurShops => "furriers_and_fur_shops",
            IssuingCardholderSpendingLimitCategories::GeneralServices => "general_services",
            IssuingCardholderSpendingLimitCategories::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            IssuingCardholderSpendingLimitCategories::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            IssuingCardholderSpendingLimitCategories::GlasswareCrystalStores => "glassware_crystal_stores",
            IssuingCardholderSpendingLimitCategories::GolfCoursesPublic => "golf_courses_public",
            IssuingCardholderSpendingLimitCategories::GovernmentLicensedHorseDogRacingUsRegionOnly => "government_licensed_horse_dog_racing_us_region_only",
            IssuingCardholderSpendingLimitCategories::GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly => "government_licensed_online_casions_online_gambling_us_region_only",
            IssuingCardholderSpendingLimitCategories::GovernmentOwnedLotteriesNonUsRegion => "government_owned_lotteries_non_us_region",
            IssuingCardholderSpendingLimitCategories::GovernmentOwnedLotteriesUsRegionOnly => "government_owned_lotteries_us_region_only",
            IssuingCardholderSpendingLimitCategories::GovernmentServices => "government_services",
            IssuingCardholderSpendingLimitCategories::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            IssuingCardholderSpendingLimitCategories::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            IssuingCardholderSpendingLimitCategories::HardwareStores => "hardware_stores",
            IssuingCardholderSpendingLimitCategories::HealthAndBeautySpas => "health_and_beauty_spas",
            IssuingCardholderSpendingLimitCategories::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            IssuingCardholderSpendingLimitCategories::HeatingPlumbingAC => "heating_plumbing_a_c",
            IssuingCardholderSpendingLimitCategories::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            IssuingCardholderSpendingLimitCategories::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            IssuingCardholderSpendingLimitCategories::Hospitals => "hospitals",
            IssuingCardholderSpendingLimitCategories::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            IssuingCardholderSpendingLimitCategories::HouseholdApplianceStores => "household_appliance_stores",
            IssuingCardholderSpendingLimitCategories::IndustrialSupplies => "industrial_supplies",
            IssuingCardholderSpendingLimitCategories::InformationRetrievalServices => "information_retrieval_services",
            IssuingCardholderSpendingLimitCategories::InsuranceDefault => "insurance_default",
            IssuingCardholderSpendingLimitCategories::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            IssuingCardholderSpendingLimitCategories::IntraCompanyPurchases => "intra_company_purchases",
            IssuingCardholderSpendingLimitCategories::JewelryStoresWatchesClocksAndSilverwareStores => "jewelry_stores_watches_clocks_and_silverware_stores",
            IssuingCardholderSpendingLimitCategories::LandscapingServices => "landscaping_services",
            IssuingCardholderSpendingLimitCategories::Laundries => "laundries",
            IssuingCardholderSpendingLimitCategories::LaundryCleaningServices => "laundry_cleaning_services",
            IssuingCardholderSpendingLimitCategories::LegalServicesAttorneys => "legal_services_attorneys",
            IssuingCardholderSpendingLimitCategories::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            IssuingCardholderSpendingLimitCategories::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            IssuingCardholderSpendingLimitCategories::ManualCashDisburse => "manual_cash_disburse",
            IssuingCardholderSpendingLimitCategories::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            IssuingCardholderSpendingLimitCategories::Marketplaces => "marketplaces",
            IssuingCardholderSpendingLimitCategories::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            IssuingCardholderSpendingLimitCategories::MassageParlors => "massage_parlors",
            IssuingCardholderSpendingLimitCategories::MedicalAndDentalLabs => "medical_and_dental_labs",
            IssuingCardholderSpendingLimitCategories::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => "medical_dental_ophthalmic_and_hospital_equipment_and_supplies",
            IssuingCardholderSpendingLimitCategories::MedicalServices => "medical_services",
            IssuingCardholderSpendingLimitCategories::MembershipOrganizations => "membership_organizations",
            IssuingCardholderSpendingLimitCategories::MensAndBoysClothingAndAccessoriesStores => "mens_and_boys_clothing_and_accessories_stores",
            IssuingCardholderSpendingLimitCategories::MensWomensClothingStores => "mens_womens_clothing_stores",
            IssuingCardholderSpendingLimitCategories::MetalServiceCenters => "metal_service_centers",
            IssuingCardholderSpendingLimitCategories::Miscellaneous => "miscellaneous",
            IssuingCardholderSpendingLimitCategories::MiscellaneousApparelAndAccessoryShops => "miscellaneous_apparel_and_accessory_shops",
            IssuingCardholderSpendingLimitCategories::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            IssuingCardholderSpendingLimitCategories::MiscellaneousBusinessServices => "miscellaneous_business_services",
            IssuingCardholderSpendingLimitCategories::MiscellaneousFoodStores => "miscellaneous_food_stores",
            IssuingCardholderSpendingLimitCategories::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            IssuingCardholderSpendingLimitCategories::MiscellaneousGeneralServices => "miscellaneous_general_services",
            IssuingCardholderSpendingLimitCategories::MiscellaneousHomeFurnishingSpecialtyStores => "miscellaneous_home_furnishing_specialty_stores",
            IssuingCardholderSpendingLimitCategories::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            IssuingCardholderSpendingLimitCategories::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            IssuingCardholderSpendingLimitCategories::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            IssuingCardholderSpendingLimitCategories::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            IssuingCardholderSpendingLimitCategories::MobileHomeDealers => "mobile_home_dealers",
            IssuingCardholderSpendingLimitCategories::MotionPictureTheaters => "motion_picture_theaters",
            IssuingCardholderSpendingLimitCategories::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            IssuingCardholderSpendingLimitCategories::MotorHomesDealers => "motor_homes_dealers",
            IssuingCardholderSpendingLimitCategories::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            IssuingCardholderSpendingLimitCategories::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            IssuingCardholderSpendingLimitCategories::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            IssuingCardholderSpendingLimitCategories::MusicStoresMusicalInstrumentsPianosAndSheetMusic => "music_stores_musical_instruments_pianos_and_sheet_music",
            IssuingCardholderSpendingLimitCategories::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            IssuingCardholderSpendingLimitCategories::NonFiMoneyOrders => "non_fi_money_orders",
            IssuingCardholderSpendingLimitCategories::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            IssuingCardholderSpendingLimitCategories::NondurableGoods => "nondurable_goods",
            IssuingCardholderSpendingLimitCategories::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            IssuingCardholderSpendingLimitCategories::NursingPersonalCare => "nursing_personal_care",
            IssuingCardholderSpendingLimitCategories::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            IssuingCardholderSpendingLimitCategories::OpticiansEyeglasses => "opticians_eyeglasses",
            IssuingCardholderSpendingLimitCategories::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            IssuingCardholderSpendingLimitCategories::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            IssuingCardholderSpendingLimitCategories::Osteopaths => "osteopaths",
            IssuingCardholderSpendingLimitCategories::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            IssuingCardholderSpendingLimitCategories::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            IssuingCardholderSpendingLimitCategories::ParkingLotsGarages => "parking_lots_garages",
            IssuingCardholderSpendingLimitCategories::PassengerRailways => "passenger_railways",
            IssuingCardholderSpendingLimitCategories::PawnShops => "pawn_shops",
            IssuingCardholderSpendingLimitCategories::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            IssuingCardholderSpendingLimitCategories::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            IssuingCardholderSpendingLimitCategories::PhotoDeveloping => "photo_developing",
            IssuingCardholderSpendingLimitCategories::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => "photographic_photocopy_microfilm_equipment_and_supplies",
            IssuingCardholderSpendingLimitCategories::PhotographicStudios => "photographic_studios",
            IssuingCardholderSpendingLimitCategories::PictureVideoProduction => "picture_video_production",
            IssuingCardholderSpendingLimitCategories::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            IssuingCardholderSpendingLimitCategories::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            IssuingCardholderSpendingLimitCategories::PoliticalOrganizations => "political_organizations",
            IssuingCardholderSpendingLimitCategories::PostalServicesGovernmentOnly => "postal_services_government_only",
            IssuingCardholderSpendingLimitCategories::PreciousStonesAndMetalsWatchesAndJewelry => "precious_stones_and_metals_watches_and_jewelry",
            IssuingCardholderSpendingLimitCategories::ProfessionalServices => "professional_services",
            IssuingCardholderSpendingLimitCategories::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            IssuingCardholderSpendingLimitCategories::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            IssuingCardholderSpendingLimitCategories::Railroads => "railroads",
            IssuingCardholderSpendingLimitCategories::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            IssuingCardholderSpendingLimitCategories::RecordStores => "record_stores",
            IssuingCardholderSpendingLimitCategories::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            IssuingCardholderSpendingLimitCategories::ReligiousGoodsStores => "religious_goods_stores",
            IssuingCardholderSpendingLimitCategories::ReligiousOrganizations => "religious_organizations",
            IssuingCardholderSpendingLimitCategories::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            IssuingCardholderSpendingLimitCategories::SecretarialSupportServices => "secretarial_support_services",
            IssuingCardholderSpendingLimitCategories::SecurityBrokersDealers => "security_brokers_dealers",
            IssuingCardholderSpendingLimitCategories::ServiceStations => "service_stations",
            IssuingCardholderSpendingLimitCategories::SewingNeedleworkFabricAndPieceGoodsStores => "sewing_needlework_fabric_and_piece_goods_stores",
            IssuingCardholderSpendingLimitCategories::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            IssuingCardholderSpendingLimitCategories::ShoeStores => "shoe_stores",
            IssuingCardholderSpendingLimitCategories::SmallApplianceRepair => "small_appliance_repair",
            IssuingCardholderSpendingLimitCategories::SnowmobileDealers => "snowmobile_dealers",
            IssuingCardholderSpendingLimitCategories::SpecialTradeServices => "special_trade_services",
            IssuingCardholderSpendingLimitCategories::SpecialtyCleaning => "specialty_cleaning",
            IssuingCardholderSpendingLimitCategories::SportingGoodsStores => "sporting_goods_stores",
            IssuingCardholderSpendingLimitCategories::SportingRecreationCamps => "sporting_recreation_camps",
            IssuingCardholderSpendingLimitCategories::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            IssuingCardholderSpendingLimitCategories::SportsClubsFields => "sports_clubs_fields",
            IssuingCardholderSpendingLimitCategories::StampAndCoinStores => "stamp_and_coin_stores",
            IssuingCardholderSpendingLimitCategories::StationaryOfficeSuppliesPrintingAndWritingPaper => "stationary_office_supplies_printing_and_writing_paper",
            IssuingCardholderSpendingLimitCategories::StationeryStoresOfficeAndSchoolSupplyStores => "stationery_stores_office_and_school_supply_stores",
            IssuingCardholderSpendingLimitCategories::SwimmingPoolsSales => "swimming_pools_sales",
            IssuingCardholderSpendingLimitCategories::TUiTravelGermany => "t_ui_travel_germany",
            IssuingCardholderSpendingLimitCategories::TailorsAlterations => "tailors_alterations",
            IssuingCardholderSpendingLimitCategories::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            IssuingCardholderSpendingLimitCategories::TaxPreparationServices => "tax_preparation_services",
            IssuingCardholderSpendingLimitCategories::TaxicabsLimousines => "taxicabs_limousines",
            IssuingCardholderSpendingLimitCategories::TelecommunicationEquipmentAndTelephoneSales => "telecommunication_equipment_and_telephone_sales",
            IssuingCardholderSpendingLimitCategories::TelecommunicationServices => "telecommunication_services",
            IssuingCardholderSpendingLimitCategories::TelegraphServices => "telegraph_services",
            IssuingCardholderSpendingLimitCategories::TentAndAwningShops => "tent_and_awning_shops",
            IssuingCardholderSpendingLimitCategories::TestingLaboratories => "testing_laboratories",
            IssuingCardholderSpendingLimitCategories::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            IssuingCardholderSpendingLimitCategories::Timeshares => "timeshares",
            IssuingCardholderSpendingLimitCategories::TireRetreadingAndRepair => "tire_retreading_and_repair",
            IssuingCardholderSpendingLimitCategories::TollsBridgeFees => "tolls_bridge_fees",
            IssuingCardholderSpendingLimitCategories::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            IssuingCardholderSpendingLimitCategories::TowingServices => "towing_services",
            IssuingCardholderSpendingLimitCategories::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            IssuingCardholderSpendingLimitCategories::TransportationServices => "transportation_services",
            IssuingCardholderSpendingLimitCategories::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            IssuingCardholderSpendingLimitCategories::TruckStopIteration => "truck_stop_iteration",
            IssuingCardholderSpendingLimitCategories::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            IssuingCardholderSpendingLimitCategories::TypesettingPlateMakingAndRelatedServices => "typesetting_plate_making_and_related_services",
            IssuingCardholderSpendingLimitCategories::TypewriterStores => "typewriter_stores",
            IssuingCardholderSpendingLimitCategories::USFederalGovernmentAgenciesOrDepartments => "u_s_federal_government_agencies_or_departments",
            IssuingCardholderSpendingLimitCategories::UniformsCommercialClothing => "uniforms_commercial_clothing",
            IssuingCardholderSpendingLimitCategories::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            IssuingCardholderSpendingLimitCategories::Utilities => "utilities",
            IssuingCardholderSpendingLimitCategories::VarietyStores => "variety_stores",
            IssuingCardholderSpendingLimitCategories::VeterinaryServices => "veterinary_services",
            IssuingCardholderSpendingLimitCategories::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            IssuingCardholderSpendingLimitCategories::VideoGameArcades => "video_game_arcades",
            IssuingCardholderSpendingLimitCategories::VideoTapeRentalStores => "video_tape_rental_stores",
            IssuingCardholderSpendingLimitCategories::VocationalTradeSchools => "vocational_trade_schools",
            IssuingCardholderSpendingLimitCategories::WatchJewelryRepair => "watch_jewelry_repair",
            IssuingCardholderSpendingLimitCategories::WeldingRepair => "welding_repair",
            IssuingCardholderSpendingLimitCategories::WholesaleClubs => "wholesale_clubs",
            IssuingCardholderSpendingLimitCategories::WigAndToupeeStores => "wig_and_toupee_stores",
            IssuingCardholderSpendingLimitCategories::WiresMoneyOrders => "wires_money_orders",
            IssuingCardholderSpendingLimitCategories::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            IssuingCardholderSpendingLimitCategories::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            IssuingCardholderSpendingLimitCategories::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for IssuingCardholderSpendingLimitCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderSpendingLimitCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingCardholderSpendingLimitCategories {
    fn default() -> Self {
        Self::AcRefrigerationRepair
    }
}

/// An enum representing the possible values of an `IssuingCardholderSpendingLimit`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderSpendingLimitInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

impl IssuingCardholderSpendingLimitInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderSpendingLimitInterval::AllTime => "all_time",
            IssuingCardholderSpendingLimitInterval::Daily => "daily",
            IssuingCardholderSpendingLimitInterval::Monthly => "monthly",
            IssuingCardholderSpendingLimitInterval::PerAuthorization => "per_authorization",
            IssuingCardholderSpendingLimitInterval::Weekly => "weekly",
            IssuingCardholderSpendingLimitInterval::Yearly => "yearly",
        }
    }
}

impl AsRef<str> for IssuingCardholderSpendingLimitInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderSpendingLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingCardholderSpendingLimitInterval {
    fn default() -> Self {
        Self::AllTime
    }
}

/// An enum representing the possible values of an `IssuingCardholder`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderStatus {
    Active,
    Blocked,
    Inactive,
}

impl IssuingCardholderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderStatus::Active => "active",
            IssuingCardholderStatus::Blocked => "blocked",
            IssuingCardholderStatus::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for IssuingCardholderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingCardholderStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `IssuingCardholder`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderType {
    Company,
    Individual,
}

impl IssuingCardholderType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderType::Company => "company",
            IssuingCardholderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for IssuingCardholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for IssuingCardholderType {
    fn default() -> Self {
        Self::Company
    }
}
