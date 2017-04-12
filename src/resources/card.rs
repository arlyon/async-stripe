#[derive(Serialize)]
pub struct CardParams<'a> {
    pub object: &'static str, // must be "card"
    pub exp_month: &'a str,    // eg. "12"
    pub exp_year: &'a str,     // eg. "17" or 2017"

    pub number: &'a str,       // card number
    pub name: Option<&'a str>, // cardholder's full name
    pub cvc: Option<&'a str>,  // card security code
}

impl<'a> Default for CardParams<'a> {
    fn default() -> Self {
        CardParams{
            object: "card",
            exp_month: "",
            exp_year: "",
            number: "",
            name: None,
            cvc: None,
        }
    }
}

#[derive(Debug, Deserialize)]
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
