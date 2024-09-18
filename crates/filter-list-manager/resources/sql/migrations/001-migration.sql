-- Purpose: Remove download_url constraint

CREATE TABLE [filter_tmp] (
   [filter_id] INTEGER PRIMARY KEY,
   [group_id] INTEGER NOT NULL,
   [version] TEXT,
   [last_update_time] INTEGER NOT NULL,
   [last_download_time] INTEGER NOT NULL,
   [display_number] INTEGER NOT NULL DEFAULT 0,
   [title] TEXT,
   [description] TEXT,
   [homepage] TEXT,
   [license] TEXT,
   [checksum] TEXT,
   [expires] INTEGER,
   [download_url] TEXT,
   [subscription_url] TEXT,
   [is_enabled] BOOLEAN NOT NULL DEFAULT 0,
   [is_installed] BOOLEAN NOT NULL DEFAULT 0,
   [is_trusted] BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO [filter_tmp] (
   [filter_id], [group_id], [version], [last_update_time], [last_download_time],
   [display_number], [title], [description], [homepage], [license],
   [checksum], [expires], [download_url], [subscription_url],
   [is_enabled], [is_installed], [is_trusted]
)
SELECT
   [filter_id], [group_id], [version], [last_update_time], [last_download_time],
   [display_number], [title], [description], [homepage], [license],
   [checksum], [expires], [download_url], [subscription_url],
   [is_enabled], [is_installed], [is_trusted]
FROM [filter];

DROP TABLE [filter];

ALTER TABLE [filter_tmp] RENAME TO [filter];
