use std::collections::{HashMap, HashSet};

use reverse_geocoder::{Locations, ReverseGeocoder};
use harana_common::tracing::Level;
use harana_common::tracing::span;

use harana_common::anyhow::{Context, Result};
use harana_common::error_adapter::log_and_return_error_string;
use harana_common::lazy_static::lazy_static;
use harana_common::serde::{self, Deserialize, Serialize};

use crate::tokenizer::tokenize;

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_memory();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
    static ref COUNTRIES: HashMap<&'static str, &'static str> = HashMap::from([
        ("AF", "Afghanistan"),
        ("AL", "Albania"),
        ("DZ", "Algeria"),
        ("AS", "American Samoa"),
        ("AD", "Andorra"),
        ("AO", "Angola"),
        ("AI", "Anguilla"),
        ("AQ", "Antarctica"),
        ("AG", "Antigua and Barbuda"),
        ("AR", "Argentina"),
        ("AM", "Armenia"),
        ("AW", "Aruba"),
        ("AU", "Australia"),
        ("AT", "Austria"),
        ("AZ", "Azerbaijan"),
        ("BS", "Bahamas"),
        ("BH", "Bahrain"),
        ("BD", "Bangladesh"),
        ("BB", "Barbados"),
        ("BY", "Belarus"),
        ("BE", "Belgium"),
        ("BZ", "Belize"),
        ("BJ", "Benin"),
        ("BM", "Bermuda"),
        ("BT", "Bhutan"),
        ("BO", "Bolivia"),
        ("BQ", "Bonaire Sint Eustatius and Saba"),
        ("BA", "Bosnia and Herzegovina"),
        ("BW", "Botswana"),
        ("BV", "Bouvet Island"),
        ("BR", "Brazil"),
        ("IO", "British Indian Ocean Territory"),
        ("BN", "Brunei Darussalam"),
        ("BG", "Bulgaria"),
        ("BF", "Burkina Faso"),
        ("BI", "Burundi"),
        ("CV", "Cabo Verde"),
        ("KH", "Cambodia"),
        ("CM", "Cameroon"),
        ("CA", "Canada"),
        ("KY", "Cayman Islands"),
        ("CF", "Central African Republic"),
        ("TD", "Chad"),
        ("CL", "Chile"),
        ("CN", "China"),
        ("CX", "Christmas Island"),
        ("CC", "Cocos (Keeling) Islands"),
        ("CO", "Colombia"),
        ("KM", "Comoros"),
        ("CD", "Congo Kinshasa"),
        ("CG", "Congo"),
        ("CK", "Cook Islands"),
        ("CR", "Costa Rica"),
        ("HR", "Croatia"),
        ("CU", "Cuba"),
        ("CW", "Curaçao"),
        ("CY", "Cyprus"),
        ("CZ", "Czechia"),
        ("CI", "Côte d'Ivoire"),
        ("DK", "Denmark"),
        ("DJ", "Djibouti"),
        ("DM", "Dominica"),
        ("DO", "Dominican Republic"),
        ("EC", "Ecuador"),
        ("EG", "Egypt"),
        ("SV", "El Salvador"),
        ("GQ", "Equatorial Guinea"),
        ("ER", "Eritrea"),
        ("EE", "Estonia"),
        ("SZ", "Eswatini"),
        ("ET", "Ethiopia"),
        ("FK", "Falkland Islands"),
        ("FO", "Faroe Islands"),
        ("FJ", "Fiji"),
        ("FI", "Finland"),
        ("FR", "France"),
        ("GF", "French Guiana"),
        ("PF", "French Polynesia"),
        ("TF", "French Southern Territories"),
        ("GA", "Gabon"),
        ("GM", "Gambia"),
        ("GE", "Georgia"),
        ("DE", "Germany"),
        ("GH", "Ghana"),
        ("GI", "Gibraltar"),
        ("GR", "Greece"),
        ("GL", "Greenland"),
        ("GD", "Grenada"),
        ("GP", "Guadeloupe"),
        ("GU", "Guam"),
        ("GT", "Guatemala"),
        ("GG", "Guernsey"),
        ("GN", "Guinea"),
        ("GW", "Guinea-Bissau"),
        ("GY", "Guyana"),
        ("HT", "Haiti"),
        ("HM", "Heard Island and McDonald Islands"),
        ("VA", "Holy See"),
        ("HN", "Honduras"),
        ("HK", "Hong Kong"),
        ("HU", "Hungary"),
        ("IS", "Iceland"),
        ("IN", "India"),
        ("ID", "Indonesia"),
        ("IR", "Iran"),
        ("IQ", "Iraq"),
        ("IE", "Ireland"),
        ("IM", "Isle of Man"),
        ("IL", "Israel"),
        ("IT", "Italy"),
        ("JM", "Jamaica"),
        ("JP", "Japan"),
        ("JE", "Jersey"),
        ("JO", "Jordan"),
        ("KZ", "Kazakhstan"),
        ("KE", "Kenya"),
        ("KI", "Kiribati"),
        ("KP", "North Korea"),
        ("KR", "South Korea"),
        ("KW", "Kuwait"),
        ("KG", "Kyrgyzstan"),
        ("LA", "Laos"),
        ("LV", "Latvia"),
        ("LB", "Lebanon"),
        ("LS", "Lesotho"),
        ("LR", "Liberia"),
        ("LY", "Libya"),
        ("LI", "Liechtenstein"),
        ("LT", "Lithuania"),
        ("LU", "Luxembourg"),
        ("MO", "Macao"),
        ("MG", "Madagascar"),
        ("MW", "Malawi"),
        ("MY", "Malaysia"),
        ("MV", "Maldives"),
        ("ML", "Mali"),
        ("MT", "Malta"),
        ("MH", "Marshall Islands"),
        ("MQ", "Martinique"),
        ("MR", "Mauritania"),
        ("MU", "Mauritius"),
        ("YT", "Mayotte"),
        ("MX", "Mexico"),
        ("FM", "Micronesia"),
        ("MD", "Moldova"),
        ("MC", "Monaco"),
        ("MN", "Mongolia"),
        ("ME", "Montenegro"),
        ("MS", "Montserrat"),
        ("MA", "Morocco"),
        ("MZ", "Mozambique"),
        ("MM", "Myanmar"),
        ("NA", "Namibia"),
        ("NR", "Nauru"),
        ("NP", "Nepal"),
        ("NL", "Netherlands"),
        ("NC", "New Caledonia"),
        ("NZ", "New Zealand"),
        ("NI", "Nicaragua"),
        ("NE", "Niger"),
        ("NG", "Nigeria"),
        ("NU", "Niue"),
        ("NF", "Norfolk Island"),
        ("MP", "Northern Mariana Islands"),
        ("NO", "Norway"),
        ("OM", "Oman"),
        ("PK", "Pakistan"),
        ("PW", "Palau"),
        ("PS", "Palestine"),
        ("PA", "Panama"),
        ("PG", "Papua New Guinea"),
        ("PY", "Paraguay"),
        ("PE", "Peru"),
        ("PH", "Philippines"),
        ("PN", "Pitcairn"),
        ("PL", "Poland"),
        ("PT", "Portugal"),
        ("PR", "Puerto Rico"),
        ("QA", "Qatar"),
        ("MK", "Republic of North Macedonia"),
        ("RO", "Romania"),
        ("RU", "Russian Federation"),
        ("RW", "Rwanda"),
        ("RE", "Réunion"),
        ("BL", "Saint Barthélemy"),
        ("SH", "Saint Helena"),
        ("KN", "Saint Kitts and Nevis"),
        ("LC", "Saint Lucia"),
        ("MF", "Saint Martin"),
        ("PM", "Saint Pierre and Miquelon"),
        ("VC", "Saint Vincent and the Grenadines"),
        ("WS", "Samoa"),
        ("SM", "San Marino"),
        ("ST", "Sao Tome and Principe"),
        ("SA", "Saudi Arabia"),
        ("SN", "Senegal"),
        ("RS", "Serbia"),
        ("SC", "Seychelles"),
        ("SL", "Sierra Leone"),
        ("SG", "Singapore"),
        ("SX", "Sint Maarten"),
        ("SK", "Slovakia"),
        ("SI", "Slovenia"),
        ("SB", "Solomon Islands"),
        ("SO", "Somalia"),
        ("ZA", "South Africa"),
        ("GS", "South Georgia"),
        ("SS", "South Sudan"),
        ("ES", "Spain"),
        ("LK", "Sri Lanka"),
        ("SD", "Sudan"),
        ("SR", "Suriname"),
        ("SJ", "Svalbard and Jan Mayen"),
        ("SE", "Sweden"),
        ("CH", "Switzerland"),
        ("SY", "Syrian Arab Republic"),
        ("TW", "Taiwan"),
        ("TJ", "Tajikistan"),
        ("TZ", "Tanzania"),
        ("TH", "Thailand"),
        ("TL", "Timor-Leste"),
        ("TG", "Togo"),
        ("TK", "Tokelau"),
        ("TO", "Tonga"),
        ("TT", "Trinidad and Tobago"),
        ("TN", "Tunisia"),
        ("TR", "Turkey"),
        ("TM", "Turkmenistan"),
        ("TC", "Turks and Caicos Islands"),
        ("TV", "Tuvalu"),
        ("UG", "Uganda"),
        ("UA", "Ukraine"),
        ("AE", "United Arab Emirates"),
        ("GB", "UK"),
        ("UM", "USA"),
        ("US", "USA"),
        ("UY", "Uruguay"),
        ("UZ", "Uzbekistan"),
        ("VU", "Vanuatu"),
        ("VE", "Venezuela"),
        ("VN", "Viet Nam"),
        ("VG", "Virgin Islands"),
        ("VI", "Virgin Islands"),
        ("WF", "Wallis and Futuna"),
        ("EH", "Western Sahara"),
        ("YE", "Yemen"),
        ("ZM", "Zambia"),
        ("ZW", "Zimbabwe"),
        ("AX", "Åland Islands")
    ]);
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    pub admin1: String,
    pub admin2: String,
    pub country_code: String,
    pub country: String
}

