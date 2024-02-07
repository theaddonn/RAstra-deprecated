#[allow(non_snake_case, non_camel_case_types)]
pub enum Lang {
    id_ID,
    /// Danish in Denmark
    da_DK,
    /// German in Germany
    de_DE,
    /// English in Great Britain
    en_GB,
    /// English in USA
    en_US,
    /// Spanish in Spain
    es_ES,
    /// Spanish in Mexico
    es_MX,
    /// French in Canada
    fr_CA,
    /// French in France
    fr_FR,
    /// Italian in Italy
    it_IT,
    /// Hungarian in Hungry
    hu_HU,
    /// Dutch in the Netherlands
    nl_NL,
    nb_NO,
    pl_PL,
    pt_BR,
    pt_PT,
    sk_SK,
    fi_FI,
    sv_SE,
    tr_TR,
    cs_CZ,
    el_GR,
    bg_BG,
    ru_RU,
    ja_JP,
    zh_CN,
    zh_TW,
    ko_KR,
}

impl Lang {
    pub fn as_str(&self) -> &'static str {
        match self {
            Lang::id_ID => { "" }
            Lang::da_DK => { "Danish" }
            Lang::de_DE => { "German" }
            Lang::en_GB => { "English (GB)" }
            Lang::en_US => { "English (USA)" }
            Lang::es_ES => { "Spanish (Spain)" }
            Lang::es_MX => { "Spanish (Mexico)" }
            Lang::fr_CA => { "French (Can)" }
            Lang::fr_FR => { "French (France)" }
            Lang::it_IT => { "Italian" }
            Lang::hu_HU => { "Hungarian" }
            Lang::nl_NL => { "Dutch" }
            Lang::nb_NO => { "" }
            Lang::pl_PL => { "Polish" }
            Lang::pt_BR => { "" }
            Lang::pt_PT => { "" }
            Lang::sk_SK => { "" }
            Lang::fi_FI => { "" }
            Lang::sv_SE => { "" }
            Lang::tr_TR => { "" }
            Lang::cs_CZ => { "" }
            Lang::el_GR => { "" }
            Lang::bg_BG => { "" }
            Lang::ru_RU => { "" }
            Lang::ja_JP => { "" }
            Lang::zh_CN => { "" }
            Lang::zh_TW => { "" }
            Lang::ko_KR => { "" }
        }
    }
}
