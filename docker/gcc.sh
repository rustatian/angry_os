#!/bin/bash

GCC_VERSION=$1

export PREFIX="/usr/local"
export TARGET=x86_64-elf
export PATH="$PREFIX/bin:$PATH"

cd $PREFIX/src
mkdir build-gcc
cd build-gcc
../gcc-${GCC_VERSION}/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make -j 24 all-gcc
make -j 24 all-target-libgcc
make install-gcc
make install-target-libgcc

cd $PREFIX/src
rm -rf build-gcc.sh build-gcc gcc-${GCC_VERSION}