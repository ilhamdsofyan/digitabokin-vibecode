# Project Digitaria: Overview
**Vision:** Platform Undangan Digital SaaS dengan fleksibilitas sekelas Canva.
**Arsitektur:** Micro-monolith dengan Rust (Backend) dan Next.js (Frontend).

## Core Stack
- **Backend:** Rust (Axum Framework).
- **Frontend:** Next.js (React) + Fabric.js untuk Editor.
- **Database:** PostgreSQL (Hybrid Relational + JSONB).
- **Deployment:** Docker & Caddy (On-Demand TLS for Custom Domains).

## Project Rules for AI
1. Backend harus **Type-safe**. Gunakan `serde` untuk mapping JSONB ke Rust Structs.
2. Jangan gunakan library Python/Django di backend.
3. Editor harus fleksibel (Drag-and-drop) menggunakan `Fabric.js`.
4. Komunikasi Frontend-Backend untuk save data menggunakan teknik **Debouncing**.

## Technical Decisions (V1)
- **Auth:** JWT Stateless with Google OAuth.
- **Guest Link:** Dynamic parameter-based (`?to=Nama+Tamu`).
- **ORM:** Hybrid (SeaORM for CRUD, SQLx for Complex JSONB).
- **Payment:** Midtrans Integration.
- **Market Focus:** Wedding Invitations.
- **Infrastructure:** Caddy for SSL, Docker Monorepo.

## Core Decisions
- **Structure:** Monorepo (backend/ & frontend/).
- **Auth:** JWT Stateless + Google OAuth.
- **Guest System:** Unique Link with `?to=` parameter.
- **Template System:** Pick-and-Edit (No blank canvas in V1).
- **Custom Domain:** Deferred to Phase 2 (Focus on subdomain V1).
- **Payment:** Midtrans (ID Market).
- **Development:** Solo/Duo, Trunk-based.

## Struktur
/digitaria
├── backend/ (Rust Axum)
├── frontend/ (Next.js)
├── docs/ (File .md)
└── docker-compose.yml