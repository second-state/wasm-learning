use wasmedge_quickjs::*;
use suborbital::runnable::*;

struct HelloEcho{}

impl Runnable for HelloEcho {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        let code = String::from_utf8(input).unwrap();

        let mut ctx = Context::new();
        let r = ctx.eval_global_str(&code);

        Ok(format!("{:?}", r).as_bytes().to_vec())
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &HelloEcho = &HelloEcho{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
