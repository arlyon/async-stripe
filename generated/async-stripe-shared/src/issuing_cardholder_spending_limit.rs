#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderSpendingLimit {
    /// Maximum amount allowed to spend per interval.
    /// This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    /// Omitting this field will apply the limit to all categories.
    pub categories: Option<Vec<IssuingCardholderSpendingLimitCategories>>,
    /// Interval (or event) to which the amount applies.
    pub interval: IssuingCardholderSpendingLimitInterval,
}
#[doc(hidden)]
pub struct IssuingCardholderSpendingLimitBuilder {
    amount: Option<i64>,
    categories: Option<Option<Vec<IssuingCardholderSpendingLimitCategories>>>,
    interval: Option<IssuingCardholderSpendingLimitInterval>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for IssuingCardholderSpendingLimit {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderSpendingLimit>,
        builder: IssuingCardholderSpendingLimitBuilder,
    }

    impl Visitor for Place<IssuingCardholderSpendingLimit> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderSpendingLimitBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardholderSpendingLimitBuilder {
        type Out = IssuingCardholderSpendingLimit;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "categories" => Deserialize::begin(&mut self.categories),
                "interval" => Deserialize::begin(&mut self.interval),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                categories: Deserialize::default(),
                interval: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(categories), Some(interval)) =
                (self.amount, self.categories.take(), self.interval)
            else {
                return None;
            };
            Some(Self::Out { amount, categories, interval })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for IssuingCardholderSpendingLimit {
        type Builder = IssuingCardholderSpendingLimitBuilder;
    }

    impl FromValueOpt for IssuingCardholderSpendingLimit {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingCardholderSpendingLimitBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "categories" => b.categories = FromValueOpt::from_value(v),
                    "interval" => b.interval = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
/// Omitting this field will apply the limit to all categories.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardholderSpendingLimitCategories {
    pub fn as_str(&self) -> &str {
        use IssuingCardholderSpendingLimitCategories::*;
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
            Miscellaneous => "miscellaneous",
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

impl std::str::FromStr for IssuingCardholderSpendingLimitCategories {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderSpendingLimitCategories::*;
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
            "miscellaneous" => Ok(Miscellaneous),
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
impl std::fmt::Display for IssuingCardholderSpendingLimitCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderSpendingLimitCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardholderSpendingLimitCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardholderSpendingLimitCategories {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardholderSpendingLimitCategories> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardholderSpendingLimitCategories::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardholderSpendingLimitCategories);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardholderSpendingLimitCategories {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Interval (or event) to which the amount applies.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use IssuingCardholderSpendingLimitInterval::*;
        match self {
            AllTime => "all_time",
            Daily => "daily",
            Monthly => "monthly",
            PerAuthorization => "per_authorization",
            Weekly => "weekly",
            Yearly => "yearly",
        }
    }
}

impl std::str::FromStr for IssuingCardholderSpendingLimitInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardholderSpendingLimitInterval::*;
        match s {
            "all_time" => Ok(AllTime),
            "daily" => Ok(Daily),
            "monthly" => Ok(Monthly),
            "per_authorization" => Ok(PerAuthorization),
            "weekly" => Ok(Weekly),
            "yearly" => Ok(Yearly),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for IssuingCardholderSpendingLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for IssuingCardholderSpendingLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingCardholderSpendingLimitInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardholderSpendingLimitInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardholderSpendingLimitInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            IssuingCardholderSpendingLimitInterval::from_str(s).map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardholderSpendingLimitInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardholderSpendingLimitInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for IssuingCardholderSpendingLimitInterval")
        })
    }
}
