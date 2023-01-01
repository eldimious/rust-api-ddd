# rust-api-ddd

# What is this repository for? #
Rust server architecture showcase using [Warp](https://github.com/seanmonstar/warp) and [Sqlx](https://github.com/launchbadge/sqlx) for Postgres queries.

# Architecture Overview #
The app is designed to use a layered architecture. The architecture is heavily influenced by the Clean Architecture.[Clean Architecture](https://8thlight.com/blog/uncle-bob/2012/08/13/the-clean-architecture.html) is an architecture where:

  1. **does not depend on the existence of some framework, database, external agency.**
  2. **does not depend on UI**
  3. **the business rules can be tested without the UI, database, web server, or any external element.** 

<p align="center">
  <img src="https://cdn-images-1.medium.com/max/719/1*ZNT5apOxDzGrTKUJQAIcvg.png" width="350"/>
  <img src="https://cdn-images-1.medium.com/max/900/0*R7uuhFwZbhcqZSvn" width="350" /> 
</p>

<p align="center">
  <img src="https://cdn-images-1.medium.com/max/1200/0*rFs1UtU4sRns5vCJ.png" width="350" />
  <img src="https://cdn-images-1.medium.com/max/1200/0*C-snK7L4sMn7b6CW.png" width="350" /> 
</p>

Also, in entry point(main.rs), I use Dependency Injection(DI). There are many reasons using Dependency Injection as:
1. Decoupling
2. Easier unit testing
3. Faster development
4. Dependency injection is really helpful when it comes to testing. You can easily mock your modules' dependencies using this pattern.

## Data Layer ##

The data layer is implemented using repositories, that hide the underlying data sources (database, network, cache, etc), and provides an abstraction over them so other parts of the application that make use of the repositories, don't care about the origin of the data and are decoupled from the specific implementations used. Furthermore, the repositories are responsible to map the entities they fetch from the data sources to the models used in the applications. This is important to enable the decoupling.

## Domain Layer ##

The domain layer is implemented using services. They depend on the repositories to get the app models and apply the business rules on them. They are not coupled to a specific database implementation and can be reused if we add more data sources to the app or even if we change the database for example from MongoDB to Couchbase Server.

## Routes/Controller Layer ##

This layer is being used in the express app and depends on the domain layer (services). Here we define the routes that can be called from outside. The services are always used as the last middleware on the routes. That means that the middlewares registered before should not alter data being passed to the domain layer. They are only allowed to act upon the data without modification, like for example validating the data.

# Quick start #

### Prerequisites ###

Create an .env file in project root to register the following required environment variables:
  - `DATABASE_URL` - Postgres connection URL
  - `PORT` - port of server

### Use Docker: ###

You can use Docker to start the app locally. The Dockerfile and the docker-compose.yml are already provided for you. You have to run the following command:

```shell
docker-compose up
```

and then run:

```shell
cargo run
```

## Support Me

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/Y8Y797KCA)

## Show your support

Give a ⭐️ if this project helped you!
