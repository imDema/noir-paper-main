CREATE TABLE discard_sink (
  seller VARCHAR,
  avg_price  BIGINT
) WITH (
  'connector' = 'blackhole'
);

INSERT INTO discard_sink
SELECT
    Q.seller,
    AVG(Q.final) OVER
        (PARTITION BY Q.seller ORDER BY Q.dateTime ROWS BETWEEN 10 PRECEDING AND CURRENT ROW)
FROM (
    SELECT MAX(B.price) AS final, A.seller, B.dateTime
    FROM auction AS A, bid AS B
    WHERE A.id = B.auction and B.dateTime between A.dateTime and A.expires
    GROUP BY A.id, A.seller
) AS Q;