#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AllowedPaymentMethodTypes {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    BokuPromptpay,
    Boleto,
    CapchasePay,
    Card,
    Cashapp,
    CheckScan,
    ClickToPay,
    Crypto,
    CustomerBalance,
    DemoPay,
    Duitnow,
    DummyAuthPush,
    DummyPassthroughCard,
    Edenred,
    Eps,
    Fpx,
    Gcash,
    Getbalance,
    GiftCard,
    Giropay,
    Gopay,
    Grabpay,
    IdBankTransfer,
    Ideal,
    KakaoPay,
    Klarna,
    Knet,
    Konbini,
    KrCard,
    KrMarket,
    Kriya,
    Link,
    MbWay,
    Mobilepay,
    Momo,
    Mondu,
    Multibanco,
    NaverPay,
    Netbanking,
    NgBank,
    NgBankTransfer,
    NgCard,
    NgMarket,
    NgUssd,
    NgWallet,
    NzBankAccount,
    Octopus,
    Oxxo,
    P24,
    PaperCheck,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    Qris,
    Rechnung,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sequra,
    ShopPay,
    Shopeepay,
    Sofort,
    SouthKoreaMarket,
    StripeBalance,
    Sunbit,
    Swish,
    Tamara,
    TestPay,
    TouchNGo,
    Truemoney,
    Twint,
    Upi,
    UsBankAccount,
    UsCashVoucher,
    Vipps,
    WechatPay,
    Wero,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl AllowedPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use AllowedPaymentMethodTypes::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            Alma => "alma",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Billie => "billie",
            Bizum => "bizum",
            Blik => "blik",
            BokuPromptpay => "boku_promptpay",
            Boleto => "boleto",
            CapchasePay => "capchase_pay",
            Card => "card",
            Cashapp => "cashapp",
            CheckScan => "check_scan",
            ClickToPay => "click_to_pay",
            Crypto => "crypto",
            CustomerBalance => "customer_balance",
            DemoPay => "demo_pay",
            Duitnow => "duitnow",
            DummyAuthPush => "dummy_auth_push",
            DummyPassthroughCard => "dummy_passthrough_card",
            Edenred => "edenred",
            Eps => "eps",
            Fpx => "fpx",
            Gcash => "gcash",
            Getbalance => "getbalance",
            GiftCard => "gift_card",
            Giropay => "giropay",
            Gopay => "gopay",
            Grabpay => "grabpay",
            IdBankTransfer => "id_bank_transfer",
            Ideal => "ideal",
            KakaoPay => "kakao_pay",
            Klarna => "klarna",
            Knet => "knet",
            Konbini => "konbini",
            KrCard => "kr_card",
            KrMarket => "kr_market",
            Kriya => "kriya",
            Link => "link",
            MbWay => "mb_way",
            Mobilepay => "mobilepay",
            Momo => "momo",
            Mondu => "mondu",
            Multibanco => "multibanco",
            NaverPay => "naver_pay",
            Netbanking => "netbanking",
            NgBank => "ng_bank",
            NgBankTransfer => "ng_bank_transfer",
            NgCard => "ng_card",
            NgMarket => "ng_market",
            NgUssd => "ng_ussd",
            NgWallet => "ng_wallet",
            NzBankAccount => "nz_bank_account",
            Octopus => "octopus",
            Oxxo => "oxxo",
            P24 => "p24",
            PaperCheck => "paper_check",
            PayByBank => "pay_by_bank",
            Payco => "payco",
            Paynow => "paynow",
            Paypal => "paypal",
            Paypay => "paypay",
            Payto => "payto",
            Pix => "pix",
            Promptpay => "promptpay",
            Qris => "qris",
            Rechnung => "rechnung",
            RevolutPay => "revolut_pay",
            SamsungPay => "samsung_pay",
            Satispay => "satispay",
            Scalapay => "scalapay",
            SepaDebit => "sepa_debit",
            Sequra => "sequra",
            ShopPay => "shop_pay",
            Shopeepay => "shopeepay",
            Sofort => "sofort",
            SouthKoreaMarket => "south_korea_market",
            StripeBalance => "stripe_balance",
            Sunbit => "sunbit",
            Swish => "swish",
            Tamara => "tamara",
            TestPay => "test_pay",
            TouchNGo => "touch_n_go",
            Truemoney => "truemoney",
            Twint => "twint",
            Upi => "upi",
            UsBankAccount => "us_bank_account",
            UsCashVoucher => "us_cash_voucher",
            Vipps => "vipps",
            WechatPay => "wechat_pay",
            Wero => "wero",
            Zip => "zip",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for AllowedPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AllowedPaymentMethodTypes::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "alma" => Ok(Alma),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "billie" => Ok(Billie),
            "bizum" => Ok(Bizum),
            "blik" => Ok(Blik),
            "boku_promptpay" => Ok(BokuPromptpay),
            "boleto" => Ok(Boleto),
            "capchase_pay" => Ok(CapchasePay),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "check_scan" => Ok(CheckScan),
            "click_to_pay" => Ok(ClickToPay),
            "crypto" => Ok(Crypto),
            "customer_balance" => Ok(CustomerBalance),
            "demo_pay" => Ok(DemoPay),
            "duitnow" => Ok(Duitnow),
            "dummy_auth_push" => Ok(DummyAuthPush),
            "dummy_passthrough_card" => Ok(DummyPassthroughCard),
            "edenred" => Ok(Edenred),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "gcash" => Ok(Gcash),
            "getbalance" => Ok(Getbalance),
            "gift_card" => Ok(GiftCard),
            "giropay" => Ok(Giropay),
            "gopay" => Ok(Gopay),
            "grabpay" => Ok(Grabpay),
            "id_bank_transfer" => Ok(IdBankTransfer),
            "ideal" => Ok(Ideal),
            "kakao_pay" => Ok(KakaoPay),
            "klarna" => Ok(Klarna),
            "knet" => Ok(Knet),
            "konbini" => Ok(Konbini),
            "kr_card" => Ok(KrCard),
            "kr_market" => Ok(KrMarket),
            "kriya" => Ok(Kriya),
            "link" => Ok(Link),
            "mb_way" => Ok(MbWay),
            "mobilepay" => Ok(Mobilepay),
            "momo" => Ok(Momo),
            "mondu" => Ok(Mondu),
            "multibanco" => Ok(Multibanco),
            "naver_pay" => Ok(NaverPay),
            "netbanking" => Ok(Netbanking),
            "ng_bank" => Ok(NgBank),
            "ng_bank_transfer" => Ok(NgBankTransfer),
            "ng_card" => Ok(NgCard),
            "ng_market" => Ok(NgMarket),
            "ng_ussd" => Ok(NgUssd),
            "ng_wallet" => Ok(NgWallet),
            "nz_bank_account" => Ok(NzBankAccount),
            "octopus" => Ok(Octopus),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paper_check" => Ok(PaperCheck),
            "pay_by_bank" => Ok(PayByBank),
            "payco" => Ok(Payco),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "paypay" => Ok(Paypay),
            "payto" => Ok(Payto),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "qris" => Ok(Qris),
            "rechnung" => Ok(Rechnung),
            "revolut_pay" => Ok(RevolutPay),
            "samsung_pay" => Ok(SamsungPay),
            "satispay" => Ok(Satispay),
            "scalapay" => Ok(Scalapay),
            "sepa_debit" => Ok(SepaDebit),
            "sequra" => Ok(Sequra),
            "shop_pay" => Ok(ShopPay),
            "shopeepay" => Ok(Shopeepay),
            "sofort" => Ok(Sofort),
            "south_korea_market" => Ok(SouthKoreaMarket),
            "stripe_balance" => Ok(StripeBalance),
            "sunbit" => Ok(Sunbit),
            "swish" => Ok(Swish),
            "tamara" => Ok(Tamara),
            "test_pay" => Ok(TestPay),
            "touch_n_go" => Ok(TouchNGo),
            "truemoney" => Ok(Truemoney),
            "twint" => Ok(Twint),
            "upi" => Ok(Upi),
            "us_bank_account" => Ok(UsBankAccount),
            "us_cash_voucher" => Ok(UsCashVoucher),
            "vipps" => Ok(Vipps),
            "wechat_pay" => Ok(WechatPay),
            "wero" => Ok(Wero),
            "zip" => Ok(Zip),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "AllowedPaymentMethodTypes");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for AllowedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for AllowedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AllowedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(AllowedPaymentMethodTypes)).finish_non_exhaustive()
    }
}
impl serde::Serialize for AllowedPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for AllowedPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<AllowedPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(AllowedPaymentMethodTypes::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(AllowedPaymentMethodTypes);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for AllowedPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
