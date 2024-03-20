/// A payment method domain represents a web domain that you have registered with Stripe.
/// Stripe Elements use registered payment method domains to control where certain payment methods are shown.
///
/// Related guides: [Payment method domains](https://stripe.com/docs/payments/payment-methods/pmd-registration).
///
/// For more details see <<https://stripe.com/docs/api/payment_method_domains/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDomain {
    pub apple_pay: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The domain name that this payment method domain object represents.
    pub domain_name: String,
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub enabled: bool,
    pub google_pay: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Unique identifier for the object.
    pub id: stripe_payment::PaymentMethodDomainId,
    pub link: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    pub paypal: stripe_payment::PaymentMethodDomainResourcePaymentMethodStatus,
}
impl stripe_types::Object for PaymentMethodDomain {
    type Id = stripe_payment::PaymentMethodDomainId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PaymentMethodDomainId);
