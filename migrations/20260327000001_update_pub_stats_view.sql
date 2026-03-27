-- migrations/20260327000001_update_pub_stats_view.sql
-- Update pub_stats materialized view to ignore 1972 trial year

DROP MATERIALIZED VIEW IF EXISTS pub_stats;

CREATE MATERIALIZED VIEW pub_stats AS
WITH RECURSIVE streaks AS (
    -- Base case: Inclusions in the most recent year (2026)
    -- 1972 is ignored by virtue of being in the past
    SELECT pub_id, year, 1 as streak
    FROM gbg_history
    WHERE year = 2026
    
    UNION ALL
    
    -- Recursive step: Join with the previous year
    -- Explicitly ignore 1972 in the recursion
    SELECT h.pub_id, h.year, s.streak + 1
    FROM gbg_history h
    JOIN streaks s ON h.pub_id = s.pub_id AND h.year = s.year - 1
    WHERE h.year != 1972
),
max_streaks AS (
    SELECT pub_id, MAX(streak) as current_streak
    FROM streaks
    GROUP BY pub_id
),
counts AS (
    SELECT 
        pub_id,
        COUNT(*) FILTER (WHERE year >= 2022 AND year != 1972) as last_5_years,
        COUNT(*) FILTER (WHERE year >= 2017 AND year != 1972) as last_10_years,
        COUNT(*) FILTER (WHERE year != 1972) as total_years,
        MIN(year) FILTER (WHERE year != 1972) as first_year,
        MAX(year) FILTER (WHERE year != 1972) as latest_year
    FROM gbg_history
    GROUP BY pub_id
)
SELECT 
    p.id as pub_id,
    COALESCE(ms.current_streak, 0) as current_streak,
    COALESCE(c.last_5_years, 0) as last_5_years,
    COALESCE(c.last_10_years, 0) as last_10_years,
    COALESCE(c.total_years, 0) as total_years,
    c.first_year,
    c.latest_year
FROM pubs p
LEFT JOIN max_streaks ms ON p.id = ms.pub_id
LEFT JOIN counts c ON p.id = c.pub_id;

CREATE UNIQUE INDEX idx_pub_stats_pub_id ON pub_stats(pub_id);
