-- Purpose: Integrity signatures for filter rules protection

ALTER TABLE [rules_list] ADD COLUMN [integrity_signature] TEXT;
ALTER TABLE [filter_includes] ADD COLUMN [integrity_signature] TEXT;
