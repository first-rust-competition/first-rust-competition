#!/usr/bin/env python3

# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

from subprocess import Popen, call, PIPE, STDOUT
from re import search
from os import getenv


def main():
    regex = '(?:#include ["<]\.\.\.[">] search starts here:\n){2}((?:(?:.*?)\n)*)(?:End of search list\.)'

    compiler = getenv("CXX_FRC", 'arm-frc2019-linux-gnueabi-g++')
    # print compiler

    p = Popen([compiler, '-E', '-Wp,-v', '-'], stdin=PIPE,
              stdout=PIPE, stderr=PIPE, shell=False)
    stdout, stderr = p.communicate(b"\n")
    rc = p.returncode
    include_dirs = search(regex, stderr).group(1).strip().split('\n')

    for x in include_dirs:
        x = x.strip()
        #call('cp -R {0}/* {1}'.format(x, "./include"), shell=True)
        print(x)


if __name__ == "__main__":
    main()
