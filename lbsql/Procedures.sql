use lbm;

-- !!!!! ATTENTION !!!!!
-- Execute the following procedures inside a transaction.

drop function if exists uuid_v4;
-- An lowercase hex-encoded UUID-v4 without hyphens.
create function uuid_v4() returns char(32)
begin
    -- 1th and 2nd block are made of 6 random bytes
    set @h1 = hex(random_bytes(4));
    set @h2 = hex(random_bytes(2));

    -- 3th block will start with a 4 indicating the version, remaining is random
    set @h3 = substr(hex(random_bytes(2)), 2, 3);

    -- 4th block first nibble can only be 8, 9 a or b, remaining is random
    set @h4 = concat(hex(floor(ascii(random_bytes(1)) / 64) + 8),
                     substr(hex(random_bytes(2)), 2, 3));

    -- 5th block is made of 6 random bytes
    set @h5 = hex(random_bytes(6));

    -- build the complete uuid
    return lower(concat(
        @h1, @h2, @h3, @h4, @h5
        ));
end;


drop procedure if exists push_mpc_message;
create procedure push_mpc_message(
    in arg_session_id char(32),
    in arg_round int unsigned,
    in arg_party_from int unsigned,
    in arg_party_to int unsigned,
    in arg_message blob
)
begin
    if arg_party_from = 0 then
        set @err = 'PartyID=0 is preserved as broadcast, thus cannot be used as senderID';
        signal sqlstate '45000' set message_text = @err;
    end if;
    insert into mpc_session_message
    values (arg_session_id, arg_round, arg_party_from, arg_party_to,
            arg_message,
            substring(sha1(arg_message), 1, 8),
            0);
end;


drop procedure if exists pull_mpc_message;
create procedure pull_mpc_message(
    in arg_session_id char(32),
    in arg_round int unsigned,
    in arg_party_from int unsigned,
    in arg_party_to int unsigned,
    out ret_message blob
)
begin
    select message
    into ret_message
    from mpc_session_message
    where session_id = arg_session_id
      and round = arg_round
      and party_from = arg_party_from
      and party_to = arg_party_to;
end;


drop procedure if exists attend_agree;
create procedure attend_agree(
    in arg_session_id char(32),
    in arg_owner_id char(32),
    out ret_session_type char(16),
    out ret_party_id int unsigned,
    out ret_expire_at bigint
)
begin
    declare var_owner_id, var_opinion char(32);
    declare var_threshold, var_n_actual int unsigned;

    CHECK_SESSION:
    begin
        select session_type, expire_at
        into ret_session_type, ret_expire_at
        from mpc_session
        where session_id = arg_session_id;
        if isnull(ret_session_type) then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ' is not created or already expired'
                );
            signal sqlstate '45000' set message_text = @message_text;
        end if;
    end CHECK_SESSION;

    CHECK_OWNER:
    begin
        select owner_id, opinion
        into var_owner_id, var_opinion
        from mpc_session_attendance
        where session_id = arg_session_id
          and owner_id = arg_owner_id;

        -- check whether the owner is allowed to join in the session.
        if isnull(var_owner_id) then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ' does not have owner_id=', arg_owner_id
                );
            signal sqlstate '45000' set message_text = @message_text;
        end if;

        -- check whether the owner have already attended.
        if var_opinion != 'unknown' then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ', owner_id=', arg_owner_id,
                ' already attended with opinion=', var_opinion
                );
            signal sqlstate '45000' set message_text = @message_text;
        end if;
    end CHECK_OWNER;

    ASSIGN_PARTY_ID:
    begin
        select count(owner_id)
        into ret_party_id
        from mpc_session_attendance
        where session_id = arg_session_id
          and opinion = 'agree';

        if ret_session_type = 'sign' then
            select threshold
            into var_threshold
            from mpc_session_params
            where session_id = arg_session_id;
            set var_n_actual = var_threshold + 1;
            if ret_party_id > var_n_actual then
                set @message_text = concat(
                    'At session_id=', arg_session_id,
                    ' party_id=', ret_party_id,
                    ' > n_actual=', var_n_actual
                    );

                signal sqlstate '45000' set message_text = @message_text;
            end if;
        end if;
    end ASSIGN_PARTY_ID;

    UPDATE_ATTENDANCE:
    begin
        update mpc_session_attendance
        set party_id = ret_party_id,
            opinion  = 'agree'
        where session_id = arg_session_id
          and owner_id = arg_owner_id;
    end UPDATE_ATTENDANCE;
