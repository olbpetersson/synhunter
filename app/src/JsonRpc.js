export default class JsonRpc {
  constructor(method, params, id) {
    this.jsonrpc = "2.0";
    this.method = method;
    this.params = params;
    this.id = id;
  }


}