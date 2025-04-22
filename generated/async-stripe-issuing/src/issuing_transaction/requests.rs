use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cardholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::IssuingTransactionType>,
}
impl ListIssuingTransactionBuilder {
    fn new() -> Self {
        Self {
            card: None,
            cardholder: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            type_: None,
        }
    }
}
/// Returns a list of Issuing `Transaction` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIssuingTransaction {
    inner: ListIssuingTransactionBuilder,
}
impl ListIssuingTransaction {
    /// Construct a new `ListIssuingTransaction`.
    pub fn new() -> Self {
        Self { inner: ListIssuingTransactionBuilder::new() }
    }
    /// Only return transactions that belong to the given card.
    pub fn card(mut self, card: impl Into<String>) -> Self {
        self.inner.card = Some(card.into());
        self
    }
    /// Only return transactions that belong to the given cardholder.
    pub fn cardholder(mut self, cardholder: impl Into<String>) -> Self {
        self.inner.cardholder = Some(cardholder.into());
        self
    }
    /// Only return transactions that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
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
    /// Only return transactions that have the given type. One of `capture` or `refund`.
    pub fn type_(mut self, type_: impl Into<stripe_shared::IssuingTransactionType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl Default for ListIssuingTransaction {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingTransaction {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingTransaction>>
    {
        stripe_client_core::ListPaginator::new_list("/issuing/transactions", &self.inner)
    }
}

impl StripeRequest for ListIssuingTransaction {
    type Output = stripe_types::List<stripe_shared::IssuingTransaction>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/transactions").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveIssuingTransactionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Transaction` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIssuingTransaction {
    inner: RetrieveIssuingTransactionBuilder,
    transaction: stripe_shared::IssuingTransactionId,
}
impl RetrieveIssuingTransaction {
    /// Construct a new `RetrieveIssuingTransaction`.
    pub fn new(transaction: impl Into<stripe_shared::IssuingTransactionId>) -> Self {
        Self { transaction: transaction.into(), inner: RetrieveIssuingTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingTransaction {
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

impl StripeRequest for RetrieveIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/transactions/{transaction}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}
impl UpdateIssuingTransactionBuilder {
    fn new() -> Self {
        Self { expand: None, metadata: None }
    }
}
/// Updates the specified Issuing `Transaction` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateIssuingTransaction {
    inner: UpdateIssuingTransactionBuilder,
    transaction: stripe_shared::IssuingTransactionId,
}
impl UpdateIssuingTransaction {
    /// Construct a new `UpdateIssuingTransaction`.
    pub fn new(transaction: impl Into<stripe_shared::IssuingTransactionId>) -> Self {
        Self { transaction: transaction.into(), inner: UpdateIssuingTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
}
impl UpdateIssuingTransaction {
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

impl StripeRequest for UpdateIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/transactions/{transaction}"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RefundIssuingTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_amount: Option<i64>,
}
impl RefundIssuingTransactionBuilder {
    fn new() -> Self {
        Self { expand: None, refund_amount: None }
    }
}
/// Refund a test-mode Transaction.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RefundIssuingTransaction {
    inner: RefundIssuingTransactionBuilder,
    transaction: String,
}
impl RefundIssuingTransaction {
    /// Construct a new `RefundIssuingTransaction`.
    pub fn new(transaction: impl Into<String>) -> Self {
        Self { transaction: transaction.into(), inner: RefundIssuingTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The total amount to attempt to refund.
    /// This amount is in the provided currency, or defaults to the cards currency, and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub fn refund_amount(mut self, refund_amount: impl Into<i64>) -> Self {
        self.inner.refund_amount = Some(refund_amount.into());
        self
    }
}
impl RefundIssuingTransaction {
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

impl StripeRequest for RefundIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        let transaction = &self.transaction;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/test_helpers/issuing/transactions/{transaction}/refund"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateForceCaptureIssuingTransactionBuilder {
    amount: i64,
    card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_data: Option<CreateForceCaptureIssuingTransactionMerchantData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purchase_details: Option<CreateForceCaptureIssuingTransactionPurchaseDetails>,
}
impl CreateForceCaptureIssuingTransactionBuilder {
    fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self {
            amount: amount.into(),
            card: card.into(),
            currency: None,
            expand: None,
            merchant_data: None,
            purchase_details: None,
        }
    }
}
/// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateForceCaptureIssuingTransactionMerchantData {
    /// A categorization of the seller's type of business.
    /// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CreateForceCaptureIssuingTransactionMerchantDataCategory>,
    /// City where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Name of the seller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier assigned to the seller by the card network.
    /// Different card networks may assign different network_id fields to the same merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// Postal code where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// An ID assigned by the seller to the location of the sale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    /// URL provided by the merchant on a 3DS request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CreateForceCaptureIssuingTransactionMerchantData {
    pub fn new() -> Self {
        Self {
            category: None,
            city: None,
            country: None,
            name: None,
            network_id: None,
            postal_code: None,
            state: None,
            terminal_id: None,
            url: None,
        }
    }
}
impl Default for CreateForceCaptureIssuingTransactionMerchantData {
    fn default() -> Self {
        Self::new()
    }
}
/// A categorization of the seller's type of business.
/// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateForceCaptureIssuingTransactionMerchantDataCategory {
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
    Unknown(String),
}
impl CreateForceCaptureIssuingTransactionMerchantDataCategory {
    pub fn as_str(&self) -> &str {
        use CreateForceCaptureIssuingTransactionMerchantDataCategory::*;
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateForceCaptureIssuingTransactionMerchantDataCategory {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateForceCaptureIssuingTransactionMerchantDataCategory::*;
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
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateForceCaptureIssuingTransactionMerchantDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateForceCaptureIssuingTransactionMerchantDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateForceCaptureIssuingTransactionMerchantDataCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateForceCaptureIssuingTransactionMerchantDataCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Additional purchase information that is optionally provided by the merchant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateForceCaptureIssuingTransactionPurchaseDetails {
    /// Fleet-specific information for transactions using Fleet cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<CreateForceCaptureIssuingTransactionPurchaseDetailsFleet>,
    /// Information about the flight that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<FlightSpecs>,
    /// Information about fuel that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<CreateForceCaptureIssuingTransactionPurchaseDetailsFuel>,
    /// Information about lodging that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lodging: Option<LodgingSpecs>,
    /// The line items in the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<ReceiptSpecs>>,
    /// A merchant-specific order number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
impl CreateForceCaptureIssuingTransactionPurchaseDetails {
    pub fn new() -> Self {
        Self {
            fleet: None,
            flight: None,
            fuel: None,
            lodging: None,
            receipt: None,
            reference: None,
        }
    }
}
impl Default for CreateForceCaptureIssuingTransactionPurchaseDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Fleet-specific information for transactions using Fleet cards.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateForceCaptureIssuingTransactionPurchaseDetailsFleet {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<FleetCardholderPromptDataSpecs>,
    /// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType>,
    /// More information about the total amount.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<FleetReportedBreakdownSpecs>,
    /// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType>,
}
impl CreateForceCaptureIssuingTransactionPurchaseDetailsFleet {
    pub fn new() -> Self {
        Self {
            cardholder_prompt_data: None,
            purchase_type: None,
            reported_breakdown: None,
            service_type: None,
        }
    }
}
impl Default for CreateForceCaptureIssuingTransactionPurchaseDetailsFleet {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType {
    FuelAndNonFuelPurchase,
    FuelPurchase,
    NonFuelPurchase,
}
impl CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType {
    pub fn as_str(self) -> &'static str {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType::*;
        match self {
            FuelAndNonFuelPurchase => "fuel_and_non_fuel_purchase",
            FuelPurchase => "fuel_purchase",
            NonFuelPurchase => "non_fuel_purchase",
        }
    }
}

impl std::str::FromStr for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType::*;
        match s {
            "fuel_and_non_fuel_purchase" => Ok(FuelAndNonFuelPurchase),
            "fuel_purchase" => Ok(FuelPurchase),
            "non_fuel_purchase" => Ok(NonFuelPurchase),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetPurchaseType"))
    }
}
/// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType {
    FullService,
    NonFuelTransaction,
    SelfService,
}
impl CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType {
    pub fn as_str(self) -> &'static str {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType::*;
        match self {
            FullService => "full_service",
            NonFuelTransaction => "non_fuel_transaction",
            SelfService => "self_service",
        }
    }
}

impl std::str::FromStr for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType::*;
        match s {
            "full_service" => Ok(FullService),
            "non_fuel_transaction" => Ok(NonFuelTransaction),
            "self_service" => Ok(SelfService),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateForceCaptureIssuingTransactionPurchaseDetailsFleetServiceType"))
    }
}
/// Information about fuel that was purchased with this transaction.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateForceCaptureIssuingTransactionPurchaseDetailsFuel {
    /// [Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying the primary fuel product purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_product_code: Option<String>,
    /// The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_decimal: Option<String>,
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType>,
    /// The units for `quantity_decimal`.
    /// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit>,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost_decimal: Option<String>,
}
impl CreateForceCaptureIssuingTransactionPurchaseDetailsFuel {
    pub fn new() -> Self {
        Self {
            industry_product_code: None,
            quantity_decimal: None,
            type_: None,
            unit: None,
            unit_cost_decimal: None,
        }
    }
}
impl Default for CreateForceCaptureIssuingTransactionPurchaseDetailsFuel {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType {
    Diesel,
    Other,
    UnleadedPlus,
    UnleadedRegular,
    UnleadedSuper,
}
impl CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType {
    pub fn as_str(self) -> &'static str {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType::*;
        match self {
            Diesel => "diesel",
            Other => "other",
            UnleadedPlus => "unleaded_plus",
            UnleadedRegular => "unleaded_regular",
            UnleadedSuper => "unleaded_super",
        }
    }
}

impl std::str::FromStr for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType::*;
        match s {
            "diesel" => Ok(Diesel),
            "other" => Ok(Other),
            "unleaded_plus" => Ok(UnleadedPlus),
            "unleaded_regular" => Ok(UnleadedRegular),
            "unleaded_super" => Ok(UnleadedSuper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelType",
            )
        })
    }
}
/// The units for `quantity_decimal`.
/// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit {
    ChargingMinute,
    ImperialGallon,
    Kilogram,
    KilowattHour,
    Liter,
    Other,
    Pound,
    UsGallon,
}
impl CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit {
    pub fn as_str(self) -> &'static str {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit::*;
        match self {
            ChargingMinute => "charging_minute",
            ImperialGallon => "imperial_gallon",
            Kilogram => "kilogram",
            KilowattHour => "kilowatt_hour",
            Liter => "liter",
            Other => "other",
            Pound => "pound",
            UsGallon => "us_gallon",
        }
    }
}

