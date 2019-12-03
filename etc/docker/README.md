Dockerizing our Rust application

# Requirements

* Docker

# Building the image

    $ cd ../..
    $ docker build -t 'pdf_printer' -f etc/docker/Dockerfile .

# Running container and checking things...

    $ cd etc/docker  # if in top-level directory

Interactive shell:

    $ docker run -it pdf_printer /bin/sh
    / # ls -la /pdf_printer
    -rwxr-xr-x    1 root     root       3080320 Nov 14 15:03 /pdf_printer
    / # exit

With bind mounting a directory containing PDF example files to test:

    $ docker run -v <PATH_TO>/examples:/examples -it pdf_printer /bin/sh
    # ls -la /examples
    total 16
    drwxr-xr-x 5 root root  160 Jul  8 20:52 .
    drwxr-xr-x 1 root root 4096 Nov 27 17:53 ..
    -rw-r--r-- 1 root root  661 Jul  8 20:52 Rosenthol_example.pdf
    -rw-r--r-- 1 root root  914 Jul  8 20:52 Rosenthol_example_2pages.pdf
    -rw-r--r-- 1 root root  739 Jun  6 21:40 minimal.pdf
    # /pdf_printer /examples/minimal.pdf
    INFO     - minimal.pdf at        733 - Found %%EOF at offset 733.
    INFO     - minimal.pdf at        725 - Found startxref at offset 725.
    INFO     - minimal.pdf at        719 -  startxref span: 719..732.
    INFO     - minimal.pdf at        565 - startxref points to offset 565 for xref
    INFO     - minimal.pdf at        570 - Found 5 objects starting at 0:
    [...]

# Housekeeping

To see docker images and all (even stopped) containers:

    $ docker images
    $ docker ps --all

To clean up dangling (e.g., intermediate builder containers) use:

    $ docker rmi $(docker images -q -f dangling=true)

# Links

Rust and Docker:
* [https://dev.to/gruberb/web-programming-in-rust-02x-deploy-your-first-app-1k05]
* [https://www.fpcomplete.com/blog/2018/07/deploying-rust-with-docker-and-kubernetes]

Alpine Linux and Docker:
* [http://containertutorials.com/alpine/get_started.html]
