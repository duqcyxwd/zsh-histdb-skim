use crate::environment::*;
use crate::focus::get_focus_dir;
use crate::focus::get_focus_session;
use crate::location::Location;
use enum_map::enum_map;

pub fn generate_title(location: &Location) -> String {
    let extra_info = |theloc: &Location| -> String {
        return match theloc {
            Location::Session => get_current_session_id(),
            Location::Directory => get_current_dir(),
            Location::Machine => get_current_host(),
            _ => String::from(""),
        };
    }(&location);

    let format_extra_info = |info: Option<String>, title: &str| -> String {
        return match info {
            Some(ri) => format!("{}: {} ", &title, &ri,),
            None => String::from(""),
        };
    };
    let focus_session = format_extra_info(get_focus_session(), "Session");
    let focus_dir = format_extra_info(get_focus_dir(), "Directory");

    let location_map = enum_map! {
        Location::Session => "Session location history",
        Location::Directory => "Directory location history",
        Location::Machine => "Machine location history",
        Location::Everywhere => "Everywhere",
    };

    let header_map = enum_map! {
        Location::Session =>
" ┏━━━━━━━━━┱───────────┬──────┬────────────┐
 ┃ Session ┃ Directory │ Host │ Everywhere │ C-g: Toggle group, C-s: Lock Session, C-d: Lock Dir
━┛         ┗━━━━━━━━━━━┷━━━━━━┷━━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
        Location::Directory =>
" ┌─────────┲━━━━━━━━━━━┱──────┬────────────┐
 │ Session ┃ Directory ┃ Host │ Everywhere │ C-g: Toggle group, C-s: Lock Session, C-d: Lock Dir
━┷━━━━━━━━━┛           ┗━━━━━━┷━━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",

        Location::Machine =>
" ┌─────────┲━━━━━━━━━━━┱──────┬────────────┐
 │ Session ┃ Directory ┃ Host ┃ Everywhere │ C-g: Toggle group, C-s: Lock Session, C-d: Lock Dir
━┷━━━━━━━━━┷━━━━━━━━━━━┛      ┗━━━━━━━━━━━━┷━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",

        Location::Everywhere =>
" ┌─────────┲━━━━━━━━━━━┱──────┬────────────┐
 │ Session ┃ Directory ┃ Host ┃ Everywhere ┃ C-g: Toggle group, C-s: Lock Session, C-d: Lock Dir
━┷━━━━━━━━━┷━━━━━━━━━━━┷━━━━━━┛            ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    };

    let title = format!(
        "{} {} {}{}\n{}\n",
        &location_map[location.clone()],
        &extra_info,
        &focus_session,
        &focus_dir,
        &header_map[location.clone()],
    );
    return title.to_string();
}
