# Zuri
The original idea behind zuri is to create a minecraft-compatible client in rust that is able to connect to any
**Minecraft: Bedrock Edition** server and that works across macOS, Windows and Linux. To accomplish this, another goal
is to maintain high-quality crates related to MCBE which can also be found in this repository.

Zuri has a discord server: [https://discord.gg/nCcxasYzbX](). Feel free to come ask questions!

This project is in no way affiliated with Minecraft, Mojang Studios or Microsoft.

## Crates
Zuri is split up in multiple crates to allow parts to be used separately in other projects.

Here is a non-exhaustive list of the most interesting crates. None of these crates are currently available on 
[https://crates.io](). This is planned after a needed cleanup.

- [**zuri**](/zuri)<br/>
  A client for the bedrock edition of minecraft. Uses the bevy engine.

- [**zuri_nbt**](/zuri_nbt)<br/>
  Crate for serializing and deserializing NBT data. Includes optional serde support.

- [**zuri_net**](/zuri_net)<br/>
  Full protocol implementation that aims to be compatible with the latest Minecraft: Bedrock Edition release.

## Progress
While the basics are down, there is still a lot of work to do. There are some work-in-progress branches that show more 
work:

- **feature/listener**<br/>
  Implements a server listening alongside the server login sequence for zuri_net.

- **feature/player-move**<br/>
  Syncs the client's movement with the server.

## Usage
To use zuri, first clone the repository. Then, run `cargo run --release` to build and run zuri in release mode. 

By default, zuri will try to connect to a server running locally on port 19132 and without xbox authentication. This
can be configured through the `ZURI_IP` and `XBOX` environment variables respectively. Zuri supports setting environment
variables with a `.env` file like the following example:
```env
ZURI_IP="127.0.0.1:19132"
XBOX=false
```

## Notable mentions

- [**Sandertv/gophertunnel**](https://github.com/Sandertv/gophertunnel)<br/>
  The main reference used for the protocol.

- [**pmmp/BedrockProtocol**](https://github.com/pmmp/BedrockProtocol)<br/>
  Additional reference used for the protocol.
