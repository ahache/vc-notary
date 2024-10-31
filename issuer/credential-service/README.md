## Credential Service

#### Setup
- Install dependencies: `npm install`
- Create `.env` file based on `.env.example`
    - Will need Infura Project ID
    - Will need Secret Key: `npx @veramo/cli config create-secret-key`
- Run `npx tsx src/create-identifier.ts` to create a new identifier

- Run server: `npx tsx src/server.ts`
