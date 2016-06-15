Ceph Plugin
===========

A Rust plugin for Ceph that gathers metrics about the IO coming and going.
Since this plugin is in the IO path it is crucial that it be as light
weight as possible.  Make every effort to allocate on the stack.
