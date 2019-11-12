// Logging for TA3 Parser Output Comparator

/* from: https://osr.jpl.nasa.gov/wiki/pages/viewpage.action?spaceKey=SD&title=TA2+PDF+Safe+Parser+Evaluation

CRITICAL	This error level must be used when the TA2 parser is going to terminate parsing based on
            unexpected input.
            => panic!
ERROR       This error level must be used when the TA2 parser has found invalid data to parse, but
            intends to continue parsing. ERROR or CRITICAL must be used to flag any "unsafe parsing
            events"
            => error!
WARNING     This error level can be used when the TA2 parser has found unexpected data to parse.
            This error level can be used to flag safe, but unexpected parsing events.
            => warn!
INFO    	This error level must be used to instrument components being parsed by the PDF parser.
            Each component should have some INFO parser output.
            => info!
DEBUG   	Any messages that the TA2 parser needs to output for debug information should use this
            error level.
            => debug!

Note: Rust level trace! is not included.  Those messages will print without the TA3 preamble.
*/

// see: https://docs.rs/log/0.4.8/log/
use chrono::Local;
use log::{Record, Level, Metadata};

#[allow(dead_code)]
struct TA3Logger {
    filename: str,
}

// TODO: add offset to Record
// TODO: test that trace! still prints?

impl log::Log for TA3Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug  // allow all but trace! messages
        //true  TODO: may need to allow all and handle trace! diff
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if record.metadata().level() == Level::Trace  {
                println!("{} {:<5} [{}] {}",
                         Local::now().format("%Y-%m-%d %H:%M:%S,%3f"),
                         record.level(),
                         record.target(),
                         record.args());
            } else {
                println!("{} - {} at <File Offset> - {}",
                         record.level().to_string().to_uppercase(),
                         "some_file.pdf", //self.filename,
                         record.args());
            }
        }
    }

    fn flush(&self) {}
}
