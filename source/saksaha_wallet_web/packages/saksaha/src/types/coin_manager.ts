import { CoinRecord, CoinStatus } from "./coin_record";

const DEV_LOCAL_1_SK = "7297b903877a957748b74068d63d6d566148197524099fc1df5cd9e8814c66c7";

export class CoinManager {
  coins: CoinRecord[]

  constructor(id?: String) {
    let coins = new Array<CoinRecord>;

    if (id == DEV_LOCAL_1_SK) {
      let coin_record_1: CoinRecord = {
        addr_pk: "0x65b4b8e3f33c9d4a5a3678715dbb56ca1613ad997f183d9e8762a5a8e020dd3b",
        addr_sk: "0x14",
        rho: "0x11",
        r: "0x12",
        s: "0x13",
        v: "0x64",
        cm: "0x1f432fca960606195092e1f9a30adc7c5537e4eccadfdc7692b1892137c91bcb",
        coin_status: CoinStatus.Unused,
        cm_idx: "0",
        coin_idx: "0",
        tx_hash: "0x87b2f2ca4c9c22de99c3b4c550c2fd09906644aa735b823bcd0446921ddec498",
      };

      let coin_record_2: CoinRecord = {
        addr_pk: "0x5554fdfb58f762cab6c219a492589a2666c62dd9af0bf65f66c5a7cf2319f4f2",
        addr_sk: "0x24",
        rho: "0x21",
        r: "0x22",
        s: "0x23",
        v: "0x64",
        cm: " 0x208bdfda89b9b5ebe1f362812778bc6b4234565a867b2c804f0c3f268b7e8cb7",
        coin_status: CoinStatus.Unused,
        cm_idx: "1",
        coin_idx: "1",
        tx_hash: "0x5fdc798f16ae272047631de0e2d925a083c39689cea69e5706a09224797fc99e",
      };

      coins.push(coin_record_1);
      coins.push(coin_record_2);
    } else {
      coins = [];
    }

    this.coins = coins;
  }
};

