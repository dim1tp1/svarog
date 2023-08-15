use lbm;

drop event if exists mpc_clean;
create event mpc_clean on schedule
    every '5' second enable
    do
    BEGIN
        -- Due to on-delete-cascade foreign keys, attendance and messages are also cleaned.
        -- In most cases, "autocommit" is enabled.
        -- Hence, the following statement won't corrupt the atomicity of the transactions in Procedures.sql
        DELETE FROM mpc_session WHERE expire_at < unix_timestamp();
    END;
