FROM    golang:latest as common
WORKDIR /tmp

RUN     go get -u github.com/grpc-ecosystem/grpc-gateway/protoc-gen-grpc-gateway &&\
        go get -u github.com/grpc-ecosystem/grpc-gateway/protoc-gen-swagger     &&\
        go get -u github.com/golang/protobuf/protoc-gen-go

COPY    scripts/install_proto.sh   /tmp
RUN     /tmp/install_proto.sh

COPY    .   /tmp

FROM    common as build-swagger
RUN     protoc  -I/usr/local/include -I.            \
                -I$GOPATH/src                       \
                -I$GOPATH/src/github.com/grpc-ecosystem/grpc-gateway/third_party/googleapis \
                --swagger_out=logtostderr=true:.    \
                echo.proto

FROM    swaggerapi/swagger-ui as swagger

ENV     SWAGGER_JSON=/mnt/swagger.json BASE_URL=/doc
EXPOSE  8080
COPY    --from=build-swagger /tmp/echo.swagger.json /mnt/swagger.json