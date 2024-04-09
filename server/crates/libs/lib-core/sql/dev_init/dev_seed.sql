-- Seed test plans
INSERT INTO plan (url_id, name, ctime) VALUES
    ('plan1_url', 'Plan 1', '2024-04-09T12:09:23Z'),
    ('plan2_url', 'Plan 2', '2024-04-09T12:09:23Z');

UPDATE plan SET description = 'PLAN 1 DESCRIPTION' where id = 1;

-- Seed test users
-- For Plan 1
INSERT INTO plan_user (plan_id, name, ctime) VALUES
    (1, 'User1_Plan1', '2024-04-09T12:09:23Z'),
    (1, 'User2_Plan1', '2024-04-09T12:09:23Z'),
    (1, 'User3_Plan1', '2024-04-09T12:09:23Z');

-- For Plan 2
INSERT INTO plan_user (plan_id, name, ctime) VALUES
    (2, 'User1_Plan2', '2024-04-09T12:09:23Z'),
    (2, 'User2_Plan2', '2024-04-09T12:09:23Z');

-- Seed test dates
-- For Plan 1, User1_Plan1
INSERT INTO user_date (user_id, date, ctime) VALUES
    (1,  '2024-02-13', '2024-04-09T12:09:23Z'),
    (1,  '2024-02-14', '2024-04-09T12:09:23Z');

-- For Plan 1, User2_Plan1
INSERT INTO user_date (user_id, date, ctime) VALUES
    (2,  '2024-02-15', '2024-04-09T12:09:23Z'),
    (2,  '2024-02-16', '2024-04-09T12:09:23Z');

-- For Plan 1, User3_Plan1
INSERT INTO user_date (user_id, date, ctime) VALUES
    (3,  '2024-02-17', '2024-04-09T12:09:23Z');

-- For Plan 2, User1_Plan2
INSERT INTO user_date (user_id, date, ctime) VALUES
    (4,  '2024-02-18', '2024-04-09T12:09:23Z'),
    (4,  '2024-02-19', '2024-04-09T12:09:23Z');

-- For Plan 2, User2_Plan2
INSERT INTO user_date (user_id, date, ctime) VALUES
    (5,  '2024-03-22', '2024-04-09T12:09:23Z'),
    (5,  '2024-01-20', '2024-04-09T12:09:23Z');