end;


drop procedure if exists attend_disagree;
create procedure attend_disagree(
    in arg_session_id char(32),
    in arg_owner_id char(32)
)
begin
    declare session_running boolean;
    declare var_owner_id, var_opinion char(32);

    CHECK_SESSION:
    begin
        select count(session_id)
        into session_running
        from mpc_session
        where session_id = arg_session_id;
        if !session_running then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ' is not created or already expired'
                );
            signal sqlstate '45000' set message_text = @message_text;
        end if;
    end;

    CHECK_OWNER:
    begin
        select owner_id, opinion
        into var_owner_id, var_opinion
        from mpc_session_attendance
        where session_id = arg_session_id
          and owner_id = arg_owner_id;

        -- check whether the owner is allowed to join in the session.
        if isnull(var_owner_id) then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ' does not have owner_id=', arg_owner_id
                );

            signal sqlstate '45000' set message_text = @message_text;
        end if;

        -- check whether the owner have already attended.
        if var_opinion != 'unknown' then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ', owner_id=', arg_owner_id,
                ' already signed up with opinion=', var_opinion
                );

            signal sqlstate '45000' set message_text = @message_text;
        end if;
    end CHECK_OWNER;

    UPDATE_ATTENDANCE:
    begin
        update mpc_session_attendance
        set opinion = 'disagree'
        where session_id = arg_session_id
          and owner_id = arg_owner_id;
    end UPDATE_ATTENDANCE;

end;


drop procedure if exists new_keygen_session;
create procedure new_keygen_session(
    in arg_threshold int unsigned,
    in table_owners char(32),
    out ret_session_id char(32),
    out ret_expire_at bigint
)
begin
    set @var_n_keygen := null;
    set @sql = concat(
        'select count(owner_id) into @var_n_keygen from ', table_owners
        );
    prepare stmt from @sql; execute stmt; deallocate prepare stmt;

    select uuid_v4() into ret_session_id;
    select unix_timestamp() + 1200 into ret_expire_at;
    insert into mpc_session values (ret_session_id, ret_expire_at, 'keygen');
    insert into mpc_session_params values (ret_session_id, arg_threshold, @var_n_keygen, null, null, null);

    -- initialize attendance status of all owners
    set @sql = concat(
        'insert into mpc_session_attendance '
            'select \'', ret_session_id, '\', null, owner_id, \'unknown\''
            'from ', table_owners
        );
    prepare stmt from @sql; execute stmt; deallocate prepare stmt;
    insert into mpc_session_attendance
    values (ret_session_id, 0, '__broadcast__', 'agree');
end;


drop procedure if exists get_params_from_session;
create procedure get_params_from_session(
    in arg_session_id char(32),
    out ret_session_type enum ('keygen', 'sign'),
    out ret_threshold int unsigned,
    out ret_n_keygen int unsigned,
    out ret_key_id char(32),
    out ret_derv_path varchar(128),
    out ret_tx_hash binary(64)
)
begin
    select ms.session_type, msp.threshold, msp.n_keygen, msp.key_id, msp.derv_path, msp.tx_hash
    into ret_session_type, ret_threshold, ret_n_keygen, ret_key_id, ret_derv_path, ret_tx_hash
    from mpc_session as ms
             join mpc_session_params as msp
                  on ms.session_id = msp.session_id
    where ms.session_id = arg_session_id;
    if isnull(ret_session_type) then
        set @message_text = concat('session_id=', arg_session_id, ' does not exist');

        signal sqlstate '45000' set message_text = @message_text;
    end if;
    if ret_session_type = 'keygen' then
        if isnull(ret_threshold) or isnull(ret_n_keygen) then
            set @message_text = concat('session_id=', arg_session_id, ' has inconsistent inner state');
            signal sqlstate '45000' set message_text = @message_text;
        end if;
    elseif ret_session_type = 'sign' then
        if isnull(ret_threshold) or isnull(ret_n_keygen) or isnull(ret_key_id)
            or isnull(ret_derv_path) or isnull(ret_tx_hash)
        then
            set @message_text = concat('session_id=', arg_session_id, ' has inconsistent inner state');
            signal sqlstate '45000' set message_text = @message_text;
        end if;
    end if;
