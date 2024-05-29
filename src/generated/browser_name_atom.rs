pub type BrowserNameAtom = ::string_cache::Atom<BrowserNameAtomStaticSet>;
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct BrowserNameAtomStaticSet;
impl ::string_cache::StaticAtomSet for BrowserNameAtomStaticSet {
    fn get() -> &'static ::string_cache::PhfStrSet {
        static SET: ::string_cache::PhfStrSet = ::string_cache::PhfStrSet {
            key: 12913932095322966823u64,
            disps: &[(16u32, 10u32), (10u32, 17u32), (1u32, 0u32), (0u32, 18u32)],
            atoms: &[
                "and_qq", "chrome", "samsung", "baidu", "android", "bb", "safari", "firefox", "",
                "and_ff", "ios_saf", "ie", "kaios", "edge", "and_chr", "op_mob", "op_mini",
                "ie_mob", "opera", "and_uc",
            ],
            hashes: &[
                3206542493u32,
                477636712u32,
                1851079857u32,
                258459304u32,
                587358767u32,
                350703895u32,
                476630482u32,
                4293802053u32,
                4082073077u32,
                216596461u32,
                2773389377u32,
                2131378110u32,
                1050666745u32,
                1630221347u32,
                3123672440u32,
                574769774u32,
                2412682903u32,
                3966619011u32,
                3084820249u32,
                956886648u32,
            ],
        };
        &SET
    }
    fn empty_string_index() -> u32 {
        8u32
    }
}
pub const ATOM_BROWSERNAMEATOM__61_6E_64_5F_71_71: BrowserNameAtom =
    BrowserNameAtom::pack_static(0u32);
pub const ATOM_BROWSERNAMEATOM__63_68_72_6F_6D_65: BrowserNameAtom =
    BrowserNameAtom::pack_static(1u32);
pub const ATOM_BROWSERNAMEATOM__73_61_6D_73_75_6E_67: BrowserNameAtom =
    BrowserNameAtom::pack_static(2u32);
pub const ATOM_BROWSERNAMEATOM__62_61_69_64_75: BrowserNameAtom =
    BrowserNameAtom::pack_static(3u32);
pub const ATOM_BROWSERNAMEATOM__61_6E_64_72_6F_69_64: BrowserNameAtom =
    BrowserNameAtom::pack_static(4u32);
pub const ATOM_BROWSERNAMEATOM__62_62: BrowserNameAtom = BrowserNameAtom::pack_static(5u32);
pub const ATOM_BROWSERNAMEATOM__73_61_66_61_72_69: BrowserNameAtom =
    BrowserNameAtom::pack_static(6u32);
pub const ATOM_BROWSERNAMEATOM__66_69_72_65_66_6F_78: BrowserNameAtom =
    BrowserNameAtom::pack_static(7u32);
pub const ATOM_BROWSERNAMEATOM_: BrowserNameAtom = BrowserNameAtom::pack_static(8u32);
pub const ATOM_BROWSERNAMEATOM__61_6E_64_5F_66_66: BrowserNameAtom =
    BrowserNameAtom::pack_static(9u32);
pub const ATOM_BROWSERNAMEATOM__69_6F_73_5F_73_61_66: BrowserNameAtom =
    BrowserNameAtom::pack_static(10u32);
pub const ATOM_BROWSERNAMEATOM__69_65: BrowserNameAtom = BrowserNameAtom::pack_static(11u32);
pub const ATOM_BROWSERNAMEATOM__6B_61_69_6F_73: BrowserNameAtom =
    BrowserNameAtom::pack_static(12u32);
pub const ATOM_BROWSERNAMEATOM__65_64_67_65: BrowserNameAtom = BrowserNameAtom::pack_static(13u32);
pub const ATOM_BROWSERNAMEATOM__61_6E_64_5F_63_68_72: BrowserNameAtom =
    BrowserNameAtom::pack_static(14u32);
pub const ATOM_BROWSERNAMEATOM__6F_70_5F_6D_6F_62: BrowserNameAtom =
    BrowserNameAtom::pack_static(15u32);
pub const ATOM_BROWSERNAMEATOM__6F_70_5F_6D_69_6E_69: BrowserNameAtom =
    BrowserNameAtom::pack_static(16u32);
pub const ATOM_BROWSERNAMEATOM__69_65_5F_6D_6F_62: BrowserNameAtom =
    BrowserNameAtom::pack_static(17u32);
pub const ATOM_BROWSERNAMEATOM__6F_70_65_72_61: BrowserNameAtom =
    BrowserNameAtom::pack_static(18u32);
pub const ATOM_BROWSERNAMEATOM__61_6E_64_5F_75_63: BrowserNameAtom =
    BrowserNameAtom::pack_static(19u32);
#[macro_export]
macro_rules! browser_name_atom {
    ("and_qq") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__61_6E_64_5F_71_71
    };
    ("chrome") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__63_68_72_6F_6D_65
    };
    ("samsung") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__73_61_6D_73_75_6E_67
    };
    ("baidu") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__62_61_69_64_75
    };
    ("android") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__61_6E_64_72_6F_69_64
    };
    ("bb") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__62_62
    };
    ("safari") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__73_61_66_61_72_69
    };
    ("firefox") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__66_69_72_65_66_6F_78
    };
    ("") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM_
    };
    ("and_ff") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__61_6E_64_5F_66_66
    };
    ("ios_saf") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__69_6F_73_5F_73_61_66
    };
    ("ie") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__69_65
    };
    ("kaios") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__6B_61_69_6F_73
    };
    ("edge") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__65_64_67_65
    };
    ("and_chr") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__61_6E_64_5F_63_68_72
    };
    ("op_mob") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__6F_70_5F_6D_6F_62
    };
    ("op_mini") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__6F_70_5F_6D_69_6E_69
    };
    ("ie_mob") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__69_65_5F_6D_6F_62
    };
    ("opera") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__6F_70_65_72_61
    };
    ("and_uc") => {
        $crate::data::browser_name::ATOM_BROWSERNAMEATOM__61_6E_64_5F_75_63
    };
}
