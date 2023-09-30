```
 █████  █████ █████                        ███████████              █████            
░░███  ░░███ ░░███                        ░░███░░░░░███            ░░███             
 ░███   ░███  ░███████   ██████  ████████  ░███    ░███ █████ ████ ███████    ██████ 
 ░███   ░███  ░███░░███ ███░░███░░███░░███ ░██████████ ░░███ ░███ ░░░███░    ███░░███
 ░███   ░███  ░███ ░███░███████  ░███ ░░░  ░███░░░░░███ ░███ ░███   ░███    ░███████ 
 ░███   ░███  ░███ ░███░███░░░   ░███      ░███    ░███ ░███ ░███   ░███ ███░███░░░  
 ░░████████   ████████ ░░██████  █████     ███████████  ░░███████   ░░█████ ░░██████ 
  ░░░░░░░░   ░░░░░░░░   ░░░░░░  ░░░░░     ░░░░░░░░░░░    ░░░░░███    ░░░░░   ░░░░░░  
                                                         ███ ░███                    
                                                        ░░██████                     
                                                         ░░░░░░                      
```

> Bit manipulation for dummies

[![Rust](https://github.com/dejanfajfar/uberbyte.rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/dejanfajfar/uberbyte.rs/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/uberbyte?style=flat-square)](https://crates.io/crates/uberbyte) [![docs.rs](https://img.shields.io/docsrs/uberbyte?style=flat-square&label=Documentation)](https://docs.rs/uberbyte/0.5.0/uberbyte/)


# Backstory

When working with a hardware interface I found it always _tedious_ to work with individual bits. 
Checking if one or the other is set and setting them in return to communicate with the hardware.

So to make my life easier and add some convenience to the whole matter I made this.

# Getting started

Add __UberByte__ to your project with:

```shell
cargo add uberbyte 
```

# Features

- Easily determine the state of each bit in the byte
- Easily change the sate of each bit in the byte
- Flip all bits in the byte
- Provide constant bit masks for each bit
- Provide OR, XOR, AND operations

# Usage

Check out the [Examples](https://github.com/dejanfajfar/uberbyte.rs/tree/main/examples) for some basic usage scenarios.

# Contributing

If you want to contribute you can do this in many ways

## Fork the repository

If you would like to introduce some changes to the project feel free to fork the project and do your changes.

It would then be nice if you create a pull request to reintegrate the changes you made. 

## Create a ticket

Found a bug? Missing a feature? 

Create a ticket at our (github issues)[https://github.com/dejanfajfar/uberbyte.rs/issues].

## Start a discussion

Any feedback that you do not want to put into the above form can just be dumped int our (github discussion page)[https://github.com/dejanfajfar/uberbyte.rs/discussions]

# License

MIT

A great overview of the license is given at (tldrlegal.com)[https://www.tldrlegal.com/license/mit-license]

The license text can be found at (LICENSE)[https://github.com/dejanfajfar/uberbyte.rs/blob/main/LICENSE]