end;


drop procedure if exists terminate_keygen;
create procedure terminate_keygen(
    in arg_session_id char(32),
    in arg_party_id int unsigned,
    in arg_root_xpub varchar(128)
)
begin
    declare round int unsigned;
    declare var_threshold int unsigned;
    declare n1, n2 int unsigned;
    declare var_root_xpub_exists boolean;
    declare all_root_xpub_equal boolean;

    select count(key_id) > 0
    into var_root_xpub_exists
    from mpc_key
    where root_xpub = arg_root_xpub;

    if var_root_xpub_exists then
        signal sqlstate '45000' set message_text = 'root_xpub already exists!';
    end if;

    -- validate the session
    select threshold
    into var_threshold
    from mpc_session_params
    where session_id = arg_session_id;
    if isnull(var_threshold) then
        set @message_text = concat(
            'session_id=', arg_session_id,
            ' is not created or already expired'
            );
        signal sqlstate '45000' set message_text = @message_text;
    end if;

    -- calculate round number for termination
    select ifnull(max(round), 0) + 1
    into round
    from mpc_session_message
    where session_id = arg_session_id
      and party_from = arg_party_id
      and is_finished = false;

    -- insert
    set @hashed_msg = substring(sha1(arg_root_xpub), 1, 8);
    insert into mpc_session_message
    values (arg_session_id, round, arg_party_id, arg_party_id, @hashed_msg, @hashed_msg, true);

    -- check if all parties have terminated
    -- To say "terminated" for keygen, is to say all parties have submitted root_xpub,
    -- and all root_xpub-s are the same.
    select count(party_id)
    into n1
    from mpc_session_attendance
    where session_id = arg_session_id
      and opinion = 'agree'
      and party_id != 0;

    select count(party_from)
    into n2
    from mpc_session_message
    where session_id = arg_session_id
      and is_finished = true;

    if n1 = n2 then
        select count(distinct hash) = 1
        into all_root_xpub_equal
        from mpc_session_message
        where session_id = arg_session_id
          and is_finished = true;
        if !all_root_xpub_equal then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ' is not properly terminated, because root_xpub-s are not equal'
                );
            signal sqlstate '45000' set message_text = @message_text;
        end if;
        insert into mpc_key values (arg_session_id, arg_root_xpub, var_threshold);
        insert into mpc_key_shard
        select arg_session_id, owner_id
        from mpc_session_attendance
        where session_id = arg_session_id
          and opinion = 'agree'
          and owner_id != '__broadcast__';
    end if;
end;


