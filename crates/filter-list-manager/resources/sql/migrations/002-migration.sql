-- Purpose: Add rules_count column

ALTER TABLE [rules_list] ADD [rules_count] INTEGER NOT NULL DEFAULT 0;

UPDATE [rules_list] SET [rules_count] = (
   WITH 
   [rules_list_shifted] AS (SELECT CASE WHEN LENGTH([rules_text]) = 0 THEN [rules_text] ELSE X'0A' || [rules_text] END AS [rules_text_shifted], [filter_id] FROM [rules_list]),
   [rules_list_trimmed] AS (SELECT REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE([rules_text_shifted], X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A'), X'0A0A', X'0A') AS [rules_text_trimmed], [filter_id] FROM [rules_list_shifted]),
   [rules_list_without_comments] AS (SELECT REPLACE ([rules_text_trimmed], X'0A' || '!', '') AS [rules_text_without_comments], [filter_id] FROM [rules_list_trimmed]),
   [rules_list_without_extra_comments] AS (SELECT REPLACE([rules_text_without_comments], X'0A' || '# ', '') AS [rules_text_without_extra_comments], [filter_id] FROM [rules_list_without_comments]),
   [rules_list_without_newlines] AS (SELECT REPLACE([rules_text_without_extra_comments], X'0A', '') AS [rules_text_without_newlines], [rules_text_without_extra_comments], [filter_id] FROM [rules_list_without_extra_comments])
   SELECT LENGTH([rules_text_without_extra_comments]) - LENGTH([rules_text_without_newlines]) FROM [rules_list_without_newlines]
   WHERE [rules_list_without_newlines].[filter_id] = [rules_list].[filter_id]
);
