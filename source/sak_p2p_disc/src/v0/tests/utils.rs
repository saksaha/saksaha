use crate::Discovery;
use crate::DiscoveryArgs;
use lazy_static::lazy_static;
use sak_p2p_addr::AddrStatus;
use sak_p2p_addr::UnknownAddr;
use sak_p2p_id::Credential;
use std::sync::Arc;

lazy_static! {
    static ref DISC_ARGS_VEC: Vec<DiscoveryArgs> = {
        let v = vec![
            // dummy
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "aa99cfd91cc6f3b541d28f3e0707f9c7\
                     bcf05cf495308294786ca450b501b5f2",
                        ),
                        String::from(
                            "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(0),
                p2p_port: 1,
                bootstrap_addrs: vec![],
            },
            // 1
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "aa99cfd91cc6f3b541d28f3e0707f9c7\
                bcf05cf495308294786ca450b501b5f2",
                        ),
                        String::from(
                            "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35521),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35521,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 2
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "445f36a1f7fafce4408b2277a5d009d1\
                 f1ba452d3996bfe7136de4adbfa34d61",
                        ),
                        String::from(
                            "04ce80d8c998044270b26eb7597bd92e\
                 b188807ace620644a34bf3be145422e61\
                 af51724079002c17758c33b88ade2e78\
                 9a2153c1fd5b808c1f971127c2592009a",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35522),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35521,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 3
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "bfb61604245ff29b29b2cbf83a6c66ec\
                 d6ae521f723ed1bc4cc70479d432450e",
                        ),
                        String::from(
                            "0490825d65eb94a696b36b5a16d42146\
                 5cb5da4bda5b698f098ff4fc9aa5ba9e\
                 2444320d083743e643ddb5c336c7062d\
                 fec41c41fa707f2d274106e4cc13d7709c",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35523),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35521,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 4
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "27d0ea0dcdcfbf72ddc3116494185d95\
                 d72d495f371bc71f20c3183156547dcc",
                        ),
                        String::from(
                            "0457a5dc3dc3f9e6f8711903c627185c\
                 8cb9278a056246e4bb8b676f6eb698a8\
                 ffa4f55e1bd90b798e032ba961a52465\
                 d4492b7d15a0133518fedc15b2ed2fa5a1",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35524),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35521,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 5
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "9c6925dfe5740c369fb029d088d12a2e\
                 8ad4acf2bff9c488c4af213cc8730123",
                        ),
                        String::from(
                            "04a59bc6b3a45525a44241b9b59cf7d2\
                 a290df5b3b171d258df7b5efc46afa49\
                 4e5429a64e040b2479c8e5b5aa0c8686\
                 5804f9ba075d6cd6dd1a6304c42536f565",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35525),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35521,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 6
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "01a1d19f1b69530c921c683271f97859\
                 661931a64942135ddd56ae6e99556dcc",
                        ),
                        String::from(
                            "04196e1bb054f5a90beb6d2ec476664f\
                 7bf009d290b214ae584120447c922b8d\
                 8d96bfecb84949d41b74545d2eba6c39\
                 a095ae052ea497a401bbe385e3d62e1a4a",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35526),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35527,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "\
                    0458478aa403b8331ce78a329bcac3\
                    9481c6388b80cc83b1107ecc402a4c\
                    6d6e2defe09ef9e74ffc0fa96da297\
                    5335a8f745a59efdd1d880279618f84f7983c339\
                    ",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 7
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "98c1c0c444c19c122f7d202abd01eeba\
                 b79cd8c1c4f6457bdcd7f1d3cc12db76",
                        ),
                        String::from(
                            "\
                0458478aa403b8331ce78a329bcac39\
                481c6388b80cc83b1107ecc402a4c6d\
                6e2defe09ef9e74ffc0fa96da297533\
                5a8f745a59efdd1d880279618f84f7983c339\
                ",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35527),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35528,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "044afabdc90398a50beee16199b9f055\
                     ea44ebd29fda680643f42126015cd2c7\
                     aad5efeef12304599697a00fd8ec9381\
                     dc6ff49e9d77729b4d078a56f3b66d9788",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 8
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "e95bdc574fc8a5159ae5c4d26bdbf09a\
                     04b95bea66315212433c8ff2bcada140",
                        ),
                        String::from(
                            "044afabdc90398a50beee16199b9f055\
                     ea44ebd29fda680643f42126015cd2c7\
                     aad5efeef12304599697a00fd8ec9381\
                     dc6ff49e9d77729b4d078a56f3b66d9788",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35528),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35529,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "04ca610d0cfa9c475cbc2ed30669bad2\
                         6939464b0012870ebd70e08489581b83\
                         76ab7d616cee93214707afeaa85ac701\
                         50e7f8b7b2638d5fb424d9c8a9775848c2",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 9
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "e3946a634b67ccccf5c28c92814c90e8\
                 ac2c5139ae00aa6ccb64bcf9e3d0e8c4",
                        ),
                        String::from(
                            "04ca610d0cfa9c475cbc2ed30669bad2\
                 6939464b0012870ebd70e08489581b83\
                 76ab7d616cee93214707afeaa85ac701\
                 50e7f8b7b2638d5fb424d9c8a9775848c2",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35529),
                p2p_port: 1,
                bootstrap_addrs: vec![UnknownAddr {
                    ip: String::from("127.0.0.1"),
                    disc_port: 35526,
                    p2p_port: None,
                    sig: None,
                    public_key_str: Some(String::from(
                        "04196e1bb054f5a90beb6d2ec476664f\
                     7bf009d290b214ae584120447c922b8d\
                     8d96bfecb84949d41b74545d2eba6c39\
                     a095ae052ea497a401bbe385e3d62e1a4a",
                    )),
                    status: AddrStatus::Initialized,
                }],
            },
            // 10
            DiscoveryArgs {
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                addr_expire_duration: None,
                credential: Arc::new(
                    Credential::new(
                        String::from(
                            "ce6ebd9c85c135e74c21f4f9f88263fd\
                 fb22874da18e34a17c0b792b326a7818",
                        ),
                        String::from(
                            "048e05fe400705a292a737fc42d0dfc0\
                 317cc133b41f7b57f46bfe44fde565cd\
                 8ab85846bf7761e26e4dc52f464dc27e\
                 f8a431cb8a7429b2b4a5c2e0b081709ee5",
                        ),
                    )
                    .unwrap(),
                ),
                disc_port: Some(35530),
                p2p_port: 1,
                bootstrap_addrs: vec![
                    UnknownAddr {
                        ip: String::from("127.0.0.1"),
                        disc_port: 35521,
                        p2p_port: None,
                        sig: None,
                        public_key_str: Some(String::from(
                            "04240874d8c323c22a571f735e835ed2\
                     f0619893a3989e557b1c9b4c699ac92b\
                     84d0dc478108629c0353f2876941f90d\
                     4b36346bcc19c6b625422adffb53b3a6af",
                        )),
                        status: AddrStatus::Initialized,
                    },
                    UnknownAddr {
                        ip: String::from("127.0.0.1"),
                        disc_port: 35527,
                        p2p_port: None,
                        sig: None,
                        public_key_str: Some(String::from(
                            "0458478aa403b8331ce78a329bcac394\
                     81c6388b80cc83b1107ecc402a4c6d6e\
                     2defe09ef9e74ffc0fa96da2975335a8\
                     f745a59efdd1d880279618f84f7983c339",
                        )),
                        status: AddrStatus::Initialized,
                    },
                ],
            },
        ];

        v
    };

}

pub(super) fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(Debug)]
pub struct BootstrapArgs {
    pub bootstrap_disc_port: u16,
    pub bootstrap_public_key_str: Option<String>,
}

pub fn discovery_run(disc: Arc<Discovery>) {
    tokio::spawn(async move {
        disc.run().await;
    });
}

pub(super) async fn create_disc(disc_idx: u16) -> (Arc<Discovery>, String) {
    let idx: usize = disc_idx as usize;

    let disc_args = DISC_ARGS_VEC
        .get(idx)
        .expect("Discovery arg should be provided");

    let public_key_str = disc_args.credential.public_key_str.clone();

    let (disc, _) = Discovery::init(disc_args.clone())
        .await
        .expect("Discovery should be initialized");

    (Arc::new(disc), public_key_str)
}
