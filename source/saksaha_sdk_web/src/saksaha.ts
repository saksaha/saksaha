import axios, { AxiosInstance } from 'axios';

const bootstrapEndpoints = [
  'http://localhost:34418/rpc/v0',
];

export class Saksaha {
  endpoints: string[];
  axios: AxiosInstance;

  constructor(endpoints?: string[]) {
    this.endpoints = bootstrapEndpoints;

    if (endpoints != undefined) {
      for (let e of endpoints) {
        this.endpoints.push(e);
      }
    }

    let ax = axios.create({
      baseURL: this.endpoints[0],
      timeout: 6000,
    });

    this.axios = ax;

    console.log("Saksaha constructed, endpoints: %o", this.endpoints);
  }

  async query(method: string, params?: Object) {
    let p = JSON.stringify(params);
    let params2 = Array.from(new TextEncoder().encode(p));

    const data = {
      jsonrpc: '2.0',
      method,
      params: params2,
      id: "1231",
    };

    return this.axios.post('', data)
      .then((res) => {
        console.log('axios success', res.data);
        if (res.data.result) {
          return res.data.result;
        } else {
          throw new Error("power");
        }
      }).catch((err) => {
        console.log('axios fail', err.response.data, params2);
      });
  }

  async send_tx(method: string, params?: Object) {
    let p = JSON.stringify(params);
    let params2 = Array.from(new TextEncoder().encode(p));

    const data = {
      jsonrpc: '2.0',
      method,
      params: params2,
      id: "1231",
    };

    return this.axios.post('', data)
      .then((res) => {
        console.log('axios success', res.data);
        if (res.data.result) {
          return res.data.result;
        } else {
          throw new Error("power");
        }
      }).catch((err) => {
        console.log('axios fail', err.response.data, params2);
      });
  }

}

export default Saksaha;
