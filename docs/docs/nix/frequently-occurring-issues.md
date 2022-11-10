# Frequently Occurring Issues

This document intends to record frequently occurring issues running our Nix setup and how to fix them.
Please note that this list is not exhaustive and only suggests some known fixes that may not apply to your issue.

## No space left on device

```
`LLVM ERROR: IO failure on output stream: No space left on device`
```

If you encounter a `No space left on device` error even though enough disk space is available,
we need to increase the maximum size of your `/tmp` directory.

You can achieve this one of two ways:

### 1. Temporary fix

To fix the issue temporarily, use the following command as `root` user.
```shell
$ mount -o remount,size=16G /tmp/
```

This command will remount the `/tmp` directory with an increased size of up to 16Gb.

### 2. Permanent fix

To fix this issue permanently, we need to edit the `/etc/fstab` file using a text editor of your choice.

Look for a line starting with `tmpfs` and update it like the following, 
or append it to the bottom if it doesn't exist yet.
```
tmpfs /tmp tmpfs rw,nodev,nosuid,size=16G 0 0
```
This change will permanently mount the `/tmp` directory with a fixed size of 16Gb.      

## Too many open files

If you encounter a `Too many open files` error, we need to increase this limit.

The easiest way is to modify the `run` command with the `ulimit` command. Like the following:
```sh
$ bash -c 'ulimit -n 10000; nix run github:composablefi/composable#devnet-dali`
```

## Nix run command does not respect `nix.conf` configuration.

If you encounter any of the error messages seen below, we may need to change the Nix configuration directory to 
`/etc/nix/nix.conf`, instead of `~/.config/nix/nix.conf`, see the [installation process](./install.md) for more details.

```
experimental Nix feature is disabled. 
```
```
derivation has '__noChroot' set, but that';s not allowed when 'sandbox' is 'true.
``` 
```
Nix run command does not respect `nix.conf` configuration.
```




## Can't re-install Nix after macOS update

To successfully re-install Nix after a macOS update, you need to remove the existing `/nix` directory,
and start the installer again.

The following command removes the `/nix` directory.
```shell
$ sudo rm -rf /nix 
```
