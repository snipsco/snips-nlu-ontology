use std::ops::Range;

use failure::format_err;
use serde::Deserialize;
use serde_json;

use crate::enum_kind;
use crate::errors::*;
use crate::language::Language;
use crate::ontology::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BuiltinEntity {
    pub value: String,
    pub range: Range<usize>,
    pub entity: SlotValue,
    #[serde(
        serialize_with = "serialize_builtin_entity_kind",
        deserialize_with = "deserialize_builtin_entity_kind"
    )]
    pub entity_kind: BuiltinEntityKind,
}

fn serialize_builtin_entity_kind<S>(
    value: &BuiltinEntityKind,
    serializer: S,
) -> ::std::result::Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    serializer.serialize_str(value.identifier())
}

fn deserialize_builtin_entity_kind<'de, D>(
    deserializer: D,
) -> ::std::result::Result<BuiltinEntityKind, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    String::deserialize(deserializer)
        .and_then(|s| BuiltinEntityKind::from_identifier(&s).map_err(::serde::de::Error::custom))
}

enum_kind!(
    BuiltinEntityKind,
    [
        AmountOfMoney,
        Duration,
        Number,
        Ordinal,
        Temperature,
        Datetime,
        Date,
        Time,
        DatePeriod,
        TimePeriod,
        Percentage,
        MusicAlbum,
        MusicArtist,
        MusicTrack
    ]
);

pub trait IntoBuiltinEntityKind: Copy {
    fn into_builtin_kind(self) -> BuiltinEntityKind;

    fn identifier(&self) -> &'static str {
        self.into_builtin_kind().identifier()
    }

    fn description(&self) -> &'static str {
        self.into_builtin_kind().description()
    }

    fn examples(&self, language: Language) -> &'static [&'static str] {
        self.into_builtin_kind().examples(language)
    }

    fn result_description(&self) -> String {
        self.into_builtin_kind().result_description()
    }

    fn supported_languages(&self) -> &'static [Language] {
        self.into_builtin_kind().supported_languages()
    }

    fn ontology_details(&self, language: Language) -> BuiltinEntityKindDetails {
        self.into_builtin_kind().ontology_details(language)
    }
}

impl BuiltinEntityKind {
    pub fn identifier(&self) -> &'static str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "snips/amountOfMoney",
            BuiltinEntityKind::Duration => "snips/duration",
            BuiltinEntityKind::Number => "snips/number",
            BuiltinEntityKind::Ordinal => "snips/ordinal",
            BuiltinEntityKind::Temperature => "snips/temperature",
            BuiltinEntityKind::Datetime => "snips/datetime",
            BuiltinEntityKind::Date => "snips/date",
            BuiltinEntityKind::Time => "snips/time",
            BuiltinEntityKind::DatePeriod => "snips/datePeriod",
            BuiltinEntityKind::TimePeriod => "snips/timePeriod",
            BuiltinEntityKind::Percentage => "snips/percentage",
            BuiltinEntityKind::MusicAlbum => "snips/musicAlbum",
            BuiltinEntityKind::MusicArtist => "snips/musicArtist",
            BuiltinEntityKind::MusicTrack => "snips/musicTrack",
        }
    }

    pub fn from_identifier(identifier: &str) -> Result<Self> {
        BuiltinEntityKind::all()
            .iter()
            .find(|kind| kind.identifier() == identifier)
            .map(|k| k.clone())
            .ok_or(format_err!("Unknown EntityKind identifier: {}", identifier))
    }
}

impl BuiltinEntityKind {
    pub fn description(&self) -> &'static str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "Matches an amount of money",
            BuiltinEntityKind::Duration => "Matches a date/time duration",
            BuiltinEntityKind::Number => "Matches a cardinal number",
            BuiltinEntityKind::Ordinal => "Matches an ordinal number",
            BuiltinEntityKind::Temperature => "Matches a temperature",
            BuiltinEntityKind::Datetime => "Matches a date, time, interval or a date and time together",
            BuiltinEntityKind::Date => "Matches a date",
            BuiltinEntityKind::Time => "Matches a time of day",
            BuiltinEntityKind::DatePeriod => "Matches a period of time spanning over days or larger grains",
            BuiltinEntityKind::TimePeriod => "Matches a period of time spanning over hours or smaller grains",
            BuiltinEntityKind::Percentage => "Matches a percentage",
            BuiltinEntityKind::MusicAlbum => "Matches a music album",
            BuiltinEntityKind::MusicArtist => "Matches a music artist",
            BuiltinEntityKind::MusicTrack => "Matches a music track",
        }
    }
}

