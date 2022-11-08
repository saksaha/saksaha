import Saksaha from "../saksaha";

export const get_tx = async (tx_hash: String) => {
  let axios: Saksaha = new Saksaha();

  const method = "get_tx";

  const params = {
    hash: tx_hash
  };

  let res = await axios.query(method, params);

  if (res === undefined) {
    return false;
  } else {
    return true;

  }
}
