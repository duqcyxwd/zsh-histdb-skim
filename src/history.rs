extern crate skim;
use crate::environment::*;
use chrono::{DateTime, Local, TimeZone};
use humantime::format_duration;
use skim::prelude::*;
use std::io::{BufWriter, Write};
use std::process::Command;
use std::time::Duration;
use std::time::SystemTime;

fn get_epoch_start_of_day() -> u64 {
    let now = SystemTime::now();
    let now_secs = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let seconds_since_midnight = now_secs % (24 * 3600);
    now_secs - seconds_since_midnight
}

pub struct History {
    pub id: i64,
    pub cmd: String,
    pub start: u64,
    pub exit_status: Option<i64>,
    pub duration: Option<i64>,
    pub count: i64,
    pub session: i64,
    pub host: String,
    pub dir: String,
    pub searchrange: [(usize, usize); 1],
}

impl History {
    pub const FORMAT_DATE_LENGTH: usize = 10;
    pub const COMMAND_START: usize = (History::FORMAT_DATE_LENGTH + 1);

    pub fn command(&self) -> &String {
        return &self.cmd;
    }
    pub fn session(&self) -> String {
        return self.session.to_string();
    }
    pub fn dir(&self) -> String {
        return self.dir.to_string();
    }
}

impl History {
    fn format_date(&self, full: bool) -> String {
        let starttime: DateTime<Local> = Local.timestamp_opt(self.start as i64, 0).unwrap();
        if full {
            let mut dateinfo = String::from("");
            dateinfo.push_str(&get_date_format());
            dateinfo.push_str(" %H:%M");
            return format!("{}", starttime.format(&dateinfo));
        } else if self.start > get_epoch_start_of_day() {
            return format!("{}", starttime.format("%H:%M"));
        } else {
            return format!("{}", starttime.format(&get_date_format()));
        }
    }

    fn format_or_none(x: Option<i64>) -> String {
        if x.is_some() {
            format!("{}", x.unwrap())
        } else {
            "\x1b[37;1m<NONE>\x1b[0m".to_string()
        }
    }

    fn format_duration(&self) -> String {
        if self.duration.is_some() {
            let duration = Duration::from_secs(self.duration.unwrap() as u64);
            format_duration(duration).to_string()
        } else {
            History::format_or_none(self.duration)
        }
    }

    fn highlight_command(&self) -> String {
        let bat_cmd = get_bat_command();
        let bat_args: Vec<&str> = bat_cmd.split_whitespace().collect();
        
        if bat_args.is_empty() {
            return self.cmd.clone();
        }
        
        let cmd_name = bat_args[0];
        let args = &bat_args[1..];
        
        match Command::new(cmd_name)
            .args(args)
            .arg("--")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
        {
            Ok(mut child) => {
                if let Some(stdin) = child.stdin.take() {
                    let _ = BufWriter::new(stdin).write_all(self.cmd.as_bytes());
                }
                
                match child.wait_with_output() {
                    Ok(output) => {
                        if output.status.success() {
                            String::from_utf8_lossy(&output.stdout).to_string()
                        } else {
                            self.cmd.clone()
                        }
                    }
                    Err(_) => self.cmd.clone()
                }
            }
            Err(_) => self.cmd.clone()
        }
    }
}

impl SkimItem for History {
    fn text(&self) -> Cow<str> {
        let information = format!("{:10} {}", self.format_date(false), self.cmd);
        Cow::Owned(information)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        let mut information = String::from(format!("\x1b[1mDetails for {}\x1b[0m\n\n", self.id));

        let mut tformat = |name: &str, value: &str| {
            information.push_str(&format!("\x1b[1m{:20}\x1b[0m{}\n", name, value));
        };

        tformat("Runtime", &self.format_duration());
        tformat("Host", &self.host);
        tformat("Executed", &self.count.to_string());
        tformat("Directory", &self.dir);
        tformat("Exit Status", &History::format_or_none(self.exit_status));
        tformat("Session", &self.session.to_string());
        tformat("Start Time", &self.format_date(false));
        
        // Use bat for syntax highlighting if available
        information.push_str("\x1b[1mCommand\x1b[0m\n\n");
        let highlighted_cmd = self.highlight_command();
        information.push_str(&highlighted_cmd);
        information.push('\n');
        
        ItemPreview::AnsiText(information)
    }

    fn get_matching_ranges(&self) -> Option<&[(usize, usize)]> {
        Some(&self.searchrange)
    }
}
