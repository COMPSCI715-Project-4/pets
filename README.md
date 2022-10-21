# gowalkies server
The backend server for GoWalkies project.

## How to run the project?
> You must make sure your local machine can be accessed by other devices through network.
1. Install the Docker

    See https://docs.docker.com/engine/install/
1. Use docker to run the project locally
    1. Boot the MongoDB container
        ```bash
        docker run -d --name mongo -p 27017:27017 gowalkies
        ```
    2. Build Docker file
        ```bash
        docker build --no-cache=false -t gowalkies .
        ```
    3. Boot the server container
        ```bash
        docker run -d --name gowalkies -p 8080:8080 gowalkies
        ```
