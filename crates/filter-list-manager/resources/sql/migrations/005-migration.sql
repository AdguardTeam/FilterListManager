-- Purpose: Hashing filters

ALTER TABLE [rules_list] ADD COLUMN [text_hash] TEXT;
ALTER TABLE [filter_includes] ADD COLUMN [body_hash] TEXT;
