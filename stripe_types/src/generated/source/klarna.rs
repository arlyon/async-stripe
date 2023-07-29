#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Klarna {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_later_asset_urls_descriptive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_later_asset_urls_standard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_later_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_later_redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_now_asset_urls_descriptive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_now_asset_urls_standard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_now_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_now_redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_over_time_asset_urls_descriptive: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_over_time_asset_urls_standard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_over_time_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_over_time_redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_categories: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_delay: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_last_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Klarna {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
