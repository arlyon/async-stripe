use stripe_shared::{
    Charge, CheckoutSession, Customer, Invoice, PaymentIntent, PaymentLink, PaymentMethod, Price,
    Product, Refund, SetupIntent, Subscription,
};

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "{}".into());

    let customer: Option<Customer> = miniserde::json::from_str(&input).ok();
    let charge: Option<Charge> = miniserde::json::from_str(&input).ok();
    let pi: Option<PaymentIntent> = miniserde::json::from_str(&input).ok();
    let si: Option<SetupIntent> = miniserde::json::from_str(&input).ok();
    let refund: Option<Refund> = miniserde::json::from_str(&input).ok();
    let sub: Option<Subscription> = miniserde::json::from_str(&input).ok();
    let inv: Option<Invoice> = miniserde::json::from_str(&input).ok();
    let price: Option<Price> = miniserde::json::from_str(&input).ok();
    let product: Option<Product> = miniserde::json::from_str(&input).ok();
    let pm: Option<PaymentMethod> = miniserde::json::from_str(&input).ok();
    let pl: Option<PaymentLink> = miniserde::json::from_str(&input).ok();
    let cs: Option<CheckoutSession> = miniserde::json::from_str(&input).ok();

    let tfa: Option<stripe_treasury::TreasuryFinancialAccount> =
        miniserde::json::from_str(&input).ok();
    let tx: Option<stripe_treasury::TreasuryTransaction> = miniserde::json::from_str(&input).ok();
    let op: Option<stripe_treasury::TreasuryOutboundPayment> =
        miniserde::json::from_str(&input).ok();
    let ot: Option<stripe_treasury::TreasuryOutboundTransfer> =
        miniserde::json::from_str(&input).ok();
    let it: Option<stripe_treasury::TreasuryInboundTransfer> =
        miniserde::json::from_str(&input).ok();
    let rc: Option<stripe_treasury::TreasuryReceivedCredit> =
        miniserde::json::from_str(&input).ok();
    let rd: Option<stripe_treasury::TreasuryReceivedDebit> =
        miniserde::json::from_str(&input).ok();

    let tr: Option<stripe_terminal::TerminalReader> = miniserde::json::from_str(&input).ok();
    let tl: Option<stripe_terminal::TerminalLocation> = miniserde::json::from_str(&input).ok();
    let tc: Option<stripe_terminal::TerminalConfiguration> =
        miniserde::json::from_str(&input).ok();

    let er: Option<stripe_misc::ExchangeRate> = miniserde::json::from_str(&input).ok();
    let co: Option<stripe_misc::ClimateOrder> = miniserde::json::from_str(&input).ok();
    let fca: Option<stripe_misc::FinancialConnectionsAccount> =
        miniserde::json::from_str(&input).ok();
    let ivs: Option<stripe_misc::IdentityVerificationSession> =
        miniserde::json::from_str(&input).ok();
    let rrr: Option<stripe_misc::ReportingReportRun> = miniserde::json::from_str(&input).ok();

    let al: Option<stripe_connect::AccountLink> = miniserde::json::from_str(&input).ok();
    let csp: Option<stripe_connect::CountrySpec> = miniserde::json::from_str(&input).ok();

    let rvl: Option<stripe_fraud::RadarValueList> = miniserde::json::from_str(&input).ok();
    let refw: Option<stripe_fraud::RadarEarlyFraudWarning> =
        miniserde::json::from_str(&input).ok();

    std::hint::black_box((
        customer, charge, pi, si, refund, sub, inv, price, product, pm, pl, cs,
    ));
    std::hint::black_box((tfa, tx, op, ot, it, rc, rd));
    std::hint::black_box((tr, tl, tc));
    std::hint::black_box((er, co, fca, ivs, rrr));
    std::hint::black_box((al, csp, rvl, refw));
}
