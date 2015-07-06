#![feature(core, flt2dec)]
extern crate core;

#[cfg(test)]
mod tests {
    use core::num::flt2dec::strategy::grisu::CACHED_POW10;

    // pub static CACHED_POW10: [(u64, i16, i16); 81] = [ // (f, e, k)
    //     (0xe61acf033d1a45df, -1087, -308),
    //     (0xab70fe17c79ac6ca, -1060, -300),
    //     (0xff77b1fcbebcdc4f, -1034, -292),
    //     (0xbe5691ef416bd60c, -1007, -284),
    //     (0x8dd01fad907ffc3c,  -980, -276),
    //     (0xd3515c2831559a83,  -954, -268),
    //     (0x9d71ac8fada6c9b5,  -927, -260),
    //     (0xea9c227723ee8bcb,  -901, -252),
    //     (0xaecc49914078536d,  -874, -244),
    //     (0x823c12795db6ce57,  -847, -236),
    //     (0xc21094364dfb5637,  -821, -228),
    //     (0x9096ea6f3848984f,  -794, -220),
    //     (0xd77485cb25823ac7,  -768, -212),
    //     (0xa086cfcd97bf97f4,  -741, -204),
    //     (0xef340a98172aace5,  -715, -196),
    //     (0xb23867fb2a35b28e,  -688, -188),
    //     (0x84c8d4dfd2c63f3b,  -661, -180),
    //     (0xc5dd44271ad3cdba,  -635, -172),
    //     (0x936b9fcebb25c996,  -608, -164),
    //     (0xdbac6c247d62a584,  -582, -156),
    //     (0xa3ab66580d5fdaf6,  -555, -148),
    //     (0xf3e2f893dec3f126,  -529, -140),
    //     (0xb5b5ada8aaff80b8,  -502, -132),
    //     (0x87625f056c7c4a8b,  -475, -124),
    //     (0xc9bcff6034c13053,  -449, -116),
    //     (0x964e858c91ba2655,  -422, -108),
    //     (0xdff9772470297ebd,  -396, -100),
    //     (0xa6dfbd9fb8e5b88f,  -369,  -92),
    //     (0xf8a95fcf88747d94,  -343,  -84),
    //     (0xb94470938fa89bcf,  -316,  -76),
    //     (0x8a08f0f8bf0f156b,  -289,  -68),
    //     (0xcdb02555653131b6,  -263,  -60),
    //     (0x993fe2c6d07b7fac,  -236,  -52),
    //     (0xe45c10c42a2b3b06,  -210,  -44),
    //     (0xaa242499697392d3,  -183,  -36),
    //     (0xfd87b5f28300ca0e,  -157,  -28),
    //     (0xbce5086492111aeb,  -130,  -20),
    //     (0x8cbccc096f5088cc,  -103,  -12),
    //     (0xd1b71758e219652c,   -77,   -4),
    //     (0x9c40000000000000,   -50,    4),
    //     (0xe8d4a51000000000,   -24,   12),
    //     (0xad78ebc5ac620000,     3,   20),
    //     (0x813f3978f8940984,    30,   28),
    //     (0xc097ce7bc90715b3,    56,   36),
    //     (0x8f7e32ce7bea5c70,    83,   44),
    //     (0xd5d238a4abe98068,   109,   52),
    //     (0x9f4f2726179a2245,   136,   60),
    //     (0xed63a231d4c4fb27,   162,   68),
    //     (0xb0de65388cc8ada8,   189,   76),
    //     (0x83c7088e1aab65db,   216,   84),
    //     (0xc45d1df942711d9a,   242,   92),
    //     (0x924d692ca61be758,   269,  100),
    //     (0xda01ee641a708dea,   295,  108),
    //     (0xa26da3999aef774a,   322,  116),
    //     (0xf209787bb47d6b85,   348,  124),
    //     (0xb454e4a179dd1877,   375,  132),
    //     (0x865b86925b9bc5c2,   402,  140),
    //     (0xc83553c5c8965d3d,   428,  148),
    //     (0x952ab45cfa97a0b3,   455,  156),
    //     (0xde469fbd99a05fe3,   481,  164),
    //     (0xa59bc234db398c25,   508,  172),
    //     (0xf6c69a72a3989f5c,   534,  180),
    //     (0xb7dcbf5354e9bece,   561,  188),
    //     (0x88fcf317f22241e2,   588,  196),
    //     (0xcc20ce9bd35c78a5,   614,  204),
    //     (0x98165af37b2153df,   641,  212),
    //     (0xe2a0b5dc971f303a,   667,  220),
    //     (0xa8d9d1535ce3b396,   694,  228),
    //     (0xfb9b7cd9a4a7443c,   720,  236),
    //     (0xbb764c4ca7a44410,   747,  244),
    //     (0x8bab8eefb6409c1a,   774,  252),
    //     (0xd01fef10a657842c,   800,  260),
    //     (0x9b10a4e5e9913129,   827,  268),
    //     (0xe7109bfba19c0c9d,   853,  276),
    //     (0xac2820d9623bf429,   880,  284),
    //     (0x80444b5e7aa7cf85,   907,  292),
    //     (0xbf21e44003acdd2d,   933,  300),
    //     (0x8e679c2f5e44ff8f,   960,  308),
    //     (0xd433179d9c8cb841,   986,  316),
    //     (0x9e19db92b4e31ba9,  1013,  324),
    //     (0xeb96bf6ebadf77d9,  1039,  332),
    // ];

