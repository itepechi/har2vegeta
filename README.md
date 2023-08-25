# har2vegeta – Convert HAR files to a Vegeta compatible format

> A barely realistic benchmark is still better than an unrealistic benchmark.
> – Nobody

## Install

```sh
cargo install --path .
```

## Usage

Export your HAR file from the Network tab of your browser's DevTools and run the
command as shown below.

I highly recommend isolating the source and target machine (which should be
yours, of course) from the public Internet before launching the attack, to
prevent connections from going out.

```sh
cat your_file.har | har2veg | vegeta attack ...
```

## Options

### **--format**, **-f**=_json_ | _http_ | _j_ | _h_

The format to output. The HTTP format is more human readable, but it can only
handle request bodies by adding a reference to real files. **har2veg** will
automatically export bodies to temporary files, but will not delete them after
running **Vegeta** because there's no realistic way to implement it. The JSON
format is the recommended format if you want to pipe directly to Vegeta
because it supports embedding base64 encoded bodies.

This option defaults to the HTTP format to be consistent with Vegeta's
default.

## License

[0BSD](https://spdx.org/licenses/0BSD.html)
