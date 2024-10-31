import { agent } from './veramo/setup'

const createCredential = async ({ did, subreddit }: { did: string, subreddit: string }) => {
    const identifier = await agent.didManagerGetByAlias({ alias: 'default' });
    
    const verifiableCredential = await agent.createVerifiableCredential({
        credential: {
            issuer: { id: identifier.did },
            credentialSubject: {
                id: did,
                moderator: subreddit,
            },
        },
        proofFormat: 'jwt',
    })

    return verifiableCredential;
}

export default createCredential
