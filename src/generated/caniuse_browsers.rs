use crate::data::caniuse::{BrowserStat, CaniuseData, VersionDetail};
use rustc_hash::FxHashMap;
use std::sync::OnceLock;
pub fn caniuse_browsers() -> &'static CaniuseData {
    static CANIUSE_BROWSERS: OnceLock<CaniuseData> = OnceLock::new();
    CANIUSE_BROWSERS.get_or_init(|| {
        FxHashMap::from_iter([
            (
                "ie",
                BrowserStat {
                    name: "ie",
                    version_list: vec![
                        VersionDetail {
                            version: "5.5",
                            global_usage: 0f32,
                            release_date: Some(962323200i64),
                        },
                        VersionDetail {
                            version: "6",
                            global_usage: 0f32,
                            release_date: Some(998870400i64),
                        },
                        VersionDetail {
                            version: "7",
                            global_usage: 0f32,
                            release_date: Some(1161129600i64),
                        },
                        VersionDetail {
                            version: "8",
                            global_usage: 0f32,
                            release_date: Some(1237420800i64),
                        },
                        VersionDetail {
                            version: "9",
                            global_usage: 0.0324821f32,
                            release_date: Some(1300060800i64),
                        },
                        VersionDetail {
                            version: "10",
                            global_usage: 0f32,
                            release_date: Some(1346716800i64),
                        },
                        VersionDetail {
                            version: "11",
                            global_usage: 0.438508f32,
                            release_date: Some(1381968000i64),
                        },
                    ],
                },
            ),
            (
                "edge",
                BrowserStat {
                    name: "edge",
                    version_list: vec![
                        VersionDetail {
                            version: "12",
                            global_usage: 0f32,
                            release_date: Some(1438128000i64),
                        },
                        VersionDetail {
                            version: "13",
                            global_usage: 0f32,
                            release_date: Some(1447286400i64),
                        },
                        VersionDetail {
                            version: "14",
                            global_usage: 0f32,
                            release_date: Some(1470096000i64),
                        },
                        VersionDetail {
                            version: "15",
                            global_usage: 0f32,
                            release_date: Some(1491868800i64),
                        },
                        VersionDetail {
                            version: "16",
                            global_usage: 0f32,
                            release_date: Some(1508198400i64),
                        },
                        VersionDetail {
                            version: "17",
                            global_usage: 0f32,
                            release_date: Some(1525046400i64),
                        },
                        VersionDetail {
                            version: "18",
                            global_usage: 0.097821f32,
                            release_date: Some(1542067200i64),
                        },
                        VersionDetail {
                            version: "79",
                            global_usage: 0f32,
                            release_date: Some(1579046400i64),
                        },
                        VersionDetail {
                            version: "80",
                            global_usage: 0f32,
                            release_date: Some(1581033600i64),
                        },
                        VersionDetail {
                            version: "81",
                            global_usage: 0f32,
                            release_date: Some(1586736000i64),
                        },
                        VersionDetail {
                            version: "83",
                            global_usage: 0f32,
                            release_date: Some(1590019200i64),
                        },
                        VersionDetail {
                            version: "84",
                            global_usage: 0f32,
                            release_date: Some(1594857600i64),
                        },
                        VersionDetail {
                            version: "85",
                            global_usage: 0f32,
                            release_date: Some(1598486400i64),
                        },
                        VersionDetail {
                            version: "86",
                            global_usage: 0f32,
                            release_date: Some(1602201600i64),
                        },
                        VersionDetail {
                            version: "87",
                            global_usage: 0f32,
                            release_date: Some(1605830400i64),
                        },
                        VersionDetail {
                            version: "88",
                            global_usage: 0f32,
                            release_date: Some(1611360000i64),
                        },
                        VersionDetail {
                            version: "89",
                            global_usage: 0f32,
                            release_date: Some(1614816000i64),
                        },
                        VersionDetail {
                            version: "90",
                            global_usage: 0f32,
                            release_date: Some(1618358400i64),
                        },
                        VersionDetail {
                            version: "91",
                            global_usage: 0f32,
                            release_date: Some(1622073600i64),
                        },
                        VersionDetail {
                            version: "92",
                            global_usage: 0.010869f32,
                            release_date: Some(1626912000i64),
                        },
                        VersionDetail {
                            version: "93",
                            global_usage: 0f32,
                            release_date: Some(1630627200i64),
                        },
                        VersionDetail {
                            version: "94",
                            global_usage: 0f32,
                            release_date: Some(1632441600i64),
                        },
                        VersionDetail {
                            version: "95",
                            global_usage: 0f32,
                            release_date: Some(1634774400i64),
                        },
                        VersionDetail {
                            version: "96",
                            global_usage: 0f32,
                            release_date: Some(1637539200i64),
                        },
                        VersionDetail {
                            version: "97",
                            global_usage: 0f32,
                            release_date: Some(1641427200i64),
                        },
                        VersionDetail {
                            version: "98",
                            global_usage: 0f32,
                            release_date: Some(1643932800i64),
                        },
                        VersionDetail {
                            version: "99",
                            global_usage: 0f32,
                            release_date: Some(1646265600i64),
                        },
                        VersionDetail {
                            version: "100",
                            global_usage: 0f32,
                            release_date: Some(1649635200i64),
                        },
                        VersionDetail {
                            version: "101",
                            global_usage: 0f32,
                            release_date: Some(1651190400i64),
                        },
                        VersionDetail {
                            version: "102",
                            global_usage: 0f32,
                            release_date: Some(1653955200i64),
                        },
                        VersionDetail {
                            version: "103",
                            global_usage: 0f32,
                            release_date: Some(1655942400i64),
                        },
                        VersionDetail {
                            version: "104",
                            global_usage: 0f32,
                            release_date: Some(1659657600i64),
                        },
                        VersionDetail {
                            version: "105",
                            global_usage: 0f32,
                            release_date: Some(1661990400i64),
                        },
                        VersionDetail {
                            version: "106",
                            global_usage: 0f32,
                            release_date: Some(1664755200i64),
                        },
                        VersionDetail {
                            version: "107",
                            global_usage: 0f32,
                            release_date: Some(1666915200i64),
                        },
                        VersionDetail {
                            version: "108",
                            global_usage: 0.003623f32,
                            release_date: Some(1670198400i64),
                        },
                        VersionDetail {
                            version: "109",
                            global_usage: 0.047099f32,
                            release_date: Some(1673481600i64),
                        },
                        VersionDetail {
                            version: "110",
                            global_usage: 0f32,
                            release_date: Some(1675900800i64),
                        },
                        VersionDetail {
                            version: "111",
                            global_usage: 0f32,
                            release_date: Some(1678665600i64),
                        },
                        VersionDetail {
                            version: "112",
                            global_usage: 0f32,
                            release_date: Some(1680825600i64),
                        },
                        VersionDetail {
                            version: "113",
                            global_usage: 0.007246f32,
                            release_date: Some(1683158400i64),
                        },
                        VersionDetail {
                            version: "114",
                            global_usage: 0.014492f32,
                            release_date: Some(1685664000i64),
                        },
                        VersionDetail {
                            version: "115",
                            global_usage: 0.007246f32,
                            release_date: Some(1689897600i64),
                        },
                        VersionDetail {
                            version: "116",
                            global_usage: 0f32,
                            release_date: Some(1692576000i64),
                        },
                        VersionDetail {
                            version: "117",
                            global_usage: 0.003623f32,
                            release_date: Some(1694649600i64),
                        },
                        VersionDetail {
                            version: "118",
                            global_usage: 0.003623f32,
                            release_date: Some(1697155200i64),
                        },
                        VersionDetail {
                            version: "119",
                            global_usage: 0.003623f32,
                            release_date: Some(1698969600i64),
                        },
                        VersionDetail {
                            version: "120",
                            global_usage: 0.03623f32,
                            release_date: Some(1701993600i64),
                        },
                        VersionDetail {
                            version: "121",
                            global_usage: 0.007246f32,
                            release_date: Some(1706227200i64),
                        },
                        VersionDetail {
                            version: "122",
                            global_usage: 0.014492f32,
                            release_date: Some(1708732800i64),
                        },
                        VersionDetail {
                            version: "123",
                            global_usage: 0.007246f32,
                            release_date: Some(1711152000i64),
                        },
                        VersionDetail {
                            version: "124",
                            global_usage: 0.007246f32,
                            release_date: Some(1713398400i64),
                        },
                        VersionDetail {
                            version: "125",
                            global_usage: 0.007246f32,
                            release_date: Some(1715990400i64),
                        },
                        VersionDetail {
                            version: "126",
                            global_usage: 0.021738f32,
                            release_date: Some(1718841600i64),
                        },
                        VersionDetail {
                            version: "127",
                            global_usage: 0.014492f32,
                            release_date: Some(1721865600i64),
                        },
                        VersionDetail {
                            version: "128",
                            global_usage: 0.014492f32,
                            release_date: Some(1724371200i64),
                        },
                        VersionDetail {
                            version: "129",
                            global_usage: 0.014492f32,
                            release_date: Some(1726704000i64),
                        },
                        VersionDetail {
                            version: "130",
                            global_usage: 0.025361f32,
                            release_date: Some(1729123200i64),
                        },
                        VersionDetail {
                            version: "131",
                            global_usage: 0.065214f32,
                            release_date: Some(1731542400i64),
                        },
                        VersionDetail {
                            version: "132",
                            global_usage: 0.079706f32,
                            release_date: Some(1737417600i64),
                        },
                        VersionDetail {
                            version: "133",
                            global_usage: 1.34051f32,
                            release_date: Some(1740614400i64),
                        },
                        VersionDetail {
                            version: "134",
                            global_usage: 3.0252f32,
                            release_date: Some(1741219200i64),
                        },
                        VersionDetail {
                            version: "135",
                            global_usage: 0f32,
                            release_date: Some(1743984000i64),
                        },
                        VersionDetail { version: "136", global_usage: 0f32, release_date: None },
                    ],
                },
            ),
            (
                "firefox",
                BrowserStat {
                    name: "firefox",
                    version_list: vec![
                        VersionDetail {
                            version: "2",
                            global_usage: 0.007246f32,
                            release_date: Some(1161648000i64),
                        },
                        VersionDetail {
                            version: "3",
                            global_usage: 0f32,
                            release_date: Some(1213660800i64),
                        },
                        VersionDetail {
                            version: "3.5",
                            global_usage: 0f32,
                            release_date: Some(1246320000i64),
                        },
                        VersionDetail {
                            version: "3.6",
                            global_usage: 0f32,
                            release_date: Some(1264032000i64),
                        },
                        VersionDetail {
                            version: "4",
                            global_usage: 0f32,
                            release_date: Some(1300752000i64),
                        },
                        VersionDetail {
                            version: "5",
                            global_usage: 0f32,
                            release_date: Some(1308614400i64),
                        },
                        VersionDetail {
                            version: "6",
                            global_usage: 0f32,
                            release_date: Some(1313452800i64),
                        },
                        VersionDetail {
                            version: "7",
                            global_usage: 0f32,
                            release_date: Some(1317081600i64),
                        },
                        VersionDetail {
                            version: "8",
                            global_usage: 0f32,
                            release_date: Some(1317081600i64),
                        },
                        VersionDetail {
                            version: "9",
                            global_usage: 0f32,
                            release_date: Some(1320710400i64),
                        },
                        VersionDetail {
                            version: "10",
                            global_usage: 0f32,
                            release_date: Some(1324339200i64),
                        },
                        VersionDetail {
                            version: "11",
                            global_usage: 0.025361f32,
                            release_date: Some(1327968000i64),
                        },
                        VersionDetail {
                            version: "12",
                            global_usage: 0f32,
                            release_date: Some(1331596800i64),
                        },
                        VersionDetail {
                            version: "13",
                            global_usage: 0f32,
                            release_date: Some(1335225600i64),
                        },
                        VersionDetail {
                            version: "14",
                            global_usage: 0f32,
                            release_date: Some(1338854400i64),
                        },
                        VersionDetail {
                            version: "15",
                            global_usage: 0f32,
                            release_date: Some(1342483200i64),
                        },
                        VersionDetail {
                            version: "16",
                            global_usage: 0f32,
                            release_date: Some(1346112000i64),
                        },
                        VersionDetail {
                            version: "17",
                            global_usage: 0f32,
                            release_date: Some(1349740800i64),
                        },
                        VersionDetail {
                            version: "18",
                            global_usage: 0f32,
                            release_date: Some(1353628800i64),
                        },
                        VersionDetail {
                            version: "19",
                            global_usage: 0f32,
                            release_date: Some(1357603200i64),
                        },
                        VersionDetail {
                            version: "20",
                            global_usage: 0f32,
                            release_date: Some(1361232000i64),
                        },
                        VersionDetail {
                            version: "21",
                            global_usage: 0f32,
                            release_date: Some(1364860800i64),
                        },
                        VersionDetail {
                            version: "22",
                            global_usage: 0f32,
                            release_date: Some(1368489600i64),
                        },
                        VersionDetail {
                            version: "23",
                            global_usage: 0f32,
                            release_date: Some(1372118400i64),
                        },
                        VersionDetail {
                            version: "24",
                            global_usage: 0f32,
                            release_date: Some(1375747200i64),
                        },
                        VersionDetail {
                            version: "25",
                            global_usage: 0f32,
                            release_date: Some(1379376000i64),
                        },
                        VersionDetail {
                            version: "26",
                            global_usage: 0f32,
                            release_date: Some(1386633600i64),
                        },
                        VersionDetail {
                            version: "27",
                            global_usage: 0f32,
                            release_date: Some(1391472000i64),
                        },
                        VersionDetail {
                            version: "28",
                            global_usage: 0f32,
                            release_date: Some(1395100800i64),
                        },
                        VersionDetail {
                            version: "29",
                            global_usage: 0f32,
                            release_date: Some(1398729600i64),
                        },
                        VersionDetail {
                            version: "30",
                            global_usage: 0f32,
                            release_date: Some(1402358400i64),
                        },
                        VersionDetail {
                            version: "31",
                            global_usage: 0f32,
                            release_date: Some(1405987200i64),
                        },
                        VersionDetail {
                            version: "32",
                            global_usage: 0f32,
                            release_date: Some(1409616000i64),
                        },
                        VersionDetail {
                            version: "33",
                            global_usage: 0f32,
                            release_date: Some(1413244800i64),
                        },
                        VersionDetail {
                            version: "34",
                            global_usage: 0f32,
                            release_date: Some(1417392000i64),
                        },
                        VersionDetail {
                            version: "35",
                            global_usage: 0f32,
                            release_date: Some(1421107200i64),
                        },
                        VersionDetail {
                            version: "36",
                            global_usage: 0f32,
                            release_date: Some(1424736000i64),
                        },
                        VersionDetail {
                            version: "37",
                            global_usage: 0f32,
                            release_date: Some(1428278400i64),
                        },
                        VersionDetail {
                            version: "38",
                            global_usage: 0f32,
                            release_date: Some(1431475200i64),
                        },
                        VersionDetail {
                            version: "39",
                            global_usage: 0f32,
                            release_date: Some(1435881600i64),
                        },
                        VersionDetail {
                            version: "40",
                            global_usage: 0f32,
                            release_date: Some(1439251200i64),
                        },
                        VersionDetail {
                            version: "41",
                            global_usage: 0f32,
                            release_date: Some(1442880000i64),
                        },
                        VersionDetail {
                            version: "42",
                            global_usage: 0f32,
                            release_date: Some(1446508800i64),
                        },
                        VersionDetail {
                            version: "43",
                            global_usage: 0f32,
                            release_date: Some(1450137600i64),
                        },
                        VersionDetail {
                            version: "44",
                            global_usage: 0.003623f32,
                            release_date: Some(1453852800i64),
                        },
                        VersionDetail {
                            version: "45",
                            global_usage: 0f32,
                            release_date: Some(1457395200i64),
                        },
                        VersionDetail {
                            version: "46",
                            global_usage: 0f32,
                            release_date: Some(1461628800i64),
                        },
                        VersionDetail {
                            version: "47",
                            global_usage: 0f32,
                            release_date: Some(1465257600i64),
                        },
                        VersionDetail {
                            version: "48",
                            global_usage: 0f32,
                            release_date: Some(1470096000i64),
                        },
                        VersionDetail {
                            version: "49",
                            global_usage: 0f32,
                            release_date: Some(1474329600i64),
                        },
                        VersionDetail {
                            version: "50",
                            global_usage: 0f32,
                            release_date: Some(1479168000i64),
                        },
                        VersionDetail {
                            version: "51",
                            global_usage: 0f32,
                            release_date: Some(1485216000i64),
                        },
                        VersionDetail {
                            version: "52",
                            global_usage: 0.028984f32,
                            release_date: Some(1488844800i64),
                        },
                        VersionDetail {
                            version: "53",
                            global_usage: 0.014492f32,
                            release_date: Some(1492560000i64),
                        },
                        VersionDetail {
                            version: "54",
                            global_usage: 0f32,
                            release_date: Some(1497312000i64),
                        },
                        VersionDetail {
                            version: "55",
                            global_usage: 0.007246f32,
                            release_date: Some(1502150400i64),
                        },
                        VersionDetail {
                            version: "56",
                            global_usage: 0.007246f32,
                            release_date: Some(1506556800i64),
                        },
                        VersionDetail {
                            version: "57",
                            global_usage: 0f32,
                            release_date: Some(1510617600i64),
                        },
                        VersionDetail {
                            version: "58",
                            global_usage: 0f32,
                            release_date: Some(1516665600i64),
                        },
                        VersionDetail {
                            version: "59",
                            global_usage: 0.007246f32,
                            release_date: Some(1520985600i64),
                        },
                        VersionDetail {
                            version: "60",
                            global_usage: 0f32,
                            release_date: Some(1525824000i64),
                        },
                        VersionDetail {
                            version: "61",
                            global_usage: 0f32,
                            release_date: Some(1529971200i64),
                        },
                        VersionDetail {
                            version: "62",
                            global_usage: 0f32,
                            release_date: Some(1536105600i64),
                        },
                        VersionDetail {
                            version: "63",
                            global_usage: 0f32,
                            release_date: Some(1540252800i64),
                        },
                        VersionDetail {
                            version: "64",
                            global_usage: 0f32,
                            release_date: Some(1544486400i64),
                        },
                        VersionDetail {
                            version: "65",
                            global_usage: 0f32,
                            release_date: Some(1548720000i64),
                        },
                        VersionDetail {
                            version: "66",
                            global_usage: 0f32,
                            release_date: Some(1552953600i64),
                        },
                        VersionDetail {
                            version: "67",
                            global_usage: 0f32,
                            release_date: Some(1558396800i64),
                        },
                        VersionDetail {
                            version: "68",
                            global_usage: 0f32,
                            release_date: Some(1562630400i64),
                        },
                        VersionDetail {
                            version: "69",
                            global_usage: 0f32,
                            release_date: Some(1567468800i64),
                        },
                        VersionDetail {
                            version: "70",
                            global_usage: 0f32,
                            release_date: Some(1571788800i64),
                        },
                        VersionDetail {
                            version: "71",
                            global_usage: 0f32,
                            release_date: Some(1575331200i64),
                        },
                        VersionDetail {
                            version: "72",
                            global_usage: 0.003623f32,
                            release_date: Some(1578355200i64),
                        },
                        VersionDetail {
                            version: "73",
                            global_usage: 0f32,
                            release_date: Some(1581379200i64),
                        },
                        VersionDetail {
                            version: "74",
                            global_usage: 0f32,
                            release_date: Some(1583798400i64),
                        },
                        VersionDetail {
                            version: "75",
                            global_usage: 0f32,
                            release_date: Some(1586304000i64),
                        },
                        VersionDetail {
                            version: "76",
                            global_usage: 0f32,
                            release_date: Some(1588636800i64),
                        },
                        VersionDetail {
                            version: "77",
                            global_usage: 0f32,
                            release_date: Some(1591056000i64),
                        },
                        VersionDetail {
                            version: "78",
                            global_usage: 0.010869f32,
                            release_date: Some(1593475200i64),
                        },
                        VersionDetail {
                            version: "79",
                            global_usage: 0f32,
                            release_date: Some(1595894400i64),
                        },
                        VersionDetail {
                            version: "80",
                            global_usage: 0f32,
                            release_date: Some(1598313600i64),
                        },
                        VersionDetail {
                            version: "81",
                            global_usage: 0f32,
                            release_date: Some(1600732800i64),
                        },
                        VersionDetail {
                            version: "82",
                            global_usage: 0f32,
                            release_date: Some(1603152000i64),
                        },
                        VersionDetail {
                            version: "83",
                            global_usage: 0f32,
                            release_date: Some(1605571200i64),
                        },
                        VersionDetail {
                            version: "84",
                            global_usage: 0f32,
                            release_date: Some(1607990400i64),
                        },
                        VersionDetail {
                            version: "85",
                            global_usage: 0f32,
                            release_date: Some(1611619200i64),
                        },
                        VersionDetail {
                            version: "86",
                            global_usage: 0f32,
                            release_date: Some(1614038400i64),
                        },
                        VersionDetail {
                            version: "87",
                            global_usage: 0f32,
                            release_date: Some(1616457600i64),
                        },
                        VersionDetail {
                            version: "88",
                            global_usage: 0.007246f32,
                            release_date: Some(1618790400i64),
                        },
                        VersionDetail {
                            version: "89",
                            global_usage: 0f32,
                            release_date: Some(1622505600i64),
                        },
                        VersionDetail {
                            version: "90",
                            global_usage: 0f32,
                            release_date: Some(1626134400i64),
                        },
                        VersionDetail {
                            version: "91",
                            global_usage: 0f32,
                            release_date: Some(1628553600i64),
                        },
                        VersionDetail {
                            version: "92",
                            global_usage: 0f32,
                            release_date: Some(1630972800i64),
                        },
                        VersionDetail {
                            version: "93",
                            global_usage: 0f32,
                            release_date: Some(1633392000i64),
                        },
                        VersionDetail {
                            version: "94",
                            global_usage: 0.003623f32,
                            release_date: Some(1635811200i64),
                        },
                        VersionDetail {
                            version: "95",
                            global_usage: 0f32,
                            release_date: Some(1638835200i64),
                        },
                        VersionDetail {
                            version: "96",
                            global_usage: 0f32,
                            release_date: Some(1641859200i64),
                        },
                        VersionDetail {
                            version: "97",
                            global_usage: 0f32,
                            release_date: Some(1644364800i64),
                        },
                        VersionDetail {
                            version: "98",
                            global_usage: 0f32,
                            release_date: Some(1646697600i64),
                        },
                        VersionDetail {
                            version: "99",
                            global_usage: 0f32,
                            release_date: Some(1649116800i64),
                        },
                        VersionDetail {
                            version: "100",
                            global_usage: 0f32,
                            release_date: Some(1651536000i64),
                        },
                        VersionDetail {
                            version: "101",
                            global_usage: 0f32,
                            release_date: Some(1653955200i64),
                        },
                        VersionDetail {
                            version: "102",
                            global_usage: 0f32,
                            release_date: Some(1656374400i64),
                        },
                        VersionDetail {
                            version: "103",
                            global_usage: 0f32,
                            release_date: Some(1658793600i64),
                        },
                        VersionDetail {
                            version: "104",
                            global_usage: 0f32,
                            release_date: Some(1661212800i64),
                        },
                        VersionDetail {
                            version: "105",
                            global_usage: 0f32,
                            release_date: Some(1663632000i64),
                        },
                        VersionDetail {
                            version: "106",
                            global_usage: 0f32,
                            release_date: Some(1666051200i64),
                        },
                        VersionDetail {
                            version: "107",
                            global_usage: 0f32,
                            release_date: Some(1668470400i64),
                        },
                        VersionDetail {
                            version: "108",
                            global_usage: 0f32,
                            release_date: Some(1670889600i64),
                        },
                        VersionDetail {
                            version: "109",
                            global_usage: 0.003623f32,
                            release_date: Some(1673913600i64),
                        },
                        VersionDetail {
                            version: "110",
                            global_usage: 0f32,
                            release_date: Some(1676332800i64),
                        },
                        VersionDetail {
                            version: "111",
                            global_usage: 0f32,
                            release_date: Some(1678752000i64),
                        },
                        VersionDetail {
                            version: "112",
                            global_usage: 0f32,
                            release_date: Some(1681171200i64),
                        },
                        VersionDetail {
                            version: "113",
                            global_usage: 0.003623f32,
                            release_date: Some(1683590400i64),
                        },
                        VersionDetail {
                            version: "114",
                            global_usage: 0f32,
                            release_date: Some(1686009600i64),
                        },
                        VersionDetail {
                            version: "115",
                            global_usage: 0.213757f32,
                            release_date: Some(1688428800i64),
                        },
                        VersionDetail {
                            version: "116",
                            global_usage: 0f32,
                            release_date: Some(1690848000i64),
                        },
                        VersionDetail {
                            version: "117",
                            global_usage: 0f32,
                            release_date: Some(1693267200i64),
                        },
                        VersionDetail {
                            version: "118",
                            global_usage: 0.094198f32,
                            release_date: Some(1695686400i64),
                        },
                        VersionDetail {
                            version: "119",
                            global_usage: 0f32,
                            release_date: Some(1698105600i64),
                        },
                        VersionDetail {
                            version: "120",
                            global_usage: 0.003623f32,
                            release_date: Some(1700524800i64),
                        },
                        VersionDetail {
                            version: "121",
                            global_usage: 0f32,
                            release_date: Some(1702944000i64),
                        },
                        VersionDetail {
                            version: "122",
                            global_usage: 0f32,
                            release_date: Some(1705968000i64),
                        },
                        VersionDetail {
                            version: "123",
                            global_usage: 0f32,
                            release_date: Some(1708387200i64),
                        },
                        VersionDetail {
                            version: "124",
                            global_usage: 0f32,
                            release_date: Some(1710806400i64),
                        },
                        VersionDetail {
                            version: "125",
                            global_usage: 0.014492f32,
                            release_date: Some(1713225600i64),
                        },
                        VersionDetail {
                            version: "126",
                            global_usage: 0f32,
                            release_date: Some(1715644800i64),
                        },
                        VersionDetail {
                            version: "127",
                            global_usage: 0.007246f32,
                            release_date: Some(1718064000i64),
                        },
                        VersionDetail {
                            version: "128",
                            global_usage: 0.083329f32,
                            release_date: Some(1720483200i64),
                        },
                        VersionDetail {
                            version: "129",
                            global_usage: 0f32,
                            release_date: Some(1722902400i64),
                        },
                        VersionDetail {
                            version: "130",
                            global_usage: 0f32,
                            release_date: Some(1725321600i64),
                        },
                        VersionDetail {
                            version: "131",
                            global_usage: 0.003623f32,
                            release_date: Some(1727740800i64),
                        },
                        VersionDetail {
                            version: "132",
                            global_usage: 0.007246f32,
                            release_date: Some(1730160000i64),
                        },
                        VersionDetail {
                            version: "133",
                            global_usage: 0.018115f32,
                            release_date: Some(1732579200i64),
                        },
                        VersionDetail {
                            version: "134",
                            global_usage: 0.025361f32,
                            release_date: Some(1736208000i64),
                        },
                        VersionDetail {
                            version: "135",
                            global_usage: 0.347808f32,
                            release_date: Some(1738627200i64),
                        },
                        VersionDetail {
                            version: "136",
                            global_usage: 1.11951f32,
                            release_date: Some(1741046400i64),
                        },
                        VersionDetail {
                            version: "137",
                            global_usage: 0.007246f32,
                            release_date: Some(1743465600i64),
                        },
                        VersionDetail {
                            version: "138",
                            global_usage: 0f32,
                            release_date: Some(1745884800i64),
                        },
                        VersionDetail { version: "139", global_usage: 0f32, release_date: None },
                        VersionDetail { version: "140", global_usage: 0f32, release_date: None },
                        VersionDetail { version: "141", global_usage: 0f32, release_date: None },
                    ],
                },
            ),
            (
                "chrome",
                BrowserStat {
                    name: "chrome",
                    version_list: vec![
                        VersionDetail {
                            version: "4",
                            global_usage: 0f32,
                            release_date: Some(1264377600i64),
                        },
                        VersionDetail {
                            version: "5",
                            global_usage: 0f32,
                            release_date: Some(1274745600i64),
                        },
                        VersionDetail {
                            version: "6",
                            global_usage: 0f32,
                            release_date: Some(1283385600i64),
                        },
                        VersionDetail {
                            version: "7",
                            global_usage: 0f32,
                            release_date: Some(1287619200i64),
                        },
                        VersionDetail {
                            version: "8",
                            global_usage: 0f32,
                            release_date: Some(1291248000i64),
                        },
                        VersionDetail {
                            version: "9",
                            global_usage: 0f32,
                            release_date: Some(1296777600i64),
                        },
                        VersionDetail {
                            version: "10",
                            global_usage: 0f32,
                            release_date: Some(1299542400i64),
                        },
                        VersionDetail {
                            version: "11",
                            global_usage: 0f32,
                            release_date: Some(1303862400i64),
                        },
                        VersionDetail {
                            version: "12",
                            global_usage: 0f32,
                            release_date: Some(1307404800i64),
                        },
                        VersionDetail {
                            version: "13",
                            global_usage: 0f32,
                            release_date: Some(1312243200i64),
                        },
                        VersionDetail {
                            version: "14",
                            global_usage: 0f32,
                            release_date: Some(1316131200i64),
                        },
                        VersionDetail {
                            version: "15",
                            global_usage: 0f32,
                            release_date: Some(1316131200i64),
                        },
                        VersionDetail {
                            version: "16",
                            global_usage: 0f32,
                            release_date: Some(1319500800i64),
                        },
                        VersionDetail {
                            version: "17",
                            global_usage: 0f32,
                            release_date: Some(1323734400i64),
                        },
                        VersionDetail {
                            version: "18",
                            global_usage: 0f32,
                            release_date: Some(1328659200i64),
                        },
                        VersionDetail {
                            version: "19",
                            global_usage: 0f32,
                            release_date: Some(1332892800i64),
                        },
                        VersionDetail {
                            version: "20",
                            global_usage: 0f32,
                            release_date: Some(1337040000i64),
                        },
                        VersionDetail {
                            version: "21",
                            global_usage: 0f32,
                            release_date: Some(1340668800i64),
                        },
                        VersionDetail {
                            version: "22",
                            global_usage: 0f32,
                            release_date: Some(1343692800i64),
                        },
                        VersionDetail {
                            version: "23",
                            global_usage: 0f32,
                            release_date: Some(1348531200i64),
                        },
                        VersionDetail {
                            version: "24",
                            global_usage: 0f32,
                            release_date: Some(1352246400i64),
                        },
                        VersionDetail {
                            version: "25",
                            global_usage: 0f32,
                            release_date: Some(1357862400i64),
                        },
                        VersionDetail {
                            version: "26",
                            global_usage: 0f32,
                            release_date: Some(1361404800i64),
                        },
                        VersionDetail {
                            version: "27",
                            global_usage: 0f32,
                            release_date: Some(1364428800i64),
                        },
                        VersionDetail {
                            version: "28",
                            global_usage: 0f32,
                            release_date: Some(1369094400i64),
                        },
                        VersionDetail {
                            version: "29",
                            global_usage: 0f32,
                            release_date: Some(1374105600i64),
                        },
                        VersionDetail {
                            version: "30",
                            global_usage: 0f32,
                            release_date: Some(1376956800i64),
                        },
                        VersionDetail {
                            version: "31",
                            global_usage: 0f32,
                            release_date: Some(1384214400i64),
                        },
                        VersionDetail {
                            version: "32",
                            global_usage: 0f32,
                            release_date: Some(1389657600i64),
                        },
                        VersionDetail {
                            version: "33",
                            global_usage: 0f32,
                            release_date: Some(1392940800i64),
                        },
                        VersionDetail {
                            version: "34",
                            global_usage: 0f32,
                            release_date: Some(1397001600i64),
                        },
                        VersionDetail {
                            version: "35",
                            global_usage: 0f32,
                            release_date: Some(1400544000i64),
                        },
                        VersionDetail {
                            version: "36",
                            global_usage: 0f32,
                            release_date: Some(1405468800i64),
                        },
                        VersionDetail {
                            version: "37",
                            global_usage: 0f32,
                            release_date: Some(1409011200i64),
                        },
                        VersionDetail {
                            version: "38",
                            global_usage: 0.003623f32,
                            release_date: Some(1412640000i64),
                        },
                        VersionDetail {
                            version: "39",
                            global_usage: 0.007246f32,
                            release_date: Some(1416268800i64),
                        },
                        VersionDetail {
                            version: "40",
                            global_usage: 0.003623f32,
                            release_date: Some(1421798400i64),
                        },
                        VersionDetail {
                            version: "41",
                            global_usage: 0.007246f32,
                            release_date: Some(1425513600i64),
                        },
                        VersionDetail {
                            version: "42",
                            global_usage: 0.007246f32,
                            release_date: Some(1429401600i64),
                        },
                        VersionDetail {
                            version: "43",
                            global_usage: 0.007246f32,
                            release_date: Some(1432080000i64),
                        },
                        VersionDetail {
                            version: "44",
                            global_usage: 0.007246f32,
                            release_date: Some(1437523200i64),
                        },
                        VersionDetail {
                            version: "45",
                            global_usage: 0.007246f32,
                            release_date: Some(1441152000i64),
                        },
                        VersionDetail {
                            version: "46",
                            global_usage: 0.003623f32,
                            release_date: Some(1444780800i64),
                        },
                        VersionDetail {
                            version: "47",
                            global_usage: 0.007246f32,
                            release_date: Some(1449014400i64),
                        },
                        VersionDetail {
                            version: "48",
                            global_usage: 0.018115f32,
                            release_date: Some(1453248000i64),
                        },
                        VersionDetail {
                            version: "49",
                            global_usage: 0.018115f32,
                            release_date: Some(1456963200i64),
                        },
                        VersionDetail {
                            version: "50",
                            global_usage: 0.007246f32,
                            release_date: Some(1460592000i64),
                        },
                        VersionDetail {
                            version: "51",
                            global_usage: 0.007246f32,
                            release_date: Some(1464134400i64),
                        },
                        VersionDetail {
                            version: "52",
                            global_usage: 0.010869f32,
                            release_date: Some(1469059200i64),
                        },
                        VersionDetail {
                            version: "53",
                            global_usage: 0.007246f32,
                            release_date: Some(1472601600i64),
                        },
                        VersionDetail {
                            version: "54",
                            global_usage: 0.007246f32,
                            release_date: Some(1476230400i64),
                        },
                        VersionDetail {
                            version: "55",
                            global_usage: 0.007246f32,
                            release_date: Some(1480550400i64),
                        },
                        VersionDetail {
                            version: "56",
                            global_usage: 0.014492f32,
                            release_date: Some(1485302400i64),
                        },
                        VersionDetail {
                            version: "57",
                            global_usage: 0.007246f32,
                            release_date: Some(1489017600i64),
                        },
                        VersionDetail {
                            version: "58",
                            global_usage: 0.010869f32,
                            release_date: Some(1492560000i64),
                        },
                        VersionDetail {
                            version: "59",
                            global_usage: 0.007246f32,
                            release_date: Some(1496707200i64),
                        },
                        VersionDetail {
                            version: "60",
                            global_usage: 0.007246f32,
                            release_date: Some(1500940800i64),
                        },
                        VersionDetail {
                            version: "61",
                            global_usage: 0f32,
                            release_date: Some(1504569600i64),
                        },
                        VersionDetail {
                            version: "62",
                            global_usage: 0f32,
                            release_date: Some(1508198400i64),
                        },
                        VersionDetail {
                            version: "63",
                            global_usage: 0f32,
                            release_date: Some(1512518400i64),
                        },
                        VersionDetail {
                            version: "64",
                            global_usage: 0f32,
                            release_date: Some(1516752000i64),
                        },
                        VersionDetail {
                            version: "65",
                            global_usage: 0f32,
                            release_date: Some(1520294400i64),
                        },
                        VersionDetail {
                            version: "66",
                            global_usage: 0.021738f32,
                            release_date: Some(1523923200i64),
                        },
                        VersionDetail {
                            version: "67",
                            global_usage: 0f32,
                            release_date: Some(1527552000i64),
                        },
                        VersionDetail {
                            version: "68",
                            global_usage: 0f32,
                            release_date: Some(1532390400i64),
                        },
                        VersionDetail {
                            version: "69",
                            global_usage: 0.010869f32,
                            release_date: Some(1536019200i64),
                        },
                        VersionDetail {
                            version: "70",
                            global_usage: 0.010869f32,
                            release_date: Some(1539648000i64),
                        },
                        VersionDetail {
                            version: "71",
                            global_usage: 0f32,
                            release_date: Some(1543968000i64),
                        },
                        VersionDetail {
                            version: "72",
                            global_usage: 0f32,
                            release_date: Some(1548720000i64),
                        },
                        VersionDetail {
                            version: "73",
                            global_usage: 0.007246f32,
                            release_date: Some(1552348800i64),
                        },
                        VersionDetail {
                            version: "74",
                            global_usage: 0.003623f32,
                            release_date: Some(1555977600i64),
                        },
                        VersionDetail {
                            version: "75",
                            global_usage: 0.007246f32,
                            release_date: Some(1559606400i64),
                        },
                        VersionDetail {
                            version: "76",
                            global_usage: 0.003623f32,
                            release_date: Some(1564444800i64),
                        },
                        VersionDetail {
                            version: "77",
                            global_usage: 0.014492f32,
                            release_date: Some(1568073600i64),
                        },
                        VersionDetail {
                            version: "78",
                            global_usage: 0.010869f32,
                            release_date: Some(1571702400i64),
                        },
                        VersionDetail {
                            version: "79",
                            global_usage: 0.068837f32,
                            release_date: Some(1575936000i64),
                        },
                        VersionDetail {
                            version: "80",
                            global_usage: 0.010869f32,
                            release_date: Some(1580860800i64),
                        },
                        VersionDetail {
                            version: "81",
                            global_usage: 0.014492f32,
                            release_date: Some(1586304000i64),
                        },
                        VersionDetail {
                            version: "83",
                            global_usage: 0.028984f32,
                            release_date: Some(1589846400i64),
                        },
                        VersionDetail {
                            version: "84",
                            global_usage: 0.003623f32,
                            release_date: Some(1594684800i64),
                        },
                        VersionDetail {
                            version: "85",
                            global_usage: 0.010869f32,
                            release_date: Some(1598313600i64),
                        },
                        VersionDetail {
                            version: "86",
                            global_usage: 0.014492f32,
                            release_date: Some(1601942400i64),
                        },
                        VersionDetail {
                            version: "87",
                            global_usage: 0.057968f32,
                            release_date: Some(1605571200i64),
                        },
                        VersionDetail {
                            version: "88",
                            global_usage: 0.014492f32,
                            release_date: Some(1611014400i64),
                        },
                        VersionDetail {
                            version: "89",
                            global_usage: 0.003623f32,
                            release_date: Some(1614556800i64),
                        },
                        VersionDetail {
                            version: "90",
                            global_usage: 0.007246f32,
                            release_date: Some(1618272000i64),
                        },
                        VersionDetail {
                            version: "91",
                            global_usage: 0.03623f32,
                            release_date: Some(1621987200i64),
                        },
                        VersionDetail {
                            version: "92",
                            global_usage: 0.010869f32,
                            release_date: Some(1626739200i64),
                        },
                        VersionDetail {
                            version: "93",
                            global_usage: 0.014492f32,
                            release_date: Some(1630368000i64),
                        },
                        VersionDetail {
                            version: "94",
                            global_usage: 0.028984f32,
                            release_date: Some(1632268800i64),
                        },
                        VersionDetail {
                            version: "95",
                            global_usage: 0.007246f32,
                            release_date: Some(1634601600i64),
                        },
                        VersionDetail {
                            version: "96",
                            global_usage: 0.007246f32,
                            release_date: Some(1637020800i64),
                        },
                        VersionDetail {
                            version: "97",
                            global_usage: 0.018115f32,
                            release_date: Some(1641340800i64),
                        },
                        VersionDetail {
                            version: "98",
                            global_usage: 0.03623f32,
                            release_date: Some(1643673600i64),
                        },
                        VersionDetail {
                            version: "99",
                            global_usage: 0.010869f32,
                            release_date: Some(1646092800i64),
                        },
                        VersionDetail {
                            version: "100",
                            global_usage: 0.028984f32,
                            release_date: Some(1648512000i64),
                        },
                        VersionDetail {
                            version: "101",
                            global_usage: 0.014492f32,
                            release_date: Some(1650931200i64),
                        },
                        VersionDetail {
                            version: "102",
                            global_usage: 0.014492f32,
                            release_date: Some(1653350400i64),
                        },
                        VersionDetail {
                            version: "103",
                            global_usage: 0.076083f32,
                            release_date: Some(1655769600i64),
                        },
                        VersionDetail {
                            version: "104",
                            global_usage: 0.050722f32,
                            release_date: Some(1659398400i64),
                        },
                        VersionDetail {
                            version: "105",
                            global_usage: 0.010869f32,
                            release_date: Some(1661817600i64),
                        },
                        VersionDetail {
                            version: "106",
                            global_usage: 0.021738f32,
                            release_date: Some(1664236800i64),
                        },
                        VersionDetail {
                            version: "107",
                            global_usage: 0.025361f32,
                            release_date: Some(1666656000i64),
                        },
                        VersionDetail {
                            version: "108",
                            global_usage: 0.039853f32,
                            release_date: Some(1669680000i64),
                        },
                        VersionDetail {
                            version: "109",
                            global_usage: 0.912996f32,
                            release_date: Some(1673308800i64),
                        },
                        VersionDetail {
                            version: "110",
                            global_usage: 0.018115f32,
                            release_date: Some(1675728000i64),
                        },
                        VersionDetail {
                            version: "111",
                            global_usage: 0.03623f32,
                            release_date: Some(1678147200i64),
                        },
                        VersionDetail {
                            version: "112",
                            global_usage: 0.03623f32,
                            release_date: Some(1680566400i64),
                        },
                        VersionDetail {
                            version: "113",
                            global_usage: 0.10869f32,
                            release_date: Some(1682985600i64),
                        },
                        VersionDetail {
                            version: "114",
                            global_usage: 0.054345f32,
                            release_date: Some(1685404800i64),
                        },
                        VersionDetail {
                            version: "115",
                            global_usage: 0.032607f32,
                            release_date: Some(1689724800i64),
                        },
                        VersionDetail {
                            version: "116",
                            global_usage: 0.101444f32,
                            release_date: Some(1692057600i64),
                        },
                        VersionDetail {
                            version: "117",
                            global_usage: 0.094198f32,
                            release_date: Some(1694476800i64),
                        },
                        VersionDetail {
                            version: "118",
                            global_usage: 0.057968f32,
                            release_date: Some(1696896000i64),
                        },
                        VersionDetail {
                            version: "119",
                            global_usage: 0.03623f32,
                            release_date: Some(1698710400i64),
                        },
                        VersionDetail {
                            version: "120",
                            global_usage: 0.086952f32,
                            release_date: Some(1701993600i64),
                        },
                        VersionDetail {
                            version: "121",
                            global_usage: 0.094198f32,
                            release_date: Some(1705968000i64),
                        },
                        VersionDetail {
                            version: "122",
                            global_usage: 0.076083f32,
                            release_date: Some(1708387200i64),
                        },
                        VersionDetail {
                            version: "123",
                            global_usage: 0.086952f32,
                            release_date: Some(1710806400i64),
                        },
                        VersionDetail {
                            version: "124",
                            global_usage: 0.123182f32,
                            release_date: Some(1713225600i64),
                        },
                        VersionDetail {
                            version: "125",
                            global_usage: 0.344185f32,
                            release_date: Some(1715644800i64),
                        },
                        VersionDetail {
                            version: "126",
                            global_usage: 0.152166f32,
                            release_date: Some(1718064000i64),
                        },
                        VersionDetail {
                            version: "127",
                            global_usage: 0.101444f32,
                            release_date: Some(1721174400i64),
                        },
                        VersionDetail {
                            version: "128",
                            global_usage: 0.130428f32,
                            release_date: Some(1724112000i64),
                        },
                        VersionDetail {
                            version: "129",
                            global_usage: 0.101444f32,
                            release_date: Some(1726531200i64),
                        },
                        VersionDetail {
                            version: "130",
                            global_usage: 0.152166f32,
                            release_date: Some(1728950400i64),
                        },
                        VersionDetail {
                            version: "131",
                            global_usage: 1.17747f32,
                            release_date: Some(1731369600i64),
                        },
                        VersionDetail {
                            version: "132",
                            global_usage: 0.815175f32,
                            release_date: Some(1736812800i64),
                        },
                        VersionDetail {
                            version: "133",
                            global_usage: 5.89462f32,
                            release_date: Some(1738627200i64),
                        },
                        VersionDetail {
                            version: "134",
                            global_usage: 9.91615f32,
                            release_date: Some(1741046400i64),
                        },
                        VersionDetail {
                            version: "135",
                            global_usage: 0.021738f32,
                            release_date: Some(1743465600i64),
                        },
                        VersionDetail {
                            version: "136",
                            global_usage: 0.014492f32,
                            release_date: Some(1745884800i64),
                        },
                        VersionDetail { version: "137", global_usage: 0f32, release_date: None },
                        VersionDetail { version: "138", global_usage: 0f32, release_date: None },
                        VersionDetail { version: "139", global_usage: 0f32, release_date: None },
                    ],
                },
            ),
            (
                "safari",
                BrowserStat {
                    name: "safari",
                    version_list: vec![
                        VersionDetail {
                            version: "3.1",
                            global_usage: 0f32,
                            release_date: Some(1205798400i64),
                        },
                        VersionDetail {
                            version: "3.2",
                            global_usage: 0f32,
                            release_date: Some(1226534400i64),
                        },
                        VersionDetail {
                            version: "4",
                            global_usage: 0f32,
                            release_date: Some(1244419200i64),
                        },
                        VersionDetail {
                            version: "5",
                            global_usage: 0f32,
                            release_date: Some(1275868800i64),
                        },
                        VersionDetail {
                            version: "5.1",
                            global_usage: 0f32,
                            release_date: Some(1311120000i64),
                        },
                        VersionDetail {
                            version: "6",
                            global_usage: 0f32,
                            release_date: Some(1343174400i64),
                        },
                        VersionDetail {
                            version: "6.1",
                            global_usage: 0f32,
                            release_date: Some(1382400000i64),
                        },
                        VersionDetail {
                            version: "7",
                            global_usage: 0f32,
                            release_date: Some(1382400000i64),
                        },
                        VersionDetail {
                            version: "7.1",
                            global_usage: 0f32,
                            release_date: Some(1410998400i64),
                        },
                        VersionDetail {
                            version: "8",
                            global_usage: 0f32,
                            release_date: Some(1413417600i64),
                        },
                        VersionDetail {
                            version: "9",
                            global_usage: 0f32,
                            release_date: Some(1443657600i64),
                        },
                        VersionDetail {
                            version: "9.1",
                            global_usage: 0f32,
                            release_date: Some(1458518400i64),
                        },
                        VersionDetail {
                            version: "10",
                            global_usage: 0f32,
                            release_date: Some(1474329600i64),
                        },
                        VersionDetail {
                            version: "10.1",
                            global_usage: 0f32,
                            release_date: Some(1490572800i64),
                        },
                        VersionDetail {
                            version: "11",
                            global_usage: 0f32,
                            release_date: Some(1505779200i64),
                        },
                        VersionDetail {
                            version: "11.1",
                            global_usage: 0.007246f32,
                            release_date: Some(1522281600i64),
                        },
                        VersionDetail {
                            version: "12",
                            global_usage: 0f32,
                            release_date: Some(1537142400i64),
                        },
                        VersionDetail {
                            version: "12.1",
                            global_usage: 0.007246f32,
                            release_date: Some(1553472000i64),
                        },
                        VersionDetail {
                            version: "13",
                            global_usage: 0f32,
                            release_date: Some(1568851200i64),
                        },
                        VersionDetail {
                            version: "13.1",
                            global_usage: 0.032607f32,
                            release_date: Some(1585008000i64),
                        },
                        VersionDetail {
                            version: "14",
                            global_usage: 0.014492f32,
                            release_date: Some(1600214400i64),
                        },
                        VersionDetail {
                            version: "14.1",
                            global_usage: 0.043476f32,
                            release_date: Some(1619395200i64),
                        },
                        VersionDetail {
                            version: "15",
                            global_usage: 0.003623f32,
                            release_date: Some(1632096000i64),
                        },
                        VersionDetail {
                            version: "15.1",
                            global_usage: 0.014492f32,
                            release_date: Some(1635292800i64),
                        },
                        VersionDetail {
                            version: "15.2-15.3",
                            global_usage: 0.003623f32,
                            release_date: Some(1639353600i64),
                        },
                        VersionDetail {
                            version: "15.4",
                            global_usage: 0.010869f32,
                            release_date: Some(1647216000i64),
                        },
                        VersionDetail {
                            version: "15.5",
                            global_usage: 0.014492f32,
                            release_date: Some(1652745600i64),
                        },
                        VersionDetail {
                            version: "15.6",
                            global_usage: 0.148543f32,
                            release_date: Some(1658275200i64),
                        },
                        VersionDetail {
                            version: "16.0",
                            global_usage: 0.032607f32,
                            release_date: Some(1662940800i64),
                        },
                        VersionDetail {
                            version: "16.1",
                            global_usage: 0.021738f32,
                            release_date: Some(1666569600i64),
                        },
                        VersionDetail {
                            version: "16.2",
                            global_usage: 0.018115f32,
                            release_date: Some(1670889600i64),
                        },
                        VersionDetail {
                            version: "16.3",
                            global_usage: 0.039853f32,
                            release_date: Some(1674432000i64),
                        },
                        VersionDetail {
                            version: "16.4",
                            global_usage: 0.014492f32,
                            release_date: Some(1679875200i64),
                        },
                        VersionDetail {
                            version: "16.5",
                            global_usage: 0.025361f32,
                            release_date: Some(1684368000i64),
                        },
                        VersionDetail {
                            version: "16.6",
                            global_usage: 0.199265f32,
                            release_date: Some(1690156800i64),
                        },
                        VersionDetail {
                            version: "17.0",
                            global_usage: 0.010869f32,
                            release_date: Some(1695686400i64),
                        },
                        VersionDetail {
                            version: "17.1",
                            global_usage: 0.123182f32,
                            release_date: Some(1698192000i64),
                        },
                        VersionDetail {
                            version: "17.2",
                            global_usage: 0.018115f32,
                            release_date: Some(1702252800i64),
                        },
                        VersionDetail {
                            version: "17.3",
                            global_usage: 0.021738f32,
                            release_date: Some(1705881600i64),
                        },
                        VersionDetail {
                            version: "17.4",
                            global_usage: 0.050722f32,
                            release_date: Some(1709596800i64),
                        },
                        VersionDetail {
                            version: "17.5",
                            global_usage: 0.086952f32,
                            release_date: Some(1715558400i64),
                        },
                        VersionDetail {
                            version: "17.6",
                            global_usage: 0.264479f32,
                            release_date: Some(1722211200i64),
                        },
                        VersionDetail {
                            version: "18.0",
                            global_usage: 0.03623f32,
                            release_date: Some(1726444800i64),
                        },
                        VersionDetail {
                            version: "18.1",
                            global_usage: 0.115936f32,
                            release_date: Some(1730073600i64),
                        },
                        VersionDetail {
                            version: "18.2",
                            global_usage: 0.057968f32,
                            release_date: Some(1733875200i64),
                        },
                        VersionDetail {
                            version: "18.3",
                            global_usage: 1.4021f32,
                            release_date: Some(1737936000i64),
                        },
                        VersionDetail {
                            version: "18.4",
                            global_usage: 0.018115f32,
                            release_date: Some(1743379200i64),
                        },
                        VersionDetail { version: "18.5", global_usage: 0f32, release_date: None },
                        VersionDetail { version: "TP", global_usage: 0f32, release_date: None },
                    ],
                },
            ),
            (
                "opera",
                BrowserStat {
                    name: "opera",
                    version_list: vec![
                        VersionDetail {
                            version: "9",
                            global_usage: 0f32,
                            release_date: Some(1150761600i64),
                        },
                        VersionDetail {
                            version: "9.5-9.6",
                            global_usage: 0f32,
                            release_date: Some(1223424000i64),
                        },
                        VersionDetail {
                            version: "10.0-10.1",
                            global_usage: 0f32,
                            release_date: Some(1251763200i64),
                        },
                        VersionDetail {
                            version: "10.5",
                            global_usage: 0f32,
                            release_date: Some(1267488000i64),
                        },
                        VersionDetail {
                            version: "10.6",
                            global_usage: 0f32,
                            release_date: Some(1277942400i64),
                        },
                        VersionDetail {
                            version: "11",
                            global_usage: 0f32,
                            release_date: Some(1292457600i64),
                        },
                        VersionDetail {
                            version: "11.1",
                            global_usage: 0f32,
                            release_date: Some(1302566400i64),
                        },
                        VersionDetail {
                            version: "11.5",
                            global_usage: 0f32,
                            release_date: Some(1309219200i64),
                        },
                        VersionDetail {
                            version: "11.6",
                            global_usage: 0f32,
                            release_date: Some(1323129600i64),
                        },
                        VersionDetail {
                            version: "12",
                            global_usage: 0f32,
                            release_date: Some(1323129600i64),
                        },
                        VersionDetail {
                            version: "12.1",
                            global_usage: 0f32,
                            release_date: Some(1352073600i64),
                        },
                        VersionDetail {
                            version: "15",
                            global_usage: 0f32,
                            release_date: Some(1372723200i64),
                        },
                        VersionDetail {
                            version: "16",
                            global_usage: 0f32,
                            release_date: Some(1377561600i64),
                        },
                        VersionDetail {
                            version: "17",
                            global_usage: 0f32,
                            release_date: Some(1381104000i64),
                        },
                        VersionDetail {
                            version: "18",
                            global_usage: 0f32,
                            release_date: Some(1386288000i64),
                        },
                        VersionDetail {
                            version: "19",
                            global_usage: 0f32,
                            release_date: Some(1390867200i64),
                        },
                        VersionDetail {
                            version: "20",
                            global_usage: 0f32,
                            release_date: Some(1393891200i64),
                        },
                        VersionDetail {
                            version: "21",
                            global_usage: 0f32,
                            release_date: Some(1399334400i64),
                        },
                        VersionDetail {
                            version: "22",
                            global_usage: 0f32,
                            release_date: Some(1401753600i64),
                        },
                        VersionDetail {
                            version: "23",
                            global_usage: 0f32,
                            release_date: Some(1405987200i64),
                        },
                        VersionDetail {
                            version: "24",
                            global_usage: 0f32,
                            release_date: Some(1409616000i64),
                        },
                        VersionDetail {
                            version: "25",
                            global_usage: 0f32,
                            release_date: Some(1413331200i64),
                        },
                        VersionDetail {
                            version: "26",
                            global_usage: 0f32,
                            release_date: Some(1417132800i64),
                        },
                        VersionDetail {
                            version: "27",
                            global_usage: 0f32,
                            release_date: Some(1422316800i64),
                        },
                        VersionDetail {
                            version: "28",
                            global_usage: 0f32,
                            release_date: Some(1425945600i64),
                        },
                        VersionDetail {
                            version: "29",
                            global_usage: 0f32,
                            release_date: Some(1430179200i64),
                        },
                        VersionDetail {
                            version: "30",
                            global_usage: 0f32,
                            release_date: Some(1433808000i64),
                        },
                        VersionDetail {
                            version: "31",
                            global_usage: 0f32,
                            release_date: Some(1438646400i64),
                        },
                        VersionDetail {
                            version: "32",
                            global_usage: 0f32,
                            release_date: Some(1442448000i64),
                        },
                        VersionDetail {
                            version: "33",
                            global_usage: 0f32,
                            release_date: Some(1445904000i64),
                        },
                        VersionDetail {
                            version: "34",
                            global_usage: 0f32,
                            release_date: Some(1449100800i64),
                        },
                        VersionDetail {
                            version: "35",
                            global_usage: 0f32,
                            release_date: Some(1454371200i64),
                        },
                        VersionDetail {
                            version: "36",
                            global_usage: 0f32,
                            release_date: Some(1457308800i64),
                        },
                        VersionDetail {
                            version: "37",
                            global_usage: 0f32,
                            release_date: Some(1462320000i64),
                        },
                        VersionDetail {
                            version: "38",
                            global_usage: 0f32,
                            release_date: Some(1465344000i64),
                        },
                        VersionDetail {
                            version: "39",
                            global_usage: 0f32,
                            release_date: Some(1470096000i64),
                        },
                        VersionDetail {
                            version: "40",
                            global_usage: 0.003623f32,
                            release_date: Some(1474329600i64),
                        },
                        VersionDetail {
                            version: "41",
                            global_usage: 0f32,
                            release_date: Some(1477267200i64),
                        },
                        VersionDetail {
                            version: "42",
                            global_usage: 0f32,
                            release_date: Some(1481587200i64),
                        },
                        VersionDetail {
                            version: "43",
                            global_usage: 0f32,
                            release_date: Some(1486425600i64),
                        },
                        VersionDetail {
                            version: "44",
                            global_usage: 0f32,
                            release_date: Some(1490054400i64),
                        },
                        VersionDetail {
                            version: "45",
                            global_usage: 0f32,
                            release_date: Some(1494374400i64),
                        },
                        VersionDetail {
                            version: "46",
                            global_usage: 0.010869f32,
                            release_date: Some(1498003200i64),
                        },
                        VersionDetail {
                            version: "47",
                            global_usage: 0f32,
                            release_date: Some(1502236800i64),
                        },
                        VersionDetail {
                            version: "48",
                            global_usage: 0f32,
                            release_date: Some(1506470400i64),
                        },
                        VersionDetail {
                            version: "49",
                            global_usage: 0f32,
                            release_date: Some(1510099200i64),
                        },
                        VersionDetail {
                            version: "50",
                            global_usage: 0f32,
                            release_date: Some(1515024000i64),
                        },
                        VersionDetail {
                            version: "51",
                            global_usage: 0f32,
                            release_date: Some(1517961600i64),
                        },
                        VersionDetail {
                            version: "52",
                            global_usage: 0f32,
                            release_date: Some(1521676800i64),
                        },
                        VersionDetail {
                            version: "53",
                            global_usage: 0f32,
                            release_date: Some(1525910400i64),
                        },
                        VersionDetail {
                            version: "54",
                            global_usage: 0f32,
                            release_date: Some(1530144000i64),
                        },
                        VersionDetail {
                            version: "55",
                            global_usage: 0f32,
                            release_date: Some(1534982400i64),
                        },
                        VersionDetail {
                            version: "56",
                            global_usage: 0f32,
                            release_date: Some(1537833600i64),
                        },
                        VersionDetail {
                            version: "57",
                            global_usage: 0f32,
                            release_date: Some(1543363200i64),
                        },
                        VersionDetail {
                            version: "58",
                            global_usage: 0f32,
                            release_date: Some(1548201600i64),
                        },
                        VersionDetail {
                            version: "60",
                            global_usage: 0f32,
                            release_date: Some(1554768000i64),
                        },
                        VersionDetail {
                            version: "62",
                            global_usage: 0f32,
                            release_date: Some(1561593600i64),
                        },
                        VersionDetail {
                            version: "63",
                            global_usage: 0f32,
                            release_date: Some(1566259200i64),
                        },
                        VersionDetail {
                            version: "64",
                            global_usage: 0f32,
                            release_date: Some(1570406400i64),
                        },
                        VersionDetail {
                            version: "65",
                            global_usage: 0f32,
                            release_date: Some(1573689600i64),
                        },
                        VersionDetail {
                            version: "66",
                            global_usage: 0f32,
                            release_date: Some(1578441600i64),
                        },
                        VersionDetail {
                            version: "67",
                            global_usage: 0f32,
                            release_date: Some(1583971200i64),
                        },
                        VersionDetail {
                            version: "68",
                            global_usage: 0f32,
                            release_date: Some(1587513600i64),
                        },
                        VersionDetail {
                            version: "69",
                            global_usage: 0f32,
                            release_date: Some(1592956800i64),
                        },
                        VersionDetail {
                            version: "70",
                            global_usage: 0f32,
                            release_date: Some(1595894400i64),
                        },
                        VersionDetail {
                            version: "71",
                            global_usage: 0f32,
                            release_date: Some(1600128000i64),
                        },
                        VersionDetail {
                            version: "72",
                            global_usage: 0f32,
                            release_date: Some(1603238400i64),
                        },
                        VersionDetail {
                            version: "73",
                            global_usage: 0f32,
                            release_date: Some(1613520000i64),
                        },
                        VersionDetail {
                            version: "74",
                            global_usage: 0f32,
                            release_date: Some(1612224000i64),
                        },
                        VersionDetail {
                            version: "75",
                            global_usage: 0f32,
                            release_date: Some(1616544000i64),
                        },
                        VersionDetail {
                            version: "76",
                            global_usage: 0f32,
                            release_date: Some(1619568000i64),
                        },
                        VersionDetail {
                            version: "77",
                            global_usage: 0f32,
                            release_date: Some(1623715200i64),
                        },
                        VersionDetail {
                            version: "78",
                            global_usage: 0f32,
                            release_date: Some(1627948800i64),
                        },
                        VersionDetail {
                            version: "79",
                            global_usage: 0f32,
                            release_date: Some(1631577600i64),
                        },
                        VersionDetail {
                            version: "80",
                            global_usage: 0f32,
                            release_date: Some(1633392000i64),
                        },
                        VersionDetail {
                            version: "81",
                            global_usage: 0f32,
                            release_date: Some(1635984000i64),
                        },
                        VersionDetail {
                            version: "82",
                            global_usage: 0f32,
                            release_date: Some(1638403200i64),
                        },
                        VersionDetail {
                            version: "83",
                            global_usage: 0f32,
                            release_date: Some(1642550400i64),
                        },
                        VersionDetail {
                            version: "84",
                            global_usage: 0f32,
                            release_date: Some(1644969600i64),
                        },
                        VersionDetail {
                            version: "85",
                            global_usage: 0f32,
                            release_date: Some(1647993600i64),
                        },
                        VersionDetail {
                            version: "86",
                            global_usage: 0f32,
                            release_date: Some(1650412800i64),
                        },
                        VersionDetail {
                            version: "87",
                            global_usage: 0.025361f32,
                            release_date: Some(1652745600i64),
                        },
                        VersionDetail {
                            version: "88",
                            global_usage: 0.007246f32,
                            release_date: Some(1654646400i64),
                        },
                        VersionDetail {
                            version: "89",
                            global_usage: 0f32,
                            release_date: Some(1657152000i64),
                        },
                        VersionDetail {
                            version: "90",
                            global_usage: 0f32,
                            release_date: Some(1660780800i64),
                        },
                        VersionDetail {
                            version: "91",
                            global_usage: 0f32,
                            release_date: Some(1663113600i64),
                        },
                        VersionDetail {
                            version: "92",
                            global_usage: 0f32,
                            release_date: Some(1668816000i64),
                        },
                        VersionDetail {
                            version: "93",
                            global_usage: 0f32,
                            release_date: Some(1668643200i64),
                        },
                        VersionDetail {
                            version: "94",
                            global_usage: 0f32,
                            release_date: Some(1671062400i64),
                        },
                        VersionDetail {
                            version: "95",
                            global_usage: 0.032607f32,
                            release_date: Some(1675209600i64),
                        },
                        VersionDetail {
                            version: "96",
                            global_usage: 0f32,
                            release_date: Some(1677024000i64),
                        },
                        VersionDetail {
                            version: "97",
                            global_usage: 0f32,
                            release_date: Some(1679529600i64),
                        },
                        VersionDetail {
                            version: "98",
                            global_usage: 0f32,
                            release_date: Some(1681948800i64),
                        },
                        VersionDetail {
                            version: "99",
                            global_usage: 0f32,
                            release_date: Some(1684195200i64),
                        },
                        VersionDetail {
                            version: "100",
                            global_usage: 0f32,
                            release_date: Some(1687219200i64),
                        },
                        VersionDetail {
                            version: "101",
                            global_usage: 0f32,
                            release_date: Some(1690329600i64),
                        },
                        VersionDetail {
                            version: "102",
                            global_usage: 0.018115f32,
                            release_date: Some(1692748800i64),
                        },
                        VersionDetail {
                            version: "103",
                            global_usage: 0f32,
                            release_date: Some(1696204800i64),
                        },
                        VersionDetail {
                            version: "104",
                            global_usage: 0f32,
                            release_date: Some(1699920000i64),
                        },
                        VersionDetail {
                            version: "105",
                            global_usage: 0f32,
                            release_date: Some(1699920000i64),
                        },
                        VersionDetail {
                            version: "106",
                            global_usage: 0f32,
                            release_date: Some(1702944000i64),
                        },
                        VersionDetail {
                            version: "107",
                            global_usage: 0f32,
                            release_date: Some(1707264000i64),
                        },
                        VersionDetail {
                            version: "108",
                            global_usage: 0f32,
                            release_date: Some(1710115200i64),
                        },
                        VersionDetail {
                            version: "109",
                            global_usage: 0f32,
                            release_date: Some(1711497600i64),
                        },
                        VersionDetail {
                            version: "110",
                            global_usage: 0f32,
                            release_date: Some(1716336000i64),
                        },
                        VersionDetail {
                            version: "111",
                            global_usage: 0f32,
                            release_date: Some(1719273600i64),
                        },
                        VersionDetail {
                            version: "112",
                            global_usage: 0f32,
                            release_date: Some(1721088000i64),
                        },
                        VersionDetail {
                            version: "113",
                            global_usage: 0f32,
                            release_date: Some(1724284800i64),
                        },
                        VersionDetail {
                            version: "114",
                            global_usage: 0.003623f32,
                            release_date: Some(1727222400i64),
                        },
                        VersionDetail {
                            version: "115",
                            global_usage: 0f32,
                            release_date: Some(1732665600i64),
                        },
                        VersionDetail {
                            version: "116",
                            global_usage: 0.202888f32,
                            release_date: Some(1736294400i64),
                        },
                        VersionDetail {
                            version: "117",
                            global_usage: 0.684747f32,
                            release_date: Some(1739404800i64),
                        },
                    ],
                },
            ),
            (
                "ios_saf",
                BrowserStat {
                    name: "ios_saf",
                    version_list: vec![
                        VersionDetail {
                            version: "3.2",
                            global_usage: 0f32,
                            release_date: Some(1270252800i64),
                        },
                        VersionDetail {
                            version: "4.0-4.1",
                            global_usage: 0f32,
                            release_date: Some(1283904000i64),
                        },
                        VersionDetail {
                            version: "4.2-4.3",
                            global_usage: 0.00289898f32,
                            release_date: Some(1299628800i64),
                        },
                        VersionDetail {
                            version: "5.0-5.1",
                            global_usage: 0f32,
                            release_date: Some(1331078400i64),
                        },
                        VersionDetail {
                            version: "6.0-6.1",
                            global_usage: 0.00869695f32,
                            release_date: Some(1359331200i64),
                        },
                        VersionDetail {
                            version: "7.0-7.1",
                            global_usage: 0.00724746f32,
                            release_date: Some(1394409600i64),
                        },
                        VersionDetail {
                            version: "8",
                            global_usage: 0f32,
                            release_date: Some(1410912000i64),
                        },
                        VersionDetail {
                            version: "8.1-8.4",
                            global_usage: 0f32,
                            release_date: Some(1413763200i64),
                        },
                        VersionDetail {
                            version: "9.0-9.2",
                            global_usage: 0.00434848f32,
                            release_date: Some(1442361600i64),
                        },
                        VersionDetail {
                            version: "9.3",
                            global_usage: 0.0202929f32,
                            release_date: Some(1458518400i64),
                        },
                        VersionDetail {
                            version: "10.0-10.2",
                            global_usage: 0.00144949f32,
                            release_date: Some(1473724800i64),
                        },
                        VersionDetail {
                            version: "10.3",
                            global_usage: 0.0333383f32,
                            release_date: Some(1490572800i64),
                        },
                        VersionDetail {
                            version: "11.0-11.2",
                            global_usage: 0.153646f32,
                            release_date: Some(1505779200i64),
                        },
                        VersionDetail {
                            version: "11.3-11.4",
                            global_usage: 0.0101464f32,
                            release_date: Some(1522281600i64),
                        },
                        VersionDetail {
                            version: "12.0-12.1",
                            global_usage: 0.00579797f32,
                            release_date: Some(1537142400i64),
                        },
                        VersionDetail {
                            version: "12.2-12.5",
                            global_usage: 0.14205f32,
                            release_date: Some(1553472000i64),
                        },
                        VersionDetail {
                            version: "13.0-13.1",
                            global_usage: 0.00289898f32,
                            release_date: Some(1568851200i64),
                        },
                        VersionDetail {
                            version: "13.2",
                            global_usage: 0.00579797f32,
                            release_date: Some(1572220800i64),
                        },
                        VersionDetail {
                            version: "13.3",
                            global_usage: 0.00579797f32,
                            release_date: Some(1580169600i64),
                        },
                        VersionDetail {
                            version: "13.4-13.7",
                            global_usage: 0.0202929f32,
                            release_date: Some(1585008000i64),
                        },
                        VersionDetail {
                            version: "14.0-14.4",
                            global_usage: 0.124656f32,
                            release_date: Some(1600214400i64),
                        },
                        VersionDetail {
                            version: "14.5-14.8",
                            global_usage: 0.0608787f32,
                            release_date: Some(1619395200i64),
                        },
                        VersionDetail {
                            version: "15.0-15.1",
                            global_usage: 0.0333383f32,
                            release_date: Some(1632096000i64),
                        },
                        VersionDetail {
                            version: "15.2-15.3",
                            global_usage: 0.0333383f32,
                            release_date: Some(1639353600i64),
                        },
                        VersionDetail {
                            version: "15.4",
                            global_usage: 0.0405858f32,
                            release_date: Some(1647216000i64),
                        },
                        VersionDetail {
                            version: "15.5",
                            global_usage: 0.0463837f32,
                            release_date: Some(1652659200i64),
                        },
                        VersionDetail {
                            version: "15.6-15.8",
                            global_usage: 0.568201f32,
                            release_date: Some(1658275200i64),
                        },
                        VersionDetail {
                            version: "16.0",
                            global_usage: 0.0797221f32,
                            release_date: Some(1662940800i64),
                        },
                        VersionDetail {
                            version: "16.1",
                            global_usage: 0.165242f32,
                            release_date: Some(1666569600i64),
                        },
                        VersionDetail {
                            version: "16.2",
                            global_usage: 0.08552f32,
                            release_date: Some(1670889600i64),
                        },
                        VersionDetail {
                            version: "16.3",
                            global_usage: 0.150747f32,
                            release_date: Some(1674432000i64),
                        },
                        VersionDetail {
                            version: "16.4",
                            global_usage: 0.0333383f32,
                            release_date: Some(1679875200i64),
                        },
                        VersionDetail {
                            version: "16.5",
                            global_usage: 0.0623282f32,
                            release_date: Some(1684368000i64),
                        },
                        VersionDetail {
                            version: "16.6-16.7",
                            global_usage: 0.672564f32,
                            release_date: Some(1690156800i64),
                        },
                        VersionDetail {
                            version: "17.0",
                            global_usage: 0.0405858f32,
                            release_date: Some(1694995200i64),
                        },
                        VersionDetail {
                            version: "17.1",
                            global_usage: 0.0724746f32,
                            release_date: Some(1698192000i64),
                        },
                        VersionDetail {
                            version: "17.2",
                            global_usage: 0.0550807f32,
                            release_date: Some(1702252800i64),
                        },
                        VersionDetail {
                            version: "17.3",
                            global_usage: 0.0768231f32,
                            release_date: Some(1705881600i64),
                        },
                        VersionDetail {
                            version: "17.4",
                            global_usage: 0.153646f32,
                            release_date: Some(1709596800i64),
                        },
                        VersionDetail {
                            version: "17.5",
                            global_usage: 0.340631f32,
                            release_date: Some(1715558400i64),
                        },
                        VersionDetail {
                            version: "17.6-17.7",
                            global_usage: 0.988554f32,
                            release_date: Some(1722211200i64),
                        },
                        VersionDetail {
                            version: "18.0",
                            global_usage: 0.276853f32,
                            release_date: Some(1726444800i64),
                        },
                        VersionDetail {
                            version: "18.1",
                            global_usage: 0.905933f32,
                            release_date: Some(1730073600i64),
                        },
                        VersionDetail {
                            version: "18.2",
                            global_usage: 0.405858f32,
                            release_date: Some(1733875200i64),
                        },
                        VersionDetail {
                            version: "18.3",
                            global_usage: 8.46503f32,
                            release_date: Some(1737936000i64),
                        },
                        VersionDetail {
                            version: "18.4",
                            global_usage: 0.126106f32,
                            release_date: Some(1743379200i64),
                        },
                        VersionDetail { version: "18.5", global_usage: 0f32, release_date: None },
                    ],
                },
            ),
            (
                "op_mini",
                BrowserStat {
                    name: "op_mini",
                    version_list: vec![VersionDetail {
                        version: "all",
                        global_usage: 0.05f32,
                        release_date: Some(1426464000i64),
                    }],
                },
            ),
            (
                "android",
                BrowserStat {
                    name: "android",
                    version_list: vec![
                        VersionDetail {
                            version: "2.1",
                            global_usage: 0f32,
                            release_date: Some(1256515200i64),
                        },
                        VersionDetail {
                            version: "2.2",
                            global_usage: 0f32,
                            release_date: Some(1274313600i64),
                        },
                        VersionDetail {
                            version: "2.3",
                            global_usage: 0f32,
                            release_date: Some(1291593600i64),
                        },
                        VersionDetail {
                            version: "3",
                            global_usage: 0f32,
                            release_date: Some(1298332800i64),
                        },
                        VersionDetail {
                            version: "4",
                            global_usage: 0f32,
                            release_date: Some(1318896000i64),
                        },
                        VersionDetail {
                            version: "4.1",
                            global_usage: 0f32,
                            release_date: Some(1341792000i64),
                        },
                        VersionDetail {
                            version: "4.2-4.3",
                            global_usage: 0.000262095f32,
                            release_date: Some(1374624000i64),
                        },
                        VersionDetail {
                            version: "4.4",
                            global_usage: 0f32,
                            release_date: Some(1386547200i64),
                        },
                        VersionDetail {
                            version: "4.4.3-4.4.4",
                            global_usage: 0.000961014f32,
                            release_date: Some(1401667200i64),
                        },
                        VersionDetail {
                            version: "135",
                            global_usage: 0.871727f32,
                            release_date: Some(1743379200i64),
                        },
                    ],
                },
            ),
            (
                "bb",
                BrowserStat {
                    name: "bb",
                    version_list: vec![
                        VersionDetail {
                            version: "7",
                            global_usage: 0f32,
                            release_date: Some(1325376000i64),
                        },
                        VersionDetail {
                            version: "10",
                            global_usage: 0f32,
                            release_date: Some(1359504000i64),
                        },
                    ],
                },
            ),
            (
                "op_mob",
                BrowserStat {
                    name: "op_mob",
                    version_list: vec![
                        VersionDetail {
                            version: "10",
                            global_usage: 0f32,
                            release_date: Some(1287100800i64),
                        },
                        VersionDetail {
                            version: "11",
                            global_usage: 0f32,
                            release_date: Some(1300752000i64),
                        },
                        VersionDetail {
                            version: "11.1",
                            global_usage: 0f32,
                            release_date: Some(1314835200i64),
                        },
                        VersionDetail {
                            version: "11.5",
                            global_usage: 0f32,
                            release_date: Some(1318291200i64),
                        },
                        VersionDetail {
                            version: "12",
                            global_usage: 0f32,
                            release_date: Some(1330300800i64),
                        },
                        VersionDetail {
                            version: "12.1",
                            global_usage: 0f32,
                            release_date: Some(1349740800i64),
                        },
                        VersionDetail {
                            version: "80",
                            global_usage: 1.04047f32,
                            release_date: Some(1709769600i64),
                        },
                    ],
                },
            ),
            (
                "and_chr",
                BrowserStat {
                    name: "and_chr",
                    version_list: vec![VersionDetail {
                        version: "135",
                        global_usage: 44.6783f32,
                        release_date: Some(1743379200i64),
                    }],
                },
            ),
            (
                "and_ff",
                BrowserStat {
                    name: "and_ff",
                    version_list: vec![VersionDetail {
                        version: "137",
                        global_usage: 0.350735f32,
                        release_date: Some(1743465600i64),
                    }],
                },
            ),
            (
                "ie_mob",
                BrowserStat {
                    name: "ie_mob",
                    version_list: vec![
                        VersionDetail {
                            version: "10",
                            global_usage: 0f32,
                            release_date: Some(1340150400i64),
                        },
                        VersionDetail {
                            version: "11",
                            global_usage: 0f32,
                            release_date: Some(1353456000i64),
                        },
                    ],
                },
            ),
            (
                "and_uc",
                BrowserStat {
                    name: "and_uc",
                    version_list: vec![VersionDetail {
                        version: "15.5",
                        global_usage: 0.848141f32,
                        release_date: Some(1710115200i64),
                    }],
                },
            ),
            (
                "samsung",
                BrowserStat {
                    name: "samsung",
                    version_list: vec![
                        VersionDetail {
                            version: "4",
                            global_usage: 0.0329016f32,
                            release_date: Some(1461024000i64),
                        },
                        VersionDetail {
                            version: "5.0-5.4",
                            global_usage: 0f32,
                            release_date: Some(1481846400i64),
                        },
                        VersionDetail {
                            version: "6.2-6.4",
                            global_usage: 0f32,
                            release_date: Some(1509408000i64),
                        },
                        VersionDetail {
                            version: "7.2-7.4",
                            global_usage: 0.0109672f32,
                            release_date: Some(1528329600i64),
                        },
                        VersionDetail {
                            version: "8.2",
                            global_usage: 0f32,
                            release_date: Some(1546128000i64),
                        },
                        VersionDetail {
                            version: "9.2",
                            global_usage: 0f32,
                            release_date: Some(1554163200i64),
                        },
                        VersionDetail {
                            version: "10.1",
                            global_usage: 0f32,
                            release_date: Some(1567900800i64),
                        },
                        VersionDetail {
                            version: "11.1-11.2",
                            global_usage: 0f32,
                            release_date: Some(1582588800i64),
                        },
                        VersionDetail {
                            version: "12.0",
                            global_usage: 0f32,
                            release_date: Some(1593475200i64),
                        },
                        VersionDetail {
                            version: "13.0",
                            global_usage: 0f32,
                            release_date: Some(1605657600i64),
                        },
                        VersionDetail {
                            version: "14.0",
                            global_usage: 0f32,
                            release_date: Some(1618531200i64),
                        },
                        VersionDetail {
                            version: "15.0",
                            global_usage: 0f32,
                            release_date: Some(1629072000i64),
                        },
                        VersionDetail {
                            version: "16.0",
                            global_usage: 0f32,
                            release_date: Some(1640736000i64),
                        },
                        VersionDetail {
                            version: "17.0",
                            global_usage: 0.0109672f32,
                            release_date: Some(1651708800i64),
                        },
                        VersionDetail {
                            version: "18.0",
                            global_usage: 0f32,
                            release_date: Some(1659657600i64),
                        },
                        VersionDetail {
                            version: "19.0",
                            global_usage: 0f32,
                            release_date: Some(1667260800i64),
                        },
                        VersionDetail {
                            version: "20",
                            global_usage: 0f32,
                            release_date: Some(1677369600i64),
                        },
                        VersionDetail {
                            version: "21",
                            global_usage: 0.0219344f32,
                            release_date: Some(1684454400i64),
                        },
                        VersionDetail {
                            version: "22",
                            global_usage: 0.0219344f32,
                            release_date: Some(1689292800i64),
                        },
                        VersionDetail {
                            version: "23",
                            global_usage: 0.0329016f32,
                            release_date: Some(1697587200i64),
                        },
                        VersionDetail {
                            version: "24",
                            global_usage: 0.0438688f32,
                            release_date: Some(1711497600i64),
                        },
                        VersionDetail {
                            version: "25",
                            global_usage: 0.0438688f32,
                            release_date: Some(1715126400i64),
                        },
                        VersionDetail {
                            version: "26",
                            global_usage: 0.0877377f32,
                            release_date: Some(1717718400i64),
                        },
                        VersionDetail {
                            version: "27",
                            global_usage: 1.96313f32,
                            release_date: Some(1725667200i64),
                        },
                    ],
                },
            ),
            (
                "and_qq",
                BrowserStat {
                    name: "and_qq",
                    version_list: vec![VersionDetail {
                        version: "14.9",
                        global_usage: 0.229572f32,
                        release_date: Some(1710288000i64),
                    }],
                },
            ),
            (
                "baidu",
                BrowserStat {
                    name: "baidu",
                    version_list: vec![VersionDetail {
                        version: "13.52",
                        global_usage: 0f32,
                        release_date: Some(1710201600i64),
                    }],
                },
            ),
            (
                "kaios",
                BrowserStat {
                    name: "kaios",
                    version_list: vec![
                        VersionDetail {
                            version: "2.5",
                            global_usage: 0.012754f32,
                            release_date: Some(1527811200i64),
                        },
                        VersionDetail {
                            version: "3.0-3.1",
                            global_usage: 0f32,
                            release_date: Some(1631664000i64),
                        },
                    ],
                },
            ),
        ])
    })
}
