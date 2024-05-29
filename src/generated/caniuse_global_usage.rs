use crate::data::BrowserName;
use once_cell::sync::Lazy;
pub static CANIUSE_GLOBAL_USAGE: Lazy<Vec<(BrowserName, &'static str, f32)>> = Lazy::new(|| {
    vec![
        ("and_chr", "122", 42.4244f32),
        ("chrome", "121", 13.6988f32),
        ("ios_saf", "17.2", 5.42163f32),
        ("edge", "121", 3.58054f32),
        ("ios_saf", "17.3", 2.54497f32),
        ("chrome", "120", 2.08675f32),
        ("samsung", "23", 2.04338f32),
        ("chrome", "122", 1.87769f32),
        ("ios_saf", "16.6-16.7", 1.65702f32),
        ("chrome", "109", 1.62303f32),
        ("op_mob", "73", 1.23017f32),
        ("firefox", "122", 1.1593f32),
        ("chrome", "119", 1.10229f32),
        ("ios_saf", "15.6-15.8", 0.978947f32),
        ("safari", "17.2", 0.919842f32),
        ("and_uc", "15.5", 0.818136f32),
        ("ios_saf", "17.1", 0.725037f32),
        ("opera", "106", 0.661374f32),
        ("android", "122", 0.580223f32),
        ("edge", "122", 0.551145f32),
        ("ios_saf", "16.1", 0.540109f32),
        ("safari", "17.3", 0.475125f32),
        ("ios_saf", "16.3", 0.457918f32),
        ("ie", "11", 0.450937f32),
        ("safari", "16.6", 0.440916f32),
        ("firefox", "115", 0.429513f32),
        ("firefox", "123", 0.349692f32),
        ("and_ff", "123", 0.291306f32),
        ("edge", "120", 0.277473f32),
        ("safari", "15.6", 0.273672f32),
        ("chrome", "118", 0.273672f32),
        ("ios_saf", "12.2-12.5", 0.265651f32),
        ("ios_saf", "16.0", 0.25391f32),
        ("ios_saf", "16.2", 0.250974f32),
        ("chrome", "116", 0.250866f32),
        ("safari", "17.1", 0.243264f32),
        ("and_qq", "13.1", 0.241722f32),
        ("ios_saf", "17.0", 0.237765f32),
        ("ios_saf", "16.5", 0.214282f32),
        ("ios_saf", "14.5-14.8", 0.15851f32),
        ("chrome", "117", 0.155841f32),
        ("chrome", "103", 0.144438f32),
        ("samsung", "4", 0.13981f32),
        ("edge", "119", 0.133035f32),
        ("safari", "16.3", 0.125433f32),
        ("ios_saf", "14.0-14.4", 0.121818f32),
        ("ios_saf", "15.5", 0.118883f32),
        ("chrome", "79", 0.11403f32),
        ("firefox", "119", 0.110229f32),
        ("firefox", "120", 0.106428f32),
        ("chrome", "113", 0.106428f32),
        ("safari", "14.1", 0.106428f32),
        ("ios_saf", "16.4", 0.102738f32),
        ("chrome", "114", 0.102627f32),
        ("firefox", "118", 0.095025f32),
        ("op_mini", "all", 0.09f32),
        ("ios_saf", "15.4", 0.0895289f32),
        ("kaios", "2.5", 0.086772f32),
        ("samsung", "22", 0.0860371f32),
        ("safari", "16.5", 0.083622f32),
        ("chrome", "87", 0.079821f32),
        ("ios_saf", "15.2-15.3", 0.0792551f32),
        ("safari", "17.0", 0.07602f32),
        ("samsung", "21", 0.0752824f32),
        ("ios_saf", "17.4", 0.0733843f32),
        ("chrome", "98", 0.072219f32),
        ("safari", "13.1", 0.072219f32),
        ("edge", "109", 0.068418f32),
        ("ios_saf", "15.0-15.1", 0.0675136f32),
        ("firefox", "121", 0.064617f32),
        ("chrome", "108", 0.060816f32),
        ("safari", "16.1", 0.060816f32),
        ("ios_saf", "10.3", 0.0587075f32),
        ("ios_saf", "13.4-13.7", 0.0572398f32),
        ("chrome", "91", 0.057015f32),
        ("chrome", "111", 0.057015f32),
        ("chrome", "115", 0.057015f32),
        ("samsung", "7.2-7.4", 0.0537732f32),
        ("safari", "16.2", 0.053214f32),
        ("chrome", "112", 0.053214f32),
        ("firefox", "52", 0.053214f32),
        ("edge", "87", 0.053214f32),
        ("chrome", "110", 0.049413f32),
        ("chrome", "70", 0.049413f32),
        ("opera", "102", 0.049413f32),
        ("safari", "16.4", 0.049413f32),
        ("chrome", "93", 0.049413f32),
        ("ie", "9", 0.0466486f32),
        ("opera", "95", 0.045612f32),
        ("chrome", "94", 0.045612f32),
        ("samsung", "19.0", 0.0430185f32),
        ("ios_saf", "13.2", 0.0425629f32),
        ("chrome", "106", 0.041811f32),
        ("chrome", "101", 0.041811f32),
        ("safari", "15.5", 0.041811f32),
        ("safari", "14", 0.03801f32),
        ("chrome", "104", 0.03801f32),
        ("chrome", "86", 0.03801f32),
        ("chrome", "83", 0.03801f32),
        ("ios_saf", "9.3", 0.0352245f32),
        ("chrome", "99", 0.034209f32),
        ("chrome", "69", 0.034209f32),
        ("chrome", "102", 0.034209f32),
        ("chrome", "107", 0.034209f32),
        ("firefox", "11", 0.034209f32),
        ("samsung", "17.0", 0.0322639f32),
        ("samsung", "20", 0.0322639f32),
        ("firefox", "103", 0.030408f32),
        ("chrome", "105", 0.030408f32),
        ("chrome", "48", 0.030408f32),
        ("chrome", "100", 0.030408f32),
        ("chrome", "81", 0.030408f32),
        ("safari", "16.0", 0.030408f32),
        ("safari", "15.1", 0.030408f32),
        ("chrome", "66", 0.026607f32),
        ("safari", "15.4", 0.026607f32),
        ("chrome", "49", 0.026607f32),
        ("chrome", "92", 0.026607f32),
        ("ios_saf", "11.0-11.2", 0.0264184f32),
        ("firefox", "102", 0.022806f32),
        ("edge", "118", 0.022806f32),
        ("chrome", "85", 0.022806f32),
        ("samsung", "18.0", 0.0215093f32),
        ("ios_saf", "11.3-11.4", 0.0190799f32),
        ("chrome", "88", 0.019005f32),
        ("firefox", "56", 0.019005f32),
        ("chrome", "89", 0.019005f32),
        ("edge", "114", 0.019005f32),
        ("firefox", "12", 0.019005f32),
        ("chrome", "60", 0.019005f32),
        ("chrome", "97", 0.019005f32),
        ("chrome", "123", 0.019005f32),
        ("ie", "8", 0.0155495f32),
        ("chrome", "80", 0.015204f32),
        ("chrome", "38", 0.015204f32),
        ("opera", "46", 0.015204f32),
        ("chrome", "90", 0.015204f32),
        ("edge", "117", 0.015204f32),
        ("firefox", "78", 0.015204f32),
        ("safari", "12.1", 0.015204f32),
        ("edge", "113", 0.015204f32),
        ("chrome", "78", 0.015204f32),
        ("ios_saf", "13.3", 0.0132092f32),
        ("safari", "17.4", 0.011403f32),
        ("chrome", "61", 0.011403f32),
        ("firefox", "88", 0.011403f32),
        ("firefox", "117", 0.011403f32),
        ("edge", "99", 0.011403f32),
        ("edge", "92", 0.011403f32),
        ("chrome", "77", 0.011403f32),
        ("edge", "18", 0.011403f32),
        ("chrome", "96", 0.011403f32),
        ("chrome", "95", 0.011403f32),
        ("edge", "116", 0.011403f32),
        ("chrome", "56", 0.011403f32),
        ("safari", "11.1", 0.011403f32),
        ("chrome", "50", 0.011403f32),
        ("safari", "15.2-15.3", 0.011403f32),
        ("edge", "115", 0.011403f32),
        ("samsung", "11.1-11.2", 0.0107546f32),
        ("samsung", "13.0", 0.0107546f32),
        ("samsung", "5.0-5.4", 0.0107546f32),
        ("samsung", "16.0", 0.0107546f32),
        ("ios_saf", "7.0-7.1", 0.0102738f32),
        ("ios_saf", "12.0-12.1", 0.0102738f32),
        ("ios_saf", "6.0-6.1", 0.00880612f32),
        ("safari", "15", 0.007602f32),
        ("chrome", "73", 0.007602f32),
        ("safari", "13", 0.007602f32),
        ("chrome", "71", 0.007602f32),
        ("firefox", "113", 0.007602f32),
        ("firefox", "114", 0.007602f32),
        ("edge", "107", 0.007602f32),
        ("chrome", "47", 0.007602f32),
        ("firefox", "109", 0.007602f32),
        ("opera", "105", 0.007602f32),
        ("chrome", "76", 0.007602f32),
        ("chrome", "75", 0.007602f32),
        ("firefox", "43", 0.007602f32),
        ("firefox", "44", 0.007602f32),
        ("chrome", "74", 0.007602f32),
        ("safari", "9.1", 0.007602f32),
        ("chrome", "84", 0.007602f32),
        ("edge", "17", 0.007602f32),
        ("edge", "112", 0.007602f32),
        ("edge", "111", 0.007602f32),
        ("chrome", "57", 0.007602f32),
        ("edge", "110", 0.007602f32),
        ("firefox", "91", 0.007602f32),
        ("firefox", "50", 0.007602f32),
        ("edge", "108", 0.007602f32),
        ("firefox", "53", 0.007602f32),
        ("firefox", "54", 0.007602f32),
        ("ios_saf", "9.0-9.2", 0.00733843f32),
        ("ios_saf", "10.0-10.2", 0.00587075f32),
        ("ios_saf", "13.0-13.1", 0.00440306f32),
        ("opera", "40", 0.003801f32),
        ("edge", "12", 0.003801f32),
        ("chrome", "55", 0.003801f32),
        ("firefox", "72", 0.003801f32),
        ("firefox", "59", 0.003801f32),
        ("chrome", "54", 0.003801f32),
        ("chrome", "53", 0.003801f32),
        ("firefox", "55", 0.003801f32),
        ("firefox", "112", 0.003801f32),
        ("safari", "9", 0.003801f32),
        ("safari", "8", 0.003801f32),
        ("chrome", "124", 0.003801f32),
        ("opera", "28", 0.003801f32),
        ("opera", "36", 0.003801f32),
        ("firefox", "45", 0.003801f32),
        ("chrome", "72", 0.003801f32),
        ("firefox", "94", 0.003801f32),
        ("firefox", "101", 0.003801f32),
        ("chrome", "34", 0.003801f32),
        ("edge", "106", 0.003801f32),
        ("firefox", "4", 0.003801f32),
        ("firefox", "105", 0.003801f32),
        ("firefox", "110", 0.003801f32),
        ("chrome", "63", 0.003801f32),
        ("firefox", "107", 0.003801f32),
        ("chrome", "65", 0.003801f32),
        ("edge", "14", 0.003801f32),
        ("chrome", "67", 0.003801f32),
        ("chrome", "68", 0.003801f32),
        ("edge", "15", 0.003801f32),
        ("firefox", "116", 0.003801f32),
        ("firefox", "108", 0.003801f32),
        ("ios_saf", "5.0-5.1", 0.00293537f32),
        ("ios_saf", "4.2-4.3", 0.00293537f32),
        ("ios_saf", "8.1-8.4", 0.00146769f32),
        ("android", "4.4.3-4.4.4", 0.00139827f32),
        ("android", "4.2-4.3", 0.000349567f32),
        ("android", "4.1", 0.0000582612f32),
        ("android", "4", 0.0000582612f32),
        ("android", "2.2", 0.0000582612f32),
        ("firefox", "76", 0f32),
        ("opera", "43", 0f32),
        ("chrome", "32", 0f32),
        ("chrome", "33", 0f32),
        ("chrome", "35", 0f32),
        ("chrome", "36", 0f32),
        ("chrome", "37", 0f32),
        ("chrome", "39", 0f32),
        ("chrome", "40", 0f32),
        ("chrome", "41", 0f32),
        ("chrome", "42", 0f32),
        ("chrome", "43", 0f32),
        ("chrome", "44", 0f32),
        ("chrome", "45", 0f32),
        ("chrome", "46", 0f32),
        ("chrome", "51", 0f32),
        ("chrome", "52", 0f32),
        ("chrome", "58", 0f32),
        ("chrome", "59", 0f32),
        ("chrome", "62", 0f32),
        ("chrome", "64", 0f32),
        ("chrome", "10", 0f32),
        ("chrome", "30", 0f32),
        ("chrome", "29", 0f32),
        ("chrome", "28", 0f32),
        ("chrome", "27", 0f32),
        ("chrome", "26", 0f32),
        ("chrome", "25", 0f32),
        ("chrome", "24", 0f32),
        ("chrome", "23", 0f32),
        ("chrome", "22", 0f32),
        ("chrome", "21", 0f32),
        ("chrome", "20", 0f32),
        ("chrome", "19", 0f32),
        ("chrome", "18", 0f32),
        ("chrome", "17", 0f32),
        ("chrome", "16", 0f32),
        ("chrome", "15", 0f32),
        ("chrome", "14", 0f32),
        ("chrome", "13", 0f32),
        ("chrome", "12", 0f32),
        ("chrome", "11", 0f32),
        ("firefox", "73", 0f32),
        ("chrome", "9", 0f32),
        ("chrome", "8", 0f32),
        ("chrome", "7", 0f32),
        ("chrome", "6", 0f32),
        ("chrome", "5", 0f32),
        ("chrome", "4", 0f32),
        ("firefox", "126", 0f32),
        ("firefox", "125", 0f32),
        ("firefox", "124", 0f32),
        ("firefox", "111", 0f32),
        ("firefox", "106", 0f32),
        ("firefox", "104", 0f32),
        ("firefox", "100", 0f32),
        ("firefox", "99", 0f32),
        ("firefox", "98", 0f32),
        ("firefox", "97", 0f32),
        ("firefox", "96", 0f32),
        ("firefox", "95", 0f32),
        ("firefox", "93", 0f32),
        ("firefox", "92", 0f32),
        ("firefox", "90", 0f32),
        ("firefox", "89", 0f32),
        ("firefox", "87", 0f32),
        ("firefox", "86", 0f32),
        ("firefox", "85", 0f32),
        ("chrome", "125", 0f32),
        ("safari", "3.1", 0f32),
        ("safari", "3.2", 0f32),
        ("safari", "4", 0f32),
        ("safari", "5", 0f32),
        ("safari", "5.1", 0f32),
        ("safari", "6", 0f32),
        ("safari", "6.1", 0f32),
        ("safari", "7", 0f32),
        ("safari", "7.1", 0f32),
        ("firefox", "84", 0f32),
        ("firefox", "83", 0f32),
        ("firefox", "82", 0f32),
        ("safari", "10", 0f32),
        ("safari", "10.1", 0f32),
        ("safari", "11", 0f32),
        ("firefox", "81", 0f32),
        ("safari", "12", 0f32),
        ("firefox", "80", 0f32),
        ("firefox", "79", 0f32),
        ("firefox", "77", 0f32),
        ("ie", "5.5", 0f32),
        ("firefox", "75", 0f32),
        ("firefox", "74", 0f32),
        ("firefox", "41", 0f32),
        ("firefox", "71", 0f32),
        ("firefox", "70", 0f32),
        ("firefox", "69", 0f32),
        ("firefox", "68", 0f32),
        ("firefox", "67", 0f32),
        ("firefox", "66", 0f32),
        ("firefox", "65", 0f32),
        ("firefox", "64", 0f32),
        ("firefox", "63", 0f32),
        ("firefox", "62", 0f32),
        ("firefox", "61", 0f32),
        ("firefox", "60", 0f32),
        ("firefox", "58", 0f32),
        ("firefox", "57", 0f32),
        ("firefox", "51", 0f32),
        ("firefox", "49", 0f32),
        ("safari", "TP", 0f32),
        ("opera", "9", 0f32),
        ("opera", "9.5-9.6", 0f32),
        ("opera", "10.0-10.1", 0f32),
        ("opera", "10.5", 0f32),
        ("opera", "10.6", 0f32),
        ("opera", "11", 0f32),
        ("opera", "11.1", 0f32),
        ("opera", "11.5", 0f32),
        ("opera", "11.6", 0f32),
        ("opera", "12", 0f32),
        ("opera", "12.1", 0f32),
        ("opera", "15", 0f32),
        ("opera", "16", 0f32),
        ("opera", "17", 0f32),
        ("opera", "18", 0f32),
        ("opera", "19", 0f32),
        ("opera", "20", 0f32),
        ("opera", "21", 0f32),
        ("opera", "22", 0f32),
        ("opera", "23", 0f32),
        ("opera", "24", 0f32),
        ("opera", "25", 0f32),
        ("opera", "26", 0f32),
        ("opera", "27", 0f32),
        ("firefox", "48", 0f32),
        ("opera", "29", 0f32),
        ("opera", "30", 0f32),
        ("opera", "31", 0f32),
        ("opera", "32", 0f32),
        ("opera", "33", 0f32),
        ("opera", "34", 0f32),
        ("opera", "35", 0f32),
        ("firefox", "47", 0f32),
        ("opera", "37", 0f32),
        ("opera", "38", 0f32),
        ("opera", "39", 0f32),
        ("firefox", "46", 0f32),
        ("opera", "41", 0f32),
        ("opera", "42", 0f32),
        ("chrome", "31", 0f32),
        ("opera", "44", 0f32),
        ("opera", "45", 0f32),
        ("firefox", "42", 0f32),
        ("opera", "47", 0f32),
        ("opera", "48", 0f32),
        ("opera", "49", 0f32),
        ("opera", "50", 0f32),
        ("opera", "51", 0f32),
        ("opera", "52", 0f32),
        ("opera", "53", 0f32),
        ("opera", "54", 0f32),
        ("opera", "55", 0f32),
        ("opera", "56", 0f32),
        ("opera", "57", 0f32),
        ("opera", "58", 0f32),
        ("opera", "60", 0f32),
        ("opera", "62", 0f32),
        ("opera", "63", 0f32),
        ("opera", "64", 0f32),
        ("opera", "65", 0f32),
        ("opera", "66", 0f32),
        ("opera", "67", 0f32),
        ("opera", "68", 0f32),
        ("opera", "69", 0f32),
        ("opera", "70", 0f32),
        ("opera", "71", 0f32),
        ("opera", "72", 0f32),
        ("opera", "73", 0f32),
        ("opera", "74", 0f32),
        ("opera", "75", 0f32),
        ("opera", "76", 0f32),
        ("opera", "77", 0f32),
        ("opera", "78", 0f32),
        ("opera", "79", 0f32),
        ("opera", "80", 0f32),
        ("opera", "81", 0f32),
        ("opera", "82", 0f32),
        ("opera", "83", 0f32),
        ("opera", "84", 0f32),
        ("opera", "85", 0f32),
        ("opera", "86", 0f32),
        ("opera", "87", 0f32),
        ("opera", "88", 0f32),
        ("opera", "89", 0f32),
        ("opera", "90", 0f32),
        ("opera", "91", 0f32),
        ("opera", "92", 0f32),
        ("opera", "93", 0f32),
        ("opera", "94", 0f32),
        ("ie", "6", 0f32),
        ("opera", "96", 0f32),
        ("opera", "97", 0f32),
        ("opera", "98", 0f32),
        ("opera", "99", 0f32),
        ("opera", "100", 0f32),
        ("opera", "101", 0f32),
        ("firefox", "40", 0f32),
        ("opera", "103", 0f32),
        ("opera", "104", 0f32),
        ("firefox", "39", 0f32),
        ("firefox", "38", 0f32),
        ("ios_saf", "3.2", 0f32),
        ("ios_saf", "4.0-4.1", 0f32),
        ("firefox", "37", 0f32),
        ("firefox", "36", 0f32),
        ("firefox", "35", 0f32),
        ("firefox", "34", 0f32),
        ("ios_saf", "8", 0f32),
        ("firefox", "33", 0f32),
        ("firefox", "32", 0f32),
        ("firefox", "31", 0f32),
        ("firefox", "30", 0f32),
        ("firefox", "29", 0f32),
        ("firefox", "28", 0f32),
        ("firefox", "27", 0f32),
        ("firefox", "26", 0f32),
        ("firefox", "25", 0f32),
        ("firefox", "24", 0f32),
        ("firefox", "23", 0f32),
        ("firefox", "22", 0f32),
        ("firefox", "21", 0f32),
        ("firefox", "20", 0f32),
        ("firefox", "19", 0f32),
        ("firefox", "18", 0f32),
        ("firefox", "17", 0f32),
        ("firefox", "16", 0f32),
        ("firefox", "15", 0f32),
        ("firefox", "14", 0f32),
        ("firefox", "13", 0f32),
        ("firefox", "10", 0f32),
        ("firefox", "9", 0f32),
        ("firefox", "8", 0f32),
        ("firefox", "7", 0f32),
        ("firefox", "6", 0f32),
        ("firefox", "5", 0f32),
        ("firefox", "3.6", 0f32),
        ("firefox", "3.5", 0f32),
        ("firefox", "3", 0f32),
        ("firefox", "2", 0f32),
        ("edge", "105", 0f32),
        ("edge", "104", 0f32),
        ("android", "2.1", 0f32),
        ("edge", "103", 0f32),
        ("android", "2.3", 0f32),
        ("android", "3", 0f32),
        ("edge", "102", 0f32),
        ("edge", "101", 0f32),
        ("edge", "100", 0f32),
        ("android", "4.4", 0f32),
        ("edge", "98", 0f32),
        ("edge", "97", 0f32),
        ("bb", "7", 0f32),
        ("bb", "10", 0f32),
        ("op_mob", "10", 0f32),
        ("op_mob", "11", 0f32),
        ("op_mob", "11.1", 0f32),
        ("op_mob", "11.5", 0f32),
        ("op_mob", "12", 0f32),
        ("op_mob", "12.1", 0f32),
        ("edge", "96", 0f32),
        ("edge", "95", 0f32),
        ("edge", "94", 0f32),
        ("ie_mob", "10", 0f32),
        ("ie_mob", "11", 0f32),
        ("edge", "93", 0f32),
        ("edge", "91", 0f32),
        ("edge", "90", 0f32),
        ("samsung", "6.2-6.4", 0f32),
        ("edge", "89", 0f32),
        ("samsung", "8.2", 0f32),
        ("samsung", "9.2", 0f32),
        ("samsung", "10.1", 0f32),
        ("edge", "88", 0f32),
        ("samsung", "12.0", 0f32),
        ("edge", "86", 0f32),
        ("samsung", "14.0", 0f32),
        ("samsung", "15.0", 0f32),
        ("edge", "85", 0f32),
        ("edge", "84", 0f32),
        ("edge", "83", 0f32),
        ("edge", "81", 0f32),
        ("edge", "80", 0f32),
        ("edge", "79", 0f32),
        ("edge", "16", 0f32),
        ("edge", "13", 0f32),
        ("ie", "10", 0f32),
        ("baidu", "13.18", 0f32),
        ("ie", "7", 0f32),
        ("kaios", "3.0-3.1", 0f32),
    ]
});
