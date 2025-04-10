use facet_pretty::FacetPretty;

#[ctor::ctor]
fn init() {
    // Initialize color backtrace for pretty stack traces
    color_backtrace::install();

    // Initialize logger to print all log levels
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();

    log::info!("Logging and color backtrace initialized");
}

#[test]
fn test_arg_parse() {
    use facet::Facet;

    #[derive(Facet)]
    struct Args {
        #[facet(positional)]
        path: String,

        #[facet(named, short = 'v')]
        verbose: bool,

        #[facet(named, short = 'j')]
        concurrency: usize,
    }

    let args: Args =
        facet_args::from_slice(&["--path", "example.rs", "--verbose", "--concurrency", "14"]);
    eprintln!("args: {}", args.pretty());
}
