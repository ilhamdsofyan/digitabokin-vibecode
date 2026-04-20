CREATE TABLE IF NOT EXISTS rsvp_responses (
    id SERIAL PRIMARY KEY,
    invitation_id UUID NOT NULL REFERENCES invitations(id) ON DELETE CASCADE,
    guest_name VARCHAR(255) NOT NULL,
    attendance_status VARCHAR(50) NOT NULL CHECK (attendance_status IN ('Hadir', 'Tidak Hadir', 'Ragu')),
    guest_count INT NOT NULL DEFAULT 1,
    message TEXT,
    extra_data JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
