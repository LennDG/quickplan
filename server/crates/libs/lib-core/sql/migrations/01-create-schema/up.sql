CREATE TABLE plan (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url_id TEXT NOT NULL UNIQUE CHECK (length(url_id) <= 32),
    name TEXT NOT NULL CHECK (length(name) <= 128),
    description TEXT CHECK (length(description) <= 1024),

    -- Metadata
    ctime TEXT NOT NULL
);
CREATE INDEX idx_plan_url_id ON plan(url_id);

CREATE TABLE plan_user (
    -- Relations
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    web_id TEXT NOT NULL, -- ID to use on the web, don't expose autoincrements
    plan_id INTEGER,
    name TEXT NOT NULL CHECK (length(name) <= 128),

    -- Metadata
    ctime TEXT NOT NULL,

    -- Constraints
    UNIQUE(plan_id, name),
    FOREIGN KEY(plan_id) REFERENCES plan(id) ON DELETE CASCADE

    
);
CREATE INDEX idx_user_plan_id ON plan_user(plan_id);

CREATE TABLE user_date (
    -- Relations
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    
    -- Data
    date TEXT NOT NULL CHECK (length(date) <= 128),
    
    -- Metadata
    ctime TEXT NOT NULL,

    -- Constraints
    FOREIGN KEY(user_id) REFERENCES plan_user(id) ON DELETE CASCADE,
    UNIQUE(user_id, date) -- A date can only be picked once per user
);
CREATE INDEX idx_date_user_id ON user_date(user_id);