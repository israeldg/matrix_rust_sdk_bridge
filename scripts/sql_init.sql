-- CREATE TABLE participants (
--     id UUID PRIMARY KEY,
--     matrix_user_id TEXT NOT NULL UNIQUE
-- );

CREATE TABLE conversations (
    id UUID PRIMARY KEY,
    room_id TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- CREATE TABLE conversation_participants (
--     conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
--     participant_id UUID NOT NULL REFERENCES participants(id) ON DELETE CASCADE,
--     PRIMARY KEY (conversation_id, participant_id)
-- );