impl BuiltinEntityKind {
    pub fn examples(&self, language: Language) -> &'static [&'static str] {
        match language {
            Language::DE => self.de_examples(),
            Language::EN => self.en_examples(),
            Language::ES => self.es_examples(),
            Language::FR => self.fr_examples(),
            Language::JA => self.ja_examples(),
            Language::IT => self.it_examples(),
            Language::PT_PT => self.pt_examples(),
            Language::PT_BR => self.pt_examples(),
            Language::KO => self.ko_examples(),
        }
    }

    fn de_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &["10$", "ungefähr 5€", "zwei tausend Dollar"],
            BuiltinEntityKind::Duration => &[
                "2stdn",
                "drei monate",
                "ein halbe Stunde",
                "8 Jahre und zwei Tage",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "einundzwanzig",
                "zwei tausend",
                "zwei tausend und drei",
            ],
            BuiltinEntityKind::Ordinal => &["Erste", "der zweite", "zwei und zwanzigster"],
            BuiltinEntityKind::Temperature => &[
                "70K",
                "3°C",
                "Dreiundzwanzig Grad",
                "zweiunddreißig Grad Fahrenheit",
            ],
            BuiltinEntityKind::Datetime => &[
                "Heute",
                "16.30 Uhr",
                "in 1 Stunde",
                "dritter Dienstag im Juni",
            ],
            // Datetime subtypes not supported for this language,
            // cf. BuiltinEntityKind.supported_languages(),
            // but how to make this function aware of that?
            BuiltinEntityKind::Date => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::DatePeriod => &[],
            BuiltinEntityKind::TimePeriod => &[],
            BuiltinEntityKind::Percentage => &[
                "25%",
                "zwanzig Prozent",
                "zwei tausend und fünfzig Prozent",
            ],
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }

    fn en_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "$10",
                "six euros",
                "around 5€",
                "ten dollars and five cents",
            ],
            BuiltinEntityKind::Duration => &[
                "1h",
                "during two minutes",
                "for 20 seconds",
                "3 months",
                "half an hour",
                "8 years and two days",
            ],
            BuiltinEntityKind::Number => &["2001", "twenty one", "three hundred and four"],
            BuiltinEntityKind::Ordinal => &["1st", "the second", "the twenty third"],
            BuiltinEntityKind::Temperature => &[
                "70K",
                "3°C",
                "Twenty three degrees",
                "one hundred degrees fahrenheit",
            ],
            // TODO: Add cases for Datetime complement (date + time combinations, mainly)
            BuiltinEntityKind::Datetime => &[
                "Today",
                "at 8 a.m.",
                "4:30 pm",
                "in 1 hour",
                "the 3rd tuesday of June",
            ],
            BuiltinEntityKind::Date => &[
                "today",
                "on Wednesday",
                "March 26th",
                "saturday january 19",
                "monday 15th april 2019",
                "the day after tomorrow",
            ],
            BuiltinEntityKind::Time => &[
                "now",
                "at noon",
                "at 8 a.m.",
                "4:30 pm",
                "in one hour",
                "for ten o'clock",
                "at ten in the evening",
            ],
            BuiltinEntityKind::DatePeriod => &[
                "january",
                "2019",
                "from monday to friday",
                "from wednesday 27th to saturday 30th",
                "this week"
            ],
            BuiltinEntityKind::TimePeriod => &[
                // "tonight" currently not interpreted as a TimePeriod because intersected with
                // today's date, which makes it interpreted as a date+time (will be fixed)
                //"tonight",
                // "this morning" currently not interpreted as a TimePeriod (same reason)
                // "this morning",
                "until dinner",
                "from five to ten",
                // This is currently bugged + interpreted as TimePeriod (same reason, with "this")
                // "this evening after eight thirty",
                "by the end of the day",
            ],
            BuiltinEntityKind::Percentage => {
                &["25%", "twenty percent", "two hundred and fifty percents"]
            }
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }

    fn es_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "$10",
                "15€",
                "cinco euros",
                "16,65 €",
                "diez dólares y cinco centavos",
                "treinta y tres mil millones de rupias",
                "ocho cientos bitcoins",
                "noventa coronas danesas",
                "845584 francos suizos",
            ],
            BuiltinEntityKind::Duration => &[
                "1h",
                "3 meses",
                "diez minutos",
                "media hora",
                "ciento dos minutos",
                "8 años y dos dias",
                "un año catorce semanas y tres horas",
                "tres cuartos de hora",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                //"durante los próximos dos años",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "dieciocho",
                "ciento dos",
                "tres mil nueve",
                "ciento cuarenta y nueve",
                "cuatro cientos dieciséis",
                "quinientos noventa y uno",
                "mil novecientos cuarenta y cuatro",
            ],
            BuiltinEntityKind::Ordinal => &[
                "primer",
                "decima",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "vigésimo primero",
            ],
            BuiltinEntityKind::Temperature => &[
                "70 grados kelvin",
                "3°C",
                "veintitrés grados",
                "tres mil grados fahrenheit",
                "veinte grados centígrados",
                "setecientos ochenta y nueve kelvin",
                "quince grados bajo cero",
                "-459,67 °F",
            ],
            BuiltinEntityKind::Datetime => &[
                "hoy",
                "esta noche",
                "a la 1:30",
                "el primer jueves de junio",
                "el 30 de julio por la tarde",
                "la primera semana de la primavera",
                "de cinco a ocho de la tarde",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "las próximas navidades",
            ],
            // Datetime subtypes not supported for this language,
            // cf. BuiltinEntityKind.supported_languages(),
            // but how to make this function aware of that?
            BuiltinEntityKind::Date => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::DatePeriod => &[],
            BuiltinEntityKind::TimePeriod => &[],
            BuiltinEntityKind::Percentage => &[
                "25%",
                "quince por ciento",
                "20 por ciento",
                "tres por ciento",
                "veinte por ciento",
                "tres mil por ciento",
                "setenta y cinco por ciento"
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "cien por cien",
            ],
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }

    fn fr_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "10$",
                "environ 5€",
                "six euros",
                "dix dollars et cinq centimes",
            ],
            BuiltinEntityKind::Duration => &[
                "1h",
                "pendant vingt minutes",
                "durant 3 secondes",
                "3 mois",
                "une demi heure",
                "8 ans et deux jours",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "vingt deux",
                "deux cent trois",
                "quatre vingt dix neuf",
            ],
            BuiltinEntityKind::Ordinal => &[
                "1er",
                "43ème",
                "le deuxième",
                "cinq centième",
                "vingt et unieme",
            ],
            BuiltinEntityKind::Temperature => &[
                "70K",
                "3°C",
                "vingt trois degrés",
                "45 degrés celsius",
                "deux cent degrés Fahrenheit",
            ],
            BuiltinEntityKind::Datetime => &[
                "Aujourd'hui",
                "à 14:30",
                "demain matin",
                "hier vers 10 heures",
                "dans 1 heure",
                "le premier jeudi de Juin",
            ],
            // Datetime subtypes not supported for this language,
            // cf. BuiltinEntityKind.supported_languages(),
            // but how to make this function aware of that?
            BuiltinEntityKind::Date => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::DatePeriod => &[],
            BuiltinEntityKind::TimePeriod => &[],
            BuiltinEntityKind::Percentage => &["25%", "20 pourcents", "quatre vingt dix pourcents"],
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }

    fn it_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "$10",
                "15€",
                "cinque euro",
                "sei mila euro",
                "quattordici franchi svizzeri",
                "cinquanta sette dollari australiani",
                "dieci dollari e cinque centesimi",
                "cento diciotto mila corone danesi",
                "sessant uno euro e novanta nove centesimi",
            ],
            BuiltinEntityKind::Duration => &[
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "1h",
                "per un mese",
                "durante tre settimane",
                "durante un quarto d'ora",
                "per tre anni e mezzo",
                "per quattro ore e venti due minuti",
                "3 mesi",
                "dieci minuti",
                "cento due minuti",
                "8 anni e due giorni",
            ],
            BuiltinEntityKind::Number => &[
                "otto",
                "sedici",
                "cento",
                "venti due",
                "sei mila",
                "cento quaranta nove",
                "tre mila cinque cento",
                "due cento novanta tré",
                "mille otto cento cinquanta sei",
                "un milione sette cento dodici mila",
                "sessanta due mila tre cento ottanta nove",
            ],
            BuiltinEntityKind::Ordinal => &[
                "primo",
                "decima",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "vent unesimo",
                // "novanta quattresima",
                // "tre cento settantesima",
            ],
            BuiltinEntityKind::Temperature => &[
                "3°C",
                "tre gradi",
                "quindici gradi celsius",
                "settant uno fahrenheit",
                "due cento novanta cinque gradi kelvin",
            ],
            BuiltinEntityKind::Datetime => &[
                "domattina",
                "giovedì prossimo",
                "a febbraio",
                "tra quindici giorni",
                "il dodici marzo 2020",
                "dopodomani a mezzanotte e dieci",
                "alle sette e mezza di sera",
                "alle 1:30",
                "il primo giovedí di giugno",
            ],
            // Datetime subtypes not supported for this language,
            // cf. BuiltinEntityKind.supported_languages(),
            // but how to make this function aware of that?
            BuiltinEntityKind::Date => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::DatePeriod => &[],
            BuiltinEntityKind::TimePeriod => &[],
            BuiltinEntityKind::Percentage => &[
                "25%",
                "due percento",
                "cento percento",
                "20 percento",
                "tre mila percento",
                "sessanta sei percento",
                "diciotto per cento",
                "venti nove per cento",
            ],
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }

    fn pt_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "10$",
                "15€",
                "cinco euros",
                "16,65 €",
                "dois euros e cinco centavos",
                "dez libras esterlinas",
                "845584 francos suíços",
            ],
            BuiltinEntityKind::Duration => &[
                "1 hora",
                "3 meses",
                "dez minutos",
                "meia hora",
                "oito anos e dois semanas",
                "um ano quatro semanas e tres horas",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
            ],
            BuiltinEntityKind::Ordinal => &[
                "primera",
            ],
            BuiltinEntityKind::Temperature => &[
                "70 graus kelvin",
                "3°C",
                "dez graus",
                "quatro graus centígrados",
                "-459,67 °F",
            ],
            BuiltinEntityKind::Datetime => &[
                "hoje",
            ],
            // Datetime subtypes not supported for this language,
            // cf. BuiltinEntityKind.supported_languages(),
            // but how to make this function aware of that?
            BuiltinEntityKind::Date => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::DatePeriod => &[],
            BuiltinEntityKind::TimePeriod => &[],
            BuiltinEntityKind::Percentage => &[
                "25%",
            ],
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }

    fn ja_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &["八ドル", "五十二アメリカドル"],
            BuiltinEntityKind::Duration => &["一秒間", "五日間", "十ヶ月間"],
            BuiltinEntityKind::Number => &["十二", "二千五", "四千三百二"],
            BuiltinEntityKind::Ordinal => &["十一番目", "九十一番目"],
            BuiltinEntityKind::Temperature => &["五度", "二十五度", "マイナス十度"],
            BuiltinEntityKind::Datetime => &[
                "一昨日",
                "次の水曜日",
                "十三時三十分",
                "二千十三年十二月二十三日",
            ],
            // Datetime subtypes not supported for this language,
            // cf. BuiltinEntityKind.supported_languages(),
            // but how to make this function aware of that?
            BuiltinEntityKind::Date => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::DatePeriod => &[],
            BuiltinEntityKind::TimePeriod => &[],
            BuiltinEntityKind::Percentage => &["十五%", "五パーセント"],
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }

    fn ko_examples(&self) -> &'static [&'static str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &["10$", "약 5 유로", "10 달러 5 센트"],
            BuiltinEntityKind::Duration => &["양일", "1시간", "3 개월"],
            BuiltinEntityKind::Number => &["2001", "삼천", "스물 둘", "천 아흔 아홉"],
            BuiltinEntityKind::Ordinal => &["첫", "첫번째"],
            BuiltinEntityKind::Temperature => &["5도", "섭씨 20도", "화씨 백 도"],
            BuiltinEntityKind::Datetime => &["오늘", "14시 30 분에", "5 월 첫째 목요일"],
            // Datetime subtypes not supported for this language,
            // cf. BuiltinEntityKind.supported_languages(),
            // but how to make this function aware of that?
            BuiltinEntityKind::Date => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::DatePeriod => &[],
            BuiltinEntityKind::TimePeriod => &[],
            BuiltinEntityKind::Percentage => &[],
            BuiltinEntityKind::MusicAlbum => &["Discovery"],
            BuiltinEntityKind::MusicArtist => &["Daft Punk"],
            BuiltinEntityKind::MusicTrack => &["Harder Better Faster Stronger"],
        }
    }
}

