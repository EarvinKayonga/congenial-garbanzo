FROM        golang:1.15-alpine as builder
ENV         GO111MODULE on
RUN         apk add -u --no-cache build-base git
ADD         .   /go/src/github.com/EarvinKayonga/Morning/bastion
WORKDIR     /go/src/github.com/EarvinKayonga/Morning/bastion
RUN         make build
RUN         strip /go/src/github.com/EarvinKayonga/Morning/bastion/bin/bastion

FROM        alpine:latest as release
EXPOSE      8080
WORKDIR     /root/
COPY        --from=builder /go/src/github.com/EarvinKayonga/Morning/bastion/bin/bastion /bastion
CMD         ["/bastion"]