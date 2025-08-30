FROM gitpod/workspace-full:latest
RUN sudo apt-get update && sudo apt-get install -y build-essential clang cmake curl binaryen
