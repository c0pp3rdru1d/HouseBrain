-- PRODUCTS
create table if not exists products (
  id uuid primary key,
  sku text not null unique,
  name text not null,
  is_active boolean not null default true,
  created_at timestamptz not null default now()
);

-- INVENTORY LEDGER (append-only)
create type inventory_event_type as enum (
  'receive',
  'adjust',
  'allocate',
  'pick',
  'return'
);

create table if not exists inventory_events (
  id uuid primary key,
  product_id uuid not null references products(id),
  qty_delta bigint not null,
  event_type inventory_event_type not null,
  created_at timestamptz not null default now()
);

create index if not exists idx_inventory_events_product_id on inventory_events(product_id);
create index if not exists idx_inventory_events_created_at on inventory_events(created_at);

-- A simple balance view (derived)
create or replace view inventory_balances as
select
  product_id,
  coalesce(sum(qty_delta), 0) as qty_on_hand
from inventory_events
group by product_id;
