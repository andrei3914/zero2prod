-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '7dadc51c-f1db-4e8c-bbf5-f3660a0e0853',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8'
)
