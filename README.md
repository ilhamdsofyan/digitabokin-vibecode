# 💌 Digitaria

**Platform Undangan Digital SaaS** — Buat undangan pernikahan digital yang cantik dengan editor drag-and-drop sekelas Canva.

---

## ✨ Fitur (V1 — Wedding Focus)

| Fitur | Deskripsi |
|-------|-----------|
| 🎨 **Canvas Editor** | WYSIWYG drag-and-drop editor (Fabric.js) |
| 📋 **Template Gallery** | Pilih template → langsung edit (pick-and-edit) |
| 💾 **Auto-Save** | Debounce 3 detik, optimistic locking |
| 👤 **Guest Personalization** | Link unik per tamu (`?to=Nama+Tamu`) |
| 📝 **RSVP** | Tamu konfirmasi kehadiran + kirim ucapan |
| 🔐 **Auth** | JWT Stateless + Google OAuth |
| 💳 **Payment** | Midtrans integration |
| 🌐 **Subdomain** | Setiap undangan punya slug unik |

## 🏗️ Tech Stack

```
Backend   → Rust (Axum + Tokio + SeaORM)
Frontend  → Next.js (App Router) + Fabric.js + Zustand
Database  → PostgreSQL (Relational + JSONB)
Deploy    → Docker + Caddy (SSL) + S3 Storage
CI/CD     → GitHub Actions
```

## 📁 Project Structure

```
digitaria/
├── backend/              # Rust Axum API
│   ├── src/
│   │   ├── main.rs           # Entry point
│   │   ├── config.rs         # Environment config
│   │   ├── errors.rs         # Unified error handling
│   │   ├── routes/           # API handlers
│   │   ├── models/           # SeaORM entities + Serde structs
│   │   ├── middleware/       # JWT auth, ownership check
│   │   ├── services/         # Business logic
│   │   └── db/               # DB connection + migrations
│   ├── Cargo.toml
│   └── .env.example
├── frontend/             # Next.js (coming soon)
├── docs/                 # Project documentation
├── docker-compose.yml    # PostgreSQL + services
└── .gitignore
```

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Docker](https://docs.docker.com/get-docker/) & Docker Compose
- PostgreSQL 16+

### 1. Start Database

```bash
docker compose up -d db
```

### 2. Setup Backend

```bash
cd backend
cp .env.example .env    # edit sesuai kebutuhan
cargo run
```

Server jalan di `http://localhost:3001`

### 3. Health Check

```bash
curl http://localhost:3001/api/v1/health
```

Response:
```json
{
  "status": "ok",
  "service": "digitaria-api",
  "version": "0.1.0"
}
```

## 📡 API Endpoints (Planned)

```
POST   /api/v1/auth/register        # Register
POST   /api/v1/auth/login            # Login
POST   /api/v1/auth/google           # Google OAuth
POST   /api/v1/auth/refresh          # Refresh token

GET    /api/v1/templates             # List templates
POST   /api/v1/invitations           # Create from template
PUT    /api/v1/invitations/:id/design # Auto-save design
PATCH  /api/v1/invitations/:id/publish # Publish

POST   /api/v1/rsvp/:invitation_id   # Submit RSVP
POST   /api/v1/payments/create       # Initiate payment
POST   /api/v1/webhooks/midtrans     # Payment webhook

GET    /undangan/:slug?to=Nama       # Guest view (public)
```

## 📖 Documentation

Dokumentasi lengkap ada di folder [`docs/`](./docs):

- [`00_PROJECT_OVERVIEW.md`](./docs/00_PROJECT_OVERVIEW.md) — Vision & tech decisions
- [`01_BACKEND_RUST_SPEC.md`](./docs/01_BACKEND_RUST_SPEC.md) — Backend spec
- [`02_FRONTEND_REACT_SPEC.md`](./docs/02_FRONTEND_REACT_SPEC.md) — Frontend spec
- [`03_DATABASE_SCHEMA.md`](./docs/03_DATABASE_SCHEMA.md) — Database schema
- [`04_BUSINESS_LOGIC_FLOWS.md`](./docs/04_BUSINESS_LOGIC_FLOWS.md) — Business logic
- [`05_DEPLOYMENT_INFRA.md`](./docs/05_DEPLOYMENT_INFRA.md) — Deployment & infra

## 🗓️ Roadmap

- [x] **Phase 1** — Project foundation & monorepo setup
- [ ] **Phase 2** — Auth system (JWT + Google OAuth)
- [ ] **Phase 3** — Template & invitation CRUD
- [ ] **Phase 4** — Guest view & RSVP
- [ ] **Phase 5** — Payment (Midtrans)
- [ ] **Phase 6** — Media upload & asset management
- [ ] **Phase 7** — Docker deployment + Caddy SSL

---

<p align="center">
  Built with 🦀 Rust &amp; ⚛️ React
</p>
