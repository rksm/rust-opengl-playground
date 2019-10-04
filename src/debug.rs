
pub fn failure_to_string(e: failure::Error) -> String {
    use std::fmt::Write;

    let mut result = String::new();
    for (i, cause) in e
        .iter_chain()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
    {
        if i > 0 {
            writeln!(&mut result, "  Which caused:").unwrap();
        }
        write!(&mut result, "{}", cause).unwrap();
        if let Some(backtrace) = cause.backtrace() {
            let backtrace_sir = format!("{}", backtrace);
            if backtrace_sir.len() > 0 {
                writeln!(&mut result, " This happened at {}", backtrace).unwrap();
            } else {
                writeln!(&mut result).unwrap();
            }
        } else {
            writeln!(&mut result).unwrap();
        }
    }

    result
}
