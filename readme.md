# Request Catcher

## A simple command line tool to debug HTTP clients

### Introduction

This project implements a simple [HTTP](https://en.wikipedia.org/wiki/HTTP) server in Rust, with the purpose of capturing incoming *HTTP* traffic on a specific port.  
The information for each request is collected in the following format:

```txt
Request: [<method>] - <path>
Headers:
[
    Host: <host>
    Connection: <connection>
    ...
]
Body: 
<body>
```

### Usage & Examples

To start the server execute `cargo run <port>`.

```sh
curl -X POST http://localhost:5000 -H "Content-Type: application/json" -d '{"foo": "bar", "bar": "foo"}'
curl -X POST http://localhost:5000 -H "Content-Type: multipart/form-data" -F "foo=bar" -F "bar=foo"
curl -X GET http://localhost:5000
```
