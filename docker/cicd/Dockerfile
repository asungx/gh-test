FROM ubuntu:22.04

RUN apt-get update \
    && apt-get -y upgrade \
    && apt-get install -y \
        build-essential \
        curl \
        git \
    && :

# Add exception to dubious repo ownership warning from git
RUN git config --global --add safe.directory /mnt/gh_test

# Install GitHub CLI
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
    && apt-get update \
    && apt-get install -y gh

# Install Rust
ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path \
    && mv /opt/rust/bin/* /usr/bin

CMD echo 'Sleeping indefinitely...' && sleep infinity