impl std::str::FromStr for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit::*;
        match s {
            "charging_minute" => Ok(ChargingMinute),
            "imperial_gallon" => Ok(ImperialGallon),
            "kilogram" => Ok(Kilogram),
            "kilowatt_hour" => Ok(KilowattHour),
            "liter" => Ok(Liter),
            "other" => Ok(Other),
            "pound" => Ok(Pound),
            "us_gallon" => Ok(UsGallon),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateForceCaptureIssuingTransactionPurchaseDetailsFuelUnit",
            )
        })
    }
}
/// Allows the user to capture an arbitrary amount, also known as a forced capture.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateForceCaptureIssuingTransaction {
    inner: CreateForceCaptureIssuingTransactionBuilder,
}
impl CreateForceCaptureIssuingTransaction {
    /// Construct a new `CreateForceCaptureIssuingTransaction`.
    pub fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self { inner: CreateForceCaptureIssuingTransactionBuilder::new(amount.into(), card.into()) }
    }
    /// The currency of the capture.
    /// If not provided, defaults to the currency of the card.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
    pub fn merchant_data(
        mut self,
        merchant_data: impl Into<CreateForceCaptureIssuingTransactionMerchantData>,
    ) -> Self {
        self.inner.merchant_data = Some(merchant_data.into());
        self
    }
    /// Additional purchase information that is optionally provided by the merchant.
    pub fn purchase_details(
        mut self,
        purchase_details: impl Into<CreateForceCaptureIssuingTransactionPurchaseDetails>,
    ) -> Self {
        self.inner.purchase_details = Some(purchase_details.into());
        self
    }
}
impl CreateForceCaptureIssuingTransaction {
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

impl StripeRequest for CreateForceCaptureIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(
            StripeMethod::Post,
            "/test_helpers/issuing/transactions/create_force_capture",
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateUnlinkedRefundIssuingTransactionBuilder {
    amount: i64,
    card: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_data: Option<CreateUnlinkedRefundIssuingTransactionMerchantData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purchase_details: Option<CreateUnlinkedRefundIssuingTransactionPurchaseDetails>,
}
impl CreateUnlinkedRefundIssuingTransactionBuilder {
    fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self {
            amount: amount.into(),
            card: card.into(),
            currency: None,
            expand: None,
            merchant_data: None,
            purchase_details: None,
        }
    }
}
/// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateUnlinkedRefundIssuingTransactionMerchantData {
    /// A categorization of the seller's type of business.
    /// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CreateUnlinkedRefundIssuingTransactionMerchantDataCategory>,
    /// City where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Name of the seller
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Identifier assigned to the seller by the card network.
    /// Different card networks may assign different network_id fields to the same merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// Postal code where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State where the seller is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// An ID assigned by the seller to the location of the sale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    /// URL provided by the merchant on a 3DS request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl CreateUnlinkedRefundIssuingTransactionMerchantData {
    pub fn new() -> Self {
        Self {
            category: None,
            city: None,
            country: None,
            name: None,
            network_id: None,
            postal_code: None,
            state: None,
            terminal_id: None,
            url: None,
        }
    }
}
impl Default for CreateUnlinkedRefundIssuingTransactionMerchantData {
    fn default() -> Self {
        Self::new()
    }
}
/// A categorization of the seller's type of business.
/// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateUnlinkedRefundIssuingTransactionMerchantDataCategory {
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
    Unknown(String),
}
impl CreateUnlinkedRefundIssuingTransactionMerchantDataCategory {
    pub fn as_str(&self) -> &str {
        use CreateUnlinkedRefundIssuingTransactionMerchantDataCategory::*;
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
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateUnlinkedRefundIssuingTransactionMerchantDataCategory {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateUnlinkedRefundIssuingTransactionMerchantDataCategory::*;
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
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateUnlinkedRefundIssuingTransactionMerchantDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateUnlinkedRefundIssuingTransactionMerchantDataCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateUnlinkedRefundIssuingTransactionMerchantDataCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateUnlinkedRefundIssuingTransactionMerchantDataCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Additional purchase information that is optionally provided by the merchant.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateUnlinkedRefundIssuingTransactionPurchaseDetails {
    /// Fleet-specific information for transactions using Fleet cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fleet: Option<CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleet>,
    /// Information about the flight that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<FlightSpecs>,
    /// Information about fuel that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuel>,
    /// Information about lodging that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lodging: Option<LodgingSpecs>,
    /// The line items in the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<ReceiptSpecs>>,
    /// A merchant-specific order number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
impl CreateUnlinkedRefundIssuingTransactionPurchaseDetails {
    pub fn new() -> Self {
        Self {
            fleet: None,
            flight: None,
            fuel: None,
            lodging: None,
            receipt: None,
            reference: None,
        }
    }
}
impl Default for CreateUnlinkedRefundIssuingTransactionPurchaseDetails {
    fn default() -> Self {
        Self::new()
    }
}
/// Fleet-specific information for transactions using Fleet cards.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleet {
    /// Answers to prompts presented to the cardholder at the point of sale.
    /// Prompted fields vary depending on the configuration of your physical fleet cards.
    /// Typical points of sale support only numeric entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_prompt_data: Option<FleetCardholderPromptDataSpecs>,
    /// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type:
        Option<CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType>,
    /// More information about the total amount.
    /// This information is not guaranteed to be accurate as some merchants may provide unreliable data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_breakdown: Option<FleetReportedBreakdownSpecs>,
    /// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType>,
}
impl CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleet {
    pub fn new() -> Self {
        Self {
            cardholder_prompt_data: None,
            purchase_type: None,
            reported_breakdown: None,
            service_type: None,
        }
    }
}
impl Default for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleet {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of purchase. One of `fuel_purchase`, `non_fuel_purchase`, or `fuel_and_non_fuel_purchase`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType {
    FuelAndNonFuelPurchase,
    FuelPurchase,
    NonFuelPurchase,
}
impl CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType {
    pub fn as_str(self) -> &'static str {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType::*;
        match self {
            FuelAndNonFuelPurchase => "fuel_and_non_fuel_purchase",
            FuelPurchase => "fuel_purchase",
            NonFuelPurchase => "non_fuel_purchase",
        }
    }
}

impl std::str::FromStr for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType::*;
        match s {
            "fuel_and_non_fuel_purchase" => Ok(FuelAndNonFuelPurchase),
            "fuel_purchase" => Ok(FuelPurchase),
            "non_fuel_purchase" => Ok(NonFuelPurchase),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetPurchaseType"))
    }
}
/// The type of fuel service. One of `non_fuel_transaction`, `full_service`, or `self_service`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType {
    FullService,
    NonFuelTransaction,
    SelfService,
}
impl CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType {
    pub fn as_str(self) -> &'static str {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType::*;
        match self {
            FullService => "full_service",
            NonFuelTransaction => "non_fuel_transaction",
            SelfService => "self_service",
        }
    }
}

