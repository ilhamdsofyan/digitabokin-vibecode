# Development Rules & Known Gotchas

Catatan penting yang ditemukan selama development. Baca ini sebelum nulis kode baru.

---

## 🦀 Rust / Axum

### 1. Axum 0.8+: Jangan Pakai `#[axum::async_trait]`
Sejak Axum 0.8, Rust native async fn in trait sudah stabil. **Jangan pakai** `#[axum::async_trait]` untuk implement `FromRequestParts`, `FromRequest`, dll.

```rust
// ❌ SALAH (Axum 0.7 style)
#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser { ... }

// ✅ BENAR (Axum 0.8+)
impl<S> FromRequestParts<S> for AuthUser { ... }
```

### 2. Raw String Literal: Gunakan `r##"..."##` untuk JSON yang Mengandung `#`
Hex color codes (`#FFFFFF`) di dalam raw string `r#"..."#` menyebabkan parse error karena `#` dianggap penutup string. Gunakan double hash `r##"..."##`.

```rust
// ❌ SALAH — `#FFFFFF` break the raw string
let json = r#"{ "color": "#FFFFFF" }"#;

// ✅ BENAR
let json = r##"{ "color": "#FFFFFF" }"##;
```

### 3. PowerShell: Jangan Pakai `&&` untuk Chain Commands
Windows PowerShell tidak support `&&`. Gunakan `;` sebagai separator.

```powershell
# ❌ SALAH
cargo fmt && cargo check

# ✅ BENAR
cargo fmt; cargo check
```

---

## 🗄️ Database / SeaORM

### 4. Hybrid ORM Strategy
- **SeaORM** → untuk operasi CRUD standar (insert, find, update, delete)
- **SQLx** (raw query) → untuk operasi JSONB yang kompleks (partial update, jsonb_set, deep query)
- Jangan campur dalam satu fungsi. Pisahkan di service layer.

### 5. Hindari PostgreSQL ENUM untuk Kolom yang Mungkin Berubah
`ALTER TYPE ... ADD VALUE` di PostgreSQL itu irreversible dalam transaction. Gunakan `VARCHAR` + `CHECK constraint` atau lookup table untuk value yang mungkin nambah di masa depan.

```sql
-- ❌ Risky
plan_type ENUM ('FREE', 'PREMIUM', 'PRO')

-- ✅ Lebih fleksibel
plan_type VARCHAR(20) NOT NULL CHECK (plan_type IN ('FREE', 'PREMIUM', 'PRO'))
```

### 6. UUID Consistency
Semua primary key entity utama pakai **UUID v4**, bukan SERIAL/INT. Ini untuk:
- Keamanan (tidak bisa ditebak/enumerate)
- Consistency antar tabel
- Siap untuk distributed system di masa depan

---

## 📁 Project Conventions

### 7. File & Module Structure
```
routes/     → HTTP handler (thin, hanya parsing request & return response)
services/   → Business logic (validasi, transformasi, orchestrasi)
models/     → SeaORM entities + Serde structs
middleware/ → Request guards (auth, rate limit, ownership check)
db/         → Connection pool + migration runner
```
**Rule:** Handler TIDAK boleh query database langsung. Selalu lewat service.

### 8. Error Response Format
Semua error API harus mengikuti format ini:
```json
{
  "success": false,
  "error": {
    "code": 401,
    "message": "Invalid or expired token"
  }
}
```

### 9. Git Commit Convention
Format: `<type>: <description>`

| Type | Kapan |
|------|-------|
| `feat` | Fitur baru |
| `fix` | Bug fix |
| `docs` | Dokumentasi |
| `chore` | Setup, config, tooling |
| `refactor` | Perubahan kode tanpa ubah behavior |

---

## 🔐 Auth Rules

### 10. JWT Token Strategy
- **Access Token**: 15 menit, stateless, di-validate tiap request
- **Refresh Token**: 7 hari, bisa dipakai untuk minta access token baru
- Dua-duanya di-sign dengan `JWT_SECRET` dari `.env`
- Token type (Access/Refresh) embedded di claims untuk mencegah misuse

### 11. Password Hashing
Selalu pakai **Argon2** (bukan bcrypt/scrypt). Sudah ter-setup di `auth_service.rs`.
