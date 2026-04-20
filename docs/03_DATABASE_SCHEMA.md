# Detailed Database Schema: PostgreSQL for Digitaria

## 1. User & Subscription Management
### `users`
- `id`: SERIAL PRIMARY KEY
- `email`: VARCHAR(255) UNIQUE NOT NULL
- `password_hash`: TEXT NOT NULL
- `full_name`: VARCHAR(255)
- `created_at`: TIMESTAMP DEFAULT NOW()

### `user_subscriptions`
- `id`: SERIAL PRIMARY KEY
- `user_id`: INT REFERENCES users(id)
- `plan_type`: ENUM ('FREE', 'PREMIUM', 'PRO')
- `is_active`: BOOLEAN DEFAULT TRUE
- `expired_at`: TIMESTAMP

## 2. Core Invitation Engine
### `invitations`
- `id`: UUID PRIMARY KEY DEFAULT gen_random_uuid()
- `user_id`: INT REFERENCES users(id)
- `template_id`: INT REFERENCES templates(id)
- `slug`: VARCHAR(100) UNIQUE NOT NULL -- (e.g. 'budi-ani-wedding')
- `title`: VARCHAR(255)
- `thumbnail_url`: TEXT -- URL hasil render canvas untuk dashboard
- `design_state`: JSONB NOT NULL -- (Lihat section JSONB Structure di bawah)
- `music_url`: TEXT -- Background music link
- `is_published`: BOOLEAN DEFAULT FALSE
- `version`: INT DEFAULT 1 -- Untuk Optimistic Locking
- `created_at`: TIMESTAMP DEFAULT NOW()
- `updated_at`: TIMESTAMP DEFAULT NOW()

### `templates`
- `id`: SERIAL PRIMARY KEY
- `name`: VARCHAR(255)
- `category`: ENUM ('WEDDING', 'BIRTHDAY', 'EVENT')
- `base_design_state`: JSONB NOT NULL -- Default layout awal
- `is_premium`: BOOLEAN DEFAULT FALSE
- `preview_image`: TEXT

## 3. Plugin & Interaction System
### `plugins`
- `id`: SERIAL PRIMARY KEY
- `name`: VARCHAR(50) -- 'RSVP', 'GIFT', 'MAPS', 'MUSIC'
- `slug`: VARCHAR(50) UNIQUE
- `is_premium`: BOOLEAN DEFAULT FALSE

### `invitation_plugins`
- `id`: SERIAL PRIMARY KEY
- `invitation_id`: UUID REFERENCES invitations(id) ON DELETE CASCADE
- `plugin_id`: INT REFERENCES plugins(id)
- `config_data`: JSONB -- (e.g. deadline RSVP, nomor rekening Gift, koordinat Maps)
- `is_enabled`: BOOLEAN DEFAULT TRUE

### `rsvp_responses`
- `id`: SERIAL PRIMARY KEY
- `invitation_id`: UUID REFERENCES invitations(id)
- `guest_name`: VARCHAR(255)
- `attendance_status`: ENUM ('Hadir', 'Tidak Hadir', 'Ragu')
- `guest_count`: INT DEFAULT 1
- `message`: TEXT
- `extra_data`: JSONB -- (e.g. Request lagu, alergi makanan)
- `created_at`: TIMESTAMP DEFAULT NOW()

## 4. Asset Management
### `media_assets`
- `id`: SERIAL PRIMARY KEY
- `user_id`: INT REFERENCES users(id)
- `file_url`: TEXT NOT NULL
- `file_type`: VARCHAR(20) -- 'image/png', 'video/mp4'
- `file_size`: INT
- `created_at`: TIMESTAMP DEFAULT NOW()

---

## 5. JSONB Design State Specification (The "Canva" Core)
*Instruksi buat AI: Gunakan struktur ini untuk mapping Serde Struct di Rust.*

Setiap `design_state` harus mengikuti struktur berikut:
```json
{
  "canvas": {
    "width": 1080,
    "height": 1920,
    "backgroundColor": "#FFFFFF",
    "globalFont": "Inter",
    "themeColor": "#D4AF37"
  },
  "layers": [
    {
      "id": "element_uuid_1",
      "type": "text",
      "content": "The Wedding of",
      "props": {
        "x": 540,
        "y": 200,
        "fontSize": 24,
        "fontWeight": "bold",
        "fontFamily": "Playfair Display",
        "color": "#000000",
        "textAlign": "center",
        "rotation": 0,
        "opacity": 1,
        "zIndex": 1
      }
    },
    {
      "id": "element_uuid_2",
      "type": "image",
      "url": "[https://s3.storage.com/asset.jpg](https://s3.storage.com/asset.jpg)",
      "props": {
        "x": 540,
        "y": 600,
        "width": 800,
        "height": 1200,
        "borderRadius": 20,
        "zIndex": 2
      }
    }
  ],
  "config": {
    "autoPlayMusic": true,
    "showWatermark": false
  }
}