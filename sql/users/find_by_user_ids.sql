SELECT
  *
FROM
  users
WHERE
  users.email = $1
  OR users.name = $2