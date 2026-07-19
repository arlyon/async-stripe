use stripe_shared::{
    Charge, CheckoutSession, Customer, Invoice, PaymentIntent, PaymentLink, PaymentMethod, Price,
    Product, Refund, SetupIntent, Subscription,
};

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "{}".into());

    let customer: Option<Customer> = stripe_miniserde::json::from_str(&input).ok();
    let charge: Option<Charge> = stripe_miniserde::json::from_str(&input).ok();
    let pi: Option<PaymentIntent> = stripe_miniserde::json::from_str(&input).ok();
    let si: Option<SetupIntent> = stripe_miniserde::json::from_str(&input).ok();
    let refund: Option<Refund> = stripe_miniserde::json::from_str(&input).ok();
    let sub: Option<Subscription> = stripe_miniserde::json::from_str(&input).ok();
    let inv: Option<Invoice> = stripe_miniserde::json::from_str(&input).ok();
    let price: Option<Price> = stripe_miniserde::json::from_str(&input).ok();
    let product: Option<Product> = stripe_miniserde::json::from_str(&input).ok();
    let pm: Option<PaymentMethod> = stripe_miniserde::json::from_str(&input).ok();
    let pl: Option<PaymentLink> = stripe_miniserde::json::from_str(&input).ok();
    let cs: Option<CheckoutSession> = stripe_miniserde::json::from_str(&input).ok();

    let tfa: Option<stripe_treasury::TreasuryFinancialAccount> =
        stripe_miniserde::json::from_str(&input).ok();
    let tx: Option<stripe_treasury::TreasuryTransaction> = stripe_miniserde::json::from_str(&input).ok();
    let op: Option<stripe_treasury::TreasuryOutboundPayment> =
        stripe_miniserde::json::from_str(&input).ok();
    let ot: Option<stripe_treasury::TreasuryOutboundTransfer> =
        stripe_miniserde::json::from_str(&input).ok();
    let it: Option<stripe_treasury::TreasuryInboundTransfer> =
        stripe_miniserde::json::from_str(&input).ok();
    let rc: Option<stripe_treasury::TreasuryReceivedCredit> =
        stripe_miniserde::json::from_str(&input).ok();
    let rd: Option<stripe_treasury::TreasuryReceivedDebit> =
        stripe_miniserde::json::from_str(&input).ok();

    let tr: Option<stripe_terminal::TerminalReader> = stripe_miniserde::json::from_str(&input).ok();
    let tl: Option<stripe_terminal::TerminalLocation> = stripe_miniserde::json::from_str(&input).ok();
    let tc: Option<stripe_terminal::TerminalConfiguration> =
        stripe_miniserde::json::from_str(&input).ok();

    let er: Option<stripe_misc::ExchangeRate> = stripe_miniserde::json::from_str(&input).ok();
    let co: Option<stripe_misc::ClimateOrder> = stripe_miniserde::json::from_str(&input).ok();
    let fca: Option<stripe_misc::FinancialConnectionsAccount> =
        stripe_miniserde::json::from_str(&input).ok();
    let ivs: Option<stripe_misc::IdentityVerificationSession> =
        stripe_miniserde::json::from_str(&input).ok();
    let rrr: Option<stripe_misc::ReportingReportRun> = stripe_miniserde::json::from_str(&input).ok();

    let al: Option<stripe_connect::AccountLink> = stripe_miniserde::json::from_str(&input).ok();
    let csp: Option<stripe_connect::CountrySpec> = stripe_miniserde::json::from_str(&input).ok();

    let rvl: Option<stripe_fraud::RadarValueList> = stripe_miniserde::json::from_str(&input).ok();
    let refw: Option<stripe_fraud::RadarEarlyFraudWarning> =
        stripe_miniserde::json::from_str(&input).ok();

    std::hint::black_box((
        customer, charge, pi, si, refund, sub, inv, price, product, pm, pl, cs,
    ));
    std::hint::black_box((tfa, tx, op, ot, it, rc, rd));
    std::hint::black_box((tr, tl, tc));
    std::hint::black_box((er, co, fca, ivs, rrr));
    std::hint::black_box((al, csp, rvl, refw));
}
