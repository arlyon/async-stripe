#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundDestinationDetails {
    pub affirm: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub afterpay_clearpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub alipay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub alma: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub amazon_pay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub au_bank_transfer: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub blik: Option<stripe_shared::RefundDestinationDetailsBlik>,
    pub br_bank_transfer: Option<stripe_shared::RefundDestinationDetailsBrBankTransfer>,
    pub card: Option<stripe_shared::RefundDestinationDetailsCard>,
    pub cashapp: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub crypto: Option<stripe_shared::RefundDestinationDetailsCrypto>,
    pub customer_cash_balance: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub eps: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub eu_bank_transfer: Option<stripe_shared::RefundDestinationDetailsEuBankTransfer>,
    pub gb_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGbBankTransfer>,
    pub giropay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub grabpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub jp_bank_transfer: Option<stripe_shared::RefundDestinationDetailsJpBankTransfer>,
    pub klarna: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub mb_way: Option<stripe_shared::RefundDestinationDetailsMbWay>,
    pub multibanco: Option<stripe_shared::RefundDestinationDetailsMultibanco>,
    pub mx_bank_transfer: Option<stripe_shared::RefundDestinationDetailsMxBankTransfer>,
    pub nz_bank_transfer: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub p24: Option<stripe_shared::RefundDestinationDetailsP24>,
    pub paynow: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub paypal: Option<stripe_shared::RefundDestinationDetailsPaypal>,
    pub pix: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub revolut: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub sofort: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub swish: Option<stripe_shared::RefundDestinationDetailsSwish>,
    pub th_bank_transfer: Option<stripe_shared::RefundDestinationDetailsThBankTransfer>,
    pub twint: Option<stripe_shared::DestinationDetailsUnimplemented>,
    /// The type of transaction-specific details of the payment method used in the refund (e.g., `card`).
    /// An additional hash is included on `destination_details` with a name matching this value.
    /// It contains information specific to the refund transaction.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_transfer: Option<stripe_shared::RefundDestinationDetailsUsBankTransfer>,
    pub wechat_pay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub zip: Option<stripe_shared::DestinationDetailsUnimplemented>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundDestinationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundDestinationDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RefundDestinationDetailsBuilder {
    affirm: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    afterpay_clearpay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    alipay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    alma: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    amazon_pay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    au_bank_transfer: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    blik: Option<Option<stripe_shared::RefundDestinationDetailsBlik>>,
    br_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsBrBankTransfer>>,
    card: Option<Option<stripe_shared::RefundDestinationDetailsCard>>,
    cashapp: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    crypto: Option<Option<stripe_shared::RefundDestinationDetailsCrypto>>,
    customer_cash_balance: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    eps: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    eu_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsEuBankTransfer>>,
    gb_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGbBankTransfer>>,
    giropay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    grabpay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    jp_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsJpBankTransfer>>,
    klarna: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    mb_way: Option<Option<stripe_shared::RefundDestinationDetailsMbWay>>,
    multibanco: Option<Option<stripe_shared::RefundDestinationDetailsMultibanco>>,
    mx_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsMxBankTransfer>>,
    nz_bank_transfer: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    p24: Option<Option<stripe_shared::RefundDestinationDetailsP24>>,
    paynow: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    paypal: Option<Option<stripe_shared::RefundDestinationDetailsPaypal>>,
    pix: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    revolut: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    sofort: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    swish: Option<Option<stripe_shared::RefundDestinationDetailsSwish>>,
    th_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsThBankTransfer>>,
    twint: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    type_: Option<String>,
    us_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsUsBankTransfer>>,
    wechat_pay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    zip: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for RefundDestinationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundDestinationDetails>,
        builder: RefundDestinationDetailsBuilder,
    }

    impl Visitor for Place<RefundDestinationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RefundDestinationDetailsBuilder {
                    affirm: Deserialize::default(),
                    afterpay_clearpay: Deserialize::default(),
                    alipay: Deserialize::default(),
                    alma: Deserialize::default(),
                    amazon_pay: Deserialize::default(),
                    au_bank_transfer: Deserialize::default(),
                    blik: Deserialize::default(),
                    br_bank_transfer: Deserialize::default(),
                    card: Deserialize::default(),
                    cashapp: Deserialize::default(),
                    crypto: Deserialize::default(),
                    customer_cash_balance: Deserialize::default(),
                    eps: Deserialize::default(),
                    eu_bank_transfer: Deserialize::default(),
                    gb_bank_transfer: Deserialize::default(),
                    giropay: Deserialize::default(),
                    grabpay: Deserialize::default(),
                    jp_bank_transfer: Deserialize::default(),
                    klarna: Deserialize::default(),
                    mb_way: Deserialize::default(),
                    multibanco: Deserialize::default(),
                    mx_bank_transfer: Deserialize::default(),
                    nz_bank_transfer: Deserialize::default(),
                    p24: Deserialize::default(),
                    paynow: Deserialize::default(),
                    paypal: Deserialize::default(),
                    pix: Deserialize::default(),
                    revolut: Deserialize::default(),
                    sofort: Deserialize::default(),
                    swish: Deserialize::default(),
                    th_bank_transfer: Deserialize::default(),
                    twint: Deserialize::default(),
                    type_: Deserialize::default(),
                    us_bank_transfer: Deserialize::default(),
                    wechat_pay: Deserialize::default(),
                    zip: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "affirm" => Deserialize::begin(&mut self.builder.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.builder.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.builder.alipay),
                "alma" => Deserialize::begin(&mut self.builder.alma),
                "amazon_pay" => Deserialize::begin(&mut self.builder.amazon_pay),
                "au_bank_transfer" => Deserialize::begin(&mut self.builder.au_bank_transfer),
                "blik" => Deserialize::begin(&mut self.builder.blik),
                "br_bank_transfer" => Deserialize::begin(&mut self.builder.br_bank_transfer),
                "card" => Deserialize::begin(&mut self.builder.card),
                "cashapp" => Deserialize::begin(&mut self.builder.cashapp),
                "crypto" => Deserialize::begin(&mut self.builder.crypto),
                "customer_cash_balance" => {
                    Deserialize::begin(&mut self.builder.customer_cash_balance)
                }
                "eps" => Deserialize::begin(&mut self.builder.eps),
                "eu_bank_transfer" => Deserialize::begin(&mut self.builder.eu_bank_transfer),
                "gb_bank_transfer" => Deserialize::begin(&mut self.builder.gb_bank_transfer),
                "giropay" => Deserialize::begin(&mut self.builder.giropay),
                "grabpay" => Deserialize::begin(&mut self.builder.grabpay),
                "jp_bank_transfer" => Deserialize::begin(&mut self.builder.jp_bank_transfer),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "mb_way" => Deserialize::begin(&mut self.builder.mb_way),
                "multibanco" => Deserialize::begin(&mut self.builder.multibanco),
                "mx_bank_transfer" => Deserialize::begin(&mut self.builder.mx_bank_transfer),
                "nz_bank_transfer" => Deserialize::begin(&mut self.builder.nz_bank_transfer),
                "p24" => Deserialize::begin(&mut self.builder.p24),
                "paynow" => Deserialize::begin(&mut self.builder.paynow),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                "pix" => Deserialize::begin(&mut self.builder.pix),
                "revolut" => Deserialize::begin(&mut self.builder.revolut),
                "sofort" => Deserialize::begin(&mut self.builder.sofort),
                "swish" => Deserialize::begin(&mut self.builder.swish),
                "th_bank_transfer" => Deserialize::begin(&mut self.builder.th_bank_transfer),
                "twint" => Deserialize::begin(&mut self.builder.twint),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "us_bank_transfer" => Deserialize::begin(&mut self.builder.us_bank_transfer),
                "wechat_pay" => Deserialize::begin(&mut self.builder.wechat_pay),
                "zip" => Deserialize::begin(&mut self.builder.zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(affirm),
                Some(afterpay_clearpay),
                Some(alipay),
                Some(alma),
                Some(amazon_pay),
                Some(au_bank_transfer),
                Some(blik),
                Some(br_bank_transfer),
                Some(card),
                Some(cashapp),
                Some(crypto),
                Some(customer_cash_balance),
                Some(eps),
                Some(eu_bank_transfer),
                Some(gb_bank_transfer),
                Some(giropay),
                Some(grabpay),
                Some(jp_bank_transfer),
                Some(klarna),
                Some(mb_way),
                Some(multibanco),
                Some(mx_bank_transfer),
                Some(nz_bank_transfer),
                Some(p24),
                Some(paynow),
                Some(paypal),
                Some(pix),
                Some(revolut),
                Some(sofort),
                Some(swish),
                Some(th_bank_transfer),
                Some(twint),
                Some(type_),
                Some(us_bank_transfer),
                Some(wechat_pay),
                Some(zip),
            ) = (
                self.builder.affirm,
                self.builder.afterpay_clearpay,
                self.builder.alipay,
                self.builder.alma,
                self.builder.amazon_pay,
                self.builder.au_bank_transfer,
                self.builder.blik.take(),
                self.builder.br_bank_transfer.take(),
                self.builder.card.take(),
                self.builder.cashapp,
                self.builder.crypto.take(),
                self.builder.customer_cash_balance,
                self.builder.eps,
                self.builder.eu_bank_transfer.take(),
                self.builder.gb_bank_transfer.take(),
                self.builder.giropay,
                self.builder.grabpay,
                self.builder.jp_bank_transfer.take(),
                self.builder.klarna,
                self.builder.mb_way.take(),
                self.builder.multibanco.take(),
                self.builder.mx_bank_transfer.take(),
                self.builder.nz_bank_transfer,
                self.builder.p24.take(),
                self.builder.paynow,
                self.builder.paypal.take(),
                self.builder.pix,
                self.builder.revolut,
                self.builder.sofort,
                self.builder.swish.take(),
                self.builder.th_bank_transfer.take(),
                self.builder.twint,
                self.builder.type_.take(),
                self.builder.us_bank_transfer.take(),
                self.builder.wechat_pay,
                self.builder.zip,
            )
            else {
                return Ok(());
            };
            *self.out = Some(RefundDestinationDetails {
                affirm,
                afterpay_clearpay,
                alipay,
                alma,
                amazon_pay,
                au_bank_transfer,
                blik,
                br_bank_transfer,
                card,
                cashapp,
                crypto,
                customer_cash_balance,
                eps,
                eu_bank_transfer,
                gb_bank_transfer,
                giropay,
                grabpay,
                jp_bank_transfer,
                klarna,
                mb_way,
                multibanco,
                mx_bank_transfer,
                nz_bank_transfer,
                p24,
                paynow,
                paypal,
                pix,
                revolut,
                sofort,
                swish,
                th_bank_transfer,
                twint,
                type_,
                us_bank_transfer,
                wechat_pay,
                zip,
            });
            Ok(())
        }
    }
};
