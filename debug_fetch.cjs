const https = require('https');

const url = 'https://dev.leclasseur.ca/static/extension/version.json';

console.log(`Fetching ${url}...`);

const req = https.get(url, (res) => {
    console.log('Status:', res.statusCode);
    console.log('Headers:', res.headers);

    let data = '';
    res.on('data', (chunk) => {
        data += chunk;
    });

    res.on('end', () => {
        console.log('Body length:', data.length);
        console.log('Body start:', data.substring(0, 100)); // First 100 chars
        try {
            JSON.parse(data);
            console.log('JSON Parse: SUCCESS');
        } catch (e) {
            console.log('JSON Parse: FAILED', e.message);
        }
    });
});

req.on('error', (e) => {
    console.error('Request Error:', e);
});
