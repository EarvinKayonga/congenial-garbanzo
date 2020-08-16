#! /bin/bash
apt-get -qq update
apt-get install -y  build-essential wget zip

wget https://github.com/protocolbuffers/protobuf/releases/download/v3.13.0/protoc-3.13.0-linux-x86_64.zip
unzip protoc-3.13.0-linux-x86_64.zip
rm -f *.zip
mv bin/* /usr/local/bin
mv include/* /usr/local/include/
protoc --version