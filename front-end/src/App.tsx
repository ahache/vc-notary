import { useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import "./App.css"

function App() {
  const [did, setDid] = useState('');

  const clientId = import.meta.env.VITE_REDDIT_CLIENT_ID;
  const redirectUri = import.meta.env.VITE_REDDIT_REDIRECT_URI;
  const scopes = import.meta.env.VITE_REDDIT_SCOPES;
  
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const code = urlParams.get('code');
    
    if (code) {
      const savedDid = localStorage.getItem('pendingDID');
      if (savedDid) {
        setDid(savedDid);
        localStorage.removeItem('pendingDID');
      }
      
      console.log('Received code:', code);
      window.history.replaceState({}, document.title, window.location.pathname);
    }
  }, []);

  const handleAuth = () => {
    localStorage.setItem('pendingDID', did);

    const state = crypto.randomUUID();

    const authUrl = `https://www.reddit.com/api/v1/authorize?client_id=${clientId}&response_type=code&state=${state}&redirect_uri=${redirectUri}&duration=temporary&scope=${scopes}`;
    window.location.href = authUrl;
  };

  return (
    <div>
      <h1 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
        VC Notary
      </h1>
      <Input 
        placeholder="Enter your DID" 
        value={did}
        onChange={(e) => setDid(e.target.value)}
      />
      <Button onClick={handleAuth}>Get Your VC</Button>
    </div>
  )
}

export default App
