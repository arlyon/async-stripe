#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry,
}
/// The desired country code of the bank account information.
///
/// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    Be,
    De,
    Es,
    Fr,
    Ie,
    Nl,
}

impl InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    pub fn as_str(self) -> &'static str {
        use InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::*;
        match self {
            Be => "BE",
            De => "DE",
            Es => "ES",
            Fr => "FR",
            Ie => "IE",
            Nl => "NL",
        }
    }
}

impl std::str::FromStr
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::*;
        match s {
            "BE" => Ok(Be),
            "DE" => Ok(De),
            "ES" => Ok(Es),
            "FR" => Ok(Fr),
            "IE" => Ok(Ie),
            "NL" => Ok(Nl),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de>
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry"))
    }
}
