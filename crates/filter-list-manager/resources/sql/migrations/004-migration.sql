-- Purpose: Must store original filters as is, so we need to move includes to a separate table

-- has_directives = 1 means that we will process filters in a regular way. No fast path
ALTER TABLE [rules_list] ADD COLUMN [has_directives] BOOLEAN NOT NULL DEFAULT 1;

-- Squashed includes for filters
CREATE TABLE [filter_includes] (
    [row_id] INTEGER PRIMARY KEY,
    [filter_id] INTEGER NOT NULL,
    [absolute_url] TEXT NOT NULL,
    [body] TEXT NOT NULL,
    [rules_count] INTEGER NOT NULL
);

CREATE INDEX [filter_includes_filter_id] ON [filter_includes] ([filter_id]);
