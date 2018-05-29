
There is a single binary, named nanomsg_test,  which checks the value of 
argv[1] if it exists, or the environment variable "mode" if it exists. The 
value "pull" will put the binary in pull mode, any the value "push" will 
put the binary in push mode. Any other value will cause the binary to 
display the usage message and exit with a non-zero return code. 

You can try it by calling it directly, using docker. or using docker-compose.

1. Manual build and test procedure:

        * Install nanomsg.rs from https://github.com/nanomsg.rs . Download it
          into deps/nanomsg.rs and run *make deps*. 
        * build /target/debug/nanomsg_test:  *cargo build*
        * run the listener detached: *./nanomsg_pull &*
        * run the messsage sender: ./nanomsg_push


2. Using Docker:

        docker build -t nanomsg_test .
        docker run -v /tmp:/tmp nanomsg_test pull
        docker run -v /tmp:/tmp nanomsg_test push


3. Using Docker-compose:

        docker-compose up


