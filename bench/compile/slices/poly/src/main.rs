use stripe_shared::{
    BalanceTransactionSource, DeletedPaymentSource, Event, ExternalAccount, PaymentSource,
};

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "{}".into());

    let payment_source: Option<PaymentSource> = stripe_miniserde::json::from_str(&input).ok();
    let deleted_source: Option<DeletedPaymentSource> = stripe_miniserde::json::from_str(&input).ok();
    let external_account: Option<ExternalAccount> = stripe_miniserde::json::from_str(&input).ok();
    let balance_source: Option<BalanceTransactionSource> = stripe_miniserde::json::from_str(&input).ok();
    let event: Option<Event> = stripe_miniserde::json::from_str(&input).ok();

    std::hint::black_box((
        payment_source,
        deleted_source,
        external_account,
        balance_source,
        event,
    ));
}
