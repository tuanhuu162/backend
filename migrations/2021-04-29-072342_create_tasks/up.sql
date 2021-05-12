-- Your SQL goes here
create table tasks (
    task_id SERIAL PRIMARY KEY,
    detail VARCHAR,
    user_id INT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user  FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);