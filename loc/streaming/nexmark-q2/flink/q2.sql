CREATE TABLE discard_sink (
  auction  BIGINT,
  price  BIGINT
) WITH (
  'connector' = 'blackhole'
);

INSERT INTO discard_sink
SELECT auction, price FROM bid WHERE MOD(auction, 123) = 0;