# Use a base image with the latest Ubuntu
FROM ubuntu:noble-20240605

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    uuid \
    binaryen \
    libfontconfig1-dev \
    wget \
    openssl \
    libssl-dev

# Install GitHub CLI
RUN mkdir -p -m 755 /etc/apt/keyrings && \
    wget -qO- https://cli.github.com/packages/githubcli-archive-keyring.gpg | tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null && \
    chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null && \
    apt-get update && \
    apt-get install gh -y

# Set environment variables
ENV RUST_CHANNEL="nightly"
ENV RUST_DATE="2024-06-21"
ENV RUST_HOST="x86_64-unknown-linux-gnu"
ENV RUST_TOOLCHAIN="${RUST_CHANNEL}-${RUST_DATE}-${RUST_HOST}"
ENV TRUNK_VERSION="0.17.5"
ENV PATH="/root/.cargo/bin:/usr/local/cargo/bin:${PATH}"

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    /root/.cargo/bin/rustup toolchain install "${RUST_TOOLCHAIN}" && \
    /root/.cargo/bin/rustup default "${RUST_TOOLCHAIN}" && \
    /root/.cargo/bin/rustup run "${RUST_TOOLCHAIN}" rustup target add wasm32-unknown-unknown && \
    /root/.cargo/bin/cargo +"${RUST_TOOLCHAIN}" install trunk@"${TRUNK_VERSION}"

# Verify Rust installation
RUN /root/.cargo/bin/rustc --version && \
    /root/.cargo/bin/cargo --version && \
    /root/.cargo/bin/rustup --version && \
    /root/.cargo/bin/rustup toolchain list

# Set permissions for /root/.cargo
RUN chmod -R 777 /root/.cargo

# Create symbolic link
RUN mkdir -p /github/home && \
    ln -s /root/.cargo /github/home/.cargo

# Set working directory
WORKDIR /app

# Copy scripts
COPY .github/scripts/build.sh /app/build.sh
COPY .github/scripts/push.sh /app/push.sh

# Make scripts executable
RUN chmod +x /app/build.sh /app/push.sh

# Set entrypoint
ENTRYPOINT ["/bin/bash"]