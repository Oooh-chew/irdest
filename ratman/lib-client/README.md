Ratman client bindings library

To learn more about Ratman and Irdest, visit https://irde.st!

In order to interact with the Ratman daemon your application must
send properly formatted API messages over a local TCP socket.
These data formats are outlined in [ratman-types](ratman_types)!

This crate provides a simple API over these API messages!

**This API is very unstable!** You may expect _slightly_ more
stability guarantees from `ratcat(1)` and `ratctl(1)`

## Version numbers

The client library MAJOR and MINOR version follow a particular
Ratman release.  So for example, version `0.3.0` of this crate is
built against version `0.3.0` of `ratmand`.  Because Ratman itself
follows semantic versioning, this crate is in turn also
semantically versioned.

Any change that needs to be applied to this library that does not
impact `ratmand` or the stability of this API will be implemented
as a patch-version.

Also: by default this library will refuse to connect to a running
`ratmand` that does not match the libraries version number.  This
behaviour can be disabled via the `RatmanIpc` API.

## Dependencies

**Important**: to use this library you need `protobuf` installed.
Also rust `1.56.0` is required.
