use tracelog::Trace;

fn main(){

    Trace::log("main function", "Some INFO", "main|std");

    Trace::new("main function").info("Some INFO").trace_filter("main|std").emit();

    Trace::new("main function").info("Some INFO").emit();

    Trace::new("main function").emit();
}