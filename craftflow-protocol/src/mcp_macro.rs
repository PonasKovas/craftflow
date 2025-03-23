/// Automatically implements MCPRead, MCPWrite traits
/// Requires that all fields implement MCPRead, MCPWrite
macro_rules! mcp {
	(
        $(#[$attr:meta])*
        pub struct $name:ident $({
            $(
                $(#[$field_attr:meta])*
                // bruh let me tell you. i should just be able to write this:
                // pub $field_name:ident: $field_type:ty
                // and then map it to
                // pub $field_name: <$field_type as MCP>::Data,
                // and it DOES work. except that rustdoc pisses its pants https://internals.rust-lang.org/t/rustdoc-resolve-actual-types-in-item-definitions/22626
                // and rust-analyzer becomes unusable too. average rust experience.
                // now i gotta do this slop (cant even use the `ty` metavar because then i cant match it anymore because of FUCKING MACRO HYGIENE or something)
                pub $field_name:ident: ($($field_type:tt)*)
            ),* $(,)?
        })? $(;)?
    ) => {
        $(#[$attr])*
        pub struct $name
        $(
            {
                $(
                $(#[$field_attr])*
                pub $field_name: mcp_map_type!($($field_type)*),
                )*
            }
            // funny hack to make the semicolon work when struct has fields (you cant normally type ; after {})
            const _:() = ()
        )?;

        impl MCP for $name {
            type Data = Self;
        }
        impl<'read> MCPRead<'read> for $name {
        	fn mcp_read(#[allow(unused_variables)] input: &mut &'read [u8]) -> Result<Self> {
                $($(
                    #[allow(unused_parens)]
                    let $field_name = <$($field_type)*>::mcp_read(input)?;
                )*)?

        		Ok(Self $({
       				$(
                        $field_name,
                    )*
        		})?)
        	}
        }

        impl MCPWrite for $name {
            #[allow(unused_parens)]
            fn mcp_write(#[allow(unused_variables)] data: &Self, #[allow(unused_variables)] output: &mut Vec<u8>) -> usize {
                #[allow(unused_mut)]
                let mut written_bytes = 0;

                $($(
                    written_bytes += <$($field_type)*>::mcp_write(&data.$field_name, output);
                )*)?

                written_bytes
            }
        }
    };
}

//   _____ _      ____  _____             _      ______ _____ _______
//  / ____| |    / __ \|  __ \      /\   | |    |  ____|  __ \__   __|
// | (___ | |   | |  | | |__) |    /  \  | |    | |__  | |__) | | |
//  \___ \| |   | |  | |  ___/    / /\ \ | |    |  __| |  _  /  | |
//  ____) | |___| |__| | |       / ____ \| |____| |____| | \ \  | |
// |_____/|______\____/|_|      /_/    \_\______|______|_|  \_\ |_|
//
// 🚨🚨🚨 SLOP ALERT 🚨🚨🚨
//
// DO NOT BE ALARMED. THIS CODE CONTAINS MAXIMUM-RUST SLOP LEVELS.
// WE ARE CURRENTLY OPERATING AT ***DEFCON 1: TOOLING MELTDOWN***.
// STAY CALM AND FOLLOW EMERGENCY PROTOCOLS:
//
// 1️⃣ **rustdoc is NOT resolving associated types properly** - Expect completely useless documentation.
// 2️⃣ **rust-analyzer is TAKING A FAT L** - Expect red squiggles, phantom errors, and ghost intellisense.
// 3️⃣ **Macros were supposed to save us, but now they only bring suffering.**
// 4️⃣ **Explicit type mapping is the last resort.**
// 5️⃣ **The trait system is theoretically sound but practically catastrophic.**
//
// 🛑 ATTEMPTING TO "FIX" THIS WITH STANDARD TOOLING WILL ONLY MAKE IT WORSE.
// 💀 ALL HOPE IS LOST. ACCEPT THE SLOP.
// 🔥 WE RIDE THIS TRAIN STRAIGHT INTO THE VOID. NO BRAKES. 🔥
//
// -- Signed, a once-hopeful Rust enjoyer, now broken beyond repair.
macro_rules! mcp_map_type {
    (VarInt) => { i32 };
    (OptVarInt) => { Option<i32> };
    (VarLong) => { i64 };
    (Nbt) => { craftflow_nbt::NbtValue };
    (NamedNbt) => { craftflow_nbt::NbtValue };
    (OptNbt) => { craftflow_nbt::NbtValue };
    (OptNamedNbt) => { craftflow_nbt::NbtValue };
    (Nbt<($($generic:tt)*)>) => { $($generic)* };
    (NamedNbt<($($generic:tt)*)>) => { $($generic)* };
    (OptNbt<($($generic:tt)*)>) => { $($generic)* };
    (OptNamedNbt<($($generic:tt)*)>) => { $($generic)* };
    (Array<($($generic1:tt)*) $(, ($($generic2:tt)*) )?>) => { Vec<mcp_map_type!( $($generic1)* )> };
    (Buffer $(<($($generic:tt)*)>)?) => { Vec<u8> };
    (RestBuffer) => { Vec<u8> };
    (Option<($($generic:tt)*)>) => { Option<mcp_map_type!( $($generic)* )> };
    ($($any:tt)*) => { $($any)* };
}
