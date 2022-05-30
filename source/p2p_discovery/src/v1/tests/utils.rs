use crate::Discovery;
use crate::DiscoveryArgs;
use p2p_addr::AddrStatus;
use p2p_addr::UnknownAddr;
use p2p_identity::Credential;
use std::ops::Index;
use std::slice::SliceIndex;
use std::sync::Arc;

pub(super) fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(Debug)]
pub struct DiscArgs {
    secret_key_str: String,
    public_key_str: String,
    bootstrap_args: Vec<BootstrapArgs>,
    disc_args_disc_port: Option<u16>,
}
#[derive(Debug)]
pub struct BootstrapArgs {
    pub bootstrap_disc_port: u16,
    pub bootstrap_public_key_str: Option<String>,
}

fn get_bootstrap_addrs(
    disc_port: u16,
    public_key_str: Option<String>,
) -> UnknownAddr {
    UnknownAddr {
        ip: String::from("127.0.0.1"),
        disc_port,
        p2p_port: None,
        sig: None,
        public_key_str,
        status: AddrStatus::Initialized,
    }
}

fn get_discovery_args(
    secret_key_str: String,
    public_key_str: String,
    disc_port: Option<u16>,
    bootstrap_addrs: Vec<UnknownAddr>,
) -> DiscoveryArgs {
    DiscoveryArgs {
        disc_dial_interval: None,
        disc_table_capacity: None,
        disc_task_interval: None,
        disc_task_queue_capacity: None,
        addr_expire_duration: None,
        credential: Arc::new(
            Credential::new(secret_key_str, public_key_str).unwrap(),
        ),
        disc_port,
        p2p_port: 1,
        bootstrap_addrs,
    }
}

pub fn discovery_run(disc: Arc<Discovery>) {
    tokio::spawn(async move {
        disc.run().await;
    });
}

pub fn get_public_key_str(index: u16) -> String {
    let index: usize = index as usize;
    let pub_keys = [
        "0",
        "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b84d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
        "04ce80d8c998044270b26eb7597bd92eb188807ace620644a34bf3be145422e61af51724079002c17758c33b88ade2e789a2153c1fd5b808c1f971127c2592009a",
        "0490825d65eb94a696b36b5a16d421465cb5da4bda5b698f098ff4fc9aa5ba9e2444320d083743e643ddb5c336c7062dfec41c41fa707f2d274106e4cc13d7709c",
        "0457a5dc3dc3f9e6f8711903c627185c8cb9278a056246e4bb8b676f6eb698a8ffa4f55e1bd90b798e032ba961a52465d4492b7d15a0133518fedc15b2ed2fa5a1",
        "04a59bc6b3a45525a44241b9b59cf7d2a290df5b3b171d258df7b5efc46afa494e5429a64e040b2479c8e5b5aa0c86865804f9ba075d6cd6dd1a6304c42536f565",
        "04196e1bb054f5a90beb6d2ec476664f7bf009d290b214ae584120447c922b8d8d96bfecb84949d41b74545d2eba6c39a095ae052ea497a401bbe385e3d62e1a4a",
        "0458478aa403b8331ce78a329bcac39481c6388b80cc83b1107ecc402a4c6d6e2defe09ef9e74ffc0fa96da2975335a8f745a59efdd1d880279618f84f7983c339",
        "044afabdc90398a50beee16199b9f055ea44ebd29fda680643f42126015cd2c7aad5efeef12304599697a00fd8ec9381dc6ff49e9d77729b4d078a56f3b66d9788",
        "04ca610d0cfa9c475cbc2ed30669bad26939464b0012870ebd70e08489581b8376ab7d616cee93214707afeaa85ac70150e7f8b7b2638d5fb424d9c8a9775848c2",
        "048e05fe400705a292a737fc42d0dfc0317cc133b41f7b57f46bfe44fde565cd8ab85846bf7761e26e4dc52f464dc27ef8a431cb8a7429b2b4a5c2e0b081709ee5",
    ];

    pub_keys[index].to_string()
}

