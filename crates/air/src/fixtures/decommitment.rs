use starknet_crypto::Felt;
use swiftness_commitment::table;

use crate::trace;

pub fn get() -> trace::Decommitment {
    trace::Decommitment {
        original: table::types::Decommitment {
            values: vec![
                Felt::from_hex_unchecked(
                    "0x5a81cfa7b8ba1dd722ce2bcaf78476fd0e0b7fda53287ed2632c2c32ab4f42c",
                ),
                Felt::from_hex_unchecked(
                    "0x437f6248b14ae3bc546eafe54a32cdc961c0821ab13a8ef15b28aae6762c6e9",
                ),
                Felt::from_hex_unchecked(
                    "0x350167cc2d1223d974e60d87bbadb0bf782ceb21bebca6657ceb3df9d2398b",
                ),
                Felt::from_hex_unchecked(
                    "0x6f5f181efbce585e6bfc816c428b043a0f7bde3e6fb62e836300d1bf215aba0",
                ),
                Felt::from_hex_unchecked(
                    "0x1e6d05d636513c6d40b99c7760e610b27a05f049e0523f444369f9b1bb98a96",
                ),
                Felt::from_hex_unchecked(
                    "0x173e602ff07512f9b50c59fc3471494203877c2da9e02b44b62c44486052ff7",
                ),
                Felt::from_hex_unchecked(
                    "0x749f215c8d13b0eedcf5419760e8455603cb0ec4a74be885ebf7a0489f3c5aa",
                ),
                Felt::from_hex_unchecked(
                    "0x5b753d91ea71a67ce18a799f695e25b7962d506dced9b78f3bdc8435ee213da",
                ),
                Felt::from_hex_unchecked(
                    "0x6e6de7b0565dcdc4b1b62152bfbecb2dba5aeca0eb52e9260dd7f0074e930b0",
                ),
                Felt::from_hex_unchecked(
                    "0x672901a267bd4b938a23bad8d6207fd73b71107835aca545ad88eee3100e586",
                ),
                Felt::from_hex_unchecked(
                    "0x2d030c78db00d0278dd5637e3456888c15521452c57f8eb3e14665614fb381",
                ),
                Felt::from_hex_unchecked(
                    "0x23c90d84e2502f1079a6c4bab62cf77b5feb89f8199276852ac09a885b2b495",
                ),
                Felt::from_hex_unchecked(
                    "0x10e35324d09571bacfcf642d0b1f872c6a8071462b5864f23340cd83c5cec11",
                ),
                Felt::from_hex_unchecked(
                    "0x24d19d603e658cdac66bcd2ecd484b0fdfb502d32e3d03f3bbfff6bfb3c13f3",
                ),
                Felt::from_hex_unchecked(
                    "0x2b85b285285a0543466f2fbc053db9995ab26578dc2db49df0536d3b68d7a21",
                ),
                Felt::from_hex_unchecked(
                    "0x67d06d0cdec229eb147948d8917c60a2d5ffaa01d3faa4ebd62c68bf7161d7",
                ),
                Felt::from_hex_unchecked(
                    "0x506ae3c00774c128e05fcd292031bc9b5748853e5ecdc2b68efad22502d9dd3",
                ),
                Felt::from_hex_unchecked(
                    "0x28e7bc0081b3f64605be467c67679b4694698d61549cfe1a61df01801d8b706",
                ),
                Felt::from_hex_unchecked(
                    "0x571894602d25aa56fca3058d68de05b420cf3a610e79710746e8b97d09d9960",
                ),
                Felt::from_hex_unchecked(
                    "0x7b36942fb2f4ca2f43f4166792a8e558f86875191d9543fe633e4f95d05bc2c",
                ),
                Felt::from_hex_unchecked(
                    "0x1b5edd7c7943548eeaa3b0f53875f7b2a864e7b96fb97264dadca98be349cbb",
                ),
                Felt::from_hex_unchecked(
                    "0x6032b3931ed6052582a1eefb1fdf475d8d5f6eca637fe659f3df222d9b8bd56",
                ),
                Felt::from_hex_unchecked(
                    "0x6ee0b7bed142500d285418ab68c14fbabbe28cead7517ba6391affbabfa07f8",
                ),
                Felt::from_hex_unchecked(
                    "0x465476fca06dcb03e8c5dd71f8b6d6983f75c3e008dd274dac89cb6859407d6",
                ),
                Felt::from_hex_unchecked(
                    "0x98128772999f1b99d53878b9337576e4898ab5734534d13895f6d17874fbe6",
                ),
                Felt::from_hex_unchecked(
                    "0x3812f410c441ec208f01e579effcf9992c2c316218a8d585e006a9b6a9586f5",
                ),
                Felt::from_hex_unchecked(
                    "0x6736142028c68e21e699235313fe5cb080bb63c47d493ca15c09cc854e3949f",
                ),
                Felt::from_hex_unchecked(
                    "0x18494ab2c44c2200a93a37adfe8ad06ccdd705aa53085827931ccddb46e2b21",
                ),
                Felt::from_hex_unchecked(
                    "0x64dba7947b0037472a738c4af52b1b1b193c02beb2443ed81f63946081b76b",
                ),
                Felt::from_hex_unchecked(
                    "0x339d037e561c5811d01ae344f03fc8f847b4a96a7c2d4a4a07db2ab6fa0acea",
                ),
                Felt::from_hex_unchecked(
                    "0x47928dcc6bdbb5bc6b3b87d3fbce6b549191693539a2ee0ba232f2d0b2f8677",
                ),
                Felt::from_hex_unchecked(
                    "0x56084701a335a410c2c0a73a51b2d59b298dc166480d55e8ca76eb2f0c24c8",
                ),
                Felt::from_hex_unchecked(
                    "0x5d9e1faf79f4720c7dd030ceacb7f3bb28c8393fb6311913730a306554b7e26",
                ),
                Felt::from_hex_unchecked(
                    "0x470641cdbfdee39e6c5d7f00afdb53062fe672a2c32df1a63ab6674f9f8b482",
                ),
                Felt::from_hex_unchecked(
                    "0x230ed14ad7be0bd1f69fd827c65dd357dbceaf022573da3b6b247b8a7965685",
                ),
                Felt::from_hex_unchecked(
                    "0x7aa7111c69f1464ca6099e2364519d075ea3113f04be2e0e294664d360b7aee",
                ),
                Felt::from_hex_unchecked(
                    "0x5cfd1f2b95c01a8445e16c38c4a268cd6b55a9fb5677b8b548ffeae1ff895a3",
                ),
                Felt::from_hex_unchecked(
                    "0xfe0782c363cdecf73f1e7ad20bbbd7bb466cbbc56ca79cbe4d49a1cedf1df8",
                ),
                Felt::from_hex_unchecked(
                    "0x47e73e996073fa3fdf22a5dc7cc34ff4119f1b87641575adc3a11e1f296ea1b",
                ),
                Felt::from_hex_unchecked(
                    "0x75af7bfc4f26e98c2938fe1452f113f83687949b476710171120b229690bcb3",
                ),
                Felt::from_hex_unchecked(
                    "0x4d16d2a61df3d18b4f93bbe01cd7163f15090a4147f73df56de70e688ebc8ae",
                ),
                Felt::from_hex_unchecked(
                    "0x49ce8f3d4264f5a7f7d87261e58aaff8e10e56e5ac56ad742b1db67c583bbd3",
                ),
                Felt::from_hex_unchecked(
                    "0x22fda7c7beab8b0561d2803247eb8659e51b653250cc18ccad1bf1b5db8686",
                ),
                Felt::from_hex_unchecked(
                    "0x590638191c4f6fd6a3bc633578c0fa4059c3ba6ce9f0acc214a55fe44e81bb3",
                ),
                Felt::from_hex_unchecked(
                    "0x28642a4c037572cd49eabfe499ce3f3cd0d1ec4a5d9bf997a4de4d79ffc2b04",
                ),
                Felt::from_hex_unchecked(
                    "0x7b278073e44677a98119164b79728a594cff1fa6a2b0577be9d28ce35da74bb",
                ),
                Felt::from_hex_unchecked(
                    "0x40c384be4aa0577bb23b12fd4151b68919212c1b2f63faa3a992eb133525c87",
                ),
                Felt::from_hex_unchecked(
                    "0x5de53658a09a08072e09f61f01fabac07ffd1773423cd2821c6270278c8df0f",
                ),
                Felt::from_hex_unchecked(
                    "0x2602d1f5b0bb3e5e49d9e2775940221124b2b2c27acc0353e77e53345903ed9",
                ),
                Felt::from_hex_unchecked(
                    "0x4889f879db89dd3394974a06381e4a4f23934fabce6619f00c45ffe1e956418",
                ),
                Felt::from_hex_unchecked(
                    "0x25cd05e979f9a05c01282e4178ae51eae662fea72eb6c8ff8290aeb36f9e5ec",
                ),
                Felt::from_hex_unchecked(
                    "0x794670d35a13811889251fcf36386e591ff014df483539049a1d16408ad2198",
                ),
                Felt::from_hex_unchecked(
                    "0x6752ae4c7342e0d7aa8a64b65f6ca33a9a4ce112c1d70ac8384beccb239eebe",
                ),
                Felt::from_hex_unchecked(
                    "0x224f2c778cbc616138aabd3fde19d5dcc3fb89cb20e9670f36aa71b9bb95d3b",
                ),
                Felt::from_hex_unchecked(
                    "0x3aede81986738227e8856c73dbc08603a620f9c227a7e72d913e09e0dd3af96",
                ),
                Felt::from_hex_unchecked(
                    "0x29adbb8d26e892c0b854f9ecf07556f33d8cb83ab4efd152e6c2168e55f4322",
                ),
                Felt::from_hex_unchecked(
                    "0x1e0065b34962535246ad7be5b4b17d66af996b950d1c24fffa317c15d48b72e",
                ),
                Felt::from_hex_unchecked(
                    "0x78dc672d38e9d3f7d4c10a91a703b874c8d5f450510775ae1eb9ec4b897918f",
                ),
                Felt::from_hex_unchecked(
                    "0xdf66dec5e9469a844fbf2a60fde593d5ecb58b8fc6f4e023dd8348d0a8b753",
                ),
                Felt::from_hex_unchecked(
                    "0x647d1a44e02b844416eb63d024eb8065b618dc5d5df1d1e5708ca2c61faa8ed",
                ),
                Felt::from_hex_unchecked(
                    "0x1cee497a299c6cb56ff4c3583f6174d8a385f1c1eefb25dc09da3450efe4b66",
                ),
                Felt::from_hex_unchecked(
                    "0x3f49a6a6668039023c67c63e8c24a61cb7a7ea93265cccf7f2d84e9ef687482",
                ),
                Felt::from_hex_unchecked(
                    "0x448a6f4bcde506f1f462055f82ed18d320337c32ba53a3f4fd4213068672bfa",
                ),
                Felt::from_hex_unchecked(
                    "0x3202dea1cd62a3aa3cfc5aa4b8c1a13101018c6d267af36b54f1653ae61c04b",
                ),
                Felt::from_hex_unchecked(
                    "0x94593db5d8e77c7a7bd15d1ba888842dd9f89c57f1b6e6e94fd906de911d30",
                ),
                Felt::from_hex_unchecked(
                    "0x454446a92177d7f0b2c61b96635abff2f6b5158497b85f29478d057ee71717c",
                ),
                Felt::from_hex_unchecked(
                    "0x713272f23405a27a0c8f46705f8c8e8f18286f1b368baace6654e788b437a85",
                ),
                Felt::from_hex_unchecked(
                    "0x52dd724126aa589695a807016b3a030f66d035f58ceda4675b08e6eb700e362",
                ),
                Felt::from_hex_unchecked(
                    "0x425606a0d129d87548a4315c0548decbc6f1ecd3f76a9b6bf77fbb859e6cf47",
                ),
                Felt::from_hex_unchecked(
                    "0x61e378adc06e8c25707453f902a88a58320f383024acb6e8766faad2dae72a4",
                ),
            ],
        },
        interaction: table::types::Decommitment {
            values: vec![
                Felt::from_hex_unchecked(
                    "0x5bf29218811908115445900bc8f367dcf29cf113bfe0e29f3c669b396e12da6",
                ),
                Felt::from_hex_unchecked(
                    "0x84945c17d78c44d588525e6012070172989c66a12f876d40def2eabd025352",
                ),
                Felt::from_hex_unchecked(
                    "0x73dd90541f088a8be7da7ff68e532d4af514312e6c367913e2d93aebded6888",
                ),
                Felt::from_hex_unchecked(
                    "0x6f925f5f494f60aae00761a87d21aa2d29b807514e30407a6049618c4d54db8",
                ),
                Felt::from_hex_unchecked(
                    "0x7d3e24eb2c919340e3e8573cdbf8330830b040dc6cd255497435039c9fff343",
                ),
                Felt::from_hex_unchecked(
                    "0x1e33d04a094e6d4dc2ccbac5f94effdf5800ec814d42f4b4b30642a1b537c5d",
                ),
                Felt::from_hex_unchecked(
                    "0x554156d17b1d78c4503b5d9ac0b5aef51cf1048c18c017318955654b81e106b",
                ),
                Felt::from_hex_unchecked(
                    "0x112cbcd71ebd64a09832ba3b69a8ac48af5176386e09e908b5d7118adfacdad",
                ),
                Felt::from_hex_unchecked(
                    "0x8fff861ec8a2f0216e8161a3a0dff2a6384f9dd9eb6c1c3cbf39f2c7a8ebf6",
                ),
                Felt::from_hex_unchecked(
                    "0x2e3dae8f2a70ca35f49f5ebe4a33613fbb8bcc59c42e9681e81c007aa139f1",
                ),
                Felt::from_hex_unchecked(
                    "0x1c9a20fc0184009000e8a93b983d6016117e8279dcd4afeef8b0231313a4068",
                ),
                Felt::from_hex_unchecked(
                    "0x3f111d9ad96bfdfc7d467e2ff0b79fe3a7e8bd5e2736a58b4296ab8e910d25f",
                ),
                Felt::from_hex_unchecked(
                    "0x3ddbf3767e51556ca3914783920d508c993d7c4b80b7a9232cc132674f4124e",
                ),
                Felt::from_hex_unchecked(
                    "0x35a280058cf32f47441a42db2dd0a41848ad8560e2a7e2ae806196006f1d306",
                ),
                Felt::from_hex_unchecked(
                    "0x3807fe3c5ccb585140ad0e58ab11f20ccf3d614684b8014a3deab4f607f4a8",
                ),
                Felt::from_hex_unchecked(
                    "0x3312b1827bf625d9269f7a2b8597f598b2ab729e2406f40dd089155cdfedeb0",
                ),
                Felt::from_hex_unchecked(
                    "0x1085a4ee6cea111d74c9ca967e5b558625bb9e93bfa4d1617051cb66378c57d",
                ),
                Felt::from_hex_unchecked(
                    "0x18b9b375d9da086d8825ba6f683a19f1a6afb30d3cd8b3ab8a5b242040d21c7",
                ),
                Felt::from_hex_unchecked(
                    "0x718e58a2f2f48f6bfecc1263b2ceb43d26fd1aa86bf12f2c75cc564c592108c",
                ),
                Felt::from_hex_unchecked(
                    "0x60c3dbaea374e6b68e8363f5769023ece533a0581a098aacd185fbe657b5a10",
                ),
                Felt::from_hex_unchecked(
                    "0x164e1191ba919ecf586655e920bae73134c7b8db66416579074725b71154177",
                ),
                Felt::from_hex_unchecked(
                    "0x4a283c8426e01924982760c0180e9b4dad90e3dce99f017eea4e62a43ec1e7a",
                ),
                Felt::from_hex_unchecked(
                    "0x72fe1e3b53b3c59593a7f77afd48966ba8a2cbc7fafa7b2ec80ee929d080134",
                ),
                Felt::from_hex_unchecked(
                    "0x54e169ac57ec327fc0fc6bfa109ccbf0877335becd6bab57e53e2c5d2963452",
                ),
                Felt::from_hex_unchecked(
                    "0xafd9d0db759ab20cfaab153a2e79339dfe6a73e0349059cf0ce17c52603dc3",
                ),
                Felt::from_hex_unchecked(
                    "0x23d84e00e69b16f915492c474a2b503eb2ff556c5b68fd3319cca404d7a3c6e",
                ),
                Felt::from_hex_unchecked(
                    "0x60ad0242b3f839ce26a48fab179628f81a02768082dfb09c48db6e44e6dd111",
                ),
                Felt::from_hex_unchecked(
                    "0x1feaa9ecc39d6ad84bc3bb79327b39731545e636ee2ca7ace1677cb8b1f6887",
                ),
                Felt::from_hex_unchecked(
                    "0x6cab97e59dae1d257f420aea18b44d84d40cc76cc6bbf81bb4307f5e28c88af",
                ),
                Felt::from_hex_unchecked(
                    "0x461c77600d1fee552f0b2ae8cb786c17c02360bcde0c1b2c568590792af759c",
                ),
            ],
        },
    }
}
