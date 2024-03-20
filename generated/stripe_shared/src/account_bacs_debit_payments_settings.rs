#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountBacsDebitPaymentsSettings {
    /// The Bacs Direct Debit display name for this account.
    /// For payments made with Bacs Direct Debit, this name appears on the mandate as the statement descriptor.
    /// Mobile banking apps display it as the name of the business.
    /// To use custom branding, set the Bacs Direct Debit Display Name during or right after creation.
    /// Custom branding incurs an additional monthly fee for the platform.
    /// The fee appears 5 business days after requesting Bacs.
    /// If you don't set the display name before requesting Bacs capability, it's automatically set as "Stripe" and the account is onboarded to Stripe branding, which is free.
    pub display_name: Option<String>,
    /// The Bacs Direct Debit Service user number for this account.
    /// For payments made with Bacs Direct Debit, this number is a unique identifier of the account with our banking partners.
    pub service_user_number: Option<String>,
}
