import express from 'express'
import bodyParser from 'body-parser'
import createCredential from './create-credential'

const PORT = 3334

const app = express()

app.use(bodyParser.json())

app.post('/create-credential', async (req, res) => {
    const { did, subreddit } = req.body;

    const credential = await createCredential({ did, subreddit });

    res.json(credential);
})

app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`)
})
