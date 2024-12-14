use crate::data::electron::ElectronVersion;
pub static ELECTRON_VERSIONS: &[(ElectronVersion, &str)] = &[
    (ElectronVersion::new(0u16, 20u16), "39"),
    (ElectronVersion::new(0u16, 21u16), "41"),
    (ElectronVersion::new(0u16, 22u16), "41"),
    (ElectronVersion::new(0u16, 23u16), "41"),
    (ElectronVersion::new(0u16, 24u16), "41"),
    (ElectronVersion::new(0u16, 25u16), "42"),
    (ElectronVersion::new(0u16, 26u16), "42"),
    (ElectronVersion::new(0u16, 27u16), "43"),
    (ElectronVersion::new(0u16, 28u16), "43"),
    (ElectronVersion::new(0u16, 29u16), "43"),
    (ElectronVersion::new(0u16, 30u16), "44"),
    (ElectronVersion::new(0u16, 31u16), "45"),
    (ElectronVersion::new(0u16, 32u16), "45"),
    (ElectronVersion::new(0u16, 33u16), "45"),
    (ElectronVersion::new(0u16, 34u16), "45"),
    (ElectronVersion::new(0u16, 35u16), "45"),
    (ElectronVersion::new(0u16, 36u16), "47"),
    (ElectronVersion::new(0u16, 37u16), "49"),
    (ElectronVersion::new(1u16, 0u16), "49"),
    (ElectronVersion::new(1u16, 1u16), "50"),
    (ElectronVersion::new(1u16, 2u16), "51"),
    (ElectronVersion::new(1u16, 3u16), "52"),
    (ElectronVersion::new(1u16, 4u16), "53"),
    (ElectronVersion::new(1u16, 5u16), "54"),
    (ElectronVersion::new(1u16, 6u16), "56"),
    (ElectronVersion::new(1u16, 7u16), "58"),
    (ElectronVersion::new(1u16, 8u16), "59"),
    (ElectronVersion::new(2u16, 0u16), "61"),
    (ElectronVersion::new(2u16, 1u16), "61"),
    (ElectronVersion::new(3u16, 0u16), "66"),
    (ElectronVersion::new(3u16, 1u16), "66"),
    (ElectronVersion::new(4u16, 0u16), "69"),
    (ElectronVersion::new(4u16, 1u16), "69"),
    (ElectronVersion::new(4u16, 2u16), "69"),
    (ElectronVersion::new(5u16, 0u16), "73"),
    (ElectronVersion::new(6u16, 0u16), "76"),
    (ElectronVersion::new(6u16, 1u16), "76"),
    (ElectronVersion::new(7u16, 0u16), "78"),
    (ElectronVersion::new(7u16, 1u16), "78"),
    (ElectronVersion::new(7u16, 2u16), "78"),
    (ElectronVersion::new(7u16, 3u16), "78"),
    (ElectronVersion::new(8u16, 0u16), "80"),
    (ElectronVersion::new(8u16, 1u16), "80"),
    (ElectronVersion::new(8u16, 2u16), "80"),
    (ElectronVersion::new(8u16, 3u16), "80"),
    (ElectronVersion::new(8u16, 4u16), "80"),
    (ElectronVersion::new(8u16, 5u16), "80"),
    (ElectronVersion::new(9u16, 0u16), "83"),
    (ElectronVersion::new(9u16, 1u16), "83"),
    (ElectronVersion::new(9u16, 2u16), "83"),
    (ElectronVersion::new(9u16, 3u16), "83"),
    (ElectronVersion::new(9u16, 4u16), "83"),
    (ElectronVersion::new(10u16, 0u16), "85"),
    (ElectronVersion::new(10u16, 1u16), "85"),
    (ElectronVersion::new(10u16, 2u16), "85"),
    (ElectronVersion::new(10u16, 3u16), "85"),
    (ElectronVersion::new(10u16, 4u16), "85"),
    (ElectronVersion::new(11u16, 0u16), "87"),
    (ElectronVersion::new(11u16, 1u16), "87"),
    (ElectronVersion::new(11u16, 2u16), "87"),
    (ElectronVersion::new(11u16, 3u16), "87"),
    (ElectronVersion::new(11u16, 4u16), "87"),
    (ElectronVersion::new(11u16, 5u16), "87"),
    (ElectronVersion::new(12u16, 0u16), "89"),
    (ElectronVersion::new(12u16, 1u16), "89"),
    (ElectronVersion::new(12u16, 2u16), "89"),
    (ElectronVersion::new(13u16, 0u16), "91"),
    (ElectronVersion::new(13u16, 1u16), "91"),
    (ElectronVersion::new(13u16, 2u16), "91"),
    (ElectronVersion::new(13u16, 3u16), "91"),
    (ElectronVersion::new(13u16, 4u16), "91"),
    (ElectronVersion::new(13u16, 5u16), "91"),
    (ElectronVersion::new(13u16, 6u16), "91"),
    (ElectronVersion::new(14u16, 0u16), "93"),
    (ElectronVersion::new(14u16, 1u16), "93"),
    (ElectronVersion::new(14u16, 2u16), "93"),
    (ElectronVersion::new(15u16, 0u16), "94"),
    (ElectronVersion::new(15u16, 1u16), "94"),
    (ElectronVersion::new(15u16, 2u16), "94"),
    (ElectronVersion::new(15u16, 3u16), "94"),
    (ElectronVersion::new(15u16, 4u16), "94"),
    (ElectronVersion::new(15u16, 5u16), "94"),
    (ElectronVersion::new(16u16, 0u16), "96"),
    (ElectronVersion::new(16u16, 1u16), "96"),
    (ElectronVersion::new(16u16, 2u16), "96"),
    (ElectronVersion::new(17u16, 0u16), "98"),
    (ElectronVersion::new(17u16, 1u16), "98"),
    (ElectronVersion::new(17u16, 2u16), "98"),
    (ElectronVersion::new(17u16, 3u16), "98"),
    (ElectronVersion::new(17u16, 4u16), "98"),
    (ElectronVersion::new(18u16, 0u16), "100"),
    (ElectronVersion::new(18u16, 1u16), "100"),
    (ElectronVersion::new(18u16, 2u16), "100"),
    (ElectronVersion::new(18u16, 3u16), "100"),
    (ElectronVersion::new(19u16, 0u16), "102"),
    (ElectronVersion::new(19u16, 1u16), "102"),
    (ElectronVersion::new(20u16, 0u16), "104"),
    (ElectronVersion::new(20u16, 1u16), "104"),
    (ElectronVersion::new(20u16, 2u16), "104"),
    (ElectronVersion::new(20u16, 3u16), "104"),
    (ElectronVersion::new(21u16, 0u16), "106"),
    (ElectronVersion::new(21u16, 1u16), "106"),
    (ElectronVersion::new(21u16, 2u16), "106"),
    (ElectronVersion::new(21u16, 3u16), "106"),
    (ElectronVersion::new(21u16, 4u16), "106"),
    (ElectronVersion::new(22u16, 0u16), "108"),
    (ElectronVersion::new(22u16, 1u16), "108"),
    (ElectronVersion::new(22u16, 2u16), "108"),
    (ElectronVersion::new(22u16, 3u16), "108"),
    (ElectronVersion::new(23u16, 0u16), "110"),
    (ElectronVersion::new(23u16, 1u16), "110"),
    (ElectronVersion::new(23u16, 2u16), "110"),
    (ElectronVersion::new(23u16, 3u16), "110"),
    (ElectronVersion::new(24u16, 0u16), "112"),
    (ElectronVersion::new(24u16, 1u16), "112"),
    (ElectronVersion::new(24u16, 2u16), "112"),
    (ElectronVersion::new(24u16, 3u16), "112"),
    (ElectronVersion::new(24u16, 4u16), "112"),
    (ElectronVersion::new(24u16, 5u16), "112"),
    (ElectronVersion::new(24u16, 6u16), "112"),
    (ElectronVersion::new(24u16, 7u16), "112"),
    (ElectronVersion::new(24u16, 8u16), "112"),
    (ElectronVersion::new(25u16, 0u16), "114"),
    (ElectronVersion::new(25u16, 1u16), "114"),
    (ElectronVersion::new(25u16, 2u16), "114"),
    (ElectronVersion::new(25u16, 3u16), "114"),
    (ElectronVersion::new(25u16, 4u16), "114"),
    (ElectronVersion::new(25u16, 5u16), "114"),
    (ElectronVersion::new(25u16, 6u16), "114"),
    (ElectronVersion::new(25u16, 7u16), "114"),
    (ElectronVersion::new(25u16, 8u16), "114"),
    (ElectronVersion::new(25u16, 9u16), "114"),
    (ElectronVersion::new(26u16, 0u16), "116"),
    (ElectronVersion::new(26u16, 1u16), "116"),
    (ElectronVersion::new(26u16, 2u16), "116"),
    (ElectronVersion::new(26u16, 3u16), "116"),
    (ElectronVersion::new(26u16, 4u16), "116"),
    (ElectronVersion::new(26u16, 5u16), "116"),
    (ElectronVersion::new(26u16, 6u16), "116"),
    (ElectronVersion::new(27u16, 0u16), "118"),
    (ElectronVersion::new(27u16, 1u16), "118"),
    (ElectronVersion::new(27u16, 2u16), "118"),
    (ElectronVersion::new(27u16, 3u16), "118"),
    (ElectronVersion::new(28u16, 0u16), "120"),
    (ElectronVersion::new(28u16, 1u16), "120"),
    (ElectronVersion::new(28u16, 2u16), "120"),
    (ElectronVersion::new(28u16, 3u16), "120"),
    (ElectronVersion::new(29u16, 0u16), "122"),
    (ElectronVersion::new(29u16, 1u16), "122"),
    (ElectronVersion::new(29u16, 2u16), "122"),
    (ElectronVersion::new(29u16, 3u16), "122"),
    (ElectronVersion::new(29u16, 4u16), "122"),
    (ElectronVersion::new(30u16, 0u16), "124"),
    (ElectronVersion::new(30u16, 1u16), "124"),
    (ElectronVersion::new(30u16, 2u16), "124"),
    (ElectronVersion::new(30u16, 3u16), "124"),
    (ElectronVersion::new(30u16, 4u16), "124"),
    (ElectronVersion::new(30u16, 5u16), "124"),
    (ElectronVersion::new(31u16, 0u16), "126"),
    (ElectronVersion::new(31u16, 1u16), "126"),
    (ElectronVersion::new(31u16, 2u16), "126"),
    (ElectronVersion::new(31u16, 3u16), "126"),
    (ElectronVersion::new(31u16, 4u16), "126"),
    (ElectronVersion::new(31u16, 5u16), "126"),
    (ElectronVersion::new(31u16, 6u16), "126"),
    (ElectronVersion::new(31u16, 7u16), "126"),
    (ElectronVersion::new(32u16, 0u16), "128"),
    (ElectronVersion::new(32u16, 1u16), "128"),
    (ElectronVersion::new(32u16, 2u16), "128"),
    (ElectronVersion::new(33u16, 0u16), "130"),
    (ElectronVersion::new(33u16, 1u16), "130"),
    (ElectronVersion::new(33u16, 2u16), "130"),
    (ElectronVersion::new(33u16, 3u16), "130"),
    (ElectronVersion::new(34u16, 0u16), "132"),
];
