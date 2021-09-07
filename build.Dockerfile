FROM rust:latest

ARG WORKDIR
WORKDIR $WORKDIR

ARG UID
ARG GID
ARG USER
COPY usersetup.sh .
RUN ./usersetup.sh
USER $USER

# make cargo update the index the first time (it takes *ages*)
RUN cargo search libc

USER root
COPY sysadmin.sh .
RUN ./sysadmin.sh
USER $USER

