# Rust Warp backend server

This is a Rust Warp backend server prototype for the demo video below. I made it for a freelance client as a POF a few months before. I had a freedom to use the language for a backend server. So, I used Rust to prove myself that I can do it with Rust. If I have to do it again, I would use Python.

[![React Rust demo](https://img.youtube.com/vi/I1iNhOuXESQ/0.jpg)](https://www.youtube.com/watch?v=I1iNhOuXESQ)

I share it because I have to send some private works for Rust working opportunity. If I have to, I prefer it to be an open source.

If writing Rust were easy and take less time, I would rewrite. You can imporve it yourselves with TODO list below.

The payload will be **session.rs** file.

## How to test it

You can use **python3 dev.py** or $cargo run --bin main or $RUST_LOG=debug cargo run --bin main to test a web server.

If you are confused, start with **hello** and **hi** apis.

You can also test other CLI commands with cargo run --bin name. Refer to **Cargo.toml** for that.

## End points

I let CURL commands for each files in routes/ folder to help you test the end points. But, you can start with these first.

* Register a user

```console
$curl -X POST localhost:8000/api/user/v1 -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }'
```

* List users

```console
$curl localhost:8000/api/user/v1
```

* Login

```console
curl -X POST localhost:8000/api/user/v1/login -c cookie.txt -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }'
```

* Update cash

```console
$curl -X PATCH localhost:8000/api/user/v1/cash -b cookie.txt -L -H "Content-Type: application/json" -d '{ "amount": 100000 }'
```

* Buy a car

```console
$curl -X POST localhost:8000/api/user/v1/car -b cookie.txt -L -H "Content-Type: application/json" -d '{ "price": 10000, "color": "red" }'
```

* List cars

```console
$curl -X GET localhost:8000/api/user/v1/car -b cookie.txt -L
```

* Gamble with cash

```console
$curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L -H "Content-Type: application/json" -d '{ "stake_amount": 10000, "car_id": null, "number_of_participants": 2 }'
```

* Gamble with a car

```console
$curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L -H "Content-Type: application/json" -d '{ "stake_amount": 10000, "car_id": null, "number_of_participants": 2 }'
```

* Ranking

```console
$curl localhost:8000/api/ranking/v1/game
```

* Delete a user

```console
$curl -X GET localhost:8000/api/user/v1/logout -b cookie.txt -L
```

## TODO

This was just a prototype to clone a function of a gambling website. It is far from perfect. I will include some lists that you can improve.

If you want working Rust code to reuse, refer to the [Rust Full Stack repository](https://github.com/steadylearner/Rust-Full-Stack).

* Proper error handling with [thiserror](https://github.com/dtolnay/thiserror) and [anyhow](https://github.com/dtolnay/thiserror).

* [Domain driven project design](https://github.com/golang-standards/project-layout) instead of [the current group by function(models/, handlers/, routes/ etc) and remove utils/ and other unecessary ones](https://www.youtube.com/watch?v=oL6JBUk6tj0). It was difficult to structure the Warp app this way.

* Extract common parts to functions.

* Find how to reuse SQLite connection or substitute it with [Postgresql and reuse connection with lazy_static](https://github.com/steadylearner/Rust-Full-Stack/tree/master/warp/database/2.%20with_db_pool).

* Currently, error responses from Warp relevant code are not perfect. It will be only worth doing that if you develop it with frontend part also.

* User session needs a timeout relevant code. You can find better solutions or use [prebuilt ones such as Redis etc](https://github.com/steadylearner/Rust-Full-Stack/tree/master/microservices_with_docker).

* [Include tests](https://github.com/steadylearner/Rust-Full-Stack/tree/master/microservices_with_docker/warp_client/src/tests/user) for every possible routes instead of CURL commands.

* Remove every unwrap(); parts.

* Use trustable 3rd API for random number generation and other manual implementations etc.

* [Use documenation features of Rust better.](https://github.com/steadylearner/born)

I did code with it a few months ago so they are only what I can think currently instead of investing so much time to read all code again.

There are not many Rust web server examples. I wouldn't write this way again if the development were for myself. But, hope you can save the compile time with it at least.

Frontend part is up to you. I wrote it because I had to make a demo video to get payment. [You can implement it on your own](https://github.com/steadylearner/Rust-Full-Stack/tree/master/parcel-react) referring to the example above.
