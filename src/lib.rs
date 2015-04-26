use std::process::Command;
use std::convert::AsRef;
use std::ffi::OsStr;

pub struct Update {
    cmd: Command,
}

pub fn update() -> Update {
    let mut update = Update { cmd: Command::new("git") };
    update.arg("submodule").arg("update");
    update
}

impl Update {
    fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Update {
        self.cmd.arg(arg);
        self
    }

    pub fn init(&mut self) -> &mut Update { self.arg("--init") }
    pub fn rebase(&mut self) -> &mut Update { self.arg("--rebase") }
    pub fn merge(&mut self) -> &mut Update { self.arg("--merge") }
    pub fn checkout(&mut self) -> &mut Update { self.arg("--checkout") }
    pub fn remote(&mut self) -> &mut Update { self.arg("--remote") }
    pub fn no_fetch(&mut self) -> &mut Update { self.arg("--no-fetch") }
    pub fn recursive(&mut self) -> &mut Update { self.arg("--recursive") }
    pub fn force(&mut self) -> &mut Update { self.arg("--force") }

    pub fn run(&mut self) { 
        println!("running: {:?}", self.cmd);
        let status = match self.cmd.status() {
            Ok(status) => status,
            Err(e) => fail(&format!("failed to execute command: {}", e)),
        };

        if !status.success() {
            fail(&format!("command did not execute successfully, got: {}", status));
        }
    }
}

fn fail(msg: &str) -> ! {
    println!("\n\n{}\n\n", msg);
    panic!()
}

#[test]
fn build_update() {
    let mut update = update();
    update.init()
        .recursive();

    assert_eq!(format!("{:?}", update.cmd),
        "\"git\" \"submodule\" \"update\" \"--init\" \"--recursive\"")
}
