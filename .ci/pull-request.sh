#!/usr/bin/env bash
set -e
docker build -t frc:latest .
docker run -it frc:latest sh -c "cd /first-rust-competition; make ci"
docker images
