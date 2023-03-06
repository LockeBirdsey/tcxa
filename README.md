# tcxa: TCX Anonymiser

# DEAD
Replaced by [GOTCXA](https://github.com/LockeBirdsey/gotcxa) because it was nicer and junk

## Why/What?

I got fed up with Fitbit serving broken TCX files when set to "Spinning" only to find out that "Bike" mode forced GPS mode on. Of course, on an exercise bike, the segments distance will sum to 0, which Strava will use for its distance calculation. 

So `tcxa` was born. Very simple: It scrubs away the long/lat from each `Trackpoint` in the exported Fitbit TCX and allows you to set a `DistanceMeters` for the `Lap`

Why `TCX Anonymiser`? Well it removes all the positional information effectively anonymising your location. (Of course it leaves timestamps with TZs but sshhhh).

## How does this work

Badly.

I wrote this after being away from Rust for about 2.5 months. It reads the input TCX and writes out to the output TCX at the same time. Not great but hey, it does what I need it to.

If you think the code is bad: it is. 

Will it be improved: probably very, very slowly.

# Usage
```
Usage: tcxa.exe [OPTIONS] --input-path <INPUT_PATH> --output-path <OUTPUT_PATH>

Options:
  -i, --input-path <INPUT_PATH>    Path to input file
  -o, --output-path <OUTPUT_PATH>  Path to output file
  -d, --distance <DISTANCE>        Manually inputted distance [default: 0]
  -s, --sum-distance               Calculate total distance
  -m, --mhr                        calculate mean heart rate
  -h, --help                       Print help information
  -V, --version                    Print version information
```

# Version history

### 0.1
* Minimal working version with key features I need
* Flags added for future tasks

# Help it doesn't work!!

Open an issue or find me on [aus.social](https://aus.social/@birlocke_)

# Please do X/Y/Z? It'd be ever so lovely

Sure, if you open an issue or a PR I might. Otherwise fork away.
