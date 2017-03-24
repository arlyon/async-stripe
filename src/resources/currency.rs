use params::to_snakecase;
use std::fmt;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Currency {
	#[serde(rename = "aed")] AED, // United Arab Emirates Dirham
	#[serde(rename = "afn")] AFN, // Afghan Afghani
	#[serde(rename = "all")] ALL, // Albanian Lek
	#[serde(rename = "amd")] AMD, // Armenian Dram
	#[serde(rename = "ang")] ANG, // Netherlands Antillean Gulden
	#[serde(rename = "aoa")] AOA, // Angolan Kwanza
	#[serde(rename = "ars")] ARS, // Argentine Peso
	#[serde(rename = "aud")] AUD, // Australian Dollar
	#[serde(rename = "awg")] AWG, // Aruban Florin
	#[serde(rename = "azn")] AZN, // Azerbaijani Manat
	#[serde(rename = "bam")] BAM, // Bosnia & Herzegovina Convertible Mark
	#[serde(rename = "bbd")] BBD, // Barbadian Dollar
	#[serde(rename = "bdt")] BDT, // Bangladeshi Taka
	#[serde(rename = "bgn")] BGN, // Bulgarian Lev
	#[serde(rename = "bif")] BIF, // Burundian Franc
	#[serde(rename = "bmd")] BMD, // Bermudian Dollar
	#[serde(rename = "bnd")] BND, // Brunei Dollar
	#[serde(rename = "bob")] BOB, // Bolivian Boliviano
	#[serde(rename = "brl")] BRL, // Brazilian Real
	#[serde(rename = "bsd")] BSD, // Bahamian Dollar
	#[serde(rename = "bwp")] BWP, // Botswana Pula
	#[serde(rename = "bzd")] BZD, // Belize Dollar
	#[serde(rename = "cad")] CAD, // Canadian Dollar
	#[serde(rename = "cdf")] CDF, // Congolese Franc
	#[serde(rename = "chf")] CHF, // Swiss Franc
	#[serde(rename = "clp")] CLP, // Chilean Peso
	#[serde(rename = "cny")] CNY, // Chinese Renminbi Yuan
	#[serde(rename = "cop")] COP, // Colombian Peso
	#[serde(rename = "crc")] CRC, // Costa Rican Colón
	#[serde(rename = "cve")] CVE, // Cape Verdean Escudo
	#[serde(rename = "czk")] CZK, // Czech Koruna
	#[serde(rename = "djf")] DJF, // Djiboutian Franc
	#[serde(rename = "dkk")] DKK, // Danish Krone
	#[serde(rename = "dop")] DOP, // Dominican Peso
	#[serde(rename = "dzd")] DZD, // Algerian Dinar
	#[serde(rename = "eek")] EEK, // Estonian Kroon
	#[serde(rename = "egp")] EGP, // Egyptian Pound
	#[serde(rename = "etb")] ETB, // Ethiopian Birr
	#[serde(rename = "eur")] EUR, // Euro
	#[serde(rename = "fjd")] FJD, // Fijian Dollar
	#[serde(rename = "fkp")] FKP, // Falkland Islands Pound
	#[serde(rename = "gbp")] GBP, // British Pound
	#[serde(rename = "gel")] GEL, // Georgian Lari
	#[serde(rename = "gip")] GIP, // Gibraltar Pound
	#[serde(rename = "gmd")] GMD, // Gambian Dalasi
	#[serde(rename = "gnf")] GNF, // Guinean Franc
	#[serde(rename = "gtq")] GTQ, // Guatemalan Quetzal
	#[serde(rename = "gyd")] GYD, // Guyanese Dollar
	#[serde(rename = "hkd")] HKD, // Hong Kong Dollar
	#[serde(rename = "hnl")] HNL, // Honduran Lempira
	#[serde(rename = "hrk")] HRK, // Croatian Kuna
	#[serde(rename = "htg")] HTG, // Haitian Gourde
	#[serde(rename = "huf")] HUF, // Hungarian Forint
	#[serde(rename = "idr")] IDR, // Indonesian Rupiah
	#[serde(rename = "ils")] ILS, // Israeli New Sheqel
	#[serde(rename = "inr")] INR, // Indian Rupee
	#[serde(rename = "isk")] ISK, // Icelandic Króna
	#[serde(rename = "jmd")] JMD, // Jamaican Dollar
	#[serde(rename = "jpy")] JPY, // Japanese Yen
	#[serde(rename = "kes")] KES, // Kenyan Shilling
	#[serde(rename = "kgs")] KGS, // Kyrgyzstani Som
	#[serde(rename = "khr")] KHR, // Cambodian Riel
	#[serde(rename = "kmf")] KMF, // Comorian Franc
	#[serde(rename = "krw")] KRW, // South Korean Won
	#[serde(rename = "kyd")] KYD, // Cayman Islands Dollar
	#[serde(rename = "kzt")] KZT, // Kazakhstani Tenge
	#[serde(rename = "lak")] LAK, // Lao Kip
	#[serde(rename = "lbp")] LBP, // Lebanese Pound
	#[serde(rename = "lkr")] LKR, // Sri Lankan Rupee
	#[serde(rename = "lrd")] LRD, // Liberian Dollar
	#[serde(rename = "lsl")] LSL, // Lesotho Loti
	#[serde(rename = "ltl")] LTL, // Lithuanian Litas
	#[serde(rename = "lvl")] LVL, // Latvian Lats
	#[serde(rename = "mad")] MAD, // Moroccan Dirham
	#[serde(rename = "mdl")] MDL, // Moldovan Leu
	#[serde(rename = "mga")] MGA, // Malagasy Ariary
	#[serde(rename = "mkd")] MKD, // Macedonian Denar
	#[serde(rename = "mnt")] MNT, // Mongolian Tögrög
	#[serde(rename = "mop")] MOP, // Macanese Pataca
	#[serde(rename = "mro")] MRO, // Mauritanian Ouguiya
	#[serde(rename = "mur")] MUR, // Mauritian Rupee
	#[serde(rename = "mvr")] MVR, // Maldivian Rufiyaa
	#[serde(rename = "mwk")] MWK, // Malawian Kwacha
	#[serde(rename = "mxn")] MXN, // Mexican Peso
	#[serde(rename = "myr")] MYR, // Malaysian Ringgit
	#[serde(rename = "mzn")] MZN, // Mozambican Metical
	#[serde(rename = "nad")] NAD, // Namibian Dollar
	#[serde(rename = "ngn")] NGN, // Nigerian Naira
	#[serde(rename = "nio")] NIO, // Nicaraguan Córdoba
	#[serde(rename = "nok")] NOK, // Norwegian Krone
	#[serde(rename = "npr")] NPR, // Nepalese Rupee
	#[serde(rename = "nzd")] NZD, // New Zealand Dollar
	#[serde(rename = "pab")] PAB, // Panamanian Balboa
	#[serde(rename = "pen")] PEN, // Peruvian Nuevo Sol
	#[serde(rename = "pgk")] PGK, // Papua New Guinean Kina
	#[serde(rename = "php")] PHP, // Philippine Peso
	#[serde(rename = "pkr")] PKR, // Pakistani Rupee
	#[serde(rename = "pln")] PLN, // Polish Złoty
	#[serde(rename = "pyg")] PYG, // Paraguayan Guaraní
	#[serde(rename = "qar")] QAR, // Qatari Riyal
	#[serde(rename = "ron")] RON, // Romanian Leu
	#[serde(rename = "rsd")] RSD, // Serbian Dinar
	#[serde(rename = "rub")] RUB, // Russian Ruble
	#[serde(rename = "rwf")] RWF, // Rwandan Franc
	#[serde(rename = "sar")] SAR, // Saudi Riyal
	#[serde(rename = "sbd")] SBD, // Solomon Islands Dollar
	#[serde(rename = "scr")] SCR, // Seychellois Rupee
	#[serde(rename = "sek")] SEK, // Swedish Krona
	#[serde(rename = "sgd")] SGD, // Singapore Dollar
	#[serde(rename = "shp")] SHP, // Saint Helenian Pound
	#[serde(rename = "sll")] SLL, // Sierra Leonean Leone
	#[serde(rename = "sos")] SOS, // Somali Shilling
	#[serde(rename = "srd")] SRD, // Surinamese Dollar
	#[serde(rename = "std")] STD, // São Tomé and Príncipe Dobra
	#[serde(rename = "svc")] SVC, // Salvadoran Colón
	#[serde(rename = "szl")] SZL, // Swazi Lilangeni
	#[serde(rename = "thb")] THB, // Thai Baht
	#[serde(rename = "tjs")] TJS, // Tajikistani Somoni
	#[serde(rename = "top")] TOP, // Tongan Paʻanga
	#[serde(rename = "try")] TRY, // Turkish Lira
	#[serde(rename = "ttd")] TTD, // Trinidad and Tobago Dollar
	#[serde(rename = "twd")] TWD, // New Taiwan Dollar
	#[serde(rename = "tzs")] TZS, // Tanzanian Shilling
	#[serde(rename = "uah")] UAH, // Ukrainian Hryvnia
	#[serde(rename = "ugx")] UGX, // Ugandan Shilling
	#[serde(rename = "usd")] USD, // United States Dollar
	#[serde(rename = "uyu")] UYU, // Uruguayan Peso
	#[serde(rename = "uzs")] UZS, // Uzbekistani Som
	#[serde(rename = "vef")] VEF, // Venezuelan Bolívar
	#[serde(rename = "vnd")] VND, // Vietnamese Đồng
	#[serde(rename = "vuv")] VUV, // Vanuatu Vatu
	#[serde(rename = "wst")] WST, // Samoan Tala
	#[serde(rename = "xaf")] XAF, // Central African Cfa Franc
	#[serde(rename = "xcd")] XCD, // East Caribbean Dollar
	#[serde(rename = "xof")] XOF, // West African Cfa Franc
	#[serde(rename = "xpf")] XPF, // Cfp Franc
	#[serde(rename = "yer")] YER, // Yemeni Rial
	#[serde(rename = "zar")] ZAR, // South African Rand
	#[serde(rename = "zmw")] ZMW, // Zambian Kwacha
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}", self)))
    }
}
