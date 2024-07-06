#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundDestinationDetails {
    pub affirm: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub afterpay_clearpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub alipay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub amazon_pay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub au_bank_transfer: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub blik: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub br_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub card: Option<stripe_shared::RefundDestinationDetailsCard>,
    pub cashapp: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub customer_cash_balance: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub eps: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub eu_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub gb_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub giropay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub grabpay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub jp_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub klarna: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub mx_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub p24: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub paynow: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub paypal: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub pix: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub revolut: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub sofort: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub swish: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub th_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    /// The type of transaction-specific details of the payment method used in the refund (e.g., `card`).
    /// An additional hash is included on `destination_details` with a name matching this value.
    /// It contains information specific to the refund transaction.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_transfer: Option<stripe_shared::RefundDestinationDetailsGeneric>,
    pub wechat_pay: Option<stripe_shared::DestinationDetailsUnimplemented>,
    pub zip: Option<stripe_shared::DestinationDetailsUnimplemented>,
}
#[doc(hidden)]
pub struct RefundDestinationDetailsBuilder {
    affirm: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    afterpay_clearpay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    alipay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    amazon_pay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    au_bank_transfer: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    blik: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    br_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    card: Option<Option<stripe_shared::RefundDestinationDetailsCard>>,
    cashapp: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    customer_cash_balance: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    eps: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    eu_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    gb_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    giropay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    grabpay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    jp_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    klarna: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    mx_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    p24: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    paynow: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    paypal: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    pix: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    revolut: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    sofort: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    swish: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    th_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    type_: Option<String>,
    us_bank_transfer: Option<Option<stripe_shared::RefundDestinationDetailsGeneric>>,
    wechat_pay: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
    zip: Option<Option<stripe_shared::DestinationDetailsUnimplemented>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
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
                "affirm" => Deserialize::begin(&mut self.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.alipay),
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "au_bank_transfer" => Deserialize::begin(&mut self.au_bank_transfer),
                "blik" => Deserialize::begin(&mut self.blik),
                "br_bank_transfer" => Deserialize::begin(&mut self.br_bank_transfer),
                "card" => Deserialize::begin(&mut self.card),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "customer_cash_balance" => Deserialize::begin(&mut self.customer_cash_balance),
                "eps" => Deserialize::begin(&mut self.eps),
                "eu_bank_transfer" => Deserialize::begin(&mut self.eu_bank_transfer),
                "gb_bank_transfer" => Deserialize::begin(&mut self.gb_bank_transfer),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "grabpay" => Deserialize::begin(&mut self.grabpay),
                "jp_bank_transfer" => Deserialize::begin(&mut self.jp_bank_transfer),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "mx_bank_transfer" => Deserialize::begin(&mut self.mx_bank_transfer),
                "p24" => Deserialize::begin(&mut self.p24),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "pix" => Deserialize::begin(&mut self.pix),
                "revolut" => Deserialize::begin(&mut self.revolut),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "swish" => Deserialize::begin(&mut self.swish),
                "th_bank_transfer" => Deserialize::begin(&mut self.th_bank_transfer),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_transfer" => Deserialize::begin(&mut self.us_bank_transfer),
                "wechat_pay" => Deserialize::begin(&mut self.wechat_pay),
                "zip" => Deserialize::begin(&mut self.zip),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                affirm: Deserialize::default(),
                afterpay_clearpay: Deserialize::default(),
                alipay: Deserialize::default(),
                amazon_pay: Deserialize::default(),
                au_bank_transfer: Deserialize::default(),
                blik: Deserialize::default(),
                br_bank_transfer: Deserialize::default(),
                card: Deserialize::default(),
                cashapp: Deserialize::default(),
                customer_cash_balance: Deserialize::default(),
                eps: Deserialize::default(),
                eu_bank_transfer: Deserialize::default(),
                gb_bank_transfer: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                jp_bank_transfer: Deserialize::default(),
                klarna: Deserialize::default(),
                mx_bank_transfer: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                revolut: Deserialize::default(),
                sofort: Deserialize::default(),
                swish: Deserialize::default(),
                th_bank_transfer: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_transfer: Deserialize::default(),
                wechat_pay: Deserialize::default(),
                zip: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                affirm: self.affirm?,
                afterpay_clearpay: self.afterpay_clearpay?,
                alipay: self.alipay?,
                amazon_pay: self.amazon_pay?,
                au_bank_transfer: self.au_bank_transfer?,
                blik: self.blik.take()?,
                br_bank_transfer: self.br_bank_transfer.take()?,
                card: self.card.take()?,
                cashapp: self.cashapp?,
                customer_cash_balance: self.customer_cash_balance?,
                eps: self.eps?,
                eu_bank_transfer: self.eu_bank_transfer.take()?,
                gb_bank_transfer: self.gb_bank_transfer.take()?,
                giropay: self.giropay?,
                grabpay: self.grabpay?,
                jp_bank_transfer: self.jp_bank_transfer.take()?,
                klarna: self.klarna?,
                mx_bank_transfer: self.mx_bank_transfer.take()?,
                p24: self.p24.take()?,
                paynow: self.paynow?,
                paypal: self.paypal?,
                pix: self.pix?,
                revolut: self.revolut?,
                sofort: self.sofort?,
                swish: self.swish.take()?,
                th_bank_transfer: self.th_bank_transfer.take()?,
                type_: self.type_.take()?,
                us_bank_transfer: self.us_bank_transfer.take()?,
                wechat_pay: self.wechat_pay?,
                zip: self.zip?,
            })
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

    impl ObjectDeser for RefundDestinationDetails {
        type Builder = RefundDestinationDetailsBuilder;
    }

    impl FromValueOpt for RefundDestinationDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RefundDestinationDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "affirm" => b.affirm = Some(FromValueOpt::from_value(v)?),
                    "afterpay_clearpay" => b.afterpay_clearpay = Some(FromValueOpt::from_value(v)?),
                    "alipay" => b.alipay = Some(FromValueOpt::from_value(v)?),
                    "amazon_pay" => b.amazon_pay = Some(FromValueOpt::from_value(v)?),
                    "au_bank_transfer" => b.au_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "blik" => b.blik = Some(FromValueOpt::from_value(v)?),
                    "br_bank_transfer" => b.br_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "cashapp" => b.cashapp = Some(FromValueOpt::from_value(v)?),
                    "customer_cash_balance" => {
                        b.customer_cash_balance = Some(FromValueOpt::from_value(v)?)
                    }
                    "eps" => b.eps = Some(FromValueOpt::from_value(v)?),
                    "eu_bank_transfer" => b.eu_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "gb_bank_transfer" => b.gb_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "giropay" => b.giropay = Some(FromValueOpt::from_value(v)?),
                    "grabpay" => b.grabpay = Some(FromValueOpt::from_value(v)?),
                    "jp_bank_transfer" => b.jp_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "klarna" => b.klarna = Some(FromValueOpt::from_value(v)?),
                    "mx_bank_transfer" => b.mx_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "p24" => b.p24 = Some(FromValueOpt::from_value(v)?),
                    "paynow" => b.paynow = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "pix" => b.pix = Some(FromValueOpt::from_value(v)?),
                    "revolut" => b.revolut = Some(FromValueOpt::from_value(v)?),
                    "sofort" => b.sofort = Some(FromValueOpt::from_value(v)?),
                    "swish" => b.swish = Some(FromValueOpt::from_value(v)?),
                    "th_bank_transfer" => b.th_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "us_bank_transfer" => b.us_bank_transfer = Some(FromValueOpt::from_value(v)?),
                    "wechat_pay" => b.wechat_pay = Some(FromValueOpt::from_value(v)?),
                    "zip" => b.zip = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
