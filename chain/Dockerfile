FROM rust:1.53.0 

# create a new empty shell project
 
WORKDIR coffe
RUN  apt update &&  apt install -y git clang curl libssl-dev llvm libudev-dev
RUN rustup default stable
RUN rustup update
RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

 

# copy over your manifests
COPY . ./
 
 
# build for release 
RUN   cargo build  


 
  

# set the startup command to run your binary
CMD ["./target/debug/enterprise-sample --dev --tmp --unsafe-ws-external --unsafe-rpc-external"]