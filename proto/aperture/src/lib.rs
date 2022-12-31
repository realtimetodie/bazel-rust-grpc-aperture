//! Protocol Buffer definitions.
//!
//! This file needs to be maintained manually.
//! The Protocol Buffer files are auto-generated by Prost.

/// Package aperture.helloworld
pub mod hello_world {
    include!(concat!(env!("OUT_DIR"), "/aperture.helloworld.rs"));
}

/// Package aperture.demo
pub mod demo {
    include!(concat!(env!("OUT_DIR"), "/aperture.demo.rs"));

    /// Package aperture.demo.a
    pub mod a {
        include!(concat!(env!("OUT_DIR"), "/aperture.demo.a.rs"));

        /// Package aperture.demo.a.b
        pub mod b {
            include!(concat!(env!("OUT_DIR"), "/aperture.demo.a.b.rs"));

            /// Package aperture.demo.a.b.c
            pub mod c {
                include!(concat!(env!("OUT_DIR"), "/aperture.demo.a.b.c.rs"));
            }
        }
    }
}