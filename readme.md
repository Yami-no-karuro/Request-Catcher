# Request Catcher

## A simple command line tool to debug HTTP clients

### Introduction

This project implements a simple HTTP server in Rust.  
Requests details are printed to the console.

### Usage & Examples

Once started the server will listen on port 5000.

```sh
curl -X POST http://localhost:5000 -H "Content-Type: application/json" -d '{"foo": "bar", "bar": "foo"}'
curl -X POST http://localhost:5000 -H "Content-Type: multipart/form-data" -F "foo=bar" -F "bar=foo"
curl -X GET http://localhost:5000
```
