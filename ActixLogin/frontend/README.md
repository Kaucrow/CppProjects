# Important
## For HTTPS on development environment
* Set `VITE_API_URI_DEV` to `https://localhost:443` on the .env file.
* Set the `NODE_EXTRA_CA_CERTS` environment variable to the root CA .pem file path. If generating the certs with mkcert on Windows, set it to `C:\Users\YOUR_USER\AppData\Local\mkcert\rootCA.pem`