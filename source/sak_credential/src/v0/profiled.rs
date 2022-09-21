pub struct CredentialProfile {
    pub secret: String,
    pub public_key_str: String,
}

impl CredentialProfile {
    pub fn dev_local_1() -> CredentialProfile {
        CredentialProfile {
            public_key_str: String::from(
                "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f",
            ),
            secret: String::from(
                "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
            ),
        }
    }

    pub fn dev_local_2() -> CredentialProfile {
        CredentialProfile {
            public_key_str: String::from(
                "042c8d005bd935597117181d8ceceaef6d1162de78c32856\
                89d0c36c6170634c124f7b9b911553a1f483ec565c199ea29ff1\
                cd641f10c9a5f8c7c4d4a026db6f7b",
            ),
            secret: String::from(
                "224d0898389759f29ad5c9a6472b26fff86b6293889\
                88eec457a88ce50e907a0",
            ),
        }
    }

    pub fn test_1() -> CredentialProfile {
        CredentialProfile {
            public_key_str: String::from(
                "04d5ea1baff3cefd494c8a6c02f731f6e987b0a359851\
                    813294b5d2169a9a7099ba37eb0590a33de3a1b366a1e5\
                    4b87ce76c9a42fdc6ba1014a926bb21d1869d8f",
            ),
            secret: String::from(
                "60db9753b678fc491bf3c7b92b2352d1ca14ec\
                591c7f9d6bbd71e641967b1063",
            ),
        }
    }

    pub fn test_2() -> CredentialProfile {
        CredentialProfile {
            public_key_str: String::from(
                "049779d70ce5d3d1741d3d19a1610e2a56b2b242fdd9955dac7a59c8\
                7c335b8b0cbc18e4781001b793ec709cc60ac122569966a\
                11cfcbd5a99b4527909c6115e67",
            ),
            secret: String::from(
                "60db9753b678fc491bf3c7b92b2352d1ca14ec\
                591c7f9d6bbd71e641967b1063",
            ),
        }
    }

    pub fn test_3() -> CredentialProfile {
        CredentialProfile {
            public_key_str: String::from(
                "\
                04cda32b405650ba37f495713c549881\
                2b4a4fcde863f8361c50d59c28440434\
                415f5c8a572a8d460c22fc87ed52c7b8\
                d8ce385b9e594502382ce833fd772c9964",
            ),
            secret: String::from(
                "e7f0a95afb2c782cf9247d5f24c728fa\
                ba565ef85df6b74712005951620e95e2",
            ),
        }
    }

    pub fn test_4() -> CredentialProfile {
        CredentialProfile {
            public_key_str: String::from(
                "\
                0442c686b6e87e2b38958f71d6d6e9d0\
                d64eae30a56ae5265c02deede65e6f4f\
                bd41b6d68ed561ea0dd878c0d388e142\
                084f1c53edef1771fc2734b1936960894d",
            ),
            secret: String::from(
                "\
                f56c0091e188099de0d982f9bd6132ab\
                c9e4aa0becafb96ae169526912eb72b2",
            ),
        }
    }
}
