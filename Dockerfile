FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -q -y --no-install-recommends \
    binutils \
    grub-common \
    xorriso \
    grub-pc-bin
