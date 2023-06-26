impl stripe_core::radar::early_fraud_warning::EarlyFraudWarning {
    /// Returns a list of early fraud warnings.
    pub fn list(
        client: &stripe::Client,
        params: ListEarlyFraudWarning,
    ) -> stripe::Response<
        stripe_types::List<stripe_core::radar::early_fraud_warning::EarlyFraudWarning>,
    > {
        client.get_query("/radar/early_fraud_warnings", params)
    }
    /// Retrieves the details of an early fraud warning that has previously been created.
    ///
    /// Please refer to the [early fraud warning](https://stripe.com/docs/api#early_fraud_warning_object) object reference for more details.
    pub fn retrieve(
        client: &stripe::Client,
        early_fraud_warning: &str,
        params: RetrieveEarlyFraudWarning,
    ) -> stripe::Response<stripe_core::radar::early_fraud_warning::EarlyFraudWarning> {
        client.get_query(
            &format!(
                "/radar/early_fraud_warnings/{early_fraud_warning}",
                early_fraud_warning = early_fraud_warning
            ),
            params,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListEarlyFraudWarning<'a> {
    /// Only return early fraud warnings for the charge specified by this charge ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Only return early fraud warnings for charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
impl<'a> ListEarlyFraudWarning<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveEarlyFraudWarning<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveEarlyFraudWarning<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
