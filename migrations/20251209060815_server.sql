CREATE TABLE servers (
    id TEXT PRIMARY KEY NOT NULL,
    hostname TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    os_type TEXT NOT NULL,
    tags TEXT,
    auth_profile_id TEXT NOT NULL,
    port INTEGER NOT NULL,
    bastion_server_id TEXT,
    wol_mac_address TEXT
);