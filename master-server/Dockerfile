FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .
# RUN cargo build --release
RUN cargo build
RUN ls target/debug

# stage 2
FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y gnupg2
# RUN apt-key adv --refresh-keys --keyserver keyserver.ubuntu.com && \
#     apt-get clean \
RUN apt-get update && apt-get install -y iputils-ping nmap wait-for-it && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/debug/master-server /usr/local/bin/app
CMD ["app"]