# Frontend Specification: Next.js & Fabric.js
**Objective:** Editor WYSIWYG yang responsif dan smooth.

## Tech Stack
- **Framework:** Next.js (App Router).
- **Canvas Engine:** `Fabric.js`.
- **State Management:** `Zustand`.
- **Styling:** Tailwind CSS.

## Frontend Logic
1. **Canvas Sync:** Setiap objek di canvas harus memiliki ID unik yang dipetakan ke objek JSON di database.
2. **Modular Components:** Plugin (RSVP, Gift, Maps) harus dirender sebagai layer/overlay di atas canvas utama.
3. **Guest View:** Gunakan Server-Side Rendering (SSR) untuk performa loading tamu yang cepat dan SEO preview.