use super::{DConfig, DefaultP2PConfig, Identity};

pub(crate) fn get_dev_local_config() -> DConfig {
    return DConfig {
        p2p: DefaultP2PConfig {
            peers: vec![
                Identity {
                    secret: String::from(
                        "\
                    9a3d9fafda4a1b0e420d339a3e42c2c8\
                    1062ba788018eb6639b5d2eedeb2d13e\
                    ",
                    ),
                    public_key: String::from(
                        "\
                    04715796a40b0d58fc14a3c4ebee21cb\
                    806763066a7f1a17adbc256999764443\
                    beb8109cfd000718535c5aa27513a2ed\
                    afc6e8bdbe7c27edc2980f9bbc25142fc5\
                    ",
                    ),
                },
                Identity {
                    secret: String::from(
                        "\
                    e364bcd1cd3769fc44cf026864f7ad98\
                    a1ba1721fb5096d0f4ea7710efe03a3d\
                    ",
                    ),
                    public_key: String::from(
                        "\
                    048e34a97e173aa7d978f3b3d720487c\
                    d10c64020e334eae6789cbee6f0856b2\
                    4953989b5067afff361e3a4534abe506\
                    91bca238e900a4dff496ef0c84400c26f4\
                    ",
                    ),
                },
                Identity {
                    secret: String::from(
                        "\
                    c50754a60a2261fc777267486a9c43d8\
                    df965e7721017da6deb5a64fa20a907c\
                    ",
                    ),
                    public_key: String::from(
                        "\
                    0467e86ec86e8a520503818a4fa09724\
                    08e56917c7cc76a514e781d8036bd923\
                    8483a3d8bc5917e2592c4ad0181a1606\
                    58a536841411f7567f6a673f16ef71191f\
                    ",
                    ),
                },
                Identity {
                    secret: String::from(
                        "\
                    6127f232f79cc329aa33a09776e0212c\
                    5ce76bd1b7f56956d5f87a74b9e07bd2\
                    ",
                    ),
                    public_key: String::from(
                        "\
                    0431f595b429a2a6d998030f9a7d71df\
                    7d617109bf70b91a144ed6ef16dcef2d\
                    0bf285a541f50bd49b3e4ffc534881b1\
                    ddeb915b8aa13f6f318b191e02d8fad885\
                    ",
                    ),
                },
            ],
        },
    };
}
