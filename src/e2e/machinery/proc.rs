use std::process::Command;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OptLevel {
    O0,
    O1,
    O2,
    O3,
    Os,
    Oz,
}

impl OptLevel {
    #[must_use]
    pub const fn cargo_value(self) -> &'static str {
        return match self {
            Self::O0 => "0",
            Self::O1 => "1",
            Self::O2 => "2",
            Self::O3 => "3",
            Self::Os => "s",
            Self::Oz => "z",
        };
    }
}

pub fn exe(
    context: &str,
    mut command: Command,
) -> String {
    let rendered = render_command(&command);
    let output = command.output().unwrap_or_else(|error| {
        panic!(
            "failed, context=`{context}`, command=`{rendered}`, error={error}"
        )
    });

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        panic!(
            "failed context=`{context}`, command=`{rendered}`, status={}\nstdout:\n{}\nstderr:\n{}",
            output.status, stdout, stderr
        );
    }

    return format!("{stdout}{stderr}");
}

#[must_use]
pub fn render_command(command: &Command) -> String {
    let mut result = command.get_program().to_string_lossy().into_owned();
    for arg in command.get_args() {
        result.push(' ');

        let value = arg.to_string_lossy();
        result.push_str(&if value.contains(char::is_whitespace) {
            format!("{value:?}")
        }
        else {
            value.into_owned()
        });
    }
    return result;
}

pub fn has_slow() -> bool {
    return match std::env::var("TYPEKIN_DISABLE_SLOW_TESTS") {
        Ok(it) => !it.is_empty() && it != "0" && it != "false",
        Err(_) => return true,
    };
}

macro_rules! slow {
    () => {
        if !$crate::has_slow() {
            return;
        }
    };
}

pub(crate) use slow;
