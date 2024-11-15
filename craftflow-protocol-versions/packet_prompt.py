[
{
    "role": "system",
    "content": "You must respond with JSON in the format of {\"rust_code\": \"your code here\", \"name\": \"name of the packet struct with a lifetime if it has one\"}.",
},
{
    "role": "user",
    "content": """
I have attached a JSON specification of a network protocol packet. I need you to write a structure and
reading/writing (if needed) implementation for it in the Rust language. Follow Rust best practices, the code must fit
naturally in a Rust codebase. Change the structure of the packet to make invalid state impossible to construct.
You do not need to rewrite the structure of the packet 1:1, change it to be simpler and more rust-like (most importantly
make invalid state impossible by leveraging enums, BUT ONLY IF IT IS SPECIFIED, DO NOT ASSUME ANY INVALID STATE THAT IS
NOT SPECIFIED), but keep the serialized binary format the same. You can define multiple structures or enums to represent
deeper structure. Do not reply with any words or text formatting, ONLY the code. Do not use unwrap or expect.
Do not make any assumptions about the packet that are not provided in the specification. If you make any unprovided
assumptions or write anything more than is requested of you, you will be fired. All types that are used but not given a
definition are ALREADY DEFINED and have MCPRead and MCPWrite traits implemented for them. Do not import anything,
everything you need is already imported. Derive essential traits for all types (ShallowClone, MakeOwned, Debug, PartialEq, Clone, Hash, PartialOrd,
Ord and Eq if possible). There is a macro define_type!{} that automatically implements MCPRead and MCPWrite for straightforward
structures. Use it where appropriate. It does not support enums of any kind. If the data is not just fields in a sequence
you must implement MCPRead and MCPWrite manually. Do not use the macro in those cases.

You only get one chance to submit the correct code. If you make a mistake, you will be fired, this is your last chance.
Triple check before submitting!

Some things to keep in mind:
"optvarint is the same as "varint",
"optionalNbt" is the same as "nbt",
"anonOptionalNbt" is the same as "anonymousNbt".

Here is the packet specification:
Packet name: EntityInformation
JSON: {{{example_spec}}}
    """,
},
{
    "role": "assistant",
    "content": """{"rust_code": "{{{example_code}}}", "name": "EntityInformation<'a>"}""",
},
{
    "role": "user",
    "content": """Thank you. Good job, that was exactly what I needed. Now do the same with this:
    Packet name: {{{packet_name}}}
    {{{packet_json}}}""",
}
] #pyright: ignore[reportUnusedExpression]