impl BuiltinEntityKind {
    pub fn result_description(&self) -> String {
        match *self {
            BuiltinEntityKind::AmountOfMoney => {
                serde_json::to_string_pretty(&vec![SlotValue::AmountOfMoney(AmountOfMoneyValue {
                    value: 10.05,
                    precision: Precision::Approximate,
                    unit: Some("€".to_string()),
                })])
            }
            BuiltinEntityKind::Duration => {
                serde_json::to_string_pretty(&vec![SlotValue::Duration(DurationValue {
                    years: 0,
                    quarters: 0,
                    months: 3,
                    weeks: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                    precision: Precision::Exact,
                })])
            }
            BuiltinEntityKind::Number => {
                serde_json::to_string_pretty(&vec![SlotValue::Number(NumberValue { value: 42. })])
            }
            BuiltinEntityKind::Ordinal => {
                serde_json::to_string_pretty(&vec![SlotValue::Ordinal(OrdinalValue { value: 2 })])
            }
            BuiltinEntityKind::Temperature => serde_json::to_string_pretty(&vec![
                SlotValue::Temperature(TemperatureValue {
                    value: 23.0,
                    unit: Some("celsius".to_string()),
                }),
                SlotValue::Temperature(TemperatureValue {
                    value: 60.0,
                    unit: Some("fahrenheit".to_string()),
                }),
            ]),
            BuiltinEntityKind::Datetime => serde_json::to_string_pretty(&vec![
                SlotValue::InstantTime(InstantTimeValue {
                    value: "2017-06-13 18:00:00 +02:00".to_string(),
                    grain: Grain::Hour,
                    precision: Precision::Exact,
                }),
                SlotValue::TimeInterval(TimeIntervalValue {
                    from: Some("2017-06-07 18:00:00 +02:00".to_string()),
                    to: Some("2017-06-08 00:00:00 +02:00".to_string()),
                }),
            ]),
            BuiltinEntityKind::Date => serde_json::to_string_pretty(&vec![
                SlotValue::InstantTime(InstantTimeValue {
                    value: "2017-06-13 18:00:00 +02:00".to_string(),
                    grain: Grain::Hour,
                    precision: Precision::Exact,
                }),
            ]),
            BuiltinEntityKind::Time => serde_json::to_string_pretty(&vec![
                SlotValue::InstantTime(InstantTimeValue {
                    value: "2017-06-13 18:00:00 +02:00".to_string(),
                    grain: Grain::Hour,
                    precision: Precision::Exact,
                }),
            ]),
            BuiltinEntityKind::DatePeriod => serde_json::to_string_pretty(&vec![
                SlotValue::TimeInterval(TimeIntervalValue {
                    from: Some("2017-06-07 18:00:00 +02:00".to_string()),
                    to: Some("2017-06-09 18:00:00 +02:00".to_string()),
                }),
            ]),
            BuiltinEntityKind::TimePeriod => serde_json::to_string_pretty(&vec![
                SlotValue::TimeInterval(TimeIntervalValue {
                    from: Some("2017-06-07 18:00:00 +02:00".to_string()),
                    to: Some("2017-06-07 20:00:00 +02:00".to_string()),
                }),
            ]),
            BuiltinEntityKind::Percentage => {
                serde_json::to_string_pretty(&vec![SlotValue::Percentage(PercentageValue {
                    value: 20.,
                })])
            }
            BuiltinEntityKind::MusicAlbum => {
                serde_json::to_string_pretty(&vec![SlotValue::MusicAlbum(StringValue {
                    value: "Discovery".to_string(),
                })])
            }
            BuiltinEntityKind::MusicArtist => {
                serde_json::to_string_pretty(&vec![SlotValue::MusicArtist(StringValue {
                    value: "Daft Punk".to_string(),
                })])
            }
            BuiltinEntityKind::MusicTrack => {
                serde_json::to_string_pretty(&vec![SlotValue::MusicTrack(StringValue {
                    value: "Harder Better Faster Stronger".to_string(),
                })])
            }
        }
        .unwrap()
    }
}

