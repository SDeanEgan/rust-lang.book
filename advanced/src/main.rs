use std::fmt::Debug;
use std::ops::Add;

static mut PROCESS_CALL_COUNT: usize = 0;

trait Processor {
    type Input: Debug;
    type Output: Debug;
    fn process(&self, input: Self::Input) -> Self::Output;
}

struct Plugin<I, O> {
    func: Box<dyn Fn(I) -> O>,
}

impl<I: Debug, O: Debug> Processor for Plugin<I, O> {
    type Input = I;
    type Output = O;
    fn process(&self, input: I) -> O {
        // encapsulate to safely update the global process call count
        unsafe {
            PROCESS_CALL_COUNT += 1;
        }
        (self.func)(input)
    }
}

// implement Add for chaining Plugins: (I -> U) + (U -> O) => (I -> O)
impl<I, U, O> Add<Plugin<U, O>> for Plugin<I, U>
where
    I: Debug + 'static,
    U: Debug + 'static,
    O: Debug + 'static,
{
    type Output = Plugin<I, O>;
    fn add(self, rhs: Plugin<U, O>) -> Self::Output {
        let f = self.func;
        let g = rhs.func;
        
        Plugin {
            func: Box::new(move |x| g(f(x)))
        }
    }
}

// helper function to create a Plugin from a regular function or closure
fn plugin<I, O>(f: impl Fn(I) -> O + 'static) -> Plugin<I, O>
where
    I: Debug + 'static,
    O: Debug + 'static,
{
    Plugin {
        func: Box::new(f)
    }
}

fn uppercase(input: String) -> String {
    input.to_uppercase()
}

fn add_exclamations(input: String) -> String {
    format!("{}!!!", input)
}

fn print_process_call_count() {
    #[allow(static_mut_refs)]
    unsafe {
        println!("\n[PROCESS CALL COUNT]: {}", PROCESS_CALL_COUNT);
    }
}

fn main() {
    let plugin1 = plugin(uppercase);
    let plugin2 = plugin(add_exclamations);
    // apply plugin to strings
    let result1 = plugin1.process("hello".to_string());
    let result2 = plugin2.process("world".to_string());
    println!("Plugin1: {}", result1);
    println!("Plugin2: {}", result2);
    // use addition to synthesize a new function
    let combined_plugin = plugin1 + plugin2;
    let result3 = combined_plugin.process("combined".to_string());
    println!("Combined: {}", result3); 
    
    print_process_call_count();
}
