#!/usr/bin/env bash

docker build -t frc:latest .
docker run -it frc:latest sh -c "cd /first-rust-competition; make all"
docker images
