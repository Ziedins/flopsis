# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/flopsis /app/flopsis
COPY --from=builder /app/out /app/out
COPY --from=builder /app/templates /app/templates
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/app/flopsis"]
