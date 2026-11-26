CREATE TABLE IF NOT EXISTS conctr_accounts (
    id VARCHAR(512) PRIMARY KEY,
    username VARCHAR(128) NOT NULL UNIQUE,
    role VARCHAR(16) NOT NULL CHECK (role IN ('admin', 'member')),
    password_hash VARCHAR(1024) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_nopad_bin;

CREATE TABLE IF NOT EXISTS conctr_auth_installation (
    singleton BOOLEAN PRIMARY KEY CHECK (singleton = TRUE),
    import_complete BOOLEAN NOT NULL,
    provisioned BOOLEAN NOT NULL
) ENGINE=InnoDB;

INSERT INTO conctr_auth_installation VALUES (true, false, false)
ON DUPLICATE KEY UPDATE singleton = true;
