# Deployment: Docker & Caddy
**Objective:** Otomasi deployment dan SSL custom domain.

## Setup
1. **Containerization:** Docker & Docker Compose.
2. **Reverse Proxy:** Caddy Server (untuk On-Demand TLS).
3. **Storage:** S3-Compatible (DigitalOcean Spaces / AWS S3) untuk foto undangan.
4. **CI/CD:** GitHub Actions (Build Rust binary $\rightarrow$ Push ke Docker Registry).