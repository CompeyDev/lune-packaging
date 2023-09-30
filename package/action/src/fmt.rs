use colored::Colorize;
use std::fmt;
use tracing_core::{Event, Level, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext, FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

pub struct LogFormatter;

impl<S, N> FormatEvent<S, N> for LogFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let meta = event.metadata();

        let scope = match meta.level() {
            &Level::DEBUG => "?".purple().bold(),
            &Level::ERROR => "!".red().bold(),
            &Level::INFO => "*".green().bold(),
            &Level::TRACE => ".".white().bold(),
            &Level::WARN => "#".yellow().bold(),
        };

        let target = if meta.level() != &Level::INFO {
            let mut sep = "::".to_owned();
            sep.push_str(&meta.target().italic().underline());

            sep
        } else {
            "::main".to_string()
        };

        write!(&mut writer, "[{}][{}] ", scope, target)?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;
                let ext = span.extensions();
                let fields = &ext.get::<FormattedFields<N>>().unwrap();

                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, " ")?;
            }
        }

        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
