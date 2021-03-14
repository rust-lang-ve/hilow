INSERT INTO users (email, name)
        VALUES($1, $2)
    RETURNING
        *