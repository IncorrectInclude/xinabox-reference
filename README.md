# Quick reference for Xinabox Sensors

## What is this
This is a [rust](https://www.rust-lang.org/) project
that just gives a quick reference for xinabox sensors.  
**Run at your own risk**.  
None of the libraries are hand-checked
by me, but they are generally trusted, but still, stuff happens.  
**You have been warned**

## What it does
Just gives brief descriptions and relevant webpages
for various xinabox parts. It is certainly not all 
inclusive, but any additional entries are welcome. 

## How to use it
It is nothing fancy, just a CLI, but I find it handy, and 
if someone else does as well, or wants to port it to something
a little more fancy like JS, be my guest.

## Installation
1. Build the project 
```
    cargo build --release
```
2. Install the project (ensure your terminal is somewhere in this project tree)
```
cargo install --path .
```

Alternatively you can very painfully write out
```
 cargo run -- [PROGRAM_FLAGS]
```

Where `[PROGRAM_FLAGS]` are the flags that cna
be passed that can be passed to the executable.

## Usage
On the command line, commands look like so
```
xinabox [FLAGS]
```
```
X-In-A-Box Chip Reference 
Andy Day
A quick way to see xinabox stats

USAGE:
    xinabox.exe [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
        --list       Lists the (currently registered) xinabox chips
    -V, --version    Prints version information

OPTIONS:
    -c, --chip <CHIP_NAME>    lists data about the given chip

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list    lists the various chips
```

## Sample Output
the command:
```
xinabox list
```
provides: 
```
┏━━━━━━━━━━━┳━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ SI01      ┃ Triple Axis Accelerometer, Magnetometer and Gyroscope   ┃
┣━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ SL01      ┃ Measures luminosity(visual brightness) and UV radiation ┃
┣━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ SW01      ┃ Digital (I2C) Humidity, Pressure and Temperature Sensor ┃
┣━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ SN01      ┃ GPS Based location, time, date, and location helpers.   ┃
┣━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ SG33      ┃ Digital gas sensor (measures CO2)                       ┃
┗━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```