use serde::ser::SerializeStruct;

#[derive(Debug, Deserialize /*, Serialize */)]
pub struct CardParams<'a> {
    pub exp_month: &'a str, // eg. "12"
    pub exp_year: &'a str, // eg. "17" or 2017"

    pub number: &'a str, // card number
    pub name: Option<&'a str>, // cardholder's full name
    pub cvc: Option<&'a str>, // card security code
}

impl<'a> Default for CardParams<'a> {
    fn default() -> Self {
        CardParams {
            exp_month: "",
            exp_year: "",
            number: "",
            name: None,
            cvc: None,
        }
    }
}

impl<'a> ::serde::Serialize for CardParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::ser::Serializer
    {
        let mut s = serializer.serialize_struct("CardParams", 6)?;
        s.serialize_field("object", "card")?;
        s.serialize_field("exp_month", &self.exp_month)?;
        s.serialize_field("exp_year", &self.exp_year)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("cvc", &self.cvc)?;
        s.end()
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub brand: String, // (Visa, American Express, MasterCard, Discover, JCB, Diners Club, or Unknown)
    pub country: String, // eg. "US"
    pub customer: Option<String>,
    pub cvc_check: Option<String>, // (pass, fail, unavailable, unchecked)
    pub exp_month: u32,
    pub exp_year: u32,
    pub fingerprint: String,
    pub funding: String, // (credit, debit, prepaid, unknown)
    pub last4: String,
}
