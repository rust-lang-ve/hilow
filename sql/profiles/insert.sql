INSERT INTO profiles (name, surname, user_id)
    VALUES($1, $2, $3)
  RETURNING
    *