impl BuiltinEntityKind {
    pub fn supported_languages(&self) -> &'static [Language] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::IT,
                Language::JA,
                Language::KO,
                Language::PT_BR,
                Language::PT_PT,
            ],
            BuiltinEntityKind::Duration => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::IT,
                Language::JA,
                Language::KO,
                Language::PT_BR,
                Language::PT_PT,
            ],
            BuiltinEntityKind::Number => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::IT,
                Language::JA,
                Language::KO,
                Language::PT_BR,
                Language::PT_PT,
            ],
            BuiltinEntityKind::Ordinal => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::IT,
                Language::JA,
                Language::KO,
                Language::PT_BR,
                Language::PT_PT,
            ],
            BuiltinEntityKind::Temperature => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::IT,
                Language::JA,
                Language::KO,
                Language::PT_BR,
                Language::PT_PT,
            ],
            BuiltinEntityKind::Datetime => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::IT,
                Language::JA,
                Language::KO,
                Language::PT_BR,
                Language::PT_PT,
            ],
            BuiltinEntityKind::Date => &[
                Language::EN,
            ],
            BuiltinEntityKind::Time => &[
                Language::EN,
            ],
            BuiltinEntityKind::DatePeriod => &[
                Language::EN,
            ],
            BuiltinEntityKind::TimePeriod => &[
                Language::EN,
            ],
            BuiltinEntityKind::Percentage => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::IT,
                Language::JA,
                Language::PT_BR,
                Language::PT_PT,
            ],
            BuiltinEntityKind::MusicAlbum => &[Language::EN, Language::FR],
            BuiltinEntityKind::MusicArtist => &[Language::EN, Language::FR],
            BuiltinEntityKind::MusicTrack => &[Language::EN, Language::FR],
        }
    }

    pub fn map_to_supported(&self, language: &Language) -> BuiltinEntityKind {
        // FIXME: That must not be the right way to get the type of self
        let builtin_entity_kind = self.clone();
        let result = match language {
                // English supports all Datetime subkinds (so all builtin kinds)
                Language::EN => builtin_entity_kind,
                // Other language need to remap Datetime subkinds to Datetime
                _ => {
                    match builtin_entity_kind {
                        BuiltinEntityKind::Date |
                        BuiltinEntityKind::Time |
                        BuiltinEntityKind::DatePeriod |
                        BuiltinEntityKind::TimePeriod => BuiltinEntityKind::Datetime,
                        _ => builtin_entity_kind,
                    }
                }
            };
        eprintln!("map_to_supported({:?}, {:?}) => {:?}", builtin_entity_kind, language, result);
        result
    }
}

