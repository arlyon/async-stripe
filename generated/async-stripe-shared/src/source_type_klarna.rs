#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeKlarna {
    pub background_image_url: Option<String>,
    pub client_token: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub locale: Option<String>,
    pub logo_url: Option<String>,
    pub page_title: Option<String>,
    pub pay_later_asset_urls_descriptive: Option<String>,
    pub pay_later_asset_urls_standard: Option<String>,
    pub pay_later_name: Option<String>,
    pub pay_later_redirect_url: Option<String>,
    pub pay_now_asset_urls_descriptive: Option<String>,
    pub pay_now_asset_urls_standard: Option<String>,
    pub pay_now_name: Option<String>,
    pub pay_now_redirect_url: Option<String>,
    pub pay_over_time_asset_urls_descriptive: Option<String>,
    pub pay_over_time_asset_urls_standard: Option<String>,
    pub pay_over_time_name: Option<String>,
    pub pay_over_time_redirect_url: Option<String>,
    pub payment_method_categories: Option<String>,
    pub purchase_country: Option<String>,
    pub purchase_type: Option<String>,
    pub redirect_url: Option<String>,
    pub shipping_delay: Option<i64>,
    pub shipping_first_name: Option<String>,
    pub shipping_last_name: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeKlarnaBuilder {
    background_image_url: Option<Option<String>>,
    client_token: Option<Option<String>>,
    first_name: Option<Option<String>>,
    last_name: Option<Option<String>>,
    locale: Option<Option<String>>,
    logo_url: Option<Option<String>>,
    page_title: Option<Option<String>>,
    pay_later_asset_urls_descriptive: Option<Option<String>>,
    pay_later_asset_urls_standard: Option<Option<String>>,
    pay_later_name: Option<Option<String>>,
    pay_later_redirect_url: Option<Option<String>>,
    pay_now_asset_urls_descriptive: Option<Option<String>>,
    pay_now_asset_urls_standard: Option<Option<String>>,
    pay_now_name: Option<Option<String>>,
    pay_now_redirect_url: Option<Option<String>>,
    pay_over_time_asset_urls_descriptive: Option<Option<String>>,
    pay_over_time_asset_urls_standard: Option<Option<String>>,
    pay_over_time_name: Option<Option<String>>,
    pay_over_time_redirect_url: Option<Option<String>>,
    payment_method_categories: Option<Option<String>>,
    purchase_country: Option<Option<String>>,
    purchase_type: Option<Option<String>>,
    redirect_url: Option<Option<String>>,
    shipping_delay: Option<Option<i64>>,
    shipping_first_name: Option<Option<String>>,
    shipping_last_name: Option<Option<String>>,
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

    impl Deserialize for SourceTypeKlarna {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeKlarna>,
        builder: SourceTypeKlarnaBuilder,
    }

    impl Visitor for Place<SourceTypeKlarna> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeKlarnaBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeKlarnaBuilder {
        type Out = SourceTypeKlarna;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "background_image_url" => Deserialize::begin(&mut self.background_image_url),
                "client_token" => Deserialize::begin(&mut self.client_token),
                "first_name" => Deserialize::begin(&mut self.first_name),
                "last_name" => Deserialize::begin(&mut self.last_name),
                "locale" => Deserialize::begin(&mut self.locale),
                "logo_url" => Deserialize::begin(&mut self.logo_url),
                "page_title" => Deserialize::begin(&mut self.page_title),
                "pay_later_asset_urls_descriptive" => {
                    Deserialize::begin(&mut self.pay_later_asset_urls_descriptive)
                }
                "pay_later_asset_urls_standard" => {
                    Deserialize::begin(&mut self.pay_later_asset_urls_standard)
                }
                "pay_later_name" => Deserialize::begin(&mut self.pay_later_name),
                "pay_later_redirect_url" => Deserialize::begin(&mut self.pay_later_redirect_url),
                "pay_now_asset_urls_descriptive" => {
                    Deserialize::begin(&mut self.pay_now_asset_urls_descriptive)
                }
                "pay_now_asset_urls_standard" => {
                    Deserialize::begin(&mut self.pay_now_asset_urls_standard)
                }
                "pay_now_name" => Deserialize::begin(&mut self.pay_now_name),
                "pay_now_redirect_url" => Deserialize::begin(&mut self.pay_now_redirect_url),
                "pay_over_time_asset_urls_descriptive" => {
                    Deserialize::begin(&mut self.pay_over_time_asset_urls_descriptive)
                }
                "pay_over_time_asset_urls_standard" => {
                    Deserialize::begin(&mut self.pay_over_time_asset_urls_standard)
                }
                "pay_over_time_name" => Deserialize::begin(&mut self.pay_over_time_name),
                "pay_over_time_redirect_url" => {
                    Deserialize::begin(&mut self.pay_over_time_redirect_url)
                }
                "payment_method_categories" => {
                    Deserialize::begin(&mut self.payment_method_categories)
                }
                "purchase_country" => Deserialize::begin(&mut self.purchase_country),
                "purchase_type" => Deserialize::begin(&mut self.purchase_type),
                "redirect_url" => Deserialize::begin(&mut self.redirect_url),
                "shipping_delay" => Deserialize::begin(&mut self.shipping_delay),
                "shipping_first_name" => Deserialize::begin(&mut self.shipping_first_name),
                "shipping_last_name" => Deserialize::begin(&mut self.shipping_last_name),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                background_image_url: Deserialize::default(),
                client_token: Deserialize::default(),
                first_name: Deserialize::default(),
                last_name: Deserialize::default(),
                locale: Deserialize::default(),
                logo_url: Deserialize::default(),
                page_title: Deserialize::default(),
                pay_later_asset_urls_descriptive: Deserialize::default(),
                pay_later_asset_urls_standard: Deserialize::default(),
                pay_later_name: Deserialize::default(),
                pay_later_redirect_url: Deserialize::default(),
                pay_now_asset_urls_descriptive: Deserialize::default(),
                pay_now_asset_urls_standard: Deserialize::default(),
                pay_now_name: Deserialize::default(),
                pay_now_redirect_url: Deserialize::default(),
                pay_over_time_asset_urls_descriptive: Deserialize::default(),
                pay_over_time_asset_urls_standard: Deserialize::default(),
                pay_over_time_name: Deserialize::default(),
                pay_over_time_redirect_url: Deserialize::default(),
                payment_method_categories: Deserialize::default(),
                purchase_country: Deserialize::default(),
                purchase_type: Deserialize::default(),
                redirect_url: Deserialize::default(),
                shipping_delay: Deserialize::default(),
                shipping_first_name: Deserialize::default(),
                shipping_last_name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(background_image_url),
                Some(client_token),
                Some(first_name),
                Some(last_name),
                Some(locale),
                Some(logo_url),
                Some(page_title),
                Some(pay_later_asset_urls_descriptive),
                Some(pay_later_asset_urls_standard),
                Some(pay_later_name),
                Some(pay_later_redirect_url),
                Some(pay_now_asset_urls_descriptive),
                Some(pay_now_asset_urls_standard),
                Some(pay_now_name),
                Some(pay_now_redirect_url),
                Some(pay_over_time_asset_urls_descriptive),
                Some(pay_over_time_asset_urls_standard),
                Some(pay_over_time_name),
                Some(pay_over_time_redirect_url),
                Some(payment_method_categories),
                Some(purchase_country),
                Some(purchase_type),
                Some(redirect_url),
                Some(shipping_delay),
                Some(shipping_first_name),
                Some(shipping_last_name),
            ) = (
                self.background_image_url.take(),
                self.client_token.take(),
                self.first_name.take(),
                self.last_name.take(),
                self.locale.take(),
                self.logo_url.take(),
                self.page_title.take(),
                self.pay_later_asset_urls_descriptive.take(),
                self.pay_later_asset_urls_standard.take(),
                self.pay_later_name.take(),
                self.pay_later_redirect_url.take(),
                self.pay_now_asset_urls_descriptive.take(),
                self.pay_now_asset_urls_standard.take(),
                self.pay_now_name.take(),
                self.pay_now_redirect_url.take(),
                self.pay_over_time_asset_urls_descriptive.take(),
                self.pay_over_time_asset_urls_standard.take(),
                self.pay_over_time_name.take(),
                self.pay_over_time_redirect_url.take(),
                self.payment_method_categories.take(),
                self.purchase_country.take(),
                self.purchase_type.take(),
                self.redirect_url.take(),
                self.shipping_delay,
                self.shipping_first_name.take(),
                self.shipping_last_name.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                background_image_url,
                client_token,
                first_name,
                last_name,
                locale,
                logo_url,
                page_title,
                pay_later_asset_urls_descriptive,
                pay_later_asset_urls_standard,
                pay_later_name,
                pay_later_redirect_url,
                pay_now_asset_urls_descriptive,
                pay_now_asset_urls_standard,
                pay_now_name,
                pay_now_redirect_url,
                pay_over_time_asset_urls_descriptive,
                pay_over_time_asset_urls_standard,
                pay_over_time_name,
                pay_over_time_redirect_url,
                payment_method_categories,
                purchase_country,
                purchase_type,
                redirect_url,
                shipping_delay,
                shipping_first_name,
                shipping_last_name,
            })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for SourceTypeKlarna {
        type Builder = SourceTypeKlarnaBuilder;
    }

    impl FromValueOpt for SourceTypeKlarna {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeKlarnaBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "background_image_url" => b.background_image_url = FromValueOpt::from_value(v),
                    "client_token" => b.client_token = FromValueOpt::from_value(v),
                    "first_name" => b.first_name = FromValueOpt::from_value(v),
                    "last_name" => b.last_name = FromValueOpt::from_value(v),
                    "locale" => b.locale = FromValueOpt::from_value(v),
                    "logo_url" => b.logo_url = FromValueOpt::from_value(v),
                    "page_title" => b.page_title = FromValueOpt::from_value(v),
                    "pay_later_asset_urls_descriptive" => {
                        b.pay_later_asset_urls_descriptive = FromValueOpt::from_value(v)
                    }
                    "pay_later_asset_urls_standard" => {
                        b.pay_later_asset_urls_standard = FromValueOpt::from_value(v)
                    }
                    "pay_later_name" => b.pay_later_name = FromValueOpt::from_value(v),
                    "pay_later_redirect_url" => {
                        b.pay_later_redirect_url = FromValueOpt::from_value(v)
                    }
                    "pay_now_asset_urls_descriptive" => {
                        b.pay_now_asset_urls_descriptive = FromValueOpt::from_value(v)
                    }
                    "pay_now_asset_urls_standard" => {
                        b.pay_now_asset_urls_standard = FromValueOpt::from_value(v)
                    }
                    "pay_now_name" => b.pay_now_name = FromValueOpt::from_value(v),
                    "pay_now_redirect_url" => b.pay_now_redirect_url = FromValueOpt::from_value(v),
                    "pay_over_time_asset_urls_descriptive" => {
                        b.pay_over_time_asset_urls_descriptive = FromValueOpt::from_value(v)
                    }
                    "pay_over_time_asset_urls_standard" => {
                        b.pay_over_time_asset_urls_standard = FromValueOpt::from_value(v)
                    }
                    "pay_over_time_name" => b.pay_over_time_name = FromValueOpt::from_value(v),
                    "pay_over_time_redirect_url" => {
                        b.pay_over_time_redirect_url = FromValueOpt::from_value(v)
                    }
                    "payment_method_categories" => {
                        b.payment_method_categories = FromValueOpt::from_value(v)
                    }
                    "purchase_country" => b.purchase_country = FromValueOpt::from_value(v),
                    "purchase_type" => b.purchase_type = FromValueOpt::from_value(v),
                    "redirect_url" => b.redirect_url = FromValueOpt::from_value(v),
                    "shipping_delay" => b.shipping_delay = FromValueOpt::from_value(v),
                    "shipping_first_name" => b.shipping_first_name = FromValueOpt::from_value(v),
                    "shipping_last_name" => b.shipping_last_name = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
