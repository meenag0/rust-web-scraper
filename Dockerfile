# Build stage
FROM rust:bookworm AS builder

# Set the working directory inside the container
# Set the working directory inside the container
WORKDIR /rws

# Copy the entire source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Use a minimal Debian base image for the final image
FROM debian:bookworm-slim AS runner

# Set the working directory inside the container
WORKDIR /rws

# Copy the compiled binary from the builder stage into the final image
COPY --from=builder /rws/target/release/scrape /app/scrape

# Define the command to run your application
CMD ["/app/scrape"]
