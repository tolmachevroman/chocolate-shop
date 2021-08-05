# sputnik

This repo demonstrates how to setup a Rust micro-service using Rocket, async-graphql, and sqlx. For some background on this, go [here]().

*Note: This repo assumes you have some familiarity with Rust, graphQL, and SQL.*
# Installation Steps
## Cloning related repo

Besides cloning this repo, you will also need to clone [async-graphql](https://github.com/async-graphql/async-graphql). The reason you need to do that is because the Rocket support for async-graphql is currently unpublished. Make the necessary changes to `Cargo.toml` to update the path information to `async-graphql` and `async-graphql-rocket`.
## Database Setup

1.  This project assumes that you have `postgres` installed locally. At this point, you will need to create a `starwars` database.

    ```psql -h localhost -U postgres -w -c "create database starwars;"```


2.  Create a `.env` file in the project root directory and add the following line:

    ```DATABASE_URL="postgres://postgres:<YOUR_PWD>@localhost:5432/starwars"```

    This environment variable is used as a configuration input to the database connection pool to indicate where to find the database

## Seeding your database with migrations

TODO explain https://github.com/launchbadge/sqlx/tree/master/sqlx-cli



## Running the service

The current configuration in `Rocket.toml` has been setup to run the micro-service on port 7600. After you have `cargo build` and `cargo run`, you can browse the graphQL server playground here:

```
http://localhost:7600/
```


