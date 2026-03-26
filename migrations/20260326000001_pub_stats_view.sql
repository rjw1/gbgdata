-- Materialized view for pub statistics
CREATE MATERIALIZED VIEW pub_stats AS
WITH RECURSIVE streaks AS (
    -- Base case: Inclusions in the most recent year (2026)
    SELECT pub_id, year, 1 as streak
    FROM gbg_history
    WHERE year = 2026
    
    UNION ALL
    
    -- Recursive step: Join with the previous year
    SELECT h.pub_id, h.year, s.streak + 1
    FROM gbg_history h
    JOIN streaks s ON h.pub_id = s.pub_id AND h.year = s.year - 1
),
max_streaks AS (
    SELECT pub_id, MAX(streak) as current_streak
    FROM streaks
    GROUP BY pub_id
),
counts AS (
    SELECT 
        pub_id,
        COUNT(*) FILTER (WHERE year >= 2022) as last_5_years,
        COUNT(*) FILTER (WHERE year >= 2017) as last_10_years,
        COUNT(*) as total_years,
        MIN(year) as first_year,
        MAX(year) as latest_year
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
