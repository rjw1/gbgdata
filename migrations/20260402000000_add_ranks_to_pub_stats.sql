-- migrations/20260402000000_add_ranks_to_pub_stats.sql
-- Add global ranks to pub_stats materialized view

DROP MATERIALIZED VIEW IF EXISTS pub_stats;

CREATE MATERIALIZED VIEW pub_stats AS
WITH RECURSIVE streaks AS (
    SELECT pub_id, year, 1 as streak
    FROM gbg_history
    WHERE year = 2026
    
    UNION ALL
    
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
),
base_stats AS (
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
    LEFT JOIN counts c ON p.id = c.pub_id
)
SELECT 
    *,
    RANK() OVER (ORDER BY total_years DESC) as entries_rank,
    RANK() OVER (ORDER BY current_streak DESC) as streak_rank
FROM base_stats;

CREATE UNIQUE INDEX idx_pub_stats_pub_id ON pub_stats(pub_id);
