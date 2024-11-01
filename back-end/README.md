## Back-End

- This Rust server acts as the primary server for the VC Notary project.
- It handles the authentication exchange from access code to bearer token, MPC with the notary server and communication with the credential service.

### Running the server

- Create a `.env` from the `.env.example` file:
  - These values are available after creating a Reddit web app.
  - `REDDIT_REDIRECT_URI` should be set to the front-end URL. Ensure every character matches exactly.
- Run `cargo run` to start the server.
- The server runs on port 8000.