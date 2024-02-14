-- Seed test plans
INSERT INTO plan (url_id, name, ctime) VALUES
    ('plan1_url', 'Plan 1', CURRENT_TIMESTAMP),
    ('plan2_url', 'Plan 2', CURRENT_TIMESTAMP);

-- Seed test users
-- For Plan 1
INSERT INTO plan_user (plan_id, name, ctime) VALUES
    (1000, 'User1_Plan1', CURRENT_TIMESTAMP),
    (1000, 'User2_Plan1', CURRENT_TIMESTAMP),
    (1000, 'User3_Plan1', CURRENT_TIMESTAMP);

-- For Plan 2
INSERT INTO plan_user (plan_id, name, ctime) VALUES
    (1001, 'User1_Plan2', CURRENT_TIMESTAMP),
    (1001, 'User2_Plan2', CURRENT_TIMESTAMP);

-- Seed test dates
-- For Plan 1, User1_Plan1
INSERT INTO user_date (user_id, date, ctime) VALUES
    (1000,  '2024-02-13', CURRENT_TIMESTAMP),
    (1000,  '2024-02-14', CURRENT_TIMESTAMP);

-- For Plan 1, User2_Plan1
INSERT INTO user_date (user_id, date, ctime) VALUES
    (1001,  '2024-02-15', CURRENT_TIMESTAMP),
    (1001,  '2024-02-16', CURRENT_TIMESTAMP);

-- For Plan 1, User3_Plan1
INSERT INTO user_date (user_id, date, ctime) VALUES
    (1002,  '2024-02-17', CURRENT_TIMESTAMP);

-- For Plan 2, User1_Plan2
INSERT INTO user_date (user_id, date, ctime) VALUES
    (1003,  '2024-02-18', CURRENT_TIMESTAMP),
    (1003,  '2024-02-19', CURRENT_TIMESTAMP);

-- For Plan 2, User2_Plan2
INSERT INTO user_date (user_id, date, ctime) VALUES
    (1004,  '2024-03-22', CURRENT_TIMESTAMP),
    (1004,  '2024-01-20', CURRENT_TIMESTAMP);