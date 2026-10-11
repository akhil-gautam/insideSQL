CREATE TABLE orders (
    id integer PRIMARY KEY,
    customer_id integer NOT NULL,
    status text NOT NULL,
    total numeric(20,2),
    ordered_at timestamptz NOT NULL
);
INSERT INTO orders VALUES
    (1,101,'paid',125.50,'2026-01-01'),
    (2,102,'pending',49.00,'2026-01-02'),
    (3,101,'paid',80.00,'2026-01-03'),
    (4,103,'pending',120.00,'2026-01-04'),
    (5,104,'paid',240.00,'2026-01-05'),
    (6,102,'cancelled',60.00,'2026-01-06'),
    (7,105,'paid',100.00,'2026-01-07'),
    (8,106,'refunded',200.00,'2026-01-08'),
    (9,107,'pending',NULL,'2026-01-09'),
    (10,108,'cancelled',240.00,'2026-01-10');
INSERT INTO orders
SELECT 1000+g,200+(g%7),
    CASE WHEN g%3=0 THEN 'paid' WHEN g%3=1 THEN 'pending' ELSE 'cancelled' END,
    CASE WHEN g%11=0 THEN NULL ELSE (g%5)*50.00 END,
    '2026-02-01'::timestamptz+g*interval '1 hour'
FROM generate_series(1,{{seed}}*24) g;
ANALYZE orders;
