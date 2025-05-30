# SSL/TLS Setup Guide

## Generating Self-Signed Certificates for Development

1. Create a certificates directory:
```bash
mkdir -p certs
```

2. Generate a private key:
```bash
openssl genrsa -out certs/key.pem 2048
```

3. Generate a certificate signing request:
```bash
openssl req -new -key certs/key.pem -out certs/cert.csr
```

4. Generate a self-signed certificate:
```bash
openssl x509 -req -days 365 -in certs/cert.csr -signkey certs/key.pem -out certs/cert.pem
```

## Configuration

Add the following to your `.env` file to enable HTTPS:

```env
TLS_CERT_PATH=./certs/cert.pem
TLS_KEY_PATH=./certs/key.pem
```

## Production Certificates

For production, use certificates from a trusted Certificate Authority (CA) like:
- Let's Encrypt (free)
- Cloudflare
- DigiCert
- Other commercial CAs

## Frontend Configuration

Update your frontend to use HTTPS URLs when connecting to the backend:

```javascript
const API_BASE_URL = 'https://localhost:3001';
```

## Security Considerations

1. **Never commit private keys** to version control
2. Use strong passwords for key files in production
3. Regularly rotate certificates
4. Use HSTS headers in production
5. Consider using a reverse proxy (nginx, Apache) for SSL termination
