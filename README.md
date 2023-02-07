# Mandatory assignment 1 Operating Systems
This repository contains the mandatory assignment 1 for the course [IDATA2305 - Operating Systems with System Programming](https://www.ntnu.edu/studies/courses/IDATA2305#tab=omEmnet) at NTNU.  

## Assignment
Simple single and multi-threaded web server. We chose to solve the assignment in rust.  

## How to run
You can either compile the code yourself or use the precompiled binaries in the [releases](https://github.com/JakobHolkestadMolnes/OSgroup6ma1/releases).  
In order to compile you will need [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.  

In order to run the server, run the binary with an argument. Either `single`, or `multi`, to run single threaded or multi-threaded server respectively.   
`$ ./osgroup6ma1 [single|multi]`  

## Our observations
The single threaded server cannot handle multiple requests at the same time. If we refresh the page quickly multiple times, we need to wait for all the previous requests to finish processing before we get the response.  
The multi threaded server can handle multiple requests at the same time. If we refresh the page quickly multiple times, we get the response immediately after "heavy computation" finishes on that request.
