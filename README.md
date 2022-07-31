# figure-second

A different approach to rendering figures in Inkscape: a focus on parsing 
xml instead of using inkscape extensions

## Blocked

Currently awaiting a way to store all unknown fields in a struct so that they
can be re-written out to the output file once changed.

This can be done with `serde_json::Value` and `#[serde(flatten)]`.

Tracking issues are (i think):

https://github.com/tafia/quick-xml/issues/320

https://github.com/tafia/quick-xml/issues/326

Then, with regards to namespacing issues, currently blocked on this:

https://github.com/tafia/quick-xml/issues/347

Alternatively, *every single* inkscape svg element and parameter can be parsed into a struct, and they 
can be outputted this way. Im not sure if that is worth the effort, though.
