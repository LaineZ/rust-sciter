#![allow(non_camel_case_types, non_snake_case)]


/* Macros */

#[macro_use] extern crate lazy_static;
#[macro_use] mod utf;


/* C interface headers */
mod scapi;
mod scbehavior;
mod scdef;
mod scdom;
mod scgraphics;
mod screquest;
mod sctiscript;
mod sctypes;
mod scvalue;


/* Rust interface */
pub mod value;
// pub use self::value::{Value};



/* Loader */

pub use self::scapi::{ISciterAPI};

#[cfg(all(windows, target_arch="x86"))]
mod ext {
	#[link(name="sciter32")]
	extern "stdcall" { pub fn SciterAPI() -> *const ::scapi::ISciterAPI;	}
}

#[cfg(all(windows, target_arch="x86_64"))]
mod ext {
	#[link(name="sciter64")]
	extern "stdcall" { pub fn SciterAPI() -> *const ::scapi::ISciterAPI;	}
}


pub fn SciterAPI<'a>() -> &'a ::scapi::ISciterAPI {
	let ap = unsafe {
		let p = ext::SciterAPI();
		&*p
	};
	return ap;
}


lazy_static! {
	static ref _API: &'static ISciterAPI = { SciterAPI() };
}
