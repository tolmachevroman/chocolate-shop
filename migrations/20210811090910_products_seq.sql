-- Add migration script here
CREATE SEQUENCE products_Seq start with 11;

ALTER TABLE products ALTER column id set default nextval('products_Seq');
