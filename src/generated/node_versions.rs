use crate::semver::Version;
pub static NODE_VERSIONS: &[Version] = &[
    Version(0u32, 2u32, 0u32),
    Version(0u32, 3u32, 0u32),
    Version(0u32, 4u32, 0u32),
    Version(0u32, 5u32, 0u32),
    Version(0u32, 6u32, 0u32),
    Version(0u32, 7u32, 0u32),
    Version(0u32, 8u32, 0u32),
    Version(0u32, 9u32, 0u32),
    Version(0u32, 10u32, 0u32),
    Version(0u32, 11u32, 0u32),
    Version(0u32, 12u32, 0u32),
    Version(4u32, 0u32, 0u32),
    Version(4u32, 1u32, 0u32),
    Version(4u32, 2u32, 0u32),
    Version(4u32, 3u32, 0u32),
    Version(4u32, 4u32, 0u32),
    Version(4u32, 5u32, 0u32),
    Version(4u32, 6u32, 0u32),
    Version(4u32, 7u32, 0u32),
    Version(4u32, 8u32, 0u32),
    Version(4u32, 9u32, 0u32),
    Version(5u32, 0u32, 0u32),
    Version(5u32, 1u32, 0u32),
    Version(5u32, 2u32, 0u32),
    Version(5u32, 3u32, 0u32),
    Version(5u32, 4u32, 0u32),
    Version(5u32, 5u32, 0u32),
    Version(5u32, 6u32, 0u32),
    Version(5u32, 7u32, 0u32),
    Version(5u32, 8u32, 0u32),
    Version(5u32, 9u32, 0u32),
    Version(5u32, 10u32, 0u32),
    Version(5u32, 11u32, 0u32),
    Version(5u32, 12u32, 0u32),
    Version(6u32, 0u32, 0u32),
    Version(6u32, 1u32, 0u32),
    Version(6u32, 2u32, 0u32),
    Version(6u32, 3u32, 0u32),
    Version(6u32, 4u32, 0u32),
    Version(6u32, 5u32, 0u32),
    Version(6u32, 6u32, 0u32),
    Version(6u32, 7u32, 0u32),
    Version(6u32, 8u32, 0u32),
    Version(6u32, 9u32, 0u32),
    Version(6u32, 10u32, 0u32),
    Version(6u32, 11u32, 0u32),
    Version(6u32, 12u32, 0u32),
    Version(6u32, 13u32, 0u32),
    Version(6u32, 14u32, 0u32),
    Version(6u32, 15u32, 0u32),
    Version(6u32, 16u32, 0u32),
    Version(6u32, 17u32, 0u32),
    Version(7u32, 0u32, 0u32),
    Version(7u32, 1u32, 0u32),
    Version(7u32, 2u32, 0u32),
    Version(7u32, 3u32, 0u32),
    Version(7u32, 4u32, 0u32),
    Version(7u32, 5u32, 0u32),
    Version(7u32, 6u32, 0u32),
    Version(7u32, 7u32, 0u32),
    Version(7u32, 8u32, 0u32),
    Version(7u32, 9u32, 0u32),
    Version(7u32, 10u32, 0u32),
    Version(8u32, 0u32, 0u32),
    Version(8u32, 1u32, 0u32),
    Version(8u32, 2u32, 0u32),
    Version(8u32, 3u32, 0u32),
    Version(8u32, 4u32, 0u32),
    Version(8u32, 5u32, 0u32),
    Version(8u32, 6u32, 0u32),
    Version(8u32, 7u32, 0u32),
    Version(8u32, 8u32, 0u32),
    Version(8u32, 9u32, 0u32),
    Version(8u32, 10u32, 0u32),
    Version(8u32, 11u32, 0u32),
    Version(8u32, 12u32, 0u32),
    Version(8u32, 13u32, 0u32),
    Version(8u32, 14u32, 0u32),
    Version(8u32, 15u32, 0u32),
    Version(8u32, 16u32, 0u32),
    Version(8u32, 17u32, 0u32),
    Version(9u32, 0u32, 0u32),
    Version(9u32, 1u32, 0u32),
    Version(9u32, 2u32, 0u32),
    Version(9u32, 3u32, 0u32),
    Version(9u32, 4u32, 0u32),
    Version(9u32, 5u32, 0u32),
    Version(9u32, 6u32, 0u32),
    Version(9u32, 7u32, 0u32),
    Version(9u32, 8u32, 0u32),
    Version(9u32, 9u32, 0u32),
    Version(9u32, 10u32, 0u32),
    Version(9u32, 11u32, 0u32),
    Version(10u32, 0u32, 0u32),
    Version(10u32, 1u32, 0u32),
    Version(10u32, 2u32, 0u32),
    Version(10u32, 3u32, 0u32),
    Version(10u32, 4u32, 0u32),
    Version(10u32, 5u32, 0u32),
    Version(10u32, 6u32, 0u32),
    Version(10u32, 7u32, 0u32),
    Version(10u32, 8u32, 0u32),
    Version(10u32, 9u32, 0u32),
    Version(10u32, 10u32, 0u32),
    Version(10u32, 11u32, 0u32),
    Version(10u32, 12u32, 0u32),
    Version(10u32, 13u32, 0u32),
    Version(10u32, 14u32, 0u32),
    Version(10u32, 15u32, 0u32),
    Version(10u32, 16u32, 0u32),
    Version(10u32, 17u32, 0u32),
    Version(10u32, 18u32, 0u32),
    Version(10u32, 19u32, 0u32),
    Version(10u32, 20u32, 0u32),
    Version(10u32, 21u32, 0u32),
    Version(10u32, 22u32, 0u32),
    Version(10u32, 23u32, 0u32),
    Version(10u32, 24u32, 0u32),
    Version(11u32, 0u32, 0u32),
    Version(11u32, 1u32, 0u32),
    Version(11u32, 2u32, 0u32),
    Version(11u32, 3u32, 0u32),
    Version(11u32, 4u32, 0u32),
    Version(11u32, 5u32, 0u32),
    Version(11u32, 6u32, 0u32),
    Version(11u32, 7u32, 0u32),
    Version(11u32, 8u32, 0u32),
    Version(11u32, 9u32, 0u32),
    Version(11u32, 10u32, 0u32),
    Version(11u32, 11u32, 0u32),
    Version(11u32, 12u32, 0u32),
    Version(11u32, 13u32, 0u32),
    Version(11u32, 14u32, 0u32),
    Version(11u32, 15u32, 0u32),
    Version(12u32, 0u32, 0u32),
    Version(12u32, 1u32, 0u32),
    Version(12u32, 2u32, 0u32),
    Version(12u32, 3u32, 0u32),
    Version(12u32, 4u32, 0u32),
    Version(12u32, 5u32, 0u32),
    Version(12u32, 6u32, 0u32),
    Version(12u32, 7u32, 0u32),
    Version(12u32, 8u32, 0u32),
    Version(12u32, 9u32, 0u32),
    Version(12u32, 10u32, 0u32),
    Version(12u32, 11u32, 0u32),
    Version(12u32, 12u32, 0u32),
    Version(12u32, 13u32, 0u32),
    Version(12u32, 14u32, 0u32),
    Version(12u32, 15u32, 0u32),
    Version(12u32, 16u32, 0u32),
    Version(12u32, 17u32, 0u32),
    Version(12u32, 18u32, 0u32),
    Version(12u32, 19u32, 0u32),
    Version(12u32, 20u32, 0u32),
    Version(12u32, 21u32, 0u32),
    Version(12u32, 22u32, 0u32),
    Version(13u32, 0u32, 0u32),
    Version(13u32, 1u32, 0u32),
    Version(13u32, 2u32, 0u32),
    Version(13u32, 3u32, 0u32),
    Version(13u32, 4u32, 0u32),
    Version(13u32, 5u32, 0u32),
    Version(13u32, 6u32, 0u32),
    Version(13u32, 7u32, 0u32),
    Version(13u32, 8u32, 0u32),
    Version(13u32, 9u32, 0u32),
    Version(13u32, 10u32, 0u32),
    Version(13u32, 11u32, 0u32),
    Version(13u32, 12u32, 0u32),
    Version(13u32, 13u32, 0u32),
    Version(13u32, 14u32, 0u32),
    Version(14u32, 0u32, 0u32),
    Version(14u32, 1u32, 0u32),
    Version(14u32, 2u32, 0u32),
    Version(14u32, 3u32, 0u32),
    Version(14u32, 4u32, 0u32),
    Version(14u32, 5u32, 0u32),
    Version(14u32, 6u32, 0u32),
    Version(14u32, 7u32, 0u32),
    Version(14u32, 8u32, 0u32),
    Version(14u32, 9u32, 0u32),
    Version(14u32, 10u32, 0u32),
    Version(14u32, 11u32, 0u32),
    Version(14u32, 12u32, 0u32),
    Version(14u32, 13u32, 0u32),
    Version(14u32, 14u32, 0u32),
    Version(14u32, 15u32, 0u32),
    Version(14u32, 16u32, 0u32),
    Version(14u32, 17u32, 0u32),
    Version(14u32, 18u32, 0u32),
    Version(14u32, 19u32, 0u32),
    Version(14u32, 20u32, 0u32),
    Version(14u32, 21u32, 0u32),
    Version(15u32, 0u32, 0u32),
    Version(15u32, 1u32, 0u32),
    Version(15u32, 2u32, 0u32),
    Version(15u32, 3u32, 0u32),
    Version(15u32, 4u32, 0u32),
    Version(15u32, 5u32, 0u32),
    Version(15u32, 6u32, 0u32),
    Version(15u32, 7u32, 0u32),
    Version(15u32, 8u32, 0u32),
    Version(15u32, 9u32, 0u32),
    Version(15u32, 10u32, 0u32),
    Version(15u32, 11u32, 0u32),
    Version(15u32, 12u32, 0u32),
    Version(15u32, 13u32, 0u32),
    Version(15u32, 14u32, 0u32),
    Version(16u32, 0u32, 0u32),
    Version(16u32, 1u32, 0u32),
    Version(16u32, 2u32, 0u32),
    Version(16u32, 3u32, 0u32),
    Version(16u32, 4u32, 0u32),
    Version(16u32, 5u32, 0u32),
    Version(16u32, 6u32, 0u32),
    Version(16u32, 7u32, 0u32),
    Version(16u32, 8u32, 0u32),
    Version(16u32, 9u32, 0u32),
    Version(16u32, 10u32, 0u32),
    Version(16u32, 11u32, 0u32),
    Version(16u32, 12u32, 0u32),
    Version(16u32, 13u32, 0u32),
    Version(16u32, 14u32, 0u32),
    Version(16u32, 15u32, 0u32),
    Version(16u32, 16u32, 0u32),
    Version(16u32, 17u32, 0u32),
    Version(16u32, 18u32, 0u32),
    Version(16u32, 19u32, 0u32),
    Version(16u32, 20u32, 0u32),
    Version(17u32, 0u32, 0u32),
    Version(17u32, 1u32, 0u32),
    Version(17u32, 2u32, 0u32),
    Version(17u32, 3u32, 0u32),
    Version(17u32, 4u32, 0u32),
    Version(17u32, 5u32, 0u32),
    Version(17u32, 6u32, 0u32),
    Version(17u32, 7u32, 0u32),
    Version(17u32, 8u32, 0u32),
    Version(17u32, 9u32, 0u32),
    Version(18u32, 0u32, 0u32),
    Version(18u32, 1u32, 0u32),
    Version(18u32, 2u32, 0u32),
    Version(18u32, 3u32, 0u32),
    Version(18u32, 4u32, 0u32),
    Version(18u32, 5u32, 0u32),
    Version(18u32, 6u32, 0u32),
    Version(18u32, 7u32, 0u32),
    Version(18u32, 8u32, 0u32),
    Version(18u32, 9u32, 0u32),
    Version(18u32, 10u32, 0u32),
    Version(18u32, 11u32, 0u32),
    Version(18u32, 12u32, 0u32),
    Version(18u32, 13u32, 0u32),
    Version(18u32, 14u32, 0u32),
    Version(18u32, 15u32, 0u32),
    Version(18u32, 16u32, 0u32),
    Version(18u32, 17u32, 0u32),
    Version(18u32, 18u32, 0u32),
    Version(18u32, 19u32, 0u32),
    Version(18u32, 20u32, 0u32),
    Version(19u32, 0u32, 0u32),
    Version(19u32, 1u32, 0u32),
    Version(19u32, 2u32, 0u32),
    Version(19u32, 3u32, 0u32),
    Version(19u32, 4u32, 0u32),
    Version(19u32, 5u32, 0u32),
    Version(19u32, 6u32, 0u32),
    Version(19u32, 7u32, 0u32),
    Version(19u32, 8u32, 0u32),
    Version(19u32, 9u32, 0u32),
    Version(20u32, 0u32, 0u32),
    Version(20u32, 1u32, 0u32),
    Version(20u32, 2u32, 0u32),
    Version(20u32, 3u32, 0u32),
    Version(20u32, 4u32, 0u32),
    Version(20u32, 5u32, 0u32),
    Version(20u32, 6u32, 0u32),
    Version(20u32, 7u32, 0u32),
    Version(20u32, 8u32, 0u32),
    Version(20u32, 9u32, 0u32),
    Version(20u32, 10u32, 0u32),
    Version(20u32, 11u32, 0u32),
    Version(20u32, 12u32, 0u32),
    Version(20u32, 13u32, 0u32),
    Version(20u32, 14u32, 0u32),
    Version(20u32, 15u32, 0u32),
    Version(20u32, 16u32, 0u32),
    Version(20u32, 17u32, 0u32),
    Version(20u32, 18u32, 0u32),
    Version(21u32, 0u32, 0u32),
    Version(21u32, 1u32, 0u32),
    Version(21u32, 2u32, 0u32),
    Version(21u32, 3u32, 0u32),
    Version(21u32, 4u32, 0u32),
    Version(21u32, 5u32, 0u32),
    Version(21u32, 6u32, 0u32),
    Version(21u32, 7u32, 0u32),
    Version(22u32, 0u32, 0u32),
    Version(22u32, 1u32, 0u32),
    Version(22u32, 2u32, 0u32),
    Version(22u32, 3u32, 0u32),
    Version(22u32, 4u32, 0u32),
    Version(22u32, 5u32, 0u32),
    Version(22u32, 6u32, 0u32),
    Version(22u32, 7u32, 0u32),
    Version(22u32, 8u32, 0u32),
    Version(22u32, 9u32, 0u32),
    Version(22u32, 10u32, 0u32),
    Version(22u32, 11u32, 0u32),
    Version(22u32, 12u32, 0u32),
    Version(23u32, 0u32, 0u32),
    Version(23u32, 1u32, 0u32),
    Version(23u32, 2u32, 0u32),
    Version(23u32, 3u32, 0u32),
];
