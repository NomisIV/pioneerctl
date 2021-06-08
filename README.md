# pioneerctl

This is a command line utility to control older pioneer A/V recievers over the home network.
The only officially supported recievers so far is the VSX-923, only because it's the only one I own.

This project is inspired by [mkulesh/onpc](https://github.com/mkulesh/onpc),
and in theory, pioneerctl should be able to support the pioneer models that onpc doesn't.

This would be the following:

- VSX-529
- VSX-830
- VSX-923
- VSX-924 (This might be a typo from mkulesh)
- VSX-1021
- VSX-1121
- SC-95
- N-50
- N-50a
- N-70A

If you have an old pioneer reciever, feel free to test pioneerctl and return with your results :)

## Documentation

Currently only the four most used commands are implemented:

- Power
- Volume
- Mute
- Input

The goal is to implement almost everything, but it will be taken in small steps.

### Usage

The syntax is quite self explanatory,
and thanks to structopt, the `--help`-pages are basically all the documentation you'll ever want.

Here are some examples:

```
$ PIONEERCTL_ADDRESS="192.168.1.3:8102" pioneerctl power on
(The reciever turns on)

$ pioneerctl --ip-address "192.168.1.3:8102" volume up
(Connect to the reciever at the specified address, and increase the volume by one step)

$ pioneerctl --zone zone2 mute on
(Mute only zone2)

# Assuming PIONEERCTL_ADDRESS is already an exported variable
$ pioneerctl
pioneerctl $
(Enter REPL mode)
```

The IP address of the reciever must either be supplied with the `--ip-address` flag,
or via environment the environment variable `PIONEERCTL_ADDRESS`.
Note that the environment variable can be exported in your shell's profile,
to not have to set it twice.

If no command is supplied pioneerctl will enter REPL mode.
In there you can run multiple commands after each other without setting up a new connection with each command.

### Building

This is still very much in alpha, and I haven't released this to [crates.io](https://crates.io) yet.
But you can always build it yourself, like this:

```
git clone https://github.com/NomisIV/pioneerctl
cd pioneerctl
cargo build --release
sudo cp target/release/pioneerctl /bin/pioneerctl
```

However, this requires you to have a working cargo environment.

In the future I will publish binaries with each release on GitHub,
in addition to uploading it as a crate to [crates.io](https://crates.io)

## Contributing

Contributions are very welcome,
and I recommend taking a look at
[this official specification for the protocol](https://github.com/NomisIV/pioneerctl/blob/master/Pioneer_AVR_FY16_CIAMX.xlsx)
to get started.
