CREATE TABLE discard_sink (
  name  VARCHAR,
  city  VARCHAR,
  state  VARCHAR,
  id  BIGINT
) WITH (
  'connector' = 'blackhole'
);

INSERT INTO discard_sink
SELECT
    P.name, P.city, P.state, A.id
FROM
    auction AS A INNER JOIN person AS P on A.seller = P.id
WHERE
    A.category = 10 and (P.state = 'OR' OR P.state = 'ID' OR P.state = 'CA');