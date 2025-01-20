import http from 'k6/http';

let seq = 1

export default function () {
  const url = 'http://127.0.0.1:1630/rpc';
  const payload = JSON.stringify({
    "jsonrpc": "2.0",
    "id": `${seq++}`,
    "method": "hello",
    "params": []
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  http.post(url, payload, params);

  // http.get("http://127.0.0.1:12080/cmV0dXJuICJVUCI=?env=default")
}
