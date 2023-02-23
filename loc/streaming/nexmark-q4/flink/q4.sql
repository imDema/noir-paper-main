DROP VIEW IF EXISTS auction;
DROP VIEW IF EXISTS bid;
CREATE VIEW auction AS SELECT auction.* FROM ${NEXMARK_TABLE} WHERE event_type = 1;
CREATE VIEW bid AS SELECT bid.* FROM ${NEXMARK_TABLE} WHERE event_type = 2;

CREATE TABLE discard_sink (
  id BIGINT,
  final BIGINT
) WITH (
  'connector' = 'blackhole'
);

INSERT INTO discard_sink
SELECT
    Q.category,
    AVG(Q.final)
FROM (
    SELECT MAX(B.price) AS final, A.category
    FROM auction A, bid B
    WHERE A.id = B.auction AND B.dateTime BETWEEN A.dateTime AND A.expires
    GROUP BY A.id, A.category
) Q
GROUP BY Q.category;