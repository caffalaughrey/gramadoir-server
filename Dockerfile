FROM rust:1.88-slim AS rust-build

WORKDIR /build

COPY . .
RUN cp -R /build/deps/GramBridge /tmp/GramBridge && \
    apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    llvm-dev \
    pkg-config \
    make \
    perl \
    libperl-dev \
    zlib1g-dev \
    curl \
    git \
    build-essential \
    cpanminus

ENV PERL_CARTON_PATH=/opt/perl5 PERL5LIB=/opt/perl5/lib/perl5
RUN cpanm --notest --local-lib=/opt/perl5 Lingua::GA::Gramadoir /tmp/GramBridge && \
    rm -rf /var/lib/apt/lists/* && \
    cargo clean && cargo build --release 

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends perl libstdc++6 zlib1g curl \
 && rm -rf /var/lib/apt/lists/*
ENV PERL5LIB=/opt/perl5/lib/perl5
COPY --from=rust-build /opt/perl5 /opt/perl5
COPY --from=rust-build /build/target/release/gramadoir /usr/local/bin/gramadoir
EXPOSE 5000
CMD ["gramadoir"]

