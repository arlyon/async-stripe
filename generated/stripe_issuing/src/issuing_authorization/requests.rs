#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListIssuingAuthorization<'a> {
    /// Only return authorizations that belong to the given card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<&'a str>,
    /// Only return authorizations that belong to the given cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    /// Only return authorizations that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Only return authorizations with the given status. One of `pending`, `closed`, or `reversed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_shared::IssuingAuthorizationStatus>,
}
impl<'a> ListIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListIssuingAuthorization<'a> {
    /// Returns a list of Issuing `Authorization` objects.
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::IssuingAuthorization>> {
        client.get_query("/issuing/authorizations", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_shared::IssuingAuthorization>> {
        stripe::ListPaginator::from_list_params("/issuing/authorizations", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIssuingAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIssuingAuthorization<'a> {
    /// Retrieves an Issuing `Authorization` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_shared::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.get_query(&format!("/issuing/authorizations/{authorization}"), self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> UpdateIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateIssuingAuthorization<'a> {
    /// Updates the specified Issuing `Authorization` object by setting the values of the parameters passed.
    /// Any parameters not provided will be left unchanged.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_shared::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form(
            &format!("/issuing/authorizations/{authorization}"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ApproveIssuingAuthorization<'a> {
    /// If the authorization's `pending_request.is_amount_controllable` property is `true`, you may provide this value to control how much to hold for the authorization.
    /// Must be positive (use [`decline`](https://stripe.com/docs/api/issuing/authorizations/decline) to decline an authorization request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> ApproveIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ApproveIssuingAuthorization<'a> {
    /// \[Deprecated\] Approves a pending Issuing `Authorization` object.
    /// This request should be made within the timeout window of the [real-time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow.
    ///
    /// This method is deprecated.
    /// Instead, [respond directly to the webhook request to approve an authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_shared::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form(
            &format!("/issuing/authorizations/{authorization}/approve"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeclineIssuingAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
}
impl<'a> DeclineIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> DeclineIssuingAuthorization<'a> {
    /// \[Deprecated\] Declines a pending Issuing `Authorization` object.
    /// This request should be made within the timeout window of the [real time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow.
    /// This method is deprecated.
    /// Instead, [respond directly to the webhook request to decline an authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_shared::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form(
            &format!("/issuing/authorizations/{authorization}/decline"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateIssuingAuthorization<'a> {
    /// The total amount to attempt to authorize.
    /// This amount is in the provided currency, or defaults to the card's currency, and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Detailed breakdown of amount components.
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<CreateIssuingAuthorizationAmountDetails>,
    /// How the card details were provided. Defaults to online.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_method: Option<stripe_shared::IssuingAuthorizationAuthorizationMethod>,
    /// Card associated with this authorization.
    pub card: &'a str,
    /// The currency of the authorization.
    /// If not provided, defaults to the currency of the card.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// If set `true`, you may provide [amount](https://stripe.com/docs/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_amount_controllable: Option<bool>,
    /// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_data: Option<CreateIssuingAuthorizationMerchantData<'a>>,
    /// Details about the authorization, such as identifiers, set by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<CreateIssuingAuthorizationNetworkData<'a>>,
    /// Verifications that Stripe performed on information that the cardholder provided to the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_data: Option<CreateIssuingAuthorizationVerificationData>,
    /// The digital wallet used for this transaction.
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    /// Will populate as `null` when no digital wallet was utilized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<CreateIssuingAuthorizationWallet>,
}
impl<'a> CreateIssuingAuthorization<'a> {
    pub fn new(amount: i64, card: &'a str) -> Self {
        Self {
            amount,
            amount_details: None,
            authorization_method: None,
            card,
            currency: None,
            expand: None,
            is_amount_controllable: None,
            merchant_data: None,
            network_data: None,
            verification_data: None,
            wallet: None,
        }
    }
}
/// Detailed breakdown of amount components.
/// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingAuthorizationAmountDetails {
    /// The ATM withdrawal fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atm_fee: Option<i64>,
    /// The amount of cash requested by the cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashback_amount: Option<i64>,
}
impl CreateIssuingAuthorizationAmountDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingAuthorizationMerchantData<'a> {
    /// A categorization of the seller's type of business.
    /// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CreateIssuingAuthorizationMerchantDataCategory>,
    /// City where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<&'a str>,
    /// Country where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// Name of the seller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// Identifier assigned to the seller by the card network.
    /// Different card networks may assign different network_id fields to the same merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<&'a str>,
    /// Postal code where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    /// State where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// An ID assigned by the seller to the location of the sale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<&'a str>,
    /// URL provided by the merchant on a 3DS request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}
impl<'a> CreateIssuingAuthorizationMerchantData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A categorization of the seller's type of business.
/// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateIssuingAuthorizationMerchantDataCategory {
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl CreateIssuingAuthorizationMerchantDataCategory {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationMerchantDataCategory::*;
        match self {
            AcRefrigerationRepair => "ac_refrigeration_repair",
            AccountingBookkeepingServices => "accounting_bookkeeping_services",
            AdvertisingServices => "advertising_services",
            AgriculturalCooperative => "agricultural_cooperative",
            AirlinesAirCarriers => "airlines_air_carriers",
            AirportsFlyingFields => "airports_flying_fields",
            AmbulanceServices => "ambulance_services",
            AmusementParksCarnivals => "amusement_parks_carnivals",
            AntiqueReproductions => "antique_reproductions",
            AntiqueShops => "antique_shops",
            Aquariums => "aquariums",
            ArchitecturalSurveyingServices => "architectural_surveying_services",
            ArtDealersAndGalleries => "art_dealers_and_galleries",
            ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            AutoBodyRepairShops => "auto_body_repair_shops",
            AutoPaintShops => "auto_paint_shops",
            AutoServiceShops => "auto_service_shops",
            AutomatedCashDisburse => "automated_cash_disburse",
            AutomatedFuelDispensers => "automated_fuel_dispensers",
            AutomobileAssociations => "automobile_associations",
            AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            AutomotiveTireStores => "automotive_tire_stores",
            BailAndBondPayments => "bail_and_bond_payments",
            Bakeries => "bakeries",
            BandsOrchestras => "bands_orchestras",
            BarberAndBeautyShops => "barber_and_beauty_shops",
            BettingCasinoGambling => "betting_casino_gambling",
            BicycleShops => "bicycle_shops",
            BilliardPoolEstablishments => "billiard_pool_establishments",
            BoatDealers => "boat_dealers",
            BoatRentalsAndLeases => "boat_rentals_and_leases",
            BookStores => "book_stores",
            BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            BowlingAlleys => "bowling_alleys",
            BusLines => "bus_lines",
            BusinessSecretarialSchools => "business_secretarial_schools",
            BuyingShoppingServices => "buying_shopping_services",
            CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            CarRentalAgencies => "car_rental_agencies",
            CarWashes => "car_washes",
            CarpentryServices => "carpentry_services",
            CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Caterers => "caterers",
            CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            ChildCareServices => "child_care_services",
            ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Chiropractors => "chiropractors",
            CigarStoresAndStands => "cigar_stores_and_stands",
            CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            CleaningAndMaintenance => "cleaning_and_maintenance",
            ClothingRental => "clothing_rental",
            CollegesUniversities => "colleges_universities",
            CommercialEquipment => "commercial_equipment",
            CommercialFootwear => "commercial_footwear",
            CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            CommuterTransportAndFerries => "commuter_transport_and_ferries",
            ComputerNetworkServices => "computer_network_services",
            ComputerProgramming => "computer_programming",
            ComputerRepair => "computer_repair",
            ComputerSoftwareStores => "computer_software_stores",
            ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            ConcreteWorkServices => "concrete_work_services",
            ConstructionMaterials => "construction_materials",
            ConsultingPublicRelations => "consulting_public_relations",
            CorrespondenceSchools => "correspondence_schools",
            CosmeticStores => "cosmetic_stores",
            CounselingServices => "counseling_services",
            CountryClubs => "country_clubs",
            CourierServices => "courier_services",
            CourtCosts => "court_costs",
            CreditReportingAgencies => "credit_reporting_agencies",
            CruiseLines => "cruise_lines",
            DairyProductsStores => "dairy_products_stores",
            DanceHallStudiosSchools => "dance_hall_studios_schools",
            DatingEscortServices => "dating_escort_services",
            DentistsOrthodontists => "dentists_orthodontists",
            DepartmentStores => "department_stores",
            DetectiveAgencies => "detective_agencies",
            DigitalGoodsApplications => "digital_goods_applications",
            DigitalGoodsGames => "digital_goods_games",
            DigitalGoodsLargeVolume => "digital_goods_large_volume",
            DigitalGoodsMedia => "digital_goods_media",
            DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            DirectMarketingOther => "direct_marketing_other",
            DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            DirectMarketingSubscription => "direct_marketing_subscription",
            DirectMarketingTravel => "direct_marketing_travel",
            DiscountStores => "discount_stores",
            Doctors => "doctors",
            DoorToDoorSales => "door_to_door_sales",
            DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            DrinkingPlaces => "drinking_places",
            DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            DryCleaners => "dry_cleaners",
            DurableGoods => "durable_goods",
            DutyFreeStores => "duty_free_stores",
            EatingPlacesRestaurants => "eating_places_restaurants",
            EducationalServices => "educational_services",
            ElectricRazorStores => "electric_razor_stores",
            ElectricVehicleCharging => "electric_vehicle_charging",
            ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            ElectricalServices => "electrical_services",
            ElectronicsRepairShops => "electronics_repair_shops",
            ElectronicsStores => "electronics_stores",
            ElementarySecondarySchools => "elementary_secondary_schools",
            EmergencyServicesGcasVisaUseOnly => "emergency_services_gcas_visa_use_only",
            EmploymentTempAgencies => "employment_temp_agencies",
            EquipmentRental => "equipment_rental",
            ExterminatingServices => "exterminating_services",
            FamilyClothingStores => "family_clothing_stores",
            FastFoodRestaurants => "fast_food_restaurants",
            FinancialInstitutions => "financial_institutions",
            FinesGovernmentAdministrativeEntities => "fines_government_administrative_entities",
            FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            FloorCoveringStores => "floor_covering_stores",
            Florists => "florists",
            FloristsSuppliesNurseryStockAndFlowers => "florists_supplies_nursery_stock_and_flowers",
            FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            FuneralServicesCrematories => "funeral_services_crematories",
            FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            FurnitureRepairRefinishing => "furniture_repair_refinishing",
            FurriersAndFurShops => "furriers_and_fur_shops",
            GeneralServices => "general_services",
            GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            GlasswareCrystalStores => "glassware_crystal_stores",
            GolfCoursesPublic => "golf_courses_public",
            GovernmentLicensedHorseDogRacingUsRegionOnly => {
                "government_licensed_horse_dog_racing_us_region_only"
            }
            GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly => {
                "government_licensed_online_casions_online_gambling_us_region_only"
            }
            GovernmentOwnedLotteriesNonUsRegion => "government_owned_lotteries_non_us_region",
            GovernmentOwnedLotteriesUsRegionOnly => "government_owned_lotteries_us_region_only",
            GovernmentServices => "government_services",
            GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            HardwareStores => "hardware_stores",
            HealthAndBeautySpas => "health_and_beauty_spas",
            HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            HeatingPlumbingAC => "heating_plumbing_a_c",
            HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Hospitals => "hospitals",
            HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            HouseholdApplianceStores => "household_appliance_stores",
            IndustrialSupplies => "industrial_supplies",
            InformationRetrievalServices => "information_retrieval_services",
            InsuranceDefault => "insurance_default",
            InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            IntraCompanyPurchases => "intra_company_purchases",
            JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            LandscapingServices => "landscaping_services",
            Laundries => "laundries",
            LaundryCleaningServices => "laundry_cleaning_services",
            LegalServicesAttorneys => "legal_services_attorneys",
            LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            ManualCashDisburse => "manual_cash_disburse",
            MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Marketplaces => "marketplaces",
            MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            MassageParlors => "massage_parlors",
            MedicalAndDentalLabs => "medical_and_dental_labs",
            MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            MedicalServices => "medical_services",
            MembershipOrganizations => "membership_organizations",
            MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            MensWomensClothingStores => "mens_womens_clothing_stores",
            MetalServiceCenters => "metal_service_centers",
            MiscellaneousApparelAndAccessoryShops => "miscellaneous_apparel_and_accessory_shops",
            MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            MiscellaneousBusinessServices => "miscellaneous_business_services",
            MiscellaneousFoodStores => "miscellaneous_food_stores",
            MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            MiscellaneousGeneralServices => "miscellaneous_general_services",
            MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            MiscellaneousRepairShops => "miscellaneous_repair_shops",
            MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            MobileHomeDealers => "mobile_home_dealers",
            MotionPictureTheaters => "motion_picture_theaters",
            MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            MotorHomesDealers => "motor_homes_dealers",
            MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            MotorcycleShopsDealers => "motorcycle_shops_dealers",
            MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            NonFiMoneyOrders => "non_fi_money_orders",
            NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            NondurableGoods => "nondurable_goods",
            NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            NursingPersonalCare => "nursing_personal_care",
            OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            OpticiansEyeglasses => "opticians_eyeglasses",
            OptometristsOphthalmologist => "optometrists_ophthalmologist",
            OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Osteopaths => "osteopaths",
            PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            ParkingLotsGarages => "parking_lots_garages",
            PassengerRailways => "passenger_railways",
            PawnShops => "pawn_shops",
            PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            PhotoDeveloping => "photo_developing",
            PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            PhotographicStudios => "photographic_studios",
            PictureVideoProduction => "picture_video_production",
            PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            PoliticalOrganizations => "political_organizations",
            PostalServicesGovernmentOnly => "postal_services_government_only",
            PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            ProfessionalServices => "professional_services",
            PublicWarehousingAndStorage => "public_warehousing_and_storage",
            QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Railroads => "railroads",
            RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            RecordStores => "record_stores",
            RecreationalVehicleRentals => "recreational_vehicle_rentals",
            ReligiousGoodsStores => "religious_goods_stores",
            ReligiousOrganizations => "religious_organizations",
            RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            SecretarialSupportServices => "secretarial_support_services",
            SecurityBrokersDealers => "security_brokers_dealers",
            ServiceStations => "service_stations",
            SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            ShoeStores => "shoe_stores",
            SmallApplianceRepair => "small_appliance_repair",
            SnowmobileDealers => "snowmobile_dealers",
            SpecialTradeServices => "special_trade_services",
            SpecialtyCleaning => "specialty_cleaning",
            SportingGoodsStores => "sporting_goods_stores",
            SportingRecreationCamps => "sporting_recreation_camps",
            SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            SportsClubsFields => "sports_clubs_fields",
            StampAndCoinStores => "stamp_and_coin_stores",
            StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            SwimmingPoolsSales => "swimming_pools_sales",
            TUiTravelGermany => "t_ui_travel_germany",
            TailorsAlterations => "tailors_alterations",
            TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            TaxPreparationServices => "tax_preparation_services",
            TaxicabsLimousines => "taxicabs_limousines",
            TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            TelecommunicationServices => "telecommunication_services",
            TelegraphServices => "telegraph_services",
            TentAndAwningShops => "tent_and_awning_shops",
            TestingLaboratories => "testing_laboratories",
            TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Timeshares => "timeshares",
            TireRetreadingAndRepair => "tire_retreading_and_repair",
            TollsBridgeFees => "tolls_bridge_fees",
            TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            TowingServices => "towing_services",
            TrailerParksCampgrounds => "trailer_parks_campgrounds",
            TransportationServices => "transportation_services",
            TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            TruckStopIteration => "truck_stop_iteration",
            TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            TypewriterStores => "typewriter_stores",
            USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            UniformsCommercialClothing => "uniforms_commercial_clothing",
            UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Utilities => "utilities",
            VarietyStores => "variety_stores",
            VeterinaryServices => "veterinary_services",
            VideoAmusementGameSupplies => "video_amusement_game_supplies",
            VideoGameArcades => "video_game_arcades",
            VideoTapeRentalStores => "video_tape_rental_stores",
            VocationalTradeSchools => "vocational_trade_schools",
            WatchJewelryRepair => "watch_jewelry_repair",
            WeldingRepair => "welding_repair",
            WholesaleClubs => "wholesale_clubs",
            WigAndToupeeStores => "wig_and_toupee_stores",
            WiresMoneyOrders => "wires_money_orders",
            WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            WomensReadyToWearStores => "womens_ready_to_wear_stores",
            WreckingAndSalvageYards => "wrecking_and_salvage_yards",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationMerchantDataCategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationMerchantDataCategory::*;
        match s {
            "ac_refrigeration_repair" => Ok(AcRefrigerationRepair),
            "accounting_bookkeeping_services" => Ok(AccountingBookkeepingServices),
            "advertising_services" => Ok(AdvertisingServices),
            "agricultural_cooperative" => Ok(AgriculturalCooperative),
            "airlines_air_carriers" => Ok(AirlinesAirCarriers),
            "airports_flying_fields" => Ok(AirportsFlyingFields),
            "ambulance_services" => Ok(AmbulanceServices),
            "amusement_parks_carnivals" => Ok(AmusementParksCarnivals),
            "antique_reproductions" => Ok(AntiqueReproductions),
            "antique_shops" => Ok(AntiqueShops),
            "aquariums" => Ok(Aquariums),
            "architectural_surveying_services" => Ok(ArchitecturalSurveyingServices),
            "art_dealers_and_galleries" => Ok(ArtDealersAndGalleries),
            "artists_supply_and_craft_shops" => Ok(ArtistsSupplyAndCraftShops),
            "auto_and_home_supply_stores" => Ok(AutoAndHomeSupplyStores),
            "auto_body_repair_shops" => Ok(AutoBodyRepairShops),
            "auto_paint_shops" => Ok(AutoPaintShops),
            "auto_service_shops" => Ok(AutoServiceShops),
            "automated_cash_disburse" => Ok(AutomatedCashDisburse),
            "automated_fuel_dispensers" => Ok(AutomatedFuelDispensers),
            "automobile_associations" => Ok(AutomobileAssociations),
            "automotive_parts_and_accessories_stores" => Ok(AutomotivePartsAndAccessoriesStores),
            "automotive_tire_stores" => Ok(AutomotiveTireStores),
            "bail_and_bond_payments" => Ok(BailAndBondPayments),
            "bakeries" => Ok(Bakeries),
            "bands_orchestras" => Ok(BandsOrchestras),
            "barber_and_beauty_shops" => Ok(BarberAndBeautyShops),
            "betting_casino_gambling" => Ok(BettingCasinoGambling),
            "bicycle_shops" => Ok(BicycleShops),
            "billiard_pool_establishments" => Ok(BilliardPoolEstablishments),
            "boat_dealers" => Ok(BoatDealers),
            "boat_rentals_and_leases" => Ok(BoatRentalsAndLeases),
            "book_stores" => Ok(BookStores),
            "books_periodicals_and_newspapers" => Ok(BooksPeriodicalsAndNewspapers),
            "bowling_alleys" => Ok(BowlingAlleys),
            "bus_lines" => Ok(BusLines),
            "business_secretarial_schools" => Ok(BusinessSecretarialSchools),
            "buying_shopping_services" => Ok(BuyingShoppingServices),
            "cable_satellite_and_other_pay_television_and_radio" => {
                Ok(CableSatelliteAndOtherPayTelevisionAndRadio)
            }
            "camera_and_photographic_supply_stores" => Ok(CameraAndPhotographicSupplyStores),
            "candy_nut_and_confectionery_stores" => Ok(CandyNutAndConfectioneryStores),
            "car_and_truck_dealers_new_used" => Ok(CarAndTruckDealersNewUsed),
            "car_and_truck_dealers_used_only" => Ok(CarAndTruckDealersUsedOnly),
            "car_rental_agencies" => Ok(CarRentalAgencies),
            "car_washes" => Ok(CarWashes),
            "carpentry_services" => Ok(CarpentryServices),
            "carpet_upholstery_cleaning" => Ok(CarpetUpholsteryCleaning),
            "caterers" => Ok(Caterers),
            "charitable_and_social_service_organizations_fundraising" => {
                Ok(CharitableAndSocialServiceOrganizationsFundraising)
            }
            "chemicals_and_allied_products" => Ok(ChemicalsAndAlliedProducts),
            "child_care_services" => Ok(ChildCareServices),
            "childrens_and_infants_wear_stores" => Ok(ChildrensAndInfantsWearStores),
            "chiropodists_podiatrists" => Ok(ChiropodistsPodiatrists),
            "chiropractors" => Ok(Chiropractors),
            "cigar_stores_and_stands" => Ok(CigarStoresAndStands),
            "civic_social_fraternal_associations" => Ok(CivicSocialFraternalAssociations),
            "cleaning_and_maintenance" => Ok(CleaningAndMaintenance),
            "clothing_rental" => Ok(ClothingRental),
            "colleges_universities" => Ok(CollegesUniversities),
            "commercial_equipment" => Ok(CommercialEquipment),
            "commercial_footwear" => Ok(CommercialFootwear),
            "commercial_photography_art_and_graphics" => Ok(CommercialPhotographyArtAndGraphics),
            "commuter_transport_and_ferries" => Ok(CommuterTransportAndFerries),
            "computer_network_services" => Ok(ComputerNetworkServices),
            "computer_programming" => Ok(ComputerProgramming),
            "computer_repair" => Ok(ComputerRepair),
            "computer_software_stores" => Ok(ComputerSoftwareStores),
            "computers_peripherals_and_software" => Ok(ComputersPeripheralsAndSoftware),
            "concrete_work_services" => Ok(ConcreteWorkServices),
            "construction_materials" => Ok(ConstructionMaterials),
            "consulting_public_relations" => Ok(ConsultingPublicRelations),
            "correspondence_schools" => Ok(CorrespondenceSchools),
            "cosmetic_stores" => Ok(CosmeticStores),
            "counseling_services" => Ok(CounselingServices),
            "country_clubs" => Ok(CountryClubs),
            "courier_services" => Ok(CourierServices),
            "court_costs" => Ok(CourtCosts),
            "credit_reporting_agencies" => Ok(CreditReportingAgencies),
            "cruise_lines" => Ok(CruiseLines),
            "dairy_products_stores" => Ok(DairyProductsStores),
            "dance_hall_studios_schools" => Ok(DanceHallStudiosSchools),
            "dating_escort_services" => Ok(DatingEscortServices),
            "dentists_orthodontists" => Ok(DentistsOrthodontists),
            "department_stores" => Ok(DepartmentStores),
            "detective_agencies" => Ok(DetectiveAgencies),
            "digital_goods_applications" => Ok(DigitalGoodsApplications),
            "digital_goods_games" => Ok(DigitalGoodsGames),
            "digital_goods_large_volume" => Ok(DigitalGoodsLargeVolume),
            "digital_goods_media" => Ok(DigitalGoodsMedia),
            "direct_marketing_catalog_merchant" => Ok(DirectMarketingCatalogMerchant),
            "direct_marketing_combination_catalog_and_retail_merchant" => {
                Ok(DirectMarketingCombinationCatalogAndRetailMerchant)
            }
            "direct_marketing_inbound_telemarketing" => Ok(DirectMarketingInboundTelemarketing),
            "direct_marketing_insurance_services" => Ok(DirectMarketingInsuranceServices),
            "direct_marketing_other" => Ok(DirectMarketingOther),
            "direct_marketing_outbound_telemarketing" => Ok(DirectMarketingOutboundTelemarketing),
            "direct_marketing_subscription" => Ok(DirectMarketingSubscription),
            "direct_marketing_travel" => Ok(DirectMarketingTravel),
            "discount_stores" => Ok(DiscountStores),
            "doctors" => Ok(Doctors),
            "door_to_door_sales" => Ok(DoorToDoorSales),
            "drapery_window_covering_and_upholstery_stores" => {
                Ok(DraperyWindowCoveringAndUpholsteryStores)
            }
            "drinking_places" => Ok(DrinkingPlaces),
            "drug_stores_and_pharmacies" => Ok(DrugStoresAndPharmacies),
            "drugs_drug_proprietaries_and_druggist_sundries" => {
                Ok(DrugsDrugProprietariesAndDruggistSundries)
            }
            "dry_cleaners" => Ok(DryCleaners),
            "durable_goods" => Ok(DurableGoods),
            "duty_free_stores" => Ok(DutyFreeStores),
            "eating_places_restaurants" => Ok(EatingPlacesRestaurants),
            "educational_services" => Ok(EducationalServices),
            "electric_razor_stores" => Ok(ElectricRazorStores),
            "electric_vehicle_charging" => Ok(ElectricVehicleCharging),
            "electrical_parts_and_equipment" => Ok(ElectricalPartsAndEquipment),
            "electrical_services" => Ok(ElectricalServices),
            "electronics_repair_shops" => Ok(ElectronicsRepairShops),
            "electronics_stores" => Ok(ElectronicsStores),
            "elementary_secondary_schools" => Ok(ElementarySecondarySchools),
            "emergency_services_gcas_visa_use_only" => Ok(EmergencyServicesGcasVisaUseOnly),
            "employment_temp_agencies" => Ok(EmploymentTempAgencies),
            "equipment_rental" => Ok(EquipmentRental),
            "exterminating_services" => Ok(ExterminatingServices),
            "family_clothing_stores" => Ok(FamilyClothingStores),
            "fast_food_restaurants" => Ok(FastFoodRestaurants),
            "financial_institutions" => Ok(FinancialInstitutions),
            "fines_government_administrative_entities" => Ok(FinesGovernmentAdministrativeEntities),
            "fireplace_fireplace_screens_and_accessories_stores" => {
                Ok(FireplaceFireplaceScreensAndAccessoriesStores)
            }
            "floor_covering_stores" => Ok(FloorCoveringStores),
            "florists" => Ok(Florists),
            "florists_supplies_nursery_stock_and_flowers" => {
                Ok(FloristsSuppliesNurseryStockAndFlowers)
            }
            "freezer_and_locker_meat_provisioners" => Ok(FreezerAndLockerMeatProvisioners),
            "fuel_dealers_non_automotive" => Ok(FuelDealersNonAutomotive),
            "funeral_services_crematories" => Ok(FuneralServicesCrematories),
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => {
                Ok(FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances)
            }
            "furniture_repair_refinishing" => Ok(FurnitureRepairRefinishing),
            "furriers_and_fur_shops" => Ok(FurriersAndFurShops),
            "general_services" => Ok(GeneralServices),
            "gift_card_novelty_and_souvenir_shops" => Ok(GiftCardNoveltyAndSouvenirShops),
            "glass_paint_and_wallpaper_stores" => Ok(GlassPaintAndWallpaperStores),
            "glassware_crystal_stores" => Ok(GlasswareCrystalStores),
            "golf_courses_public" => Ok(GolfCoursesPublic),
            "government_licensed_horse_dog_racing_us_region_only" => {
                Ok(GovernmentLicensedHorseDogRacingUsRegionOnly)
            }
            "government_licensed_online_casions_online_gambling_us_region_only" => {
                Ok(GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly)
            }
            "government_owned_lotteries_non_us_region" => Ok(GovernmentOwnedLotteriesNonUsRegion),
            "government_owned_lotteries_us_region_only" => Ok(GovernmentOwnedLotteriesUsRegionOnly),
            "government_services" => Ok(GovernmentServices),
            "grocery_stores_supermarkets" => Ok(GroceryStoresSupermarkets),
            "hardware_equipment_and_supplies" => Ok(HardwareEquipmentAndSupplies),
            "hardware_stores" => Ok(HardwareStores),
            "health_and_beauty_spas" => Ok(HealthAndBeautySpas),
            "hearing_aids_sales_and_supplies" => Ok(HearingAidsSalesAndSupplies),
            "heating_plumbing_a_c" => Ok(HeatingPlumbingAC),
            "hobby_toy_and_game_shops" => Ok(HobbyToyAndGameShops),
            "home_supply_warehouse_stores" => Ok(HomeSupplyWarehouseStores),
            "hospitals" => Ok(Hospitals),
            "hotels_motels_and_resorts" => Ok(HotelsMotelsAndResorts),
            "household_appliance_stores" => Ok(HouseholdApplianceStores),
            "industrial_supplies" => Ok(IndustrialSupplies),
            "information_retrieval_services" => Ok(InformationRetrievalServices),
            "insurance_default" => Ok(InsuranceDefault),
            "insurance_underwriting_premiums" => Ok(InsuranceUnderwritingPremiums),
            "intra_company_purchases" => Ok(IntraCompanyPurchases),
            "jewelry_stores_watches_clocks_and_silverware_stores" => {
                Ok(JewelryStoresWatchesClocksAndSilverwareStores)
            }
            "landscaping_services" => Ok(LandscapingServices),
            "laundries" => Ok(Laundries),
            "laundry_cleaning_services" => Ok(LaundryCleaningServices),
            "legal_services_attorneys" => Ok(LegalServicesAttorneys),
            "luggage_and_leather_goods_stores" => Ok(LuggageAndLeatherGoodsStores),
            "lumber_building_materials_stores" => Ok(LumberBuildingMaterialsStores),
            "manual_cash_disburse" => Ok(ManualCashDisburse),
            "marinas_service_and_supplies" => Ok(MarinasServiceAndSupplies),
            "marketplaces" => Ok(Marketplaces),
            "masonry_stonework_and_plaster" => Ok(MasonryStoneworkAndPlaster),
            "massage_parlors" => Ok(MassageParlors),
            "medical_and_dental_labs" => Ok(MedicalAndDentalLabs),
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => {
                Ok(MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies)
            }
            "medical_services" => Ok(MedicalServices),
            "membership_organizations" => Ok(MembershipOrganizations),
            "mens_and_boys_clothing_and_accessories_stores" => {
                Ok(MensAndBoysClothingAndAccessoriesStores)
            }
            "mens_womens_clothing_stores" => Ok(MensWomensClothingStores),
            "metal_service_centers" => Ok(MetalServiceCenters),
            "miscellaneous_apparel_and_accessory_shops" => {
                Ok(MiscellaneousApparelAndAccessoryShops)
            }
            "miscellaneous_auto_dealers" => Ok(MiscellaneousAutoDealers),
            "miscellaneous_business_services" => Ok(MiscellaneousBusinessServices),
            "miscellaneous_food_stores" => Ok(MiscellaneousFoodStores),
            "miscellaneous_general_merchandise" => Ok(MiscellaneousGeneralMerchandise),
            "miscellaneous_general_services" => Ok(MiscellaneousGeneralServices),
            "miscellaneous_home_furnishing_specialty_stores" => {
                Ok(MiscellaneousHomeFurnishingSpecialtyStores)
            }
            "miscellaneous_publishing_and_printing" => Ok(MiscellaneousPublishingAndPrinting),
            "miscellaneous_recreation_services" => Ok(MiscellaneousRecreationServices),
            "miscellaneous_repair_shops" => Ok(MiscellaneousRepairShops),
            "miscellaneous_specialty_retail" => Ok(MiscellaneousSpecialtyRetail),
            "mobile_home_dealers" => Ok(MobileHomeDealers),
            "motion_picture_theaters" => Ok(MotionPictureTheaters),
            "motor_freight_carriers_and_trucking" => Ok(MotorFreightCarriersAndTrucking),
            "motor_homes_dealers" => Ok(MotorHomesDealers),
            "motor_vehicle_supplies_and_new_parts" => Ok(MotorVehicleSuppliesAndNewParts),
            "motorcycle_shops_and_dealers" => Ok(MotorcycleShopsAndDealers),
            "motorcycle_shops_dealers" => Ok(MotorcycleShopsDealers),
            "music_stores_musical_instruments_pianos_and_sheet_music" => {
                Ok(MusicStoresMusicalInstrumentsPianosAndSheetMusic)
            }
            "news_dealers_and_newsstands" => Ok(NewsDealersAndNewsstands),
            "non_fi_money_orders" => Ok(NonFiMoneyOrders),
            "non_fi_stored_value_card_purchase_load" => Ok(NonFiStoredValueCardPurchaseLoad),
            "nondurable_goods" => Ok(NondurableGoods),
            "nurseries_lawn_and_garden_supply_stores" => Ok(NurseriesLawnAndGardenSupplyStores),
            "nursing_personal_care" => Ok(NursingPersonalCare),
            "office_and_commercial_furniture" => Ok(OfficeAndCommercialFurniture),
            "opticians_eyeglasses" => Ok(OpticiansEyeglasses),
            "optometrists_ophthalmologist" => Ok(OptometristsOphthalmologist),
            "orthopedic_goods_prosthetic_devices" => Ok(OrthopedicGoodsProstheticDevices),
            "osteopaths" => Ok(Osteopaths),
            "package_stores_beer_wine_and_liquor" => Ok(PackageStoresBeerWineAndLiquor),
            "paints_varnishes_and_supplies" => Ok(PaintsVarnishesAndSupplies),
            "parking_lots_garages" => Ok(ParkingLotsGarages),
            "passenger_railways" => Ok(PassengerRailways),
            "pawn_shops" => Ok(PawnShops),
            "pet_shops_pet_food_and_supplies" => Ok(PetShopsPetFoodAndSupplies),
            "petroleum_and_petroleum_products" => Ok(PetroleumAndPetroleumProducts),
            "photo_developing" => Ok(PhotoDeveloping),
            "photographic_photocopy_microfilm_equipment_and_supplies" => {
                Ok(PhotographicPhotocopyMicrofilmEquipmentAndSupplies)
            }
            "photographic_studios" => Ok(PhotographicStudios),
            "picture_video_production" => Ok(PictureVideoProduction),
            "piece_goods_notions_and_other_dry_goods" => Ok(PieceGoodsNotionsAndOtherDryGoods),
            "plumbing_heating_equipment_and_supplies" => Ok(PlumbingHeatingEquipmentAndSupplies),
            "political_organizations" => Ok(PoliticalOrganizations),
            "postal_services_government_only" => Ok(PostalServicesGovernmentOnly),
            "precious_stones_and_metals_watches_and_jewelry" => {
                Ok(PreciousStonesAndMetalsWatchesAndJewelry)
            }
            "professional_services" => Ok(ProfessionalServices),
            "public_warehousing_and_storage" => Ok(PublicWarehousingAndStorage),
            "quick_copy_repro_and_blueprint" => Ok(QuickCopyReproAndBlueprint),
            "railroads" => Ok(Railroads),
            "real_estate_agents_and_managers_rentals" => Ok(RealEstateAgentsAndManagersRentals),
            "record_stores" => Ok(RecordStores),
            "recreational_vehicle_rentals" => Ok(RecreationalVehicleRentals),
            "religious_goods_stores" => Ok(ReligiousGoodsStores),
            "religious_organizations" => Ok(ReligiousOrganizations),
            "roofing_siding_sheet_metal" => Ok(RoofingSidingSheetMetal),
            "secretarial_support_services" => Ok(SecretarialSupportServices),
            "security_brokers_dealers" => Ok(SecurityBrokersDealers),
            "service_stations" => Ok(ServiceStations),
            "sewing_needlework_fabric_and_piece_goods_stores" => {
                Ok(SewingNeedleworkFabricAndPieceGoodsStores)
            }
            "shoe_repair_hat_cleaning" => Ok(ShoeRepairHatCleaning),
            "shoe_stores" => Ok(ShoeStores),
            "small_appliance_repair" => Ok(SmallApplianceRepair),
            "snowmobile_dealers" => Ok(SnowmobileDealers),
            "special_trade_services" => Ok(SpecialTradeServices),
            "specialty_cleaning" => Ok(SpecialtyCleaning),
            "sporting_goods_stores" => Ok(SportingGoodsStores),
            "sporting_recreation_camps" => Ok(SportingRecreationCamps),
            "sports_and_riding_apparel_stores" => Ok(SportsAndRidingApparelStores),
            "sports_clubs_fields" => Ok(SportsClubsFields),
            "stamp_and_coin_stores" => Ok(StampAndCoinStores),
            "stationary_office_supplies_printing_and_writing_paper" => {
                Ok(StationaryOfficeSuppliesPrintingAndWritingPaper)
            }
            "stationery_stores_office_and_school_supply_stores" => {
                Ok(StationeryStoresOfficeAndSchoolSupplyStores)
            }
            "swimming_pools_sales" => Ok(SwimmingPoolsSales),
            "t_ui_travel_germany" => Ok(TUiTravelGermany),
            "tailors_alterations" => Ok(TailorsAlterations),
            "tax_payments_government_agencies" => Ok(TaxPaymentsGovernmentAgencies),
            "tax_preparation_services" => Ok(TaxPreparationServices),
            "taxicabs_limousines" => Ok(TaxicabsLimousines),
            "telecommunication_equipment_and_telephone_sales" => {
                Ok(TelecommunicationEquipmentAndTelephoneSales)
            }
            "telecommunication_services" => Ok(TelecommunicationServices),
            "telegraph_services" => Ok(TelegraphServices),
            "tent_and_awning_shops" => Ok(TentAndAwningShops),
            "testing_laboratories" => Ok(TestingLaboratories),
            "theatrical_ticket_agencies" => Ok(TheatricalTicketAgencies),
            "timeshares" => Ok(Timeshares),
            "tire_retreading_and_repair" => Ok(TireRetreadingAndRepair),
            "tolls_bridge_fees" => Ok(TollsBridgeFees),
            "tourist_attractions_and_exhibits" => Ok(TouristAttractionsAndExhibits),
            "towing_services" => Ok(TowingServices),
            "trailer_parks_campgrounds" => Ok(TrailerParksCampgrounds),
            "transportation_services" => Ok(TransportationServices),
            "travel_agencies_tour_operators" => Ok(TravelAgenciesTourOperators),
            "truck_stop_iteration" => Ok(TruckStopIteration),
            "truck_utility_trailer_rentals" => Ok(TruckUtilityTrailerRentals),
            "typesetting_plate_making_and_related_services" => {
                Ok(TypesettingPlateMakingAndRelatedServices)
            }
            "typewriter_stores" => Ok(TypewriterStores),
            "u_s_federal_government_agencies_or_departments" => {
                Ok(USFederalGovernmentAgenciesOrDepartments)
            }
            "uniforms_commercial_clothing" => Ok(UniformsCommercialClothing),
            "used_merchandise_and_secondhand_stores" => Ok(UsedMerchandiseAndSecondhandStores),
            "utilities" => Ok(Utilities),
            "variety_stores" => Ok(VarietyStores),
            "veterinary_services" => Ok(VeterinaryServices),
            "video_amusement_game_supplies" => Ok(VideoAmusementGameSupplies),
            "video_game_arcades" => Ok(VideoGameArcades),
            "video_tape_rental_stores" => Ok(VideoTapeRentalStores),
            "vocational_trade_schools" => Ok(VocationalTradeSchools),
            "watch_jewelry_repair" => Ok(WatchJewelryRepair),
            "welding_repair" => Ok(WeldingRepair),
            "wholesale_clubs" => Ok(WholesaleClubs),
            "wig_and_toupee_stores" => Ok(WigAndToupeeStores),
            "wires_money_orders" => Ok(WiresMoneyOrders),
            "womens_accessory_and_specialty_shops" => Ok(WomensAccessoryAndSpecialtyShops),
            "womens_ready_to_wear_stores" => Ok(WomensReadyToWearStores),
            "wrecking_and_salvage_yards" => Ok(WreckingAndSalvageYards),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationMerchantDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationMerchantDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationMerchantDataCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Details about the authorization, such as identifiers, set by the card network.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingAuthorizationNetworkData<'a> {
    /// Identifier assigned to the acquirer by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquiring_institution_id: Option<&'a str>,
}
impl<'a> CreateIssuingAuthorizationNetworkData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Verifications that Stripe performed on information that the cardholder provided to the merchant.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateIssuingAuthorizationVerificationData {
    /// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<CreateIssuingAuthorizationVerificationDataAddressLine1Check>,
    /// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_postal_code_check:
        Option<CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck>,
    /// The exemption applied to this authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_exemption:
        Option<CreateIssuingAuthorizationVerificationDataAuthenticationExemption>,
    /// Whether the cardholder provided a CVC and if it matched Stripe’s record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<CreateIssuingAuthorizationVerificationDataCvcCheck>,
    /// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_check: Option<CreateIssuingAuthorizationVerificationDataExpiryCheck>,
    /// 3D Secure details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<CreateIssuingAuthorizationVerificationDataThreeDSecure>,
}
impl CreateIssuingAuthorizationVerificationData {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationVerificationDataAddressLine1Check {
    Match,
    Mismatch,
    NotProvided,
}
impl CreateIssuingAuthorizationVerificationDataAddressLine1Check {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationVerificationDataAddressLine1Check::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationVerificationDataAddressLine1Check {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationVerificationDataAddressLine1Check::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationVerificationDataAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataAddressLine1Check {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationVerificationDataAddressLine1Check {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationVerificationDataAddressPostalCodeCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The exemption applied to this authorization.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateIssuingAuthorizationVerificationDataAuthenticationExemption {
    /// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
    pub claimed_by: CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy,
    /// The specific exemption claimed for this authorization.
    #[serde(rename = "type")]
    pub type_: CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType,
}
impl CreateIssuingAuthorizationVerificationDataAuthenticationExemption {
    pub fn new(
        claimed_by: CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy,
        type_: CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType,
    ) -> Self {
        Self { claimed_by, type_ }
    }
}
/// The entity that requested the exemption, either the acquiring merchant or the Issuing user.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
}
impl CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy::*;
        match self {
            Acquirer => "acquirer",
            Issuer => "issuer",
        }
    }
}

impl std::str::FromStr
    for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy::*;
        match s {
            "acquirer" => Ok(Acquirer),
            "issuer" => Ok(Issuer),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display
    for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionClaimedBy
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The specific exemption claimed for this authorization.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType {
    LowValueTransaction,
    TransactionRiskAnalysis,
    Unknown,
}
impl CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType::*;
        match self {
            LowValueTransaction => "low_value_transaction",
            TransactionRiskAnalysis => "transaction_risk_analysis",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType::*;
        match s {
            "low_value_transaction" => Ok(LowValueTransaction),
            "transaction_risk_analysis" => Ok(TransactionRiskAnalysis),
            "unknown" => Ok(Unknown),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationVerificationDataAuthenticationExemptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Whether the cardholder provided a CVC and if it matched Stripe’s record.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationVerificationDataCvcCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl CreateIssuingAuthorizationVerificationDataCvcCheck {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationVerificationDataCvcCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationVerificationDataCvcCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationVerificationDataCvcCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationVerificationDataCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataCvcCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationVerificationDataCvcCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Whether the cardholder provided an expiry date and if it matched Stripe’s record.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationVerificationDataExpiryCheck {
    Match,
    Mismatch,
    NotProvided,
}
impl CreateIssuingAuthorizationVerificationDataExpiryCheck {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationVerificationDataExpiryCheck::*;
        match self {
            Match => "match",
            Mismatch => "mismatch",
            NotProvided => "not_provided",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationVerificationDataExpiryCheck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationVerificationDataExpiryCheck::*;
        match s {
            "match" => Ok(Match),
            "mismatch" => Ok(Mismatch),
            "not_provided" => Ok(NotProvided),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationVerificationDataExpiryCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataExpiryCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationVerificationDataExpiryCheck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// 3D Secure details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateIssuingAuthorizationVerificationDataThreeDSecure {
    /// The outcome of the 3D Secure authentication request.
    pub result: CreateIssuingAuthorizationVerificationDataThreeDSecureResult,
}
impl CreateIssuingAuthorizationVerificationDataThreeDSecure {
    pub fn new(result: CreateIssuingAuthorizationVerificationDataThreeDSecureResult) -> Self {
        Self { result }
    }
}
/// The outcome of the 3D Secure authentication request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationVerificationDataThreeDSecureResult {
    AttemptAcknowledged,
    Authenticated,
    Failed,
    Required,
}
impl CreateIssuingAuthorizationVerificationDataThreeDSecureResult {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationVerificationDataThreeDSecureResult::*;
        match self {
            AttemptAcknowledged => "attempt_acknowledged",
            Authenticated => "authenticated",
            Failed => "failed",
            Required => "required",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationVerificationDataThreeDSecureResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationVerificationDataThreeDSecureResult::*;
        match s {
            "attempt_acknowledged" => Ok(AttemptAcknowledged),
            "authenticated" => Ok(Authenticated),
            "failed" => Ok(Failed),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationVerificationDataThreeDSecureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationVerificationDataThreeDSecureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationVerificationDataThreeDSecureResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The digital wallet used for this transaction.
/// One of `apple_pay`, `google_pay`, or `samsung_pay`.
/// Will populate as `null` when no digital wallet was utilized.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateIssuingAuthorizationWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}
impl CreateIssuingAuthorizationWallet {
    pub fn as_str(self) -> &'static str {
        use CreateIssuingAuthorizationWallet::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
        }
    }
}

impl std::str::FromStr for CreateIssuingAuthorizationWallet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateIssuingAuthorizationWallet::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreateIssuingAuthorizationWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateIssuingAuthorizationWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateIssuingAuthorizationWallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> CreateIssuingAuthorization<'a> {
    /// Create a test-mode authorization.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form("/test_helpers/issuing/authorizations", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureIssuingAuthorization<'a> {
    /// The amount to capture from the authorization.
    /// If not provided, the full amount of the authorization will be captured.
    /// This amount is in the authorization currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_amount: Option<i64>,
    /// Whether to close the authorization after capture.
    /// Defaults to true.
    /// Set to false to enable multi-capture flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_authorization: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Additional purchase information that is optionally provided by the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<CaptureIssuingAuthorizationPurchaseDetails<'a>>,
}
impl<'a> CaptureIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Additional purchase information that is optionally provided by the merchant.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetails<'a> {
    /// Information about the flight that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<CaptureIssuingAuthorizationPurchaseDetailsFlight<'a>>,
    /// Information about fuel that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<CaptureIssuingAuthorizationPurchaseDetailsFuel<'a>>,
    /// Information about lodging that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lodging: Option<CaptureIssuingAuthorizationPurchaseDetailsLodging>,
    /// The line items in the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<&'a [CaptureIssuingAuthorizationPurchaseDetailsReceipt<'a>]>,
    /// A merchant-specific order number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<&'a str>,
}
impl<'a> CaptureIssuingAuthorizationPurchaseDetails<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about the flight that was purchased with this transaction.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsFlight<'a> {
    /// The time that the flight departed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<stripe_types::Timestamp>,
    /// The name of the passenger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<&'a str>,
    /// Whether the ticket is refundable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable: Option<bool>,
    /// The legs of the trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<&'a [CaptureIssuingAuthorizationPurchaseDetailsFlightSegments<'a>]>,
    /// The travel agency that issued the ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_agency: Option<&'a str>,
}
impl<'a> CaptureIssuingAuthorizationPurchaseDetailsFlight<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The legs of the trip.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsFlightSegments<'a> {
    /// The three-letter IATA airport code of the flight's destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_airport_code: Option<&'a str>,
    /// The airline carrier code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<&'a str>,
    /// The three-letter IATA airport code that the flight departed from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<&'a str>,
    /// The flight number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<&'a str>,
    /// The flight's service class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_class: Option<&'a str>,
    /// Whether a stopover is allowed on this flight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopover_allowed: Option<bool>,
}
impl<'a> CaptureIssuingAuthorizationPurchaseDetailsFlightSegments<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Information about fuel that was purchased with this transaction.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsFuel<'a> {
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CaptureIssuingAuthorizationPurchaseDetailsFuelType>,
    /// The units for `volume_decimal`. One of `us_gallon` or `liter`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<CaptureIssuingAuthorizationPurchaseDetailsFuelUnit>,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost_decimal: Option<&'a str>,
    /// The volume of the fuel that was pumped, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_decimal: Option<&'a str>,
}
impl<'a> CaptureIssuingAuthorizationPurchaseDetailsFuel<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CaptureIssuingAuthorizationPurchaseDetailsFuelType {
    Diesel,
    Other,
    UnleadedPlus,
    UnleadedRegular,
    UnleadedSuper,
}
impl CaptureIssuingAuthorizationPurchaseDetailsFuelType {
    pub fn as_str(self) -> &'static str {
        use CaptureIssuingAuthorizationPurchaseDetailsFuelType::*;
        match self {
            Diesel => "diesel",
            Other => "other",
            UnleadedPlus => "unleaded_plus",
            UnleadedRegular => "unleaded_regular",
            UnleadedSuper => "unleaded_super",
        }
    }
}

impl std::str::FromStr for CaptureIssuingAuthorizationPurchaseDetailsFuelType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CaptureIssuingAuthorizationPurchaseDetailsFuelType::*;
        match s {
            "diesel" => Ok(Diesel),
            "other" => Ok(Other),
            "unleaded_plus" => Ok(UnleadedPlus),
            "unleaded_regular" => Ok(UnleadedRegular),
            "unleaded_super" => Ok(UnleadedSuper),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CaptureIssuingAuthorizationPurchaseDetailsFuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetailsFuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CaptureIssuingAuthorizationPurchaseDetailsFuelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The units for `volume_decimal`. One of `us_gallon` or `liter`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CaptureIssuingAuthorizationPurchaseDetailsFuelUnit {
    Liter,
    UsGallon,
}
impl CaptureIssuingAuthorizationPurchaseDetailsFuelUnit {
    pub fn as_str(self) -> &'static str {
        use CaptureIssuingAuthorizationPurchaseDetailsFuelUnit::*;
        match self {
            Liter => "liter",
            UsGallon => "us_gallon",
        }
    }
}

impl std::str::FromStr for CaptureIssuingAuthorizationPurchaseDetailsFuelUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CaptureIssuingAuthorizationPurchaseDetailsFuelUnit::*;
        match s {
            "liter" => Ok(Liter),
            "us_gallon" => Ok(UsGallon),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CaptureIssuingAuthorizationPurchaseDetailsFuelUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CaptureIssuingAuthorizationPurchaseDetailsFuelUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CaptureIssuingAuthorizationPurchaseDetailsFuelUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Information about lodging that was purchased with this transaction.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsLodging {
    /// The time of checking into the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_at: Option<stripe_types::Timestamp>,
    /// The number of nights stayed at the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nights: Option<i64>,
}
impl CaptureIssuingAuthorizationPurchaseDetailsLodging {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The line items in the purchase.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CaptureIssuingAuthorizationPurchaseDetailsReceipt<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<i64>,
}
impl<'a> CaptureIssuingAuthorizationPurchaseDetailsReceipt<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> CaptureIssuingAuthorization<'a> {
    /// Capture a test-mode authorization.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &str,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form(
            &format!("/test_helpers/issuing/authorizations/{authorization}/capture"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ExpireIssuingAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ExpireIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ExpireIssuingAuthorization<'a> {
    /// Expire a test-mode Authorization.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &str,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form(
            &format!("/test_helpers/issuing/authorizations/{authorization}/expire"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct IncrementIssuingAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The amount to increment the authorization by.
    /// This amount is in the authorization currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub increment_amount: i64,
    /// If set `true`, you may provide [amount](https://stripe.com/docs/api/issuing/authorizations/approve#approve_issuing_authorization-amount) to control how much to hold for the authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_amount_controllable: Option<bool>,
}
impl<'a> IncrementIssuingAuthorization<'a> {
    pub fn new(increment_amount: i64) -> Self {
        Self { expand: None, increment_amount, is_amount_controllable: None }
    }
}
impl<'a> IncrementIssuingAuthorization<'a> {
    /// Increment a test-mode Authorization.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &str,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form(
            &format!("/test_helpers/issuing/authorizations/{authorization}/increment"),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ReverseIssuingAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The amount to reverse from the authorization.
    /// If not provided, the full amount of the authorization will be reversed.
    /// This amount is in the authorization currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_amount: Option<i64>,
}
impl<'a> ReverseIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ReverseIssuingAuthorization<'a> {
    /// Reverse a test-mode Authorization.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &str,
    ) -> stripe::Response<stripe_shared::IssuingAuthorization> {
        client.send_form(
            &format!("/test_helpers/issuing/authorizations/{authorization}/reverse"),
            self,
            http_types::Method::Post,
        )
    }
}
