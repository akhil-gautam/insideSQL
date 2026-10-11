INSERT INTO orders (id,customer_id,status,total,ordered_at)
SELECT 900000,109,'pending',25+{{seed}}*10,'2026-01-11'::timestamptz
WHERE {{seed}}<>1;
ANALYZE orders;
