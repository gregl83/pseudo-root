# pseudo-root

Sudo make me root

## Problem Statement

Running programs as adminstrator, or root, using the `sudo` command can compromise system security.

## Intent

Demonstrate how a trojan horse can exploit "sudo all things" in order to capture an adminstrator password.

## Goal

Reinforce security intuition when executing a command as an administrator.

## Mitigation

Trojan horses are introduced into a systems via downloads that appear to be authentic. Taking precautions with downloads can greatly reduce the risk of trojan horse infection. Virus/Spyware protection software is designed to evaluate software for vulnerabilities. When downloading software from a known trusted source, it's important to verify that the download is exactly, to the bit, as the trusted source published. A quick checksum comparison uses cryptographic hashes to determine if the download is identical to what was published. This verifies that vulnerabilities haven't been introduced into the software, by third-parties, prior to running it on a machine. Treat all data/downloads as unsafe until proven otherwise.

Don't run as `root`. Executing any command using `sudo` gives that process elevated privileges on a given machine. It's too common to blindly accept requests from software to run as `administrator` or `root` without actually determining if they warrant broad access to a machine. When installing or running software, running a program with elevated privileges should always raise red flags. Don't run any untrusted or unverified software as `root` that you don't want to have total control over a machine.

## License

[MIT](LICENSE)
