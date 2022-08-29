DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

CREATE USER app_user PASSWORD 'password_need_to_change';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';
