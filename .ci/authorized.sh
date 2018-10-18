#!/usr/bin/env bash

set -e
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
docker pull "$DOCKER_USERNAME"/frc:latest

# nightly cache busting
# extract the date of the latest image and comparing it to today
if [ "$(docker inspect -f '{{ .Created }}' $DOCKER_USERNAME/frc:latest | cut -d - -f3 | cut -d T -f1)" = "$(date +'%d')" ]; then
    echo "Using cached docker image"
    docker build -t "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT" -t "$DOCKER_USERNAME"/frc:latest . --cache-from "$DOCKER_USERNAME"/frc:latest
else
    echo "Rebuilding docker cache"
    docker build -t "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT" -t "$DOCKER_USERNAME"/frc:latest . --no-cache
fi

docker build -t "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT" -t "$DOCKER_USERNAME"/frc:latest . --cache-from "$DOCKER_USERNAME"/frc:latest
docker run -it -e CRATESIO_TOKEN -e TRAVIS_TAG "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT" sh -c "cd /first-rust-competition; make ci"
docker images
docker push "$DOCKER_USERNAME"/frc:"$TRAVIS_COMMIT"
docker push "$DOCKER_USERNAME"/frc:latest