impl BuiltinEntityKind {
    pub fn supported_entity_kinds(language: Language) -> Vec<BuiltinEntityKind> {
        Self::all()
            .to_vec()
            .into_iter()
            .filter(|e| e.supported_languages().contains(&language))
            .collect()
    }
}

impl BuiltinEntityKind {
    fn ontology_details(&self, language: Language) -> BuiltinEntityKindDetails {
        BuiltinEntityKindDetails {
            name: self.to_string(),
            label: self.identifier().to_string(),
            description: self.description().to_string(),
            examples: self
                .examples(language)
                .into_iter()
                .map(|ex| ex.to_string())
                .collect(),
            result_description: self.result_description(),
            supported_languages: self
                .supported_languages()
                .into_iter()
                .map(|l| l.to_string())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuiltinEntityKindDetails {
    name: String,
    label: String,
    description: String,
    examples: Vec<String>,
    result_description: String,
    supported_languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LanguageBuiltinEntityOntology {
    language: String,
    entities: Vec<BuiltinEntityKindDetails>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompleteBuiltinEntityOntology(Vec<LanguageBuiltinEntityOntology>);

pub fn complete_entity_ontology() -> CompleteBuiltinEntityOntology {
    let language_ontologies = Language::all()
        .iter()
        .map(|language| language_entity_ontology(*language))
        .collect();
    CompleteBuiltinEntityOntology(language_ontologies)
}

pub fn language_entity_ontology(language: Language) -> LanguageBuiltinEntityOntology {
    let entities = BuiltinEntityKind::supported_entity_kinds(language)
        .iter()
        .map(|entity_kind| entity_kind.ontology_details(language))
        .collect();
    LanguageBuiltinEntityOntology {
        language: language.to_string(),
        entities,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_result_descriptions() {
        // Given
        let description = BuiltinEntityKind::Percentage.result_description();

        // When/Then
        let expected_description =
            "[\n  {\n    \"kind\": \"Percentage\",\n    \"value\": 20.0\n  }\n]";
        assert_eq!(expected_description, description);
    }

    #[test]
    fn test_entity_examples_should_be_provided_for_all_supported_languages() {
        for entity_kind in BuiltinEntityKind::all() {
            for language in entity_kind.supported_languages() {
                let examples = entity_kind.examples(*language);
                assert!(
                    examples.len() >= 1,
                    "No examples provided for entity '{:?}' in language '{:?}'",
                    entity_kind,
                    language
                )
            }
        }
    }

    #[test]
    fn test_builtin_entity_ser_de() {
        let entity = BuiltinEntity {
            value: "hello".to_string(),
            range: 12..42,
            entity: SlotValue::InstantTime(InstantTimeValue {
                value: "some_value".into(),
                grain: Grain::Year,
                precision: Precision::Exact,
            }),
            entity_kind: BuiltinEntityKind::Datetime,
        };

        assert_tokens(
            &entity,
            &[
                Token::Struct {
                    name: "BuiltinEntity",
                    len: 4,
                },
                Token::Str("value"),
                Token::Str("hello"),
                Token::Str("range"),
                Token::Struct {
                    name: "Range",
                    len: 2,
                },
                Token::Str("start"),
                Token::U64(12),
                Token::Str("end"),
                Token::U64(42),
                Token::StructEnd,
                Token::Str("entity"),
                Token::Struct {
                    name: "InstantTimeValue",
                    len: 4,
                },
                Token::Str("kind"),
                Token::Str("InstantTime"),
                Token::Str("value"),
                Token::String("some_value"),
                Token::Str("grain"),
                Token::UnitVariant {
                    name: "Grain",
                    variant: "Year",
                },
                Token::Str("precision"),
                Token::UnitVariant {
                    name: "Precision",
                    variant: "Exact",
                },
                Token::StructEnd,
                Token::Str("entity_kind"),
                Token::Str("snips/datetime"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_complete_entities_ontology() {
        assert_eq!(
            true,
            serde_json::to_string_pretty(&complete_entity_ontology()).is_ok()
        )
    }

    #[test]
    fn test_entities_ontology() {
        for language in Language::all() {
            assert_eq!(
                true,
                serde_json::to_string_pretty(&language_entity_ontology(*language)).is_ok()
            )
        }
    }
}
