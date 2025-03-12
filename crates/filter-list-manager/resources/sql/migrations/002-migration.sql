-- Purpose: Add rules_count column

ALTER TABLE [rules_list] ADD [rules_count] INTEGER NOT NULL DEFAULT 0;

UPDATE [rules_list] SET [rules_count] = (
   SELECT
      LENGTH(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
         CASE
            WHEN LENGTH([rules_text]) = 0
            THEN [rules_text]
            ELSE X'0A' || [rules_text]
         END,
         X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'),
         X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'),
         X'0A' || '!', ''),
         X'0A' || '# ', ''))
    - LENGTH(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
         CASE
            WHEN LENGTH([rules_text]) = 0
            THEN [rules_text]
            ELSE X'0A' || [rules_text]
         END,
         X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'),
         X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'),
         X'0A' || '!', ''),
         X'0A' || '# ', ''),
         X'0A', '')) AS [rules_count]
   FROM [rules_list] AS [rules_list_count]
   WHERE [rules_list_count].[filter_id] = [rules_list].[filter_id]
);
