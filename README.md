There is a single binary which checks the value of argv[1] if it exists, or
the environment variable "MODE" if it exists. The value of "pull" will put the
binary in pull mode, any the value of "push" will put the binary in push mode. Any
other value will cause the binary to display the usage message and exit with a
non-zero return code. ( The program will panic! if called without arguments and the
environment variable "MODE" is not set either, because I overlooked to handle this
case, but still fits the spec thanks to the marvel that Rust delivers. :) )

1. Manual build and test procedure:

        cargo build
        ./nanomsg_pull &
        ./nanomsg_push


2. Using Docker:

        docker build -t nanomsg_test .
        docker run -v /tmp:/tmp nanomsg_test pull
        docker run -v /tmp:/tmp nanomsg_test push


3. Using Docker-compose:
        docker-compose up


