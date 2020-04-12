# tmock

tmock is a learning project in Rust. For this reason the project uses the 
minimal amount of dependencies required (proc-macro2, syn, and quote) being the
only external ones. The project is also split into multiple dependencies, per
the functionality.

## What tmock does

The basic functionality of tmock is mocking upload of torrents to trackers.

## Disclaimer

I do not encourage nor condone downloading of illegal materials. You must
respect the applicable law in your country. I cannot be held responsible for
illegal activities performed by your usage of this software.

tmock has been created as a learning project to understand the functionality of
bencode, torrent clients and trackers.

## How to use it

tmock requires a basic configuration file that contains minimum and maximum 
speeds to use when uploading. The file must be named `config.txt`, with the
following contents, the values are provided in bytes:

```
MIN_UPLOAD=1
MAX_UPLOAD=2
```

When running the program a directory `torrents` will be created if it doesn't
exist. From this directory all files with `.torrent` extensions will be loaded.

The directory is also observed, so if any files are added to it when the 
program is running they will be loaded automatically. If files are removed the
seeding for them will stop as well.

## Contained crates

As mentioned above the project has been created for learning purposes. For this
reason most of the functionality has been manually implemented. The features
have been split into individual crates.

The docs for the whole project can be read by running `cargo doc --open`.

### bencode

The bencode crate contains code for representation, encoding and decoding of
bencoded values and files. It provides traits to implement if the user of the
crate wants to decode these values in structs instead of enum values.

### bencode_derive

This crate contains derive macros for Encodable and Decodable and allows for
automatic implementation of traits in (bencode)[#bencode] crate.

### directories

directories is a tiny crate that only contains logic to create a directory if
it doesn't exist.

### http

Contains a simple http-only client, that only supports GET requests. It is used
to retrieve information from trackers, but should be able to handle other http
requests as well.

### mock

The meat and boney of the whole application, contains announcers and client for
notifying the tracker. Essential functionality looks something like the 
following:

    - When announcing starts a send start even is called
    - While announcing in the intervals specified by the server the new 
      uploaded value is sent to let the server know we've been uploaded (server
      does not check whether or not we really did, at least not by default)
    - If the seeding stops the provided drop implementation will send a stop 
      event to let the server know we stopped "seeding"

For simplicity of implementation only one client is provided (hardcoded):
Transmission 2.94. Changing the client requires a change in key, peer_id and
query.

### rand

Minimalistic random implementation, dependent on the OS, as it only supports
unix /dev/urandom for generating these values.

### runner

The runner application, putting the puzzle pieces together, using the mock and
torrent crates for the full functionality.

### sha1

A sha1 implementation in pure rust. It was implemented as per the pseudocode in
the wikipedia article.

### torrent

Basic models for torrent file handling, as well as a client that could be used
in implementing the full torrent client functionality, if it was completed.

### watcher

Bindings for inotify for linux system, as well as a receiver/sender 
implementation for file observation. This can be started on separate thread to
observe the events in a non-blocking manner.