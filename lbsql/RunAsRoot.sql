create database if not exists lbm;

show grants for `******`@`localhost`;

GRANT SELECT, INSERT, UPDATE, DELETE, CREATE,
      DROP, RELOAD, REFERENCES, INDEX, ALTER,
      CREATE TEMPORARY TABLES, CREATE VIEW, CREATE ROUTINE, ALTER ROUTINE, EVENT,
      EXECUTE
      ON *.* TO `******`@`localhost`;

FLUSH PRIVILEGES;
