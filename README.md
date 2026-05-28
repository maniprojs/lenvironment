# Lenvironment

A Linux environment manager for Windows, Mac and Linux

## Installation

Make sure you have Rust installed. If you don't, [install it](https://rust-lang.org/).

Also, you need to install [Docker](https://docker.com/). You could use either Docker Desktop (Windows, Mac, Linux) or Colima (Mac, Linux).

Run:

```shell
cargo install lenvironment
```

Then, you can use it like this:

```shell
lenv [subcommand]
```

## How to use it?

**NOTE:** [environment] means the name of your environment

Run `lenv create [NAME]` to create an environment. Like this:

```shell
lenv create develop
```

By default, it uses Ubuntu image from Docker (ubuntu:latest) but you can specify image to use (such as "alpine" or "archlinux") with --image flag. Also, you can mount a host directory to the container directory in this format: "{host dir}:{container dir}", (Example: "./ubuntu_mnt:/workspace") You can use these information like this:

```shell
lenv create develop --image ubuntu:26.04 --mount "./ubuntu_mnt:/mnt"
```

Run `lenv remove [environment]` to remove an environment

Run `lenv stop [environment]` to stop an environment

Run `lenv status [environment]` to show an status of an environment

Run `lenv start [environment]` to start an environment

Run `lenv restart [environment]` to restart an environment

Run `lenv list` to list the environments

Run `lenv enter [environment]` to enter an environment
