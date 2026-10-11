CREATE TABLE customers (id integer PRIMARY KEY, name text NOT NULL);
CREATE TABLE orders (
    id integer PRIMARY KEY,
    customer_id integer NOT NULL REFERENCES customers(id),
    status text NOT NULL CHECK (status IN ('paid','pending','cancelled','refunded')),
    total numeric(20,2),
    ordered_at timestamptz NOT NULL
);
CREATE TABLE order_items (
    id integer PRIMARY KEY,
    order_id integer NOT NULL REFERENCES orders(id),
    sku text NOT NULL,
    quantity integer NOT NULL CHECK (quantity > 0),
    unit_price numeric(20,2) NOT NULL CHECK (unit_price >= 0)
);
CREATE TABLE payments (
    id integer PRIMARY KEY,
    order_id integer REFERENCES orders(id),
    status text NOT NULL CHECK (status IN ('captured','failed','refunded')),
    amount numeric(20,2) NOT NULL CHECK (amount >= 0),
    occurred_at timestamptz NOT NULL
);
INSERT INTO customers VALUES
    (101,'Mira'),(102,'Omar'),(103,'Mira'),(104,'Sana'),
    (105,'Ari'),(106,'Ivo'),(107,'Zoe'),(108,'Niko'),
    (109,'Noor'),(110,'Dana'),(111,'Mila'),(112,'Arjun');
INSERT INTO customers SELECT 200+g,'Customer '||(g%10) FROM generate_series(1,40) g;
INSERT INTO orders VALUES
    (1,101,'paid',125.50,'2026-01-01'),(2,102,'pending',49.00,'2026-01-02'),
    (3,101,'paid',80.00,'2026-01-03'),(4,103,'pending',120.00,'2026-01-04'),
    (5,104,'paid',240.00,'2026-01-05'),(6,102,'cancelled',60.00,'2026-01-06'),
    (7,105,'paid',100.00,'2026-01-07'),(8,106,'refunded',200.00,'2026-01-08'),
    (9,107,'pending',NULL,'2026-01-09'),(10,108,'cancelled',240.00,'2026-01-10');
INSERT INTO orders
SELECT 1000+g,
    CASE WHEN g%10<7 THEN 201 ELSE 202+(g%39) END,
    CASE WHEN (g+{{seed}})%3=0 THEN 'paid' WHEN (g+{{seed}})%3=1 THEN 'pending' ELSE 'cancelled' END,
    CASE WHEN g%13=0 THEN NULL ELSE (g%23)*12.50 END,
    '2026-02-01'::timestamptz+g*interval '1 hour'
FROM generate_series(1,300+{{seed}}*80) g;
INSERT INTO order_items VALUES
    (1,1,'BOOK',1,25.50),(2,1,'SEAT',1,50.00),(3,1,'SEAT',1,50.00),
    (4,3,'BOOK',2,20.00),(5,3,'BOOK',2,20.00),(6,5,'SEAT',3,80.00),
    (7,7,'BOOK',1,100.00),(8,8,'SEAT',2,100.00);
INSERT INTO order_items
SELECT 10000+g*10+j,1000+g,'SKU-'||(j%3),1+(g%3),(g%17+1)*4.25
FROM generate_series(1,300+{{seed}}*80) g
CROSS JOIN LATERAL generate_series(1,1+(g%4)) j
WHERE g%7<>0;
INSERT INTO payments VALUES
    (1,1,'captured',70.00,'2026-01-01'),(2,1,'captured',55.50,'2026-01-01'),
    (3,1,'failed',40.00,'2026-01-01'),(4,3,'captured',40.00,'2026-01-03'),
    (5,3,'captured',40.00,'2026-01-03'),(6,5,'captured',240.00,'2026-01-05'),
    (7,8,'refunded',200.00,'2026-01-08'),(8,2,'failed',49.00,'2026-01-02'),
    (9,NULL,'captured',8.00,'2026-01-09');
INSERT INTO payments
SELECT 10000+g*10+j,1000+g,
    CASE WHEN j=1 AND g%4=0 THEN 'failed' ELSE 'captured' END,
    (g%17+1)*10.25,'2026-02-01'::timestamptz+g*interval '1 hour'+j*interval '1 minute'
FROM generate_series(1,300+{{seed}}*80) g
CROSS JOIN LATERAL generate_series(1,1+(g%3)) j
WHERE g%5<>0;
ANALYZE customers;
ANALYZE orders;
ANALYZE order_items;
ANALYZE payments;
