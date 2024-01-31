-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "users" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS "posts" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "title" VARCHAR(255) NOT NULL,
    "content" TEXT NOT NULL,
    "user_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

create index posts_user_id_idx on posts(user_id);
create index posts_created_at_idx on posts(created_at);

CREATE TABLE IF NOT EXISTS "comments" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "content" TEXT NOT NULL,
    "user_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
    "post_id" UUID NOT NULL REFERENCES "posts" ("id") ON DELETE CASCADE,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

create index comments_user_id_idx on comments(user_id);
create index comments_post_id_idx on comments(post_id);

CREATE TABLE IF NOT EXISTS "likes" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "user_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
    "post_id" UUID NOT NULL REFERENCES "posts" ("id") ON DELETE CASCADE,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

create index likes_user_id_idx on likes(user_id);
create index likes_post_id_idx on likes(post_id);

CREATE TABLE IF NOT EXISTS "follows" (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "follower_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
    "followee_id" UUID NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

create index follows_follower_id_idx on follows(follower_id);
create index follows_followee_id_idx on follows(followee_id);