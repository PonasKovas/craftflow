<p align="center">
  <img src="https://raw.githubusercontent.com/PonasKovas/craftflow/refs/heads/master/assets/icon.png" width="256" />
</p>

<h1 align="center">CraftFlow-NBT</h1>

<p align="center">
<a href="https://docs.rs/craftflow-nbt/latest/craftflow_nbt/"><img alt="docs.rs" src="https://img.shields.io/docsrs/craftflow-nbt?style=flat"></a>
<a href="https://crates.io/crates/craftflow-nbt"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/craftflow-nbt?style=flat"></a>
</p>
<p align="center">A serde-based implementation of the Minecraft NBT binary format.</p>

## Serde deriving

- **Tagged enums not supported**
- **Untagged enums**: the default implementation of `Deserialize` derived by serde macros does some crazy stuff with types,
  so if you have multiple variants with differrent "flavors" of the same base type (integers: byte, short, int, long, or lists: list, byte array, int array, long array)
  serde will automatically convert it the value that is first defined in the enum. For example:
  ```rust
  #[derive(serde::Deserialize)]
  #[serde(untagged)]
  enum Example {
    First(u32),
    Second(u64),
  }
  ```
  Using the code above, even if you encounter a value that is encoded as a `Long` in NBT, serde will still always give you `Example::First`,
  except for when the number is big enough that it can't fit in a `u32`.
  This is very inconsistent and therefore if you need this functionality, consider implementing `Deserialize` manually. See the implementation of `DynNBT` for an example.
- **Option**: if serialized/deserialized standalone, will write/read a `Tag::End` byte. However, if inside a compound, will be skipped altogether.
