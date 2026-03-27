-- migrations/20260327000000_rename_county_to_region.sql
-- Rename county to region and add country_code column

ALTER TABLE pubs RENAME COLUMN county TO region;
ALTER TABLE pubs ADD COLUMN country_code VARCHAR(10);
