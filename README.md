# mel (Matter-Eater Lad)

<img src="images/matter-eater.gif" width="100" >


Mel is a small Rust binary that carries a payload of tools... beyond that not sure what I want it to do yet or if I'll throw it into the campfire when I'm done.

The reason for this little tool is first so I can learn some Rust. The second reason is in the infrastructure automation space there's a medium-sized toolbox worth of tools needed to stand up infrastructure and test it.  And if the tools are not the correct version that just makes things break or act in odd ways. Mel will embed with all the needed tools inside of it that contain the exact versions required. I'll be making use of this Rust Crate https://crates.io/crates/rust-embed.  This approach also protects against a file artifact system from being down. This last reasons I'm kinda stretching things as I'm not sure that is a reason for doing this. I mean this isn't running in an air-gapped context where the network shouldn't be available. A future version of Mel may just have the option to just download the wanted tools from a endpont so that Mel stays lean. But I like the idea and I want to use some Rust for a project that could be useful so I'll see where it goes.
