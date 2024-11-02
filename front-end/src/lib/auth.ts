export const getRedditAuthUrl = () => {
    const clientId = import.meta.env.VITE_REDDIT_CLIENT_ID;
    const redirectUri = import.meta.env.VITE_REDDIT_REDIRECT_URI;
    const scopes = import.meta.env.VITE_REDDIT_SCOPES;

    const state = crypto.randomUUID();
  
    const params = new URLSearchParams({
      client_id: clientId,
      response_type: 'code',
      state,
      redirect_uri: redirectUri,
      duration: 'temporary',
      scope: scopes,
    });
  
    return `https://www.reddit.com/api/v1/authorize?${params.toString()}`;

}