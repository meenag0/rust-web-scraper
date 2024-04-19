# Build stage
FROM rust:bookworm AS builder
# Use a Rust base image

# Set the working directory inside the container
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev


# Copy the entire source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Use a minimal Debian base image for the final image
FROM debian:bookworm-slim AS runner

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage into the final image
COPY --from=builder /app/target/release/scrape /app/scrape

# Define the command to run your application
CMD ["/app/scrape"]