# segment_dumper

### Installing

WIP

### Use

Just pass `segment_dumper` a path to a mach-o binary:

`segment_dumper /bin/bash`

it will print the `section,segment` pairs from the binary as so:

```
__TEXT,__text
__TEXT,__stubs
__TEXT,__const
__TEXT,__cstring
__TEXT,__unwind_info
__TEXT,__eh_frame
__DATA_CONST,__got
__DATA_CONST,__const
__DATA,__data
__DATA,__common
__DATA,__bss
__TEXT,__text
__TEXT,__auth_stubs
__TEXT,__const
__TEXT,__cstring
__TEXT,__unwind_info
__DATA_CONST,__auth_got
__DATA_CONST,__got
__DATA_CONST,__auth_ptr
__DATA_CONST,__const
__DATA,__data
__DATA,__common
__DATA,__bss
```

These are conveniently formatted for passing to `launchctl plist`.

### Contributing

This binary is really just a thin wrapper around the [goblin](https://crates.io/crates/goblin) crate to expose particular functionality, while contributions are welcome, that effort would be more useful contributing to goblin instead.
