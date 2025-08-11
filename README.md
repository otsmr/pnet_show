# pnet_show

`pnet_show` is a `proc-macro` crate, that derives a `.show()` function for [pnet_packet](https://docs.rs/pnet_packet/latest/pnet_packet/)'s to display the struct similar to scapy.

First define your struct and add the derive `Show`.

```rs
#[packet]
#[derive(Show)]
pub struct Tether {
    version: u8,
    unknown0: u8,
    tether_type: u8,
    unknown1: u8,
    length: u16be,
    unknown2: u16be,
    unknown3: u32be,
    crc32: u32be,
    options: u16be,
    function_id: u16be,
    #[payload]
    payload: Vec<u8>,
}
```
Calling now `.show()` will then return a string displaying the struct including the values as follow:

```plain
##[ Tether ]##
 version: 1
 unknown0: 0
 tether_type: 5
 unknown1: 0
 length: 8
 unknown2: 0
 unknown3: 12
 cr032: 1035144919
 options: 257
 function_id: 1601
```