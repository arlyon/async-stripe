use stripe_billing::{Invoice, Subscription};
use stripe_checkout::CheckoutSession;
use stripe_core::{Charge, Customer, PaymentIntent, Refund, SetupIntent};
use stripe_payment::{PaymentLink, PaymentMethod};
use stripe_product::{Price, Product};

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "{}".into());

    // Keep every deserializer reachable without aggregating large response values.
    std::hint::black_box(miniserde::json::from_str::<Customer>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<Charge>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<PaymentIntent>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<SetupIntent>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<Refund>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<Subscription>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<Invoice>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<Price>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<Product>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<PaymentMethod>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<PaymentLink>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<CheckoutSession>(&input).ok());

    std::hint::black_box(
        miniserde::json::from_str::<stripe_treasury::TreasuryFinancialAccount>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_treasury::TreasuryTransaction>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_treasury::TreasuryOutboundPayment>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_treasury::TreasuryOutboundTransfer>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_treasury::TreasuryInboundTransfer>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_treasury::TreasuryReceivedCredit>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_treasury::TreasuryReceivedDebit>(&input).ok(),
    );

    std::hint::black_box(miniserde::json::from_str::<stripe_terminal::TerminalReader>(&input).ok());
    std::hint::black_box(
        miniserde::json::from_str::<stripe_terminal::TerminalLocation>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_terminal::TerminalConfiguration>(&input).ok(),
    );

    std::hint::black_box(miniserde::json::from_str::<stripe_misc::ExchangeRate>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<stripe_misc::ClimateOrder>(&input).ok());
    std::hint::black_box(
        miniserde::json::from_str::<stripe_misc::FinancialConnectionsAccount>(&input).ok(),
    );
    std::hint::black_box(
        miniserde::json::from_str::<stripe_misc::IdentityVerificationSession>(&input).ok(),
    );
    std::hint::black_box(miniserde::json::from_str::<stripe_misc::ReportingReportRun>(&input).ok());

    std::hint::black_box(miniserde::json::from_str::<stripe_connect::AccountLink>(&input).ok());
    std::hint::black_box(miniserde::json::from_str::<stripe_connect::CountrySpec>(&input).ok());

    std::hint::black_box(miniserde::json::from_str::<stripe_fraud::RadarValueList>(&input).ok());
    std::hint::black_box(
        miniserde::json::from_str::<stripe_fraud::RadarEarlyFraudWarning>(&input).ok(),
    );
}
