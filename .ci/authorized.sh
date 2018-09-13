#!/usr/bin/env bash

echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
docker pull "$DOCKER_USERNAME"/frc:latest
docker build -t "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT" -t "$DOCKER_USERNAME"/frc:latest . --cache-from "$DOCKER_USERNAME"/frc:latest
docker run -it -e CRATESIO_TOKEN -e TRAVIS_TAG "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT" sh -c "cd /first-rust-competition; make all"
docker images
docker push "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT"
docker push "$DOCKER_USERNAME"/frc:latest
