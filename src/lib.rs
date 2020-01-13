extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_thirteen(possibly_thirteen: &str) -> bool{
    THIRTEENS.iter().find(|&&x| x == possibly_thirteen.to_lowercase().as_str()) != None
}

const THIRTEENS: [&str; 233] = [
    "thirteen",
    "13",
    "xiii", // Roman numeral 13
    "1.3", // Basically 13, see proof in #420
    "1️⃣3️⃣", // emoji sequence of 1 and 3
    "13+0i", //imaginary variants
    "13 + 13i",
    "13i",
    "B", //these look like 13
    "ß",
    "ẞ",
    "β",
    "Β", //actually upper case Beta, not B
    "阝", //(Kangxi radical)
    "i3",
    "l3",
    "|3",
    "!3",
    "ei",
    "e1",
    "el",
    "e|",
    "ƖƐ",
    "ƐƖ",
    "th1rt33n",
    "th1rte3n",
    "th1rteen",
    "thirt3en",
    "thirt33n",
    "thirte3n",
    "00001101",
    "0b1101",
    "0o15",
    "0xd",
    ".---- ...--", //morse code
    "- .... .. .-. - . . -.",
    "- .... .. .-. - . . -.",
    "wkluwhhq", //caeasar cipher
    "Wkluwhhq",
    "WKLUWHHQ",
    "74 68 69 72 74 65 65 6e", //octal
    "54 48 49 52 54 45 45 4e",
    "31 33",
    "74 68 69 72 74 65 65 6e 0d 0a",
    "54 68 69 72 74 65 65 6e 0d 0a",
    "54 48 49 52 54 45 45 4e 0d 0a 0d 0a",
    "dGhpcnRlZW4=", //base64
    "VGhpcnRlZW4=",
    "VEhJUlRFRU4=",
    "MTM=",
    "thirteen", // English
    "ثلاثة عشر", // Arabic (masculine)
    "ثلاث عشرة", // Arabic (feminine)
    "تلطاشر", // Arabic Slang
    "تلتاشر", // Arabic Slang
    "طلتاشر", // Arabic Slang
    "طلطاشر", // Arabic Slang
    "يج", //Arabic (gematria)
    "سیزده", // Persian
    "۱۳", // Persian number
    "dertien", // Afrikaans / Dutch
    "dertiendertien", // Double Dutch
    "seri-un-teng", // Belter creole
    "seriunteng",
    "serí-un-teng",
    "seríunteng",
    "тринадесет", // Bulgarian
    "тринайсет", // Also Bulgarian
    "tretze", // Catalan
    "napulo ug tulo", // Cebuano
    "十三", // Chinese / Japanese
    "拾參", // Chinese (traditional, upper case)
    "拾叁", // Chinese (simplified, upper case)
    "拾叄", // Chinese (variant)
    "拾参", // Chinese (variant)
    "サーティーン", // Japanese
    "１３", // Japanese full-width
    "trinaest", // Croatian / Serbian (latin)
    "tretten", // Danish / Norwegian
    "senthi", //Dothraki
    "þrettán", // Icelandic, following are different inflections
    "þrettándi", // e. thirteenth
    "þrettánda",
    "þrettándinn", // e. the thirteenth
    "þrettándann",
    "þrettándanum",
    "þrettándans",
    "þrettándar", // e. multiple thirteenths
    "þrettándu",
    "þrettándum",
    "þrettándarnir", // e. the multiple thirteenths
    "þrettándana",
    "þrettándunum",
    "þrettándanna",
    "threttan", // strings without special icelandic characters
    "threttandi",
    "threttanda",
    "threttandinn",
    "threttandann",
    "threttandanum",
    "threttandans",
    "threttandar",
    "threttandu",
    "threttandum",
    "threttandarnir",
    "threttandana",
    "threttandunum",
    "threttandanna",// end of Icelandic
    "threttandum", // end of Icelandic
    "třináct", // Czech
    "kolmteist", // Estonian
    "labintatlo", // Filipino
    "kolmetoista", // Finnish
    "treize", // French
    "treizième", //French (ordinal form)
    "dreizehn", // German
    "ცამეტი", // Georgian
    "δεκατρία", // Greek
    "drizäh", // Swiss German
    "wa’maH wej", // Klingon
    "‘umikūmākolu", // Hawaiian
    "שלוש עשרה", // Hebrew
    "שלושעשרה", // Hebrew (without space)
    "ֹשְלֹש- עֶשְֹרֵה", // Hebrew (with punctuation)
    "שלושה עשר", // Hebrew (male form)
    "שלושהעשר", // Hebrew (male form, without space)
    "ֹשְלֹשָה- עָשָֹר", // Hebrew (male form, with punctuation)
    "יג", // Hebrew (gematria)
    "י״ג", // Hebrew (gematria - apostrophes)
    "quainel", // Quenya
    "mînuiug", // Sindarin
    "dektri", // Esperanto
    "tizenhárom", // Hungarian
    "trí déag", // Irish
    "tredici", // Italian
    "ಹದಿಮೂರು", //Kannada (for thirteen)
    "೧೩",//Kannada (for 13)
    "열셋", // Korean
    "십삼", // Korean
    "sêzdeh", // Kurdish
    "tredecim", // Latin
    "trīspadsmit", // Latvian
    "trylika", // Lithuanian
    "dräizéng", // Luxembourgish
    "тринаесет", // Macedonian
    "tiga belas", // Malay
    "പതിമൂന്ന്", //Malayalam
    "तेरा", // Marathi (१३)
    "арван", // Mongolian
    "irteenthay", // Pig Latin
    "trzynaście", // Polish
    "trzynasty", // Polish
    "trzynasta", // Polish
    "trzynaste", // Polish
    "trzynaści", // Polish
    "trzynastego", // Polish
    "trzynastej", // Polish
    "trzynastych", // Polish
    "trzynastemu", // Polish
    "trzynastym", // Polish
    "trzynastą", // Polish
    "trzynastymi", // Polish
    "trzynastu", // Polish
    "trzynastek", // Polish
    "trzynastoma", // Polish
    "trzynaścioro", // Polish
    "trzynastka", // Polish
    "trzynastki", // Polish
    "trzynastką", // Polish
    "trzynastce", // Polish
    "trzynastko", // Polish
    "trzynaściorgiem", // Polish
    "trzynaściorgu", // Polish
    "trzynaściorga", // Polish
    "trzynastokrotny", // Polish
    "trzynastokrotnie", // Polish
    "trzynastokrotną", // Polish
    "trzynastokrotnemu", // Polish
    "trzynastokrotnej", // Polish
    "trzynastokrotnych", // Polish
    "trzynastokrotność", // Polish
    "trzynastokrotności", // Polish
    "trzynastokrotnością", // Polish
    "১৩", // Bengali numeral
    "তেরো",
    "তের",
    "ত্রয়োদশ",
    "treze", // Portuguese
    "ਤੇਰਾਂ", // Punjabi - thirteen
    "੧੩", // Punjabi Numeral - 13
    "treisprezece", // Romanian
    "treispe", // Romanian
    "тринадцать", // Russian (cyrillic)
    "ⱅⱃⰺⱀⰰⰴⱌⰰⱅⱐ", // Russian (glagolitic)
    "тринаест", // Serbian (cyrillic)
    "trinásť", // Slovak
    "trinajst", // Slovenian
    "trece", // Spanish
    "diez-y-tres", // Spanglish
    "trese", // Tagalog
    "on üç", // Turkish
    "dektri", //Speranto
    "tlettax", // Maltese
    "tretton", // Swedish
    "பதின்மூன்று", // Tamil
    "สิบสาม", // Thai
    "๑๓", // Thai Numeral
    "SipSam", // Thai Transcription
    "Sip Sam", // Thai Transcription with space
    "тринадцять", // Ukrainian
    "تیرہ", // Urdu
    "mười ba", // Vietnamese
    "tri ar ddeg", // Welsh
    "דרייַצן", // Yiddish,
    "דרייצן", // Yiddish (without diacritics),
    "kumi na tatu", // Swahili
    "तेह्र", //Nepali
    "१३", //Devanagari
    "तेरह", //Hindi
    "7h1r733n", // Crypto
    "θərˈtiːn",
    "పదమూడు", //Telugu
    "shí sān", // Pinyin (formal)
    "shi san", // Pinyin (without tones)
    "shísān",  // Pinyin (without spaces)
    "shisan", // Pinyin (without spaces and tones)
    "он үш", // Kazakh
    "он уш", // Kazakh
    "onúsh", // Kazakh latin,
    "онүш", // Kazakh
    "онуш", // Kazakh
    "onúsh", // Kazakh latin
    "ishumi nantathu" // isiZulu
];

#[cfg(test)]
mod tests {
    use is_thirteen;
    #[test]
    fn is_thirteen_numerals_test() {
        assert_eq!(is_thirteen("13"), true);
    }
    #[test]
    fn is_thirteen_text_test() {
        assert_eq!(is_thirteen("thirteen"), true);
    }
    #[test]
    fn is_thirteen_caps_test() {
        assert_eq!(is_thirteen("ThIrTeEn"), true);
    }
    #[test]
    fn is_not_thirteen_test() {
        assert_eq!(is_thirteen("12"), false);
    }
}
