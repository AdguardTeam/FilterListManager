-- Purpose: Add is_user_title, is_user_description columns

ALTER TABLE [filter] ADD [is_user_title] BOOLEAN NOT NULL DEFAULT 0;

ALTER TABLE [filter] ADD [is_user_description] BOOLEAN NOT NULL DEFAULT 0;
