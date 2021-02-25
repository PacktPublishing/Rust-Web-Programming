-- Your SQL goes here
ALTER  TABLE to_do ADD CONSTRAINT uc_item UNIQUE (title, user_id);
