FROM fedora:41 AS builder

RUN dnf install -y curl gcc openssl-devel
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app

COPY . /app

RUN cargo build --release

FROM fedora:41
COPY --from=builder /app/target/release/discord_notifications /root/discord_notifications

ENTRYPOINT [ "/root/discord_notifications" ]
