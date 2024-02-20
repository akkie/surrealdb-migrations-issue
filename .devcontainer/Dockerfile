FROM mcr.microsoft.com/devcontainers/rust

# https://techoverflow.net/2021/01/13/how-to-use-apt-install-correctly-in-your-dockerfile/
ENV DEBIAN_FRONTEND=noninteractive

# Install build tools
RUN apt update && apt install -y cmake build-essential

# https://code.visualstudio.com/remote/advancedcontainers/persist-bash-history
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/bash_history/.bash_history" \
    && mkdir /bash_history \
    && touch /bash_history/.bash_history \
    && echo "$SNIPPET" >> "/root/.bashrc"

# Install Starship prompt: https://starship.rs/
RUN apt install -y fonts-firacode  \
    && cargo install starship --locked \
    && echo "eval \"\$(starship init bash)\"" >> "/root/.bashrc"

# Install Cargo dev packages
RUN cargo install cargo-watch \
    && cargo install surrealdb-migrations