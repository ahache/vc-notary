## Credential Issuer Proxy

- This Rust server acts as a proxy to verify a TLSN presentation and extract data from the response portion of the presentation.
- It then sends the data to the credential service and routes the response to the back-end server.

### Running the server

- Run `cargo run` to start the server.
- The proxy runs on port 3333 and the credential service runs on port 3334.
