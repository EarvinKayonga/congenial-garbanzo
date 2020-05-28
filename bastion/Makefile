 
commit = $(shell git log --pretty=format:'%H' -n 1)
now = $(shell date "+%Y-%m-%d %T UTC%z")
compiler = $(shell go version)
branch = $(shell git rev-parse --abbrev-ref HEAD)

bin = bin/bastion

build:
	@echo "Compiling the binary"
	@CGO_ENABLED=0 go build  -v						\
		-o $(bin)									\
	    -ldflags                              		\
	       "-X 'main.branch=$(branch)'        		\
	       	-X 'main.sha=$(commit)'           		\
	       	-X 'main.compiledAt=$(now)'       		\
	       	-X 'main.compiler=$(compiler)'			\
			-s -w"   								\
	    -a -installsuffix cgo 


clean:
	rm -rf $(bin)

re: clean build
