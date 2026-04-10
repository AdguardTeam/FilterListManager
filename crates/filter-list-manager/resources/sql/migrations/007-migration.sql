-- Purpose: Integrity signatures for filter metadata and filter count protection

ALTER TABLE [filter] ADD COLUMN [integrity_signature] TEXT;
ALTER TABLE [metadata] ADD COLUMN [filter_count_signature] TEXT;