impl std::str::FromStr for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType::*;
        match s {
            "full_service" => Ok(FullService),
            "non_fuel_transaction" => Ok(NonFuelTransaction),
            "self_service" => Ok(SelfService),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFleetServiceType"))
    }
}
/// Information about fuel that was purchased with this transaction.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuel {
    /// [Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying the primary fuel product purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_product_code: Option<String>,
    /// The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_decimal: Option<String>,
    /// The type of fuel that was purchased.
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType>,
    /// The units for `quantity_decimal`.
    /// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit>,
    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost_decimal: Option<String>,
}
impl CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuel {
    pub fn new() -> Self {
        Self {
            industry_product_code: None,
            quantity_decimal: None,
            type_: None,
            unit: None,
            unit_cost_decimal: None,
        }
    }
}
impl Default for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuel {
    fn default() -> Self {
        Self::new()
    }
}
/// The type of fuel that was purchased.
/// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType {
    Diesel,
    Other,
    UnleadedPlus,
    UnleadedRegular,
    UnleadedSuper,
}
impl CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType {
    pub fn as_str(self) -> &'static str {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType::*;
        match self {
            Diesel => "diesel",
            Other => "other",
            UnleadedPlus => "unleaded_plus",
            UnleadedRegular => "unleaded_regular",
            UnleadedSuper => "unleaded_super",
        }
    }
}