impl Location {
    pub fn tokens(&self) -> Vec<String> {
        let mut tokens = Vec::new();
        tokens.extend(tokenize(self.name.as_str()));
        tokens.extend(tokenize(self.admin1.as_str()));
        tokens.extend(tokenize(self.admin2.as_str()));
        tokens.extend(tokenize(self.country.as_str()));
        tokens
    }
}

pub fn geocode_tokens(latitude: f64, longitude: f64) -> HashSet<String> {
    match geocode(latitude, longitude) {
        Ok(location) => {
            let mut tokens = HashSet::new();
            tokens.insert(location.name);
            tokens.insert(location.admin1);
            tokens.insert(location.admin2);
            tokens.insert(location.country);
            tokens
        }
        Err(_) => HashSet::new()
    }
}

pub fn geocode(latitude: f64, longitude: f64) -> Result<Location> {
    span!(Level::INFO, "geocoder: Look up the coordinates").in_scope(|| -> Result<Location> {
        let record = GEOCODER.search((latitude, longitude))
            .with_context(|| log_and_return_error_string(
                format!("geocoder: Failed to search for location: ({:?}, {:?})", latitude, longitude))
            )?.record.clone();

        Ok(
            Location {
                latitude: record.lat,
                longitude: record.lon,
                name: record.name,
                admin1: record.admin1,
                admin2: record.admin2,
                country_code: record.cc.clone(),
                country: COUNTRIES.get(record.cc.as_str()).unwrap().to_string()
            }
        )
    })
}