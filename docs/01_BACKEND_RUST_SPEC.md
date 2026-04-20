# Backend Specification: Rust (Axum)
**Objective:** High-performance, concurrent, and memory-safe API.

## Frameworks & Tools
- **Web Server:** `Axum` + `Tokio` runtime.
- **ORM:** `SeaORM` (Async-first).
- **Serialization:** `Serde` & `Serde_json`.
- **Database:** `SQLx` (PostgreSQL driver).

## Key Implementation Logic
1. **Design State:** Terima payload JSONB dari frontend, validasi strukturnya dengan `serde`.
2. **Access Control:** Middleware untuk mengecek kepemilikan undangan dan status pembayaran (IAP) sebelum modifikasi fitur premium.
3. **Optimistic Locking:** Gunakan kolom `version` (int) untuk mencegah tabrakan data saat edit di banyak tab.