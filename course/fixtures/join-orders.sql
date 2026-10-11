CREATE TABLE customers (id integer PRIMARY KEY, name text NOT NULL);
INSERT INTO customers
SELECT g, 'Customer '||((g+{{seed}}*3)%25) FROM generate_series(1,1000) g;
ALTER TABLE orders ADD CONSTRAINT orders_customer_fk FOREIGN KEY(customer_id) REFERENCES customers(id);

CREATE TABLE order_items (
    id integer PRIMARY KEY,
    order_id integer NOT NULL REFERENCES orders(id),
    sku text NOT NULL,
    quantity integer NOT NULL CHECK(quantity>0),
    unit_price numeric(20,2) NOT NULL CHECK(unit_price>=0)
);
INSERT INTO order_items
SELECT g*10+j,g,'SKU-'||((g+j+{{seed}})%7),1+(g+{{seed}})%3,(g%17+{{seed}}+1)*4.25
FROM generate_series(1,100000) g
CROSS JOIN LATERAL generate_series(1,1+(g+{{seed}})%4) j
WHERE g%7<>0;
INSERT INTO order_items VALUES
    (2000001,97,'DUPLICATE',3,10.25),(2000002,97,'DUPLICATE',3,10.25),
    (2000003,1000-{{seed}},'DUPLICATE',3,10.25),(2000004,1000-{{seed}},'DUPLICATE',3,10.25);
CREATE INDEX order_items_order_idx ON order_items(order_id);

CREATE TABLE payments (
    id integer PRIMARY KEY,
    order_id integer REFERENCES orders(id),
    status text NOT NULL CHECK(status IN('captured','failed','refunded')),
    amount numeric(20,2) NOT NULL CHECK(amount>=0),
    occurred_at timestamptz NOT NULL
);
INSERT INTO payments
SELECT g*10+j,g,
    CASE WHEN j=1 AND g%4=0 THEN 'failed' WHEN j=3 AND g%17=0 THEN 'refunded' ELSE 'captured' END,
    (g%19+{{seed}}+1)*7.25,'2026-01-01 00:00:00+00'::timestamptz+g*interval '1 minute'+j*interval '1 second'
FROM generate_series(1,100000) g
CROSS JOIN LATERAL generate_series(1,1+(g+{{seed}})%3) j
WHERE g%11<>0;
INSERT INTO payments VALUES
    (2000001,97,'captured',25,'2026-01-11 00:01:00+00'),
    (2000002,97,'captured',25,'2026-01-11 00:01:01+00'),
    (2000003,1000-{{seed}},'captured',30,'2026-01-01 00:00:00+00'),
    (2000004,1000-{{seed}},'captured',30,'2026-01-01 00:00:01+00'),
    (2000005,NULL,'captured',8+{{seed}},'2026-01-01 00:00:02+00');
CREATE INDEX payments_order_idx ON payments(order_id);
ANALYZE customers;
ANALYZE order_items;
ANALYZE payments;
