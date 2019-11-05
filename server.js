const express = require('express')
const path = require('path')

const app = express()

app.use(express.static(path.join(__dirname, 'dist')))

port = 3000
app.listen(port, () => console.log(`Example app listening on port ${port}!`))
