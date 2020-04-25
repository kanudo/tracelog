//! Easy logging and tracing

use std::fmt::Display;

pub struct Trace {
    title : String,
    info: String,
    backtrace : String,
}

impl Trace {

    /// Provides title to the trace logs
    pub fn new<T : Display>(title: T) -> Trace {
        Trace {
            title: title.to_string(),
            info: String::new(),
            backtrace : String::new(),
        }
    }

    /// Filters backtrace based on "|" separated string provided
    pub fn trace_filter<T: Display>(&mut self, traces_of: T) -> &mut Trace {

        let mut count = 0;

        backtrace::trace(|frame| {

            backtrace::resolve(frame.ip(), |symbol| {

                let name = if let Some(name) = symbol.name() {
                    name.to_string()
                } else {
                    " - <unknown caller name>".to_string()
                };

                let filename = if let Some(filename) = symbol.filename() {
                    filename.display().to_string()
                } else {
                    " - <unknown file name>".to_string()
                };

                let line = if let Some(line) = symbol.lineno() {
                    line
                } else {
                    0
                };

                let output = format!("{:<2} -- {} \n {:10} {}:{} ", count, name, "", filename, line);

                let matches = traces_of.to_string();
                let matches = matches.split("|");

                for text_match in matches {
                    if output.contains(text_match) {
                        self.backtrace = format!("{}\n{}", self.backtrace, output);
                        break;
                    }
                }
            });

            count += 1;

            true // keep going
        });

        self
    }

    /// Any information to be shown inside the log block
    pub fn info<T: Display>(&mut self, info: T) -> &mut Trace {
        self.info = info.to_string();
        self
    }

    /// Emits the trace log
    pub fn emit(&self){

        let output = format!("----------------------------------------------------------------------- BEGIN ------ {} \n", self.title);

        let output = if self.info.len() > 0 {
            format!("{}{} \n",output, self.info)
        } else { output };
        let output = if self.backtrace.len() > 0 {
            format!("{}{} \n\n",output, self.backtrace)
        } else { output };

        let output = format!("{}----------------------------------------------------------------------- END -------- {} \n\n", output, self.title);

        println!("{}", output);
    }

    /// Quick log handle
    pub fn log<T: Display, U: Display, V: Display> (title: T, info: U, trace_filter: V){
        Self::new(title).info(info).trace_filter(trace_filter).emit();
    }

}

