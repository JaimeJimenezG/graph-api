-- Your SQL goes here
ALTER TABLE navigations DROP COLUMN icon;
ALTER TABLE navigations ADD icon varchar(255) NOT NULL DEFAULT 'BookmarkOutline';