drop procedure if exists new_sign_session;
create procedure new_sign_session(
    in arg_key_id char(32),
    in arg_derv_path char(64),
    in arg_tx_hash binary(64),
    out ret_session_id char(32),
    out ret_expire_at bigint
)
begin
    declare var_threshold, var_n_keygen int unsigned;

    -- validate tx_hash non-existence
    set @var_tx_hash = 0;
    select count(tx_hash) into @var_tx_hash from mpc_key_signature where tx_hash = arg_tx_hash;
    if @var_tx_hash > 0 then
        set @message_text = 'It is forbidden to sign a duplicate Tx.';

        signal sqlstate '45000' set message_text = @message_text;
    end if;

    -- fetch keygen parameters
    select th into var_threshold from mpc_key where key_id = arg_key_id;
    select count(owner_id) into var_n_keygen from mpc_key_shard where key_id = arg_key_id;
    if isnull(var_threshold) then
        set @message_text = concat('key_id=', arg_key_id, ' does not exist');
        signal sqlstate '45000' set message_text = @message_text;
    end if;

    -- create new session
    select uuid_v4() into ret_session_id;
    select unix_timestamp() + 1200 into ret_expire_at;
    insert into mpc_session values (ret_session_id, ret_expire_at, 'sign');
    insert into mpc_session_params
    values (ret_session_id, var_threshold, var_n_keygen, arg_key_id, arg_derv_path, arg_tx_hash);

    -- initialize attendance status of all owners
    insert into mpc_session_attendance
    select ret_session_id, null, owner_id, 'unknown'
    from mpc_key_shard
    where key_id = arg_key_id;
    insert into mpc_session_attendance
    values (ret_session_id, 0, '__broadcast__', 'agree');
end;


drop procedure if exists terminate_sign;
create procedure terminate_sign(
    in arg_session_id char(32),
    in arg_party_id int unsigned,
    in arg_r binary(64),
    in arg_s binary(64),
    in arg_v tinyint unsigned
)
begin
    declare all_signature_equal boolean;
    declare round, var_threshold, n1, n2 int unsigned;
    declare var_tx_hash binary(64);
    declare var_key_id char(32);
    declare var_derv_path, var_sig_hash varchar(255);
    declare var_owners JSON;

    -- validate the session
    select threshold, tx_hash, key_id, derv_path
    into var_threshold, var_tx_hash, var_key_id, var_derv_path
    from mpc_session_params
    where session_id = arg_session_id;
    if isnull(var_key_id) then
        set @message_text = concat(
            'session_id=', arg_session_id,
            ' is not created or already expired'
            );
        signal sqlstate '45000' set message_text = @message_text;
    end if;

    -- calculate round number for termination
    select ifnull(max(round), 0) + 1
    into round
    from mpc_session_message
    where session_id = arg_session_id
      and party_from = arg_party_id
      and is_finished = false;

    -- insert
    set var_sig_hash = concat(
        sha1(arg_r), ',',
        sha1(arg_s), ',',
        arg_v
        );
    set var_sig_hash = substring(sha1(var_sig_hash), 1, 8);
    insert into mpc_session_message
    values (arg_session_id, round, arg_party_id, arg_party_id,
            var_sig_hash,
            var_sig_hash,
            true);

    -- check if all parties have terminated
    -- To say "terminated" for keygen, is to say all parties have submitted root_xpub,
    -- and all root_xpub-s are the same.
    select count(party_id)
    into n1
    from mpc_session_attendance
    where session_id = arg_session_id
      and opinion = 'agree'
      and party_id != 0;

    select count(party_from)
    into n2
    from mpc_session_message
    where session_id = arg_session_id
      and is_finished = true;

    -- use the last party's termination message
    -- to terminate the session.
    if n1 = n2 then
        select count(distinct hash) = 1
        into all_signature_equal
        from mpc_session_message
        where session_id = arg_session_id
          and is_finished = true;
        if !all_signature_equal then
            set @message_text = concat(
                'session_id=', arg_session_id,
                ' is not properly terminated, because signatures are not equal'
                );

            signal sqlstate '45000' set message_text = @message_text;
        end if;
        select json_arrayagg(owner_id)
        into var_owners
        from mpc_session_attendance
        where session_id = arg_session_id
          and opinion = 'agree'
          and owner_id != '__broadcast__';

        insert into mpc_key_signature
        values (var_tx_hash,
                var_key_id,
                utc_timestamp(3),
                var_derv_path,
                arg_r,
                arg_s,
                arg_v,
                var_owners);
    end if;
end;

