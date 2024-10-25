-- DB metadata tables
CREATE TABLE IF NOT EXISTS [metadata] (
    [rowid] INTEGER PRIMARY KEY,
	[schema_version] INTEGER NOT NULL,
    [custom_filter_increment] INTEGER NOT NULL
);

-- Main filters tables

CREATE TABLE IF NOT EXISTS [filter] (
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
   [is_trusted] BOOLEAN NOT NULL DEFAULT 0,
   CONSTRAINT [download_url] UNIQUE (download_url)
);

CREATE TABLE IF NOT EXISTS [filter_group] (
    [group_id] INTEGER NOT NULL PRIMARY KEY,
    [name] TEXT,
    [display_number] INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS [filter_tag] (
    tag_id INTEGER NOT NULL PRIMARY KEY,
    keyword TEXT
);

CREATE TABLE IF NOT EXISTS [rules_list] (
    [filter_id] INTEGER NOT NULL,
    [rules_text] TEXT,
    [disabled_rules_text] TEXT,
    CONSTRAINT [filter_id] UNIQUE (filter_id)
);

CREATE TABLE IF NOT EXISTS [diff_updates] (
    [filter_id] INTEGER NOT NULL,
    [next_path] TEXT, -- which path should be resolved next
    [next_check_time] INTEGER NOT NULL,
    CONSTRAINT [filter_id] UNIQUE (filter_id)
);

CREATE TABLE IF NOT EXISTS [filter_filter_tag] (
   [tag_id] INTEGER NOT NULL,
   [filter_id] INTEGER NOT NULL,
   CONSTRAINT [filter_filter_tag_pkey] PRIMARY KEY ([tag_id], [filter_id])
);

CREATE TABLE IF NOT EXISTS [filter_locale] (
     [filter_id] INTEGER NOT NULL,
     [lang] TEXT,
     CONSTRAINT [filter_locale_pkey] PRIMARY KEY ([filter_id], [lang])
);

-- Localisation tables

CREATE TABLE IF NOT EXISTS [filter_localisation] (
	[filter_id] INTEGER NOT NULL,
	[lang] TEXT,
    [name] TEXT,
	[description] TEXT,
    CONSTRAINT [filter_localisation_pkey] PRIMARY KEY ([filter_id], [lang])
);

CREATE TABLE IF NOT EXISTS [filter_group_localisation] (
    [group_id] INTEGER NOT NULL,
    [lang] TEXT NOT NULL,
    [name] TEXT,
    CONSTRAINT [pkey] PRIMARY KEY ([group_id], [lang])
);

CREATE TABLE IF NOT EXISTS [filter_tag_localisation] (
    [tag_id] INTEGER NOT NULL,
    [lang] TEXT NOT NULL,
    [name] TEXT,
    [description] TEXT,
    CONSTRAINT [pkey] PRIMARY KEY ([tag_id], [lang])
);
