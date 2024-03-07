use ctru::services::cfgu::Language;

fn to_code(lang: Language) -> &'static str {
    match lang {
        Language::Japanese => "ja",
        Language::English => "en",
        Language::French => "fr",
        Language::German => "de",
        Language::Italian => "it",
        Language::Spanish => "es",
        Language::SimplifiedChinese => "zh-CN",
        Language::Korean => "ko",
        Language::Dutch => "nl",
        Language::Portuguese => "pt",
        Language::Russian => "ru",
        Language::TraditionalChinese => "zh-TW",
    }
}

pub fn set_to_system(lang: Language) {
    let locale = to_code(lang);
    let available_locales: Vec<&str> = available_locales!();

    if !available_locales.contains(&locale) && available_locales.contains(&&locale[..2])  {
        rust_i18n::set_locale(&&locale[..2]);
    } else {
        rust_i18n::set_locale(&locale);
    }
}