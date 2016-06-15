Ceph Plugin
===========

A Rust plugin for Ceph that gathers metrics about the IO coming and going.
Since this plugin is in the IO path it is crucial that it be as light
weight as possible.  Make every effort to allocate on the stack.

Loading this code into Ceph:
  * Make sure the name of the .so file looks like libcls*.so.  Ceph will only load
classes that start with libcls
  * Add this file to /usr/lib/x86_64-linux-gnu/rados-classes
  * Change /etc/ceph/ceph.conf: debug osd = 20/20
  * Reload the osd and check the logs for cls_ sections describing the loading
process
