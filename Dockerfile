FROM rust:1.45

WORKDIR /app
COPY . .
RUN rustup default nightly && cargo build


FROM ubuntu:18.04

WORKDIR /app
RUN apt-get update && \
    apt-get install -y wget build-essential && \
    wget http://download.redis.io/redis-stable.tar.gz && \
    tar xvzf redis-stable.tar.gz && \
    cd redis-stable && \
    make && make install

COPY --from=0 app/target/debug/rocket_redis_r2d2 .
EXPOSE 8000
ADD init.sh .
RUN chmod +x init.sh

CMD ["./init.sh"]