    #[test]
    fn alpha_test1() {
	assert_eq!(CACHED_POW10[0], (0xe61acf033d1a45df, -1087, -308));
	assert_eq!(CACHED_POW10[1], (0xab70fe17c79ac6ca, -1060, -300));
	assert_eq!(CACHED_POW10[2], (0xff77b1fcbebcdc4f, -1034, -292));
	assert_eq!(CACHED_POW10[3], (0xbe5691ef416bd60c, -1007, -284));
	assert_eq!(CACHED_POW10[4], (0x8dd01fad907ffc3c,  -980, -276));
	assert_eq!(CACHED_POW10[5], (0xd3515c2831559a83,  -954, -268));
	assert_eq!(CACHED_POW10[6], (0x9d71ac8fada6c9b5,  -927, -260));
	assert_eq!(CACHED_POW10[7], (0xea9c227723ee8bcb,  -901, -252));
	assert_eq!(CACHED_POW10[8], (0xaecc49914078536d,  -874, -244));
	assert_eq!(CACHED_POW10[9], (0x823c12795db6ce57,  -847, -236));
	assert_eq!(CACHED_POW10[10], (0xc21094364dfb5637,  -821, -228));
	assert_eq!(CACHED_POW10[11], (0x9096ea6f3848984f,  -794, -220));
	assert_eq!(CACHED_POW10[12], (0xd77485cb25823ac7,  -768, -212));
	assert_eq!(CACHED_POW10[13], (0xa086cfcd97bf97f4,  -741, -204));
	assert_eq!(CACHED_POW10[14], (0xef340a98172aace5,  -715, -196));
	assert_eq!(CACHED_POW10[15], (0xb23867fb2a35b28e,  -688, -188));
	assert_eq!(CACHED_POW10[16], (0x84c8d4dfd2c63f3b,  -661, -180));
	assert_eq!(CACHED_POW10[17], (0xc5dd44271ad3cdba,  -635, -172));
	assert_eq!(CACHED_POW10[18], (0x936b9fcebb25c996,  -608, -164));
	assert_eq!(CACHED_POW10[19], (0xdbac6c247d62a584,  -582, -156));
	assert_eq!(CACHED_POW10[20], (0xa3ab66580d5fdaf6,  -555, -148));
	assert_eq!(CACHED_POW10[21], (0xf3e2f893dec3f126,  -529, -140));
	assert_eq!(CACHED_POW10[22], (0xb5b5ada8aaff80b8,  -502, -132));
	assert_eq!(CACHED_POW10[23], (0x87625f056c7c4a8b,  -475, -124));
	assert_eq!(CACHED_POW10[24], (0xc9bcff6034c13053,  -449, -116));
	assert_eq!(CACHED_POW10[25], (0x964e858c91ba2655,  -422, -108));
	assert_eq!(CACHED_POW10[26], (0xdff9772470297ebd,  -396, -100));
	assert_eq!(CACHED_POW10[27], (0xa6dfbd9fb8e5b88f,  -369,  -92));
	assert_eq!(CACHED_POW10[28], (0xf8a95fcf88747d94,  -343,  -84));
	assert_eq!(CACHED_POW10[29], (0xb94470938fa89bcf,  -316,  -76));
	assert_eq!(CACHED_POW10[30], (0x8a08f0f8bf0f156b,  -289,  -68));
	assert_eq!(CACHED_POW10[31], (0xcdb02555653131b6,  -263,  -60));
	assert_eq!(CACHED_POW10[32], (0x993fe2c6d07b7fac,  -236,  -52));
	assert_eq!(CACHED_POW10[33], (0xe45c10c42a2b3b06,  -210,  -44));
	assert_eq!(CACHED_POW10[34], (0xaa242499697392d3,  -183,  -36));
	assert_eq!(CACHED_POW10[35], (0xfd87b5f28300ca0e,  -157,  -28));
	assert_eq!(CACHED_POW10[36], (0xbce5086492111aeb,  -130,  -20));
	assert_eq!(CACHED_POW10[37], (0x8cbccc096f5088cc,  -103,  -12));
	assert_eq!(CACHED_POW10[38], (0xd1b71758e219652c,   -77,   -4));
	assert_eq!(CACHED_POW10[39], (0x9c40000000000000,   -50,    4));
	assert_eq!(CACHED_POW10[40], (0xe8d4a51000000000,   -24,   12));
	assert_eq!(CACHED_POW10[41], (0xad78ebc5ac620000,     3,   20));
	assert_eq!(CACHED_POW10[42], (0x813f3978f8940984,    30,   28));
	assert_eq!(CACHED_POW10[43], (0xc097ce7bc90715b3,    56,   36));
	assert_eq!(CACHED_POW10[44], (0x8f7e32ce7bea5c70,    83,   44));
	assert_eq!(CACHED_POW10[45], (0xd5d238a4abe98068,   109,   52));
	assert_eq!(CACHED_POW10[46], (0x9f4f2726179a2245,   136,   60));
	assert_eq!(CACHED_POW10[47], (0xed63a231d4c4fb27,   162,   68));
	assert_eq!(CACHED_POW10[48], (0xb0de65388cc8ada8,   189,   76));
	assert_eq!(CACHED_POW10[49], (0x83c7088e1aab65db,   216,   84));
	assert_eq!(CACHED_POW10[50], (0xc45d1df942711d9a,   242,   92));
	assert_eq!(CACHED_POW10[51], (0x924d692ca61be758,   269,  100));
	assert_eq!(CACHED_POW10[52], (0xda01ee641a708dea,   295,  108));
	assert_eq!(CACHED_POW10[53], (0xa26da3999aef774a,   322,  116));
	assert_eq!(CACHED_POW10[54], (0xf209787bb47d6b85,   348,  124));
	assert_eq!(CACHED_POW10[55], (0xb454e4a179dd1877,   375,  132));
	assert_eq!(CACHED_POW10[56], (0x865b86925b9bc5c2,   402,  140));
	assert_eq!(CACHED_POW10[57], (0xc83553c5c8965d3d,   428,  148));
	assert_eq!(CACHED_POW10[58], (0x952ab45cfa97a0b3,   455,  156));
	assert_eq!(CACHED_POW10[59], (0xde469fbd99a05fe3,   481,  164));
	assert_eq!(CACHED_POW10[60], (0xa59bc234db398c25,   508,  172));
	assert_eq!(CACHED_POW10[61], (0xf6c69a72a3989f5c,   534,  180));
	assert_eq!(CACHED_POW10[62], (0xb7dcbf5354e9bece,   561,  188));
	assert_eq!(CACHED_POW10[63], (0x88fcf317f22241e2,   588,  196));
	assert_eq!(CACHED_POW10[64], (0xcc20ce9bd35c78a5,   614,  204));
	assert_eq!(CACHED_POW10[65], (0x98165af37b2153df,   641,  212));
	assert_eq!(CACHED_POW10[66], (0xe2a0b5dc971f303a,   667,  220));
	assert_eq!(CACHED_POW10[67], (0xa8d9d1535ce3b396,   694,  228));
	assert_eq!(CACHED_POW10[68], (0xfb9b7cd9a4a7443c,   720,  236));
	assert_eq!(CACHED_POW10[69], (0xbb764c4ca7a44410,   747,  244));
	assert_eq!(CACHED_POW10[70], (0x8bab8eefb6409c1a,   774,  252));
	assert_eq!(CACHED_POW10[71], (0xd01fef10a657842c,   800,  260));
	assert_eq!(CACHED_POW10[72], (0x9b10a4e5e9913129,   827,  268));
	assert_eq!(CACHED_POW10[73], (0xe7109bfba19c0c9d,   853,  276));
	assert_eq!(CACHED_POW10[74], (0xac2820d9623bf429,   880,  284));
	assert_eq!(CACHED_POW10[75], (0x80444b5e7aa7cf85,   907,  292));
	assert_eq!(CACHED_POW10[76], (0xbf21e44003acdd2d,   933,  300));
	assert_eq!(CACHED_POW10[77], (0x8e679c2f5e44ff8f,   960,  308));
	assert_eq!(CACHED_POW10[78], (0xd433179d9c8cb841,   986,  316));
	assert_eq!(CACHED_POW10[79], (0x9e19db92b4e31ba9,  1013,  324));
	assert_eq!(CACHED_POW10[80], (0xeb96bf6ebadf77d9,  1039,  332));
    }
}