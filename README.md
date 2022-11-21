<!-- omit in TOC -->

# calife-rs

> **Lightweight alternative to sudo/doas**

[![Build status](https://github.com/keltia/calife-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/keltia/calife-rs/actions/workflows/rust.yml)
[![Buildstatus (develop)](https://github.com/keltia/calife-rs/actions/workflows/develop.yml/badge.svg)](https://github.com/keltia/calife-rs/actions/workflows/develop.yml)
[![Docs](https://img.shields.io/docsrs/dmarc-rs)](https://docs.rs/drone-utils)
[![GitHub release](https://img.shields.io/github/release/keltia/dmarc-rs.svg)](https://github.com/keltia/calife-rs/releases/)
[![GitHub issues](https://img.shields.io/github/issues/keltia/calife-rs.svg)](https://github.com/keltia/calife-rs/issues)
[![calife-rs: 1.56+]][Rust 1.56]
[![SemVer](https://img.shields.io/badge/semver-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
[![License](https://img.shields.io/crates/l/mit)](https://opensource.org/licenses/MIT)

Licensed under the [MIT](LICENSE).

1. [About](#about)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Compatibility](#compatibility)
5. [References](#references)
6. [MSRV](#msrv)
7. [History](#history)
8. [Contributing](#contributing)

## About

Calife is a small utility which enable some selected users (present in a configuration file) to become another user,
traditionally but not limited to "root" by entering their own password.

The primary goal is to avoid sharing a common password (à la su(1)) while enabling these users to use the power of the
superuser.

Entering a password is mandatory as to avoid the security risk associated with leaving an unattended terminal (like
sudo(1) has because it uses a timeout-based time window).

It pushes the security boundary down to the users' passwords but we will assume they have been trained for the various
administrative tasks they are allowed to achieve and the security risks associated with this mechanism.

Going through the "root" superuser privileges can be avoided by using the "list" feature as selected users will be able
to become another one directly.

Finally, the "group" feature allows users to be put in a specific group in order to avoid listing all of them in the
configuration file.

## Distribution

The [Rust] version of `calife` is under the MIT license and not the GPL as the original one. There is absolutely not
shared code between the two.

See the [LICENSE](LICENSE) file in the distribution.

There is no other conditions but if you use it, please tell me about it and I would never refuse a Guinness if we meet
one day :-)

Original web/distribution site for C Calife is [Calife].

Source code is available at: [Github repository]

## Installation

This is the [Rust] version so we'll follow the usual Rust steps.

You can also specify some special options to configure in order to get specific behaviours.

    --with-etcdir=PATH        Directory containing calife.auth (default /etc)
    --disable-shadow=yes|no   Disable getspnam(3) usage for some Linuxes
    --enable-debug=yes|no     Set the debugging mode (default is no)
    --enable-shellhack=yes|no (default is no)
    --enable-dynamic          Build the program dynamically (default static)
    --enable-global-rc        Add /etc/calife.out support.

`--with-etcdir=DIR`

Specify the directory where calife will look for `calife.auth` and `calife.out`. Default is `/etc`.

`--enable-global-rc`

Add `/etc/calife.out` support. This script will be run upon exit. It could be used to perform certain tasks like sending
a mail, checking changes and so forth.

## Usage

## Compatibility

Calife has been developed and tested on the following UNIX systems (actual versions may differ, but I can not guarantee
full support all time as I may not have access to all of them).

In the pam/ subdirectory, you can find some sample files for PAM configuration. Choose the one suited to your system and
pace it either inside `pam.conf` (Solaris up to 9) or within the `pam.d` directory under the `calife` name.

- `macosx`    For MacOS X 10.3/Panther and up to 10.5/Leopard
- `macosx10`  For MacOS X 10.6/Snow Leopard
- `freebsd5`  For FreeBSD 5.x up to 8.x.
- `freebsd9`  For FreeBSD 9.x and up.
- `solaris`   For Solaris 8, 9 et 10 (to be cut/pasted in pam.conf)
- `linux`

If you want to use LDAP/NIS or any other authentication system based on PAM, you will need to add to or modify the host
PAM configuration. Any input on these matter would be appreciated.

## MSRV

The Minimum Supported Rust Version is 1.56 due to the 2021 Edition.

## History

Calife was created in 1991 from a common idea between François Berjon and I when we were system administrators. At that
time, we wrote a simple wrapper that was checking uid and running a root shell, without asking for a password.

It evolved later when I was working at Antenne 2 (French TV channel) and during my term as an intern in the University
of Marne-la-Vallée, where it gained most of its features. NIS support was added later for Laurent Chemla and Brainstorm.

If you want to send any comment, critics or patches, please contact me with the following address or connect to the
following website:

E-mail: [adresse privée maison]
Web site: [Main site]

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for some simple rules.

I use Git Flow for this package so please use something similar or the usual github workflow.

1. Fork it ( https://github.com/keltia/dmarc-rs/fork )
2. Checkout the develop branch (`git checkout develop`)
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Commit your changes (`git commit -am 'Add some feature'`)
5. Push to the branch (`git push origin my-new-feature`)
6. Create a new Pull Request

[adresse privée maison]: mailto:roberto@keltia.net

[Calife]: https://www.keltia.net/programs/calife/
[Github repository]: https://github.com/keltia/calife-rs/
[Rust]: https://rust-lang.org/
[Rust 1.56]: https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html
[Main site]: https://www.keltia.net/programs/calife/