pub(super) async fn create_disc(cli_index: u16) -> Arc<Discovery> {
    let index: usize = cli_index as usize;

    let test_clients = vec![ //
        DiscArgs {
            secret_key_str: String::from("secret_key_str"),
            public_key_str: String::from("public_key_str"),
            bootstrap_args: vec![BootstrapArgs {
                bootstrap_disc_port: 65535,
                bootstrap_public_key_str: Some(String::from("bootstrap_public_key_str")),
            }],
            disc_args_disc_port: Some(65535),
        },

        // 1
        DiscArgs {
        secret_key_str: String::from(
            "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786ca450b501b5f2",
        ),
        public_key_str: String::from(
            "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b84d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
        ),
            
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35521,
            bootstrap_public_key_str: Some(String::from(
               "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
                4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
           )),
        }],
        disc_args_disc_port: Some(35521),
    },
        // 2
        DiscArgs {
        secret_key_str: String::from(
            "445f36a1f7fafce4408b2277a5d009d1f1ba452d3996bfe7136de4adbfa34d61",
        ),
        // public_key_str: get_public_key_str(2),
        public_key_str: String::from(
            "04ce80d8c998044270b26eb7597bd92eb188807ace620644a34bf3be145422e61\
             af51724079002c17758c33b88ade2e789a2153c1fd5b808c1f971127c2592009a",
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35521,
            bootstrap_public_key_str: Some(String::from(
               "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
                4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
           )),
        }],
        disc_args_disc_port: Some(35522),
    },
        // 3
        DiscArgs {
        secret_key_str: String::from(
            "bfb61604245ff29b29b2cbf83a6c66ecd6ae521f723ed1bc4cc70479d432450e",
        ),
        public_key_str: String::from(
            "0490825d65eb94a696b36b5a16d421465cb5da4bda5b698f098ff4fc9aa5ba9e2444320d083743e643ddb5c336c7062dfec41c41fa707f2d274106e4cc13d7709c",
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35521,
            bootstrap_public_key_str: Some(String::from(
               "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
                4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
           )),
        }],
       disc_args_disc_port: Some(35523),
    },
        // 4
        DiscArgs {
        secret_key_str: String::from(
            "27d0ea0dcdcfbf72ddc3116494185d95d72d495f371bc71f20c3183156547dcc",
        ),
        public_key_str: String::from(
            "0457a5dc3dc3f9e6f8711903c627185c8cb9278a056246e4bb8b676f6eb698a8ffa4f55e1bd90b798e032ba961a52465d4492b7d15a0133518fedc15b2ed2fa5a1",
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35521,
            bootstrap_public_key_str: Some(String::from(
               "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
                4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
           )),
        }],
        disc_args_disc_port: Some(35524),
    },
        // 5
        DiscArgs {
        secret_key_str: String::from(
            "9c6925dfe5740c369fb029d088d12a2e8ad4acf2bff9c488c4af213cc8730123",
        ),
        public_key_str: String::from(
            "04a59bc6b3a45525a44241b9b59cf7d2a290df5b3b171d258df7b5efc46afa494e5429a64e040b2479c8e5b5aa0c86865804f9ba075d6cd6dd1a6304c42536f565",
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35521,
            bootstrap_public_key_str: Some(String::from(
               "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
                4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af",
           )),
        }],
        disc_args_disc_port: Some(35525),
    },
        // 6
        DiscArgs {
        secret_key_str: String::from(
            "01a1d19f1b69530c921c683271f97859661931a64942135ddd56ae6e99556dcc",
        ),
        public_key_str: String::from(
            "04196e1bb054f5a90beb6d2ec476664f7bf009d290b214ae584120447c922b8d8d96bfecb84949d41b74545d2eba6c39a095ae052ea497a401bbe385e3d62e1a4a",
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35527,
            bootstrap_public_key_str: Some(String::from(
                "\
                0458478aa403b8331ce78a329bcac3\
                9481c6388b80cc83b1107ecc402a4c\
                6d6e2defe09ef9e74ffc0fa96da297\
                5335a8f745a59efdd1d880279618f84f7983c339\
                "
           )),
        }],
        disc_args_disc_port: Some(35526),
    },
        // 7
        DiscArgs {
        secret_key_str: String::from(
            "98c1c0c444c19c122f7d202abd01eebab79cd8c1c4f6457bdcd7f1d3cc12db76",
        ),
        public_key_str: String::from(
            "\
            0458478aa403b8331ce78a329bcac39\
            481c6388b80cc83b1107ecc402a4c6d\
            6e2defe09ef9e74ffc0fa96da297533\
            5a8f745a59efdd1d880279618f84f7983c339\
            ",
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35528,
            bootstrap_public_key_str: Some(String::from(
                "044afabdc90398a50beee16199b9f055ea44ebd29fda680643f42126015cd2c7aad5efeef12304599697a00fd8ec9381dc6ff49e9d77729b4d078a56f3b66d9788"
            )),
        }],
        disc_args_disc_port: Some(35527),
    },
        // 8
        DiscArgs {
        secret_key_str: String::from(
            "e95bdc574fc8a5159ae5c4d26bdbf09a04b95bea66315212433c8ff2bcada140",
        ),
        public_key_str: String::from(
            "044afabdc90398a50beee16199b9f055ea44ebd29fda680643f42126015cd2c7aad5efeef12304599697a00fd8ec9381dc6ff49e9d77729b4d078a56f3b66d9788"
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35529,
            bootstrap_public_key_str: Some(String::from(
            "04ca610d0cfa9c475cbc2ed30669bad26939464b0012870ebd70e08489581b8376ab7d616cee93214707afeaa85ac70150e7f8b7b2638d5fb424d9c8a9775848c2",
            )),
        }],
        disc_args_disc_port: Some(35528),
    },
        // 9
        DiscArgs {
        secret_key_str: String::from(
            "e3946a634b67ccccf5c28c92814c90e8ac2c5139ae00aa6ccb64bcf9e3d0e8c4",
        ),
        public_key_str: String::from(
            "04ca610d0cfa9c475cbc2ed30669bad26939464b0012870ebd70e08489581b8376ab7d616cee93214707afeaa85ac70150e7f8b7b2638d5fb424d9c8a9775848c2",
        ),
        bootstrap_args: vec![BootstrapArgs {
            bootstrap_disc_port: 35526,
            bootstrap_public_key_str: Some(String::from(
            "04196e1bb054f5a90beb6d2ec476664f7bf009d290b214ae584120447c922b8d8d96bfecb84949d41b74545d2eba6c39a095ae052ea497a401bbe385e3d62e1a4a",
            )),
        }],
        disc_args_disc_port: Some(35529),
    },
        // 10
        DiscArgs {
        secret_key_str: String::from(
            "ce6ebd9c85c135e74c21f4f9f88263fdfb22874da18e34a17c0b792b326a7818",
        ),
        public_key_str: String::from(
            "048e05fe400705a292a737fc42d0dfc0317cc133b41f7b57f46bfe44fde565cd8ab85846bf7761e26e4dc52f464dc27ef8a431cb8a7429b2b4a5c2e0b081709ee5",
                    ),

        bootstrap_args: vec![ //
            BootstrapArgs {
            bootstrap_disc_port: 35521,
            bootstrap_public_key_str: Some(String::from(
                "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b84d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af"
            ))},
            BootstrapArgs {
            bootstrap_disc_port: 35527,
            bootstrap_public_key_str: Some(String::from(
                "0458478aa403b8331ce78a329bcac39481c6388b80cc83b1107ecc402a4c6d6e2defe09ef9e74ffc0fa96da2975335a8f745a59efdd1d880279618f84f7983c339"
           ))}],
        disc_args_disc_port: Some(35530),
    },


    ];

    let secret_key_str = test_clients[index].secret_key_str.clone();
    let public_key_str = test_clients[index].public_key_str.clone();

    let mut bootstrap_addrs = Vec::new();

    for boot_addr in test_clients[index].bootstrap_args.iter() {
        bootstrap_addrs.push(get_bootstrap_addrs(
            boot_addr.bootstrap_disc_port,
            boot_addr.bootstrap_public_key_str.clone(),
        ))
    }

    let disc_args = get_discovery_args(
        secret_key_str,
        public_key_str,
        test_clients[index].disc_args_disc_port.clone(),
        bootstrap_addrs,
    );

    let (p2p_discovery, disc_port) = {
        let (disc, disc_port) = Discovery::init(disc_args)
            .await
            .expect("Discovery should be initailized");

        (Arc::new(disc), disc_port)
    };

    p2p_discovery
}
