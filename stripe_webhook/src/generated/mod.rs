#[derive(Clone, Debug, serde::Deserialize)]
#[non_exhaustive]
#[serde(tag = "object")]
pub enum EventObject {
    #[serde(rename = "account")]
    Account(stripe_types::Account),
    #[serde(rename = "capability")]
    AccountCapability(stripe_types::AccountCapability),
    #[serde(rename = "application")]
    Application(stripe_types::Application),
    #[serde(rename = "application_fee")]
    PlatformFee(stripe_types::PlatformFee),
    #[serde(rename = "fee_refund")]
    FeeRefund(stripe_types::FeeRefund),
    #[cfg(feature = "stripe_core")]
    #[serde(rename = "balance")]
    Balance(stripe_core::Balance),
    #[serde(rename = "bank_account")]
    BankAccount(stripe_types::BankAccount),
    #[cfg(feature = "stripe_billing")]
    #[serde(rename = "billing_portal.configuration")]
    PortalConfiguration(stripe_billing::PortalConfiguration),
    #[serde(rename = "card")]
    Card(stripe_types::Card),
    #[cfg(feature = "stripe_checkout")]
    #[serde(rename = "checkout.session")]
    Session(stripe_checkout::Session),
    #[serde(rename = "coupon")]
    Coupon(stripe_types::Coupon),
    #[serde(rename = "customer")]
    Customer(stripe_types::Customer),
    #[serde(rename = "discount")]
    Discount(stripe_types::Discount),
    #[serde(rename = "dispute")]
    Dispute(stripe_types::Dispute),
    #[serde(rename = "file")]
    File(stripe_types::File),
    #[serde(rename = "invoice")]
    Invoice(stripe_types::Invoice),
    #[serde(rename = "invoiceitem")]
    InvoiceItem(stripe_types::InvoiceItem),
    #[serde(rename = "issuing.authorization")]
    IssuingAuthorization(stripe_types::IssuingAuthorization),
    #[serde(rename = "issuing.card")]
    IssuingCard(stripe_types::IssuingCard),
    #[serde(rename = "issuing.cardholder")]
    IssuingCardholder(stripe_types::IssuingCardholder),
    #[serde(rename = "issuing.dispute")]
    IssuingDispute(stripe_types::IssuingDispute),
    #[serde(rename = "issuing.transaction")]
    IssuingTransaction(stripe_types::IssuingTransaction),
    #[serde(rename = "mandate")]
    Mandate(stripe_types::Mandate),
    #[serde(rename = "payment_intent")]
    PaymentIntent(stripe_types::PaymentIntent),
    #[serde(rename = "payment_link")]
    PaymentLink(stripe_types::PaymentLink),
    #[serde(rename = "payment_method")]
    PaymentMethod(stripe_types::PaymentMethod),
    #[serde(rename = "payout")]
    Payout(stripe_types::Payout),
    #[serde(rename = "person")]
    Person(stripe_types::Person),
    #[serde(rename = "plan")]
    Plan(stripe_types::Plan),
    #[serde(rename = "price")]
    Price(stripe_types::Price),
    #[serde(rename = "product")]
    Product(stripe_types::Product),
    #[serde(rename = "promotion_code")]
    PromotionCode(stripe_types::PromotionCode),
    #[serde(rename = "quote")]
    Quote(stripe_types::Quote),
    #[serde(rename = "refund")]
    Refund(stripe_types::Refund),
    #[serde(rename = "review")]
    RadarReview(stripe_types::RadarReview),
    #[serde(rename = "setup_intent")]
    SetupIntent(stripe_types::SetupIntent),
    #[serde(rename = "subscription")]
    Subscription(stripe_types::Subscription),
    #[serde(rename = "subscription_schedule")]
    SubscriptionSchedule(stripe_types::SubscriptionSchedule),
    #[serde(rename = "tax_id")]
    TaxId(stripe_types::TaxId),
    #[serde(rename = "tax_rate")]
    TaxRate(stripe_types::TaxRate),
    #[serde(rename = "test_helpers.test_clock")]
    TestClock(stripe_types::TestClock),
    #[serde(rename = "topup")]
    Topup(stripe_types::Topup),
    #[serde(rename = "transfer")]
    Transfer(stripe_types::Transfer),
    #[serde(other)]
    Unknown,
}
