I implemented the test project using the following steps:

## 1. Find Rust bindings for Nanomsg C library.

I found three of them:
* 1a) https://github.com/thehydroimpulse/nanomsg.rs
* 1b) https://github.com/tmccombs/nmsg-rust
* 1c) https://github.com/blabaere/scaproust

scaproust states it is experimental and recommends using nanomsg.rs, because that is 'working stuff', so I decided to stick to it. Unlike nmsg-rust, nanomsg.rs also had .travis.yml and appveyor.yml ready.

## 2. Implement 2 Rust binaries, or just 1 accepting socket mode as command line argument, which connect to each other using nanomsg.1 in pull mode, other in push.

I chose to use a single binary with command line arguments. The pipeline.rs examples show how to do this.

## 3. "Push" pushes a message and exits with zero code if push was successful. Exits with non-zero code on any error.

I found Rust currently does not provide a clean fn main() -> i32, so I looked for alternatives. Currently, rust provides an

               fn main() -> Result<(), ()>

signature which can be useful for the user, but still did not get the exit code right. So I tried calling

               std::process::exit(code:i32)

which seem to work under Linux, but under Windows always 0 is returned.

Finally, I decided to use libc::exit() directly, whis always seem to work as expected.

               extern crate libc;
               fn main() {
                  unsafe { libc::exit(1); };
               }


## 4. "Pull" waits for message for 10 seconds, if it doesn't receive message it exits with non-zero code.

* 4.1 At this point I decided to create the project locally:

              cargo new nanomsg_test --bin

* 4.2 I added anticipated dependencies to cargo.toml:

              [dependencies]
              nanomsg = "0.6.2"
              libc = "*"

    and imported the nanomsg crate into main.rs:

              extern crate nanomsg;'''

* 4.3 Downloaded and built depedencies

              git clone -b 1.0.0 --depth 1 https://github.com/nanomsg/nanomsg.git nanomsg-1.0.0

    Created a build folder from which :

            cmake ..  (It found the Visual Studio 14 2015 toolset, which was OK)

    Then built nanomsg.lib:

            cmake --build .

    So after abt 3 minutes I got my debug version of nanomsg.lib:

            Done Building Project "F:\Projects\Komodo Blockchain\Rust Test\nanomsg_test\nanomsg-1.0.0\build\ALL_BUILD.vcxproj" (default targets).

            Build succeeded.
               0 Warning(s)
               0 Error(s)

            Time Elapsed 00:03:33.16'''

    then, installed it:

            cmake --build . --target install

    this command caused

               F:\Projects\Komodo Blockchain\Rust Test\nanomsg_test\nanomsg-1.0.0\build\Debug\nanomsg.lib

    to be copied to

               C:\Program Files (x86)\nanomsg\lib

    Also the include and bin directories have been set up properly:

              -- Install configuration: "Debug"
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/nn.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/inproc.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/ipc.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/tcp.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/ws.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/pair.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/pubsub.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/reqrep.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/pipeline.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/survey.h
              -- Installing: C:/Program Files (x86)/nanomsg/include/nanomsg/bus.h
              -- Installing: C:/Program Files (x86)/nanomsg/bin/nanocat.exe
              -- Installing: C:/Program Files (x86)/nanomsg/lib/pkgconfig/nanomsg.pc
              -- Installing: C:/Program Files (x86)/nanomsg/lib/nanomsg.lib
              -- Installing: C:/Program Files (x86)/nanomsg/bin/nanomsg.dll


* 4.4 Then, tested to see if exit codes using libc work properly. They did:


      fn main() {
          unsafe { libc::exit(1); };
      }

  gives the following output on Windows:

      cargo run
      Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
      Running `target\debug\nanomsg_test.exe`
      error: process didn't exit successfully: `target\debug\nanomsg_test.exe` (exit code: 1)


  Changing the exit code to 0:


      fn main() {
          unsafe { libc::exit(0); };
      }


  gives:

      cargo run
      Compiling nanomsg_test v0.1.0 (file:///F:/Projects/Komodo%20Blockchain/Rust%20Test/nanomsg_test)
      Finished dev [unoptimized + debuginfo] target(s) in 3.38 secs
      Running `target\debug\nanomsg_test.exe`

* 4.5 Implemented the command line arguments in order to be able to run the same executable in PULL and PUSH modes.

* 4.6 After testing it, at this point I decided to publish it on Github:

       >git init
       >git config user.name ""
       >git config user.email ""
       :updated .gitignore not to upload nanomsg-1.0.0 dependency and other junk
       >git add -A .
       >git commit -m "Initial commit"
       >git remote add origin https://github.com/latsa/nanomsg_test.git
       >git push origin master

* 4.7 Initially puller() was waiting forever. I added a 10 second inactivity timeout
      using poll. ( Another alternative would have been to implement a 'monitor'
      in a background thread and use a on second interval timer to increment a counter.
      Whenever a message arrives, the counter would be reset. Whenever a timer tick
      arrives, the counter value is checked whether is greater than 10, in which case
      the program would exit. Using this approach, care should be taken to close the
      network connection and also to serialize access to the counter (e.g. with a mutex)
      as it is accessed from two different threads.)

## 5. Add docker-compose config with containers connecting to each other.

* 5.1 Created a Dockerfile based on the official rust:latest image.
* 5.2 Added tools necessary to download, make and install the nanomsg dependency.
* 5.3 Built the image with:
    docker build -t nanomsg_test .
* 5.4 Tested the images with these commands:
    docker run -v /tmp:tmp nanomsg_test pull
    docker run -v /tmp:tmp nanomsg_test push

As expected, pull times out, unless a message arrives within 10 seconds.

* 5.5 Created docker-compose.yml . The command "docker-compose up" gives
the following expected output:

    laszlos@MS-7821:~/Projects/Learning Rust/nanomsg_test$ docker-compose up
    Creating network "nanomsgtest_default" with the default driver
    Creating nanomsgtest_puller_1
    Creating nanomsgtest_pusher_1
    Attaching to nanomsgtest_puller_1, nanomsgtest_pusher_1
    puller_1 | Puller listening on 'ipc:///tmp/pipeline1.ipc'.
    puller_1 | Puller got a message: 'Hello'.
    pusher_1 | Simon sez Hello.
    pusher_1 | Simon sez Bye
    nanomsgtest_puller_1 exited with code 0
    nanomsgtest_pusher_1 exited with code 0

