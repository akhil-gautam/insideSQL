CREATE TABLE orders (
    id integer PRIMARY KEY,
    customer_id integer NOT NULL CHECK (customer_id BETWEEN 1 AND 998),
    status text NOT NULL CHECK (status IN ('paid','pending','cancelled','refunded')),
    total numeric(20,2) CHECK (total >= 0),
    ordered_at timestamptz NOT NULL,
    payload text NOT NULL CHECK (length(payload) = 192)
);
INSERT INTO orders
SELECT g,
    CASE WHEN g IN (97,98,99) THEN 777 WHEN g%100 < 65-{{seed}}*20 THEN 1 ELSE 2+(g*17+{{seed}}*137)%997 END,
    CASE (g+{{seed}})%5 WHEN 0 THEN 'paid' WHEN 1 THEN 'pending' WHEN 2 THEN 'pending' WHEN 3 THEN 'cancelled' ELSE 'refunded' END,
    CASE WHEN g IN (97,98,99) OR (g+{{seed}})%17=0 THEN NULL ELSE ((g*37+{{seed}}*19)%50000)::numeric/100 END,
    CASE WHEN g IN (97,98,99) THEN '2026-01-11 00:00:00+00'::timestamptz+(g-97)*interval '1 second'
         ELSE CASE g WHEN 1 THEN '2026-01-03 08:00:00+00'::timestamptz
           WHEN 2 THEN '2026-01-03 08:30:00+00'::timestamptz
           WHEN 3 THEN '2026-01-03 07:59:59+00'::timestamptz
           ELSE '2026-01-01 00:00:00+00'::timestamptz+((g*37+{{seed}}*241)%864000)*interval '1 second' END END,
    repeat(md5(g::text||'-'||{{seed}}::text),6)
FROM generate_series(1,100000) g;
CREATE INDEX orders_customer_idx ON orders(customer_id);
CREATE INDEX orders_time_idx ON orders(ordered_at);
CREATE INDEX orders_status_idx ON orders(status);
ANALYZE orders;
