CREATE TABLE discard_sink (
  auction  BIGINT,
  bidder  BIGINT,
  price  BIGINT,
  dateTime  TIMESTAMP(3),
  extra  VARCHAR
) WITH (
  'connector' = 'blackhole'
);

INSERT INTO discard_sink
SELECT auction, bidder, price, dateTime, extra FROM bid;