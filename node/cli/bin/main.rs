// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Stafi Node CLI

#![warn(missing_docs)]

use futures::channel::oneshot;
use futures::{future, FutureExt};
use substrate_cli::VersionInfo;

use std::cell::RefCell;

// handles ctrl-c
struct Exit;
impl substrate_cli::IntoExit for Exit {
	type Exit = future::Map<oneshot::Receiver<()>, fn(Result<(), oneshot::Canceled>) -> ()>;
	fn into_exit(self) -> Self::Exit {
		// can't use signal directly here because CtrlC takes only `Fn`.
		let (exit_send, exit) = oneshot::channel();

		let exit_send_cell = RefCell::new(Some(exit_send));
		ctrlc::set_handler(move || {
			if let Some(exit_send) = exit_send_cell.try_borrow_mut().expect("signal handler not reentrant; qed").take() {
				exit_send.send(()).expect("Error sending exit notification");
			}
		}).expect("Error setting Ctrl-C handler");

		exit.map(|_| ())
	}
}

fn main() -> Result<(), substrate_cli::error::Error> {
	let version = VersionInfo {
		name: "Stafi Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "stafi",
		author: "Stafi Protocol <technical@stafi.com>",
		description: "Generic stafi node",
		support_url: "https://github.com/stafiprotocol/stafi/issues/new",
	};

	node_cli::run(std::env::args(), Exit, version)
}
