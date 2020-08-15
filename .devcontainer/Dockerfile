FROM debian:buster

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
  && apt-get -y install --no-install-recommends apt-utils apt-transport-https 2>&1 \
  && apt-get -y install git procps lsb-release wget curl gnupg tar unzip make \
  build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev \
  wget curl llvm libncurses5-dev libncursesw5-dev xz-utils tk-dev libffi-dev \
  liblzma-dev python-openssl

# Rust Lang
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Utils
RUN apt-get update \
  && apt-get -y install vim zsh \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Config ZSH
RUN curl -L git.io/antigen > ~/antigen.zsh
COPY zshrc /root/.zshrc

# Dev Env
ENV DEBIAN_FRONTEND=dialog
ENV PATH=/root/.cargo/bin:$PATH

# Rust Components
RUN rustup component add rust-analysis --toolchain stable-x86_64-unknown-linux-gnu
RUN rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
RUN rustup component add rls --toolchain stable-x86_64-unknown-linux-gnu

CMD /bin/zsh -l
