use rustling_ontology::Lang as RustlingLang;

enum_kind!(Language,
    [
        DE,
        EN,
        ES,
        FR,
        KO,
        ZH
    ]
);


impl From<Language> for RustlingLang {
    fn from(lang: Language) -> Self {
        match lang {
            Language::DE => RustlingLang::DE,
            Language::EN => RustlingLang::EN,
            Language::ES => RustlingLang::ES,
            Language::FR => RustlingLang::FR,
            Language::KO => RustlingLang::KO,
            Language::ZH => RustlingLang::ZH,
        }
    }
}
