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
    pub scalapay: Option<stripe_shared::DestinationDetailsUnimplemented>,
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
    affirm: Option<stripe_shared::DestinationDetailsUnimplemented>,
    afterpay_clearpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    alipay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    alma: Option<stripe_shared::DestinationDetailsUnimplemented>,
    amazon_pay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    au_bank_transfer: Option<stripe_shared::DestinationDetailsUnimplemented>,
    blik: Option<stripe_shared::RefundDestinationDetailsBlik>,
    br_bank_transfer: Option<stripe_shared::RefundDestinationDetailsBrBankTransfer>,
    card: Option<stripe_shared::RefundDestinationDetailsCard>,
    cashapp: Option<stripe_shared::DestinationDetailsUnimplemented>,
    crypto: Option<stripe_shared::RefundDestinationDetailsCrypto>,
    customer_cash_balance: Option<stripe_shared::DestinationDetailsUnimplemented>,
    eps: Option<stripe_shared::DestinationDetailsUnimplemented>,
    eu_bank_transfer: Option<stripe_shared::RefundDestinationDetailsEuBankTransfer>,
    gb_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGbBankTransfer>,
    giropay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    grabpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    jp_bank_transfer: Option<stripe_shared::RefundDestinationDetailsJpBankTransfer>,
    klarna: Option<stripe_shared::DestinationDetailsUnimplemented>,
    mb_way: Option<stripe_shared::RefundDestinationDetailsMbWay>,
    multibanco: Option<stripe_shared::RefundDestinationDetailsMultibanco>,
    mx_bank_transfer: Option<stripe_shared::RefundDestinationDetailsMxBankTransfer>,
    nz_bank_transfer: Option<stripe_shared::DestinationDetailsUnimplemented>,
    p24: Option<stripe_shared::RefundDestinationDetailsP24>,
    paynow: Option<stripe_shared::DestinationDetailsUnimplemented>,
    paypal: Option<stripe_shared::RefundDestinationDetailsPaypal>,
    pix: Option<stripe_shared::DestinationDetailsUnimplemented>,
    revolut: Option<stripe_shared::DestinationDetailsUnimplemented>,
    scalapay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    sofort: Option<stripe_shared::DestinationDetailsUnimplemented>,
    swish: Option<stripe_shared::RefundDestinationDetailsSwish>,
    th_bank_transfer: Option<stripe_shared::RefundDestinationDetailsThBankTransfer>,
    twint: Option<stripe_shared::DestinationDetailsUnimplemented>,
    type_: Option<String>,
    us_bank_transfer: Option<stripe_shared::RefundDestinationDetailsUsBankTransfer>,
    wechat_pay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    zip: Option<stripe_shared::DestinationDetailsUnimplemented>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: RefundDestinationDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RefundDestinationDetailsBuilder {
        type Out = RefundDestinationDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "affirm" => stripe_types::miniserde_helpers::begin_nullable(&mut self.affirm),
                "afterpay_clearpay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.afterpay_clearpay)
                }
                "alipay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.alipay),
                "alma" => stripe_types::miniserde_helpers::begin_nullable(&mut self.alma),
                "amazon_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amazon_pay)
                }
                "au_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.au_bank_transfer)
                }
                "blik" => stripe_types::miniserde_helpers::begin_nullable(&mut self.blik),
                "br_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.br_bank_transfer)
                }
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "crypto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.crypto),
                "customer_cash_balance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.customer_cash_balance)
                }
                "eps" => stripe_types::miniserde_helpers::begin_nullable(&mut self.eps),
                "eu_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.eu_bank_transfer)
                }
                "gb_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.gb_bank_transfer)
                }
                "giropay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.giropay),
                "grabpay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.grabpay),
                "jp_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.jp_bank_transfer)
                }
                "klarna" => stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna),
                "mb_way" => stripe_types::miniserde_helpers::begin_nullable(&mut self.mb_way),
                "multibanco" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.multibanco)
                }
                "mx_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.mx_bank_transfer)
                }
                "nz_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.nz_bank_transfer)
                }
                "p24" => stripe_types::miniserde_helpers::begin_nullable(&mut self.p24),
                "paynow" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paynow),
                "paypal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paypal),
                "pix" => stripe_types::miniserde_helpers::begin_nullable(&mut self.pix),
                "revolut" => stripe_types::miniserde_helpers::begin_nullable(&mut self.revolut),
                "scalapay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.scalapay),
                "sofort" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sofort),
                "swish" => stripe_types::miniserde_helpers::begin_nullable(&mut self.swish),
                "th_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.th_bank_transfer)
                }
                "twint" => stripe_types::miniserde_helpers::begin_nullable(&mut self.twint),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.us_bank_transfer)
                }
                "wechat_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.wechat_pay)
                }
                "zip" => stripe_types::miniserde_helpers::begin_nullable(&mut self.zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                affirm: None,
                afterpay_clearpay: None,
                alipay: None,
                alma: None,
                amazon_pay: None,
                au_bank_transfer: None,
                blik: None,
                br_bank_transfer: None,
                card: None,
                cashapp: None,
                crypto: None,
                customer_cash_balance: None,
                eps: None,
                eu_bank_transfer: None,
                gb_bank_transfer: None,
                giropay: None,
                grabpay: None,
                jp_bank_transfer: None,
                klarna: None,
                mb_way: None,
                multibanco: None,
                mx_bank_transfer: None,
                nz_bank_transfer: None,
                p24: None,
                paynow: None,
                paypal: None,
                pix: None,
                revolut: None,
                scalapay: None,
                sofort: None,
                swish: None,
                th_bank_transfer: None,
                twint: None,
                type_: None,
                us_bank_transfer: None,
                wechat_pay: None,
                zip: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
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
                    scalapay,
                    sofort,
                    swish,
                    th_bank_transfer,
                    twint,
                    type_: Some(type_),
                    us_bank_transfer,
                    wechat_pay,
                    zip,
                } => Some(Self::Out {
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
                    scalapay,
                    sofort,
                    swish,
                    th_bank_transfer,
                    twint,
                    type_,
                    us_bank_transfer,
                    wechat_pay,
                    zip,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for RefundDestinationDetails {
        type Builder = RefundDestinationDetailsBuilder;
    }

    impl FromValueOpt for RefundDestinationDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
