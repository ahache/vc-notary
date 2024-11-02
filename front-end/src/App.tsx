import { useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Skeleton } from "@/components/ui/skeleton"
import "./App.css"
import axios from "axios"
import { getRedditAuthUrl } from "@/lib/auth"
import { QRCodeSVG } from "qrcode.react"

function App() {
  const [did, setDid] = useState('');
  const [credential, setCredential] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(false);
  
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const code = urlParams.get('code');
    
    if (code) {
      const savedDid = localStorage.getItem('pendingDID');
      if (savedDid) {
        setDid(savedDid);
        setIsLoading(true);

        axios.post('http://localhost:8000/api/request_vc', { code, did: savedDid })
        .then(response => {
          setCredential(response.data);
          setDid('');
          localStorage.removeItem('pendingDID');
        })
        .catch(error => {
          console.error(error);
        })
        .finally(() => {
          setIsLoading(false);
        });
      }
      
      window.history.replaceState({}, document.title, window.location.pathname);
    }
  }, []);

  const handleAuth = () => {
    localStorage.setItem('pendingDID', did);

    window.location.href = getRedditAuthUrl();
  };

  return (
    <div className="min-h-screen flex flex-col p-8">
      <header className="mb-16">
        <h1 className="text-4xl font-bold">VC Notary</h1>
      </header>

      <main className="flex-1">
        <div className="max-w-3xl mx-auto">
          <div className="flex items-center gap-4 mb-8">
            <label htmlFor="did-input" className="min-w-24 text-right">
              Input your DID:
            </label>
            <Input 
              id="did-input"
              placeholder="Enter your DID" 
              value={did}
              onChange={(e) => setDid(e.target.value)}
              className="flex-1"
            />
            <Button onClick={handleAuth}>Get Your VC</Button>
          </div>

          {isLoading && (
            <div className="grid grid-cols-2 gap-8 mt-8">
              <div className="border rounded-lg p-4">
                <div className="space-y-3">
                  <Skeleton className="h-4 w-full" />
                  <Skeleton className="h-4 w-[90%]" />
                  <Skeleton className="h-4 w-[85%]" />
                  <Skeleton className="h-4 w-[80%]" />
                </div>
              </div>
              <div className="border rounded-lg p-4 flex items-center justify-center">
                <Skeleton className="h-64 w-64" />
              </div>
            </div>
          )}

          {credential && (
            <div className="grid grid-cols-2 gap-8 mt-8">
              <div className="border rounded-lg p-4 min-h-[300px] overflow-auto">
                {credential ? (
                  <pre className="text-xs whitespace-pre-wrap">
                    {JSON.stringify(credential, null, 2)}
                  </pre>
                ) : (
                  <p className="text-muted-foreground">Credential</p>
                )}
              </div>
              <div className="border rounded-lg p-4 min-h-[300px] flex items-center justify-center">
                {credential ? (
                  <QRCodeSVG 
                    value={JSON.stringify(credential)}
                    size={256}
                  />
                ) : (
                  <p className="text-muted-foreground">QR Code</p>
                )}
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  )
}

export default App