/*
create temporary table temp_table (
    owner_id char(32) not null,
    party_id int unsigned,
    opinion char(16) not null,
    round int unsigned not null
) engine = memory;
*/
drop procedure if exists get_keygen_progress;
create procedure get_keygen_progress(
    in arg_session_id char(32),
    in temp_table varchar(64), -- (owner_id, party_id, opinion, round)
    out ret_root_xpub varchar(128)
)
begin
    declare var_session_id char(32);
    declare var_tx_hash binary(64);

    select session_id, tx_hash
    into var_session_id, var_tx_hash
    from mpc_session_params
    where session_id = arg_session_id;
    if isnull(var_session_id) then
        set @message_text = concat(
            'session_id=', arg_session_id,
            ' is not created or already expired'
            );
        signal sqlstate '45000' set message_text = @message_text;
    end if;
    if !isnull(var_tx_hash) then
        set @message_text = concat(
            'session_id=', arg_session_id,
            ' is not a keygen session'
            );
        signal sqlstate '45000' set message_text = @message_text;
    end if;

    -- Write progress of each owner to the temporary table
    set @sql = concat(
        'insert into ', temp_table,
        ' select owner_id, party_id, opinion, 0',
        ' from mpc_session_attendance as mss',
        ' where mss.session_id = \'', arg_session_id, '\'',
        ' and mss.owner_id != \'__broadcast__\''
        );
    prepare stmt from @sql; execute stmt; deallocate prepare stmt;
    set @sql = concat(
        'update ', temp_table, ' as tt join',
        ' (select party_from as party_id, max(round) as round from mpc_session_message',
        ' where session_id = \'', var_session_id, '\'',
        ' group by party_from) msm',
        ' on tt.party_id = msm.party_id',
        ' set tt.round = msm.round'
        );
    prepare stmt from @sql; execute stmt; deallocate prepare stmt;

    -- Fetch overall progress
    select root_xpub
    into ret_root_xpub
    from mpc_key
    where key_id = arg_session_id;

end;

/*
create temporary table temp_table1 (
    owner_id char(32) not null,
    party_id int unsigned,
    opinion char(16) not null,
    round int unsigned not null
) engine = memory;
create temporary table temp_table2 (
    r binary(N) not null,
    s binary(N) not null,
    v tinyint(1) not null
) engine = memory;
*/
drop procedure if exists get_sign_progress;
create procedure get_sign_progress(
    in arg_session_id char(32),
    in temp_table1 varchar(64),
    in temp_table2 varchar(64)
)
begin
    declare var_session_id char(32);
    declare var_tx_hash binary(64);

    select session_id, tx_hash
    into var_session_id, var_tx_hash
    from mpc_session_params
    where session_id = arg_session_id;
    if isnull(var_session_id) then
        set @message_text = concat(
            'session_id=', arg_session_id,
            ' is not created or already expired'
            );
        signal sqlstate '45000' set message_text = @message_text;
    end if;
    if isnull(var_tx_hash) then
        set @message_text = concat(
            'session_id=', arg_session_id,
            ' is not a sign session'
            );
        signal sqlstate '45000' set message_text = @message_text;
    end if;

    -- Write progress of each owner to the temporary table
    set @sql = concat(
        'insert into ', temp_table1,
        ' select owner_id, party_id, opinion, 0',
        ' from mpc_session_attendance as mss ',
        ' where mss.session_id = \'', arg_session_id, '\'',
        ' and mss.owner_id != \'__broadcast__\''
        );
    prepare stmt from @sql; execute stmt; deallocate prepare stmt;
    set @sql = concat(
        'update ', temp_table1, ' as tt join',
        ' (select party_from as party_id, max(round) as round from mpc_session_message',
        ' where session_id = \'', var_session_id, '\'',
        ' group by party_from) msm',
        ' on tt.party_id = msm.party_id',
        ' set tt.round = msm.round'
        );
    prepare stmt from @sql; execute stmt; deallocate prepare stmt;

    -- Fetch overall progress
    set @sql = concat(
        'insert into ', temp_table2,
        ' select tx_hash, r, s, v from mpc_key_signature'
            ' where tx_hash = \'', var_tx_hash, '\''
        );
    prepare stmt from @sql; execute stmt; deallocate prepare stmt;
end;