#[doc = "local-lang/language"]
use lazy_static::lazy_static;
use std::collections::HashMap;
use winapi::um::winnls::GetSystemDefaultUILanguage;
use winapi::um::winnls::GetUserDefaultUILanguage;
/// create the table map langid(u16) to string
fn register_code_map() -> HashMap<u16, &'static str> {
    let mut map = HashMap::new();
    map.insert(1025, "ar-SA");
    map.insert(1026, "bg-BG");
    map.insert(1027, "ca-ES");
    map.insert(1028, "zh-TW");
    map.insert(1029, "cs-CZ");
    map.insert(1030, "da-DK");
    map.insert(1031, "de-DE");
    map.insert(1032, "el-GR");
    map.insert(1033, "en-US");
    map.insert(1034, "es-ES");
    map.insert(1035, "fi-FI");
    map.insert(1036, "fr-FR");
    map.insert(1037, "he-IL");
    map.insert(1038, "hu-HU");
    map.insert(1039, "is-IS");
    map.insert(1040, "it-IT");
    map.insert(1041, "ja-JP");
    map.insert(1042, "ko-KR");
    map.insert(1043, "nl-NL");
    map.insert(1044, "nb-NO");
    map.insert(1045, "pl-PL");
    map.insert(1046, "pt-BR");
    map.insert(1047, "rm-CH");
    map.insert(1048, "ro-RO");
    map.insert(1049, "ru-RU");
    map.insert(1050, "hr-HR");
    map.insert(1051, "sk-SK");
    map.insert(1052, "sq-AL");
    map.insert(1053, "sv-SE");
    map.insert(1054, "th-TH");
    map.insert(1055, "tr-TR");
    map.insert(1056, "ur-PK");
    map.insert(1057, "id-ID");
    map.insert(1058, "uk-UA");
    map.insert(1059, "be-BY");
    map.insert(1060, "sl-SI");
    map.insert(1061, "et-EE");
    map.insert(1062, "lv-LV");
    map.insert(1063, "lt-LT");
    map.insert(1064, "tg-Cyrl-TJ");
    map.insert(1065, "fa-IR");
    map.insert(1066, "vi-VN");
    map.insert(1067, "hy-AM");
    map.insert(1068, "az-Latn-AZ");
    map.insert(1069, "eu-ES");
    map.insert(1070, "wen-DE");
    map.insert(1071, "mk-MK");
    map.insert(1072, "st-ZA");
    map.insert(1073, "ts-ZA");
    map.insert(1074, "tn-ZA");
    map.insert(1075, "ven-ZA");
    map.insert(1076, "xh-ZA");
    map.insert(1077, "zu-ZA");
    map.insert(1078, "af-ZA");
    map.insert(1079, "ka-GE");
    map.insert(1080, "fo-FO");
    map.insert(1081, "hi-IN");
    map.insert(1082, "mt-MT");
    map.insert(1083, "se-NO");
    map.insert(1084, "gd-GB");
    map.insert(1085, "yi");
    map.insert(1086, "ms-MY");
    map.insert(1087, "kk-KZ");
    map.insert(1088, "ky-KG");
    map.insert(1089, "sw-KE");
    map.insert(1090, "tk-TM");
    map.insert(1091, "uz-Latn-UZ");
    map.insert(1092, "tt-RU");
    map.insert(1093, "bn-IN");
    map.insert(1094, "pa-IN");
    map.insert(1095, "gu-IN");
    map.insert(1096, "or-IN");
    map.insert(1097, "ta-IN");
    map.insert(1098, "te-IN");
    map.insert(1099, "kn-IN");
    map.insert(1100, "ml-IN");
    map.insert(1101, "as-IN");
    map.insert(1102, "mr-IN");
    map.insert(1103, "sa-IN");
    map.insert(1104, "mn-MN");
    map.insert(1105, "bo-CN");
    map.insert(1106, "cy-GB");
    map.insert(1107, "km-KH");
    map.insert(1108, "lo-LA");
    map.insert(1109, "my-MM");
    map.insert(1110, "gl-ES");
    map.insert(1111, "kok-IN");
    map.insert(1112, "mni");
    map.insert(1113, "sd-IN");
    map.insert(1114, "syr-SY");
    map.insert(1115, "si-LK");
    map.insert(1116, "chr-US");
    map.insert(1117, "iu-Cans-CA");
    map.insert(1118, "am-ET");
    map.insert(1119, "tmz");
    map.insert(1120, "ks-Arab-IN");
    map.insert(1121, "ne-NP");
    map.insert(1122, "fy-NL");
    map.insert(1123, "ps-AF");
    map.insert(1124, "fil-PH");
    map.insert(1125, "dv-MV");
    map.insert(1126, "bin-NG");
    map.insert(1127, "fuv-NG");
    map.insert(1128, "ha-Latn-NG");
    map.insert(1129, "ibb-NG");
    map.insert(1130, "yo-NG");
    map.insert(1131, "quz-BO");
    map.insert(1132, "nso-ZA");
    map.insert(1136, "ig-NG");
    map.insert(1137, "kr-NG");
    map.insert(1138, "gaz-ET");
    map.insert(1139, "ti-ER");
    map.insert(1140, "gn-PY");
    map.insert(1141, "haw-US");
    map.insert(1142, "la");
    map.insert(1143, "so-SO");
    map.insert(1144, "ii-CN");
    map.insert(1145, "pap-AN");
    map.insert(1152, "ug-Arab-CN");
    map.insert(1153, "mi-NZ");
    map.insert(2049, "ar-IQ");
    map.insert(2052, "zh-CN");
    map.insert(2055, "de-CH");
    map.insert(2057, "en-GB");
    map.insert(2058, "es-MX");
    map.insert(2060, "fr-BE");
    map.insert(2064, "it-CH");
    map.insert(2067, "nl-BE");
    map.insert(2068, "nn-NO");
    map.insert(2070, "pt-PT");
    map.insert(2072, "ro-MD");
    map.insert(2073, "ru-MD");
    map.insert(2074, "sr-Latn-CS");
    map.insert(2077, "sv-FI");
    map.insert(2080, "ur-IN");
    map.insert(2092, "az-Cyrl-AZ");
    map.insert(2108, "ga-IE");
    map.insert(2110, "ms-BN");
    map.insert(2115, "uz-Cyrl-UZ");
    map.insert(2117, "bn-BD");
    map.insert(2118, "pa-PK");
    map.insert(2128, "mn-Mong-CN");
    map.insert(2129, "bo-BT");
    map.insert(2137, "sd-PK");
    map.insert(2143, "tzm-Latn-DZ");
    map.insert(2144, "ks-Deva-IN");
    map.insert(2145, "ne-IN");
    map.insert(2155, "quz-EC");
    map.insert(2163, "ti-ET");
    map.insert(3073, "ar-EG");
    map.insert(3076, "zh-HK");
    map.insert(3079, "de-AT");
    map.insert(3081, "en-AU");
    map.insert(3082, "es-ES");
    map.insert(3084, "fr-CA");
    map.insert(3098, "sr-Cyrl-CS");
    map.insert(3179, "quz-PE");
    map.insert(4097, "ar-LY");
    map.insert(4100, "zh-SG");
    map.insert(4103, "de-LU");
    map.insert(4105, "en-CA");
    map.insert(4106, "es-GT");
    map.insert(4108, "fr-CH");
    map.insert(4122, "hr-BA");
    map.insert(5121, "ar-DZ");
    map.insert(5124, "zh-MO");
    map.insert(5127, "de-LI");
    map.insert(5129, "en-NZ");
    map.insert(5130, "es-CR");
    map.insert(5132, "fr-LU");
    map.insert(5146, "bs-Latn-BA");
    map.insert(6145, "ar-MO");
    map.insert(6153, "en-IE");
    map.insert(6154, "es-PA");
    map.insert(6156, "fr-MC");
    map.insert(7169, "ar-TN");
    map.insert(7177, "en-ZA");
    map.insert(7178, "es-DO");
    map.insert(7180, "fr-029");
    map.insert(8193, "ar-OM");
    map.insert(8201, "en-JM");
    map.insert(8202, "es-VE");
    map.insert(8204, "fr-RE");
    map.insert(9217, "ar-YE");
    map.insert(9225, "en-029");
    map.insert(9226, "es-CO");
    map.insert(9228, "fr-CG");
    map.insert(10241, "ar-SY");
    map.insert(10249, "en-BZ");
    map.insert(10250, "es-PE");
    map.insert(10252, "fr-SN");
    map.insert(11265, "ar-JO");
    map.insert(11273, "en-TT");
    map.insert(11274, "es-AR");
    map.insert(11276, "fr-CM");
    map.insert(12289, "ar-LB");
    map.insert(12297, "en-ZW");
    map.insert(12298, "es-EC");
    map.insert(12300, "fr-CI");
    map.insert(13313, "ar-KW");
    map.insert(13321, "en-PH");
    map.insert(13322, "es-CL");
    map.insert(13324, "fr-ML");
    map.insert(14337, "ar-AE");
    map.insert(14345, "en-ID");
    map.insert(14346, "es-UY");
    map.insert(14348, "fr-MA");
    map.insert(15361, "ar-BH");
    map.insert(15369, "en-HK");
    map.insert(15370, "es-PY");
    map.insert(15372, "fr-HT");
    map.insert(16385, "ar-QA");
    map.insert(16393, "en-IN");
    map.insert(16394, "es-BO");
    map.insert(17417, "en-MY");
    map.insert(17418, "es-SV");
    map.insert(18441, "en-SG");
    map.insert(18442, "es-HN");
    map.insert(19466, "es-NI");
    map.insert(20490, "es-PR");
    map.insert(21514, "es-US");
    map.insert(58378, "es-419");
    map.insert(58380, "fr-015");
    map
}
lazy_static! {
    /// the table map langid(u16) to string(&'static str)
    static ref LANG_CODE_NAME_TABLE: HashMap<u16, &'static str> = register_code_map();
}
/// get the user's language of ui on window<br>
///
/// # Example
/// ```
/// use local_lang::language::get_default_ui_language;
/// assert_eq!(get_default_ui_language(),"zh-CN");
/// ```
pub fn get_default_ui_language() -> &'static str {
    unsafe {
        let langid = GetUserDefaultUILanguage();
        LANG_CODE_NAME_TABLE[&langid]
    }
}
/// get the system's language of ui on window<br>
///
/// # Example
/// ```
/// use local_lang::language::get_system_default_ui_language;
/// assert_eq!(get_system_default_ui_language(),"zh-CN");
/// ```
pub fn get_system_default_ui_language() -> &'static str {
    unsafe {
        let langid = GetSystemDefaultUILanguage();
        LANG_CODE_NAME_TABLE[&langid]
    }
}

/// get the language name by number(u16)<br>
///
/// # Parameters
/// id(u16) is the number of langid
/// # Example
/// ```
/// use local_lang::language::get_language_by_number;
/// assert_eq!(get_language_by_number(2052),"zh-CN");
/// ```
pub fn get_language_by_number(id: u16) -> &'static str {
    LANG_CODE_NAME_TABLE[&id]
}
