#!/bin/sh

set -euof

install_deps() {
    sudo yum install -y git docker make tar perl-Digest-SHA libicu
}

configure_docker() {
    sudo systemctl enable docker --now
    sudo usermod -aG docker ec2-user
}

build_goloop_images() {
  git clone https://github.com/icon-project/goloop.git
  cd goloop
  make goloop-image
  make goloop-icon-image
  docker tag goloop iconloop/goloop-icon
  rm -rf ../goloop
}

pull_archway_images() {
  docker pull ghcr.io/archway-network/archwayd:v4.0.0
}

build_services() {
  build_goloop_images
  build_archway_images
}

install_deps
configure_docker
build_services