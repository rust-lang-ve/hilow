INSERT INTO secrets (hash, user_id)
    VALUES($1, $2)
  RETURNING
    *