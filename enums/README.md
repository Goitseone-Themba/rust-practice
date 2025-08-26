#Enums

Unlike structs which are used to group related fields under one name, 
enums/enumerations are used to define possible values for a type.

e.g. an ip address can either be v4 or v6, we can an `IpAddrKind` enum and list 
the possible kinds of ip addresses.
```rust
enum IpAddrKind {
    v4,
    v6,
}
```
IpAddrKind is now a custom data type than we can use elsewhere in our code.
