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
$ pioneerctl power on
(The reciever turns on)

$ pioneerctl --ip-address "192.168.1.3:8102" volume up
(Connect to the reciever at the specified address, and increase the volume by one step)

$ pioneerctl --zone zone2 mute on
(Mute only zone2)
```

If an ip-address isn't specified in the command, a configuration file must be used.
It is expected to be located at `$XDG_CONFIG_HOME/pioneerctl/config.toml`,
but this can be overridden with the `--config` flag.

### Building

This is still very much in alpha, and I haven't developed a release plan yet.
But since it's a rust project, in theory it's as simple as

```
git clone https://github.com/NomisIV/pioneerctl
cd pioneerctl
cargo build --release
sudo cp target/release/pioneerctl /bin/pioneerctl
```

However, this requires you to have a working cargo environment.

In the future I will probably publish binaries with each update.

## Contributing

Contributions are very welcome,
and I recommend taking a look at
[this official specification for the protocol](https://github.com/NomisIV/pioneerctl/blob/master/Pioneer_AVR_FY16_CIAMX.xlsx)
to get started.
