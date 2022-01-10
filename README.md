# pioneerctl

This is a command line utility to control older pioneer A/V recievers over the home network.
The only officially supported recievers so far is the VSX-923, only because it's the only one that is tested.

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

Currently only the five most used commands are implemented:

- Power
- Volume
- Mute
- Input
- Listening mode

The goal is to implement almost everything, but it will be taken in small steps.

### Usage

The syntax is quite self explanatory,
and thanks to structopt, the `--help`-pages are basically all the documentation you'll ever want.
The binary includes support for generating shell completions,
but due to cargo limitations they cannot be installed automatically.

Here are some examples:

```
$ PIONEERCTL_ADDRESS="192.168.1.3:8102" pioneerctl power on
(The reciever turns on)

$ pioneerctl --ip "192.168.1.3:8102" volume up
(Connect to the reciever at the specified address, and increase the volume by one step)

$ pioneerctl --zone zone2 mute on
(Mute only zone2)

# Assuming PIONEERCTL_ADDRESS is already an exported variable
$ pioneerctl
pioneerctl $
(Enter REPL mode)
```

The IP address of the reciever must either be supplied with the `--ip` flag,
or via environment the environment variable `PIONEERCTL_ADDRESS`.

If no command is supplied pioneerctl will enter REPL mode.
In there you can run multiple commands after each other without setting up a new connection with each command.

### Installing / Building

If you have a working cargo environment it's as simple as

```
$ cargo install pioneerctl
```

But if you don't, there is a precompiled binary in the GitHub release.

## Contributing

Contributions are very welcome,
and I recommend taking a look at
[this official specification for the protocol](https://github.com/NomisIV/pioneerctl/blob/master/Pioneer_AVR_FY16_CIAMX.xlsx)
to get started.
