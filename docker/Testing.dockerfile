FROM ubuntu:20.04

RUN apt-get update -qq &&\
    DEBIAN_FRONTEND=noninteractive \ 
    apt-get install -y -qq --no-install-recommends \
    python3 \
    python3-pip \
    python3-virtualenv \
    cargo \
    pkg-config \
    libssl-dev \
    python3-dev &&\
    virtualenv --seeder=pip --download /venv &&\
    . /venv/bin/activate &&\
    pip install maturin pytest angreal==2.0.0-rc.1
