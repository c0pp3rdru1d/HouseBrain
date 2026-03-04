-- Fix inventory balances view to cast to bigint
drop view if exists inventory_balances;

create view inventory_balances as
select
  product_id,
  coalesce(sum(qty_delta), 0)::bigint as qty_on_hand
from inventory_events
group by product_id;
