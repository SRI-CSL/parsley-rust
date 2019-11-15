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