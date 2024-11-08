FROM rust:latest

# Install necessary tools
RUN apt-get update && \
    apt-get install -y git && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Clone the rustscan repository
RUN git clone https://github.com/XiaomingX/rustscan.git

# Change working directory to rustscan
WORKDIR /app/rustscan

# Build the project
RUN cargo build --release

# Run the compiled binary as the default command
CMD ["/app/rustscan/target/release/rustscan"]
