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
}