impl std::str::FromStr for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType::*;
        match s {
            "diesel" => Ok(Diesel),
            "other" => Ok(Other),
            "unleaded_plus" => Ok(UnleadedPlus),
            "unleaded_regular" => Ok(UnleadedRegular),
            "unleaded_super" => Ok(UnleadedSuper),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelType",
            )
        })
    }
}
/// The units for `quantity_decimal`.
/// One of `charging_minute`, `imperial_gallon`, `kilogram`, `kilowatt_hour`, `liter`, `pound`, `us_gallon`, or `other`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit {
    ChargingMinute,
    ImperialGallon,
    Kilogram,
    KilowattHour,
    Liter,
    Other,
    Pound,
    UsGallon,
}
impl CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit {
    pub fn as_str(self) -> &'static str {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit::*;
        match self {
            ChargingMinute => "charging_minute",
            ImperialGallon => "imperial_gallon",
            Kilogram => "kilogram",
            KilowattHour => "kilowatt_hour",
            Liter => "liter",
            Other => "other",
            Pound => "pound",
            UsGallon => "us_gallon",
        }
    }
}

impl std::str::FromStr for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit::*;
        match s {
            "charging_minute" => Ok(ChargingMinute),
            "imperial_gallon" => Ok(ImperialGallon),
            "kilogram" => Ok(Kilogram),
            "kilowatt_hour" => Ok(KilowattHour),
            "liter" => Ok(Liter),
            "other" => Ok(Other),
            "pound" => Ok(Pound),
            "us_gallon" => Ok(UsGallon),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateUnlinkedRefundIssuingTransactionPurchaseDetailsFuelUnit",
            )
        })
    }
}
/// Allows the user to refund an arbitrary amount, also known as a unlinked refund.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateUnlinkedRefundIssuingTransaction {
    inner: CreateUnlinkedRefundIssuingTransactionBuilder,
}
impl CreateUnlinkedRefundIssuingTransaction {
    /// Construct a new `CreateUnlinkedRefundIssuingTransaction`.
    pub fn new(amount: impl Into<i64>, card: impl Into<String>) -> Self {
        Self {
            inner: CreateUnlinkedRefundIssuingTransactionBuilder::new(amount.into(), card.into()),
        }
    }
    /// The currency of the unlinked refund.
    /// If not provided, defaults to the currency of the card.
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened.
    pub fn merchant_data(
        mut self,
        merchant_data: impl Into<CreateUnlinkedRefundIssuingTransactionMerchantData>,
    ) -> Self {
        self.inner.merchant_data = Some(merchant_data.into());
        self
    }
    /// Additional purchase information that is optionally provided by the merchant.
    pub fn purchase_details(
        mut self,
        purchase_details: impl Into<CreateUnlinkedRefundIssuingTransactionPurchaseDetails>,
    ) -> Self {
        self.inner.purchase_details = Some(purchase_details.into());
        self
    }
}
impl CreateUnlinkedRefundIssuingTransaction {
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

impl StripeRequest for CreateUnlinkedRefundIssuingTransaction {
    type Output = stripe_shared::IssuingTransaction;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(
            StripeMethod::Post,
            "/test_helpers/issuing/transactions/create_unlinked_refund",
        )
        .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct FleetCardholderPromptDataSpecs {
    /// Driver ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<String>,
    /// Odometer reading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odometer: Option<i64>,
    /// An alphanumeric ID.
    /// This field is used when a vehicle ID, driver ID, or generic ID is entered by the cardholder, but the merchant or card network did not specify the prompt type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unspecified_id: Option<String>,
    /// User ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Vehicle number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_number: Option<String>,
}
impl FleetCardholderPromptDataSpecs {
    pub fn new() -> Self {
        Self {
            driver_id: None,
            odometer: None,
            unspecified_id: None,
            user_id: None,
            vehicle_number: None,
        }
    }
}
impl Default for FleetCardholderPromptDataSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct FleetReportedBreakdownFuelSpecs {
    /// Gross fuel amount that should equal Fuel Volume multipled by Fuel Unit Cost, inclusive of taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount_decimal: Option<String>,
}
impl FleetReportedBreakdownFuelSpecs {
    pub fn new() -> Self {
        Self { gross_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownFuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct FleetReportedBreakdownNonFuelSpecs {
    /// Gross non-fuel amount that should equal the sum of the line items, inclusive of taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount_decimal: Option<String>,
}
impl FleetReportedBreakdownNonFuelSpecs {
    pub fn new() -> Self {
        Self { gross_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownNonFuelSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct FleetReportedBreakdownTaxSpecs {
    /// Amount of state or provincial Sales Tax included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_amount_decimal: Option<String>,
    /// Amount of national Sales Tax or VAT included in the transaction amount.
    /// Null if not reported by merchant or not subject to tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_amount_decimal: Option<String>,
}
impl FleetReportedBreakdownTaxSpecs {
    pub fn new() -> Self {
        Self { local_amount_decimal: None, national_amount_decimal: None }
    }
}
impl Default for FleetReportedBreakdownTaxSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct FlightSegmentSpecs {
    /// The three-letter IATA airport code of the flight's destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_airport_code: Option<String>,
    /// The airline carrier code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    /// The three-letter IATA airport code that the flight departed from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,
    /// The flight number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<String>,
    /// The flight's service class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_class: Option<String>,
    /// Whether a stopover is allowed on this flight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopover_allowed: Option<bool>,
}
impl FlightSegmentSpecs {
    pub fn new() -> Self {
        Self {
            arrival_airport_code: None,
            carrier: None,
            departure_airport_code: None,
            flight_number: None,
            service_class: None,
            stopover_allowed: None,
        }
    }
}
impl Default for FlightSegmentSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct LodgingSpecs {
    /// The time of checking into the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_at: Option<stripe_types::Timestamp>,
    /// The number of nights stayed at the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nights: Option<i64>,
}
impl LodgingSpecs {
    pub fn new() -> Self {
        Self { check_in_at: None, nights: None }
    }
}
impl Default for LodgingSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReceiptSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<i64>,
}
impl ReceiptSpecs {
    pub fn new() -> Self {
        Self { description: None, quantity: None, total: None, unit_cost: None }
    }
}
impl Default for ReceiptSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct FleetReportedBreakdownSpecs {
    /// Breakdown of fuel portion of the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<FleetReportedBreakdownFuelSpecs>,
    /// Breakdown of non-fuel portion of the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_fuel: Option<FleetReportedBreakdownNonFuelSpecs>,
    /// Information about tax included in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<FleetReportedBreakdownTaxSpecs>,
}
impl FleetReportedBreakdownSpecs {
    pub fn new() -> Self {
        Self { fuel: None, non_fuel: None, tax: None }
    }
}
impl Default for FleetReportedBreakdownSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct FlightSpecs {
    /// The time that the flight departed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<stripe_types::Timestamp>,
    /// The name of the passenger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<String>,
    /// Whether the ticket is refundable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable: Option<bool>,
    /// The legs of the trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<FlightSegmentSpecs>>,
    /// The travel agency that issued the ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_agency: Option<String>,
}
impl FlightSpecs {
    pub fn new() -> Self {
        Self {
            departure_at: None,
            passenger_name: None,
            refundable: None,
            segments: None,
            travel_agency: None,
        }
    }
}
impl Default for FlightSpecs {
    fn default() -> Self {
        Self::new()
    }
}
