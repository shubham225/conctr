CREATE TABLE conctr_accounts (
    id text PRIMARY KEY,
    username text NOT NULL UNIQUE,
    role text NOT NULL CHECK (role IN ('admin', 'member')),
    password_hash text NOT NULL
);

CREATE TABLE conctr_auth_installation (
    singleton boolean PRIMARY KEY CHECK (singleton),
    import_complete boolean NOT NULL,
    provisioned boolean NOT NULL
);

INSERT INTO conctr_auth_installation VALUES (true, false, false);
