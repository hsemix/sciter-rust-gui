#![windows_subsystem="windows"]
#![allow(unused_variables)]
#![allow(non_snake_case)]

#[macro_use]
extern crate sciter;

use sciter::{ HELEMENT, Element, Value };
mod mods;

struct EventHandler {
	root: Option<Element>,
}

impl EventHandler {
	fn calc_sum(&mut self, a: i32, b: i32) -> i32 {
		a + b
    }
    
    fn getUsers(&mut self) -> String {
        mods::Lib::methods::get_users()
    }
}

impl sciter::EventHandler for EventHandler {

	fn attached(&mut self, root: HELEMENT) {
		self.root = Some(Element::from(root));
	}

	dispatch_script_call! {
        fn calc_sum(i32, i32);
        fn getUsers();
	}

	fn on_script_call(&mut self, root: HELEMENT, name: &str, argv: &[Value]) -> Option<Value> {

		let args = argv.iter().map(|x| format!("{:?}", &x)).collect::<Vec<String>>().join(", ");
		let handled = self.dispatch_script_call(root, name, argv);
		if handled.is_some() {
			return handled;
		}

		None
	}
}

fn main() {

    let file = "C:\\Users\\hsemi\\Desktop\\rust\\structs\\with_sciter_mysql\\resources\\index.htm";
    // let resources = include_bytes!("all_users.rc");
    let handler = EventHandler { root: None };
    let mut frame = sciter::Window::new();
    // frame.archive_handler(resources).expect("Invalid archive");
    frame.event_handler(handler);
    frame.load_file(file);
    // frame.load_file("this://app/index.htm");
    frame.run_app();

    // println!("{}", mods::Lib::methods::get_users());
}