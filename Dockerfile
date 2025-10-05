# базовые утилиты 
FROM rust:1.89-alpine AS base 

WORKDIR /base

RUN apk add --no-cache build-base

RUN cargo install cargo-chef 


# рецеп для сборки зависимостей 
FROM base AS prepare-recipe

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


# установка зависимостей & сборка проекта
FROM base AS builder

COPY --from=prepare-recipe /base/recipe.json ./recipe.json

RUN cargo chef cook --release --recipe-path ./recipe.json

COPY . .

RUN cargo build --release --bin app

# запуск приложения
FROM alpine:latest AS runtime

WORKDIR /runtime 

COPY --from=builder /base/target/release/app ./app

COPY ./migrations ./migrations

CMD ["./app"]