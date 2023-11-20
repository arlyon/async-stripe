#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsSepaDebit {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Branch code of bank associated with the bank account.
    pub branch_code: Option<String>,
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
    /// Uniquely identifies this particular bank account.
    ///
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four characters of the IBAN.
    pub last4: Option<String>,
    /// Find the ID of the mandate used for this payment under the [payment_method_details.sepa_debit.mandate](https://stripe.com/docs/api/charges/object#charge_object-payment_method_details-sepa_debit-mandate) property on the Charge.
    ///
    /// Use this mandate ID to [retrieve the Mandate](https://stripe.com/docs/api/mandates/retrieve).
    pub mandate: Option<String>,
}
