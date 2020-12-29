Dockerizing our Rust application

# Requirements

* Docker

# Building and deploying the image

    $ cd ../..
    $ docker build -t 'pdf_printer' -f etc/docker/Dockerfile [--no-cache] .
    
In order to set up access to Artifactory, log in to https://artifactory.sri.com 
Click on your username near the upper right corner. 
Unlock with your password (if prompted). 
Generate and copy your "encrypted API key".

Then, log into the registry.  This is only needed once. The credentials will then be stored in
`~/.docker/config.json` or the Mac OS X keychain.
Using your e-number (with lowercase 'e') and API key from the step before, when prompted about
your password then copy the API key from the clipboard:

    $ docker login -u eNNNNN safedocs-ta2-docker.cse.sri.com
    Password: <copy from clipboard>
    Login Succeeded

From now on, you will be able to push tagged images like so:

TODO: managing the version number?

    $ make deploy    
or

    $ docker tag pdf_printer safedocs-ta2-docker.cse.sri.com/pdf_printer:latest
    $ docker push safedocs-ta2-docker.cse.sri.com/pdf_printer:latest

To inspect the tags of the Docker image, do:

    $ 

# Running container and checking things...

    $ cd etc/docker  # if in top-level directory

Interactive shell:

    $ docker run -it safedocs-ta2-docker.cse.sri.com/pdf_printer /bin/sh
    / # ls -la /pdf_printer
    -rwxr-xr-x    1 root     root       3080320 Nov 14 15:03 /pdf_printer
    / # exit

With bind mounting a directory containing PDF example files to test:

    $ docker run -v <PATH_TO>/examples:/examples -it safedocs-ta2-docker.cse.sri.com/pdf_printer /bin/sh
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

Or, for example, the latest Eval One data set (downloaded and extracted locally to `~/tmp/2020-03-eval`):

    $ docker run -v ~/tmp/2020-03-eval:/2020-03-eval -it safedocs-ta2-docker.cse.sri.com/pdf_printer /bin/sh
    # /pdf_printer /2020-03-eval/0011_0000d9d2b298630800d403f88ad148fac1308ef8b0bb4de228947e0426dc28e2.pdf
    ...

# Obtaining the Image from Artifactory

You must be logged into [Artifactory](https://artifactory.sri.com) per instructions above.  Then:

    $ docker pull safedocs-ta2-docker.cse.sri.com/pdf_printer
    
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
