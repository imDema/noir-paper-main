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
SELECT B.auction, B.price, B.bidder, B.dateTime, B.extra
from bid B
JOIN (
  SELECT MAX(B1.price) AS maxprice, TUMBLE_ROWTIME(B1.dateTime, INTERVAL '10' SECOND) as dateTime
  FROM bid B1
  GROUP BY TUMBLE(B1.dateTime, INTERVAL '10' SECOND)
) B1
ON B.price = B1.maxprice
WHERE B.dateTime BETWEEN B1.dateTime  - INTERVAL '10' SECOND AND B1.dateTime;