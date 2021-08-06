# chocolate-shop

GraphQL backend for an imaginary chocolate shop implemented in Rust, `rocket`, `async-graphql` and `sqlx`. For detailed installation guide please refer to the [bootstrap project](https://github.com/lionkeng/sputnik). The only difference I used SQLx CLI to manage migrations.

# Project-specific implementation details

## Schema

GraphQL schema is quite simple:

```
enum ChocolateType {
	Bitter, White, Milk
}

type Product {
	id: Int!
	name: String!
    description: String!
	price: Int!
	chocolateType: ChocolateType!
	fillings: [String]
	images: [String]!
}
```

[Requests](/src/models/mod.rs) include querying, updating a price of a product and deleting it.

## Using SQLx CLI to perform migrations

To facilitate the migrations flow please check the [SQLx CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli). It allows you to add and run migrations in a clean fashion. Refer to [migrations](/migrations) folder to see some examples.


## Running the service

The current configuration in `Rocket.toml` has been setup to run the micro-service on port 7600. After you have `cargo build` and `cargo run`, you can browse the graphQL server playground here:

```
http://localhost:7600/
```


