# Business Logic & Monetization

## 1. Monetization Flow
- Check status `is_premium` pada template sebelum menyimpan data.
- Gunakan Webhook dari Payment Gateway (Midtrans/Stripe) untuk mengupdate status plugin secara otomatis di backend Rust.

## 2. Auto-Save Strategy
- Frontend: Debounce 3000ms (kirim data ke server setelah 3 detik berhenti berinteraksi).
- Backend: Update `design_state` dan naikkan `version` + 1.

# Core Logic Flows
1. **Creation:** User selects template -> System copies `templates.base_design` to `invitations.design_state`.
2. **Personalization:** `?to=Nama` parameter overwrites specific "Guest Name" element in JSONB during Guest View render.
3. **Paywall:** Backend checks `user.plan` before allowing `is_published = true` for premium templates.