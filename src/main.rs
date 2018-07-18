extern crate ansi_term;
extern crate getopts;
extern crate git2;

use std::usize;
use ansi_term::{Color, Style};

struct Options {
    color_scheme: usize,
    git_branch: bool,
    git_commit_id: usize,
    git_state: bool
}

impl Options {
    pub fn new() -> Options {
        Options {
            color_scheme:   0,
            git_branch:     false,
            git_commit_id:  0,
            git_state:      false,
        }
    }
}

fn match_arguments(args: &[String], opts: &getopts::Options) -> Result<Options, String> {
    let mut options = Options::new();

	let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => return Err(f.to_string())
    };

    // Help
    if matches.opt_present("h") {
        return Err("HELP".to_string());
    }

    // Formating options
    if let Some(s) = matches.opt_str("C") {
        if let Ok(n) = usize::from_str_radix(&s, 10) {
            options.color_scheme = n;
        }
        else {
            return Err("color SCHEME is not a valid number".to_string());
        }
    }

    // Information selection
    if matches.opt_present("b") {
        options.git_branch = true;
    }
    if let Some(s) = matches.opt_str("c") {
        if let Ok(n) = usize::from_str_radix(&s, 10) {
            options.git_commit_id = n;
        }
        else {
            return Err("commit LENGTH is not a valid number".to_string());
        }
    }
    if matches.opt_present("s") {
        options.git_state = true;
    }

    return Ok(options);
}


fn process_arguments() -> Options {
    let argv: Vec<String> = std::env::args().collect();
    let mut opts = getopts::Options::new();

    // Help
    opts.optflag("h", "help", "print this help menu");
    // Formating options
    opts.optopt("C", "color", "render using a color scheme", "SCHEME");
    // Information selection
    opts.optflag("b", "branch", "print the branch name");
    opts.optopt("c", "commit", "print the commit id, truncated to LENGTH", "LENGTH");
    opts.optflag("s", "state", "print the repository state");

    match match_arguments(&argv[1..], &opts) {
        Ok(o) => return o,
        Err(e) => {
            let brief = format!("Usage: {} [options]", argv[0]);
            print!("{}", opts.usage(&brief));
            if e != "HELP" {
                println!("Argument parsing error: {}", e);
            }
            std::process::exit(1);
        }
    }
}

fn get_color_scheme(color_scheme: usize) -> [Style;4] {
    match color_scheme {
        1 => [Color::Blue.normal(), Color::Green.normal(), Color::Yellow.normal(), Color::Red.normal()],
        _ => [Style::new(), Style::new(), Style::new(), Style::new()],
    }
}


fn main() {
    let options = process_arguments();
    let scheme = get_color_scheme(options.color_scheme);

    if let Ok(repo) = git2::Repository::discover(".") {
        print!("(");
        if let Ok(head) = repo.head() {
            if options.git_branch {
                if let Some(branch_name) = head.shorthand() {
                    print!("{}", scheme[0].paint(branch_name));
                }
            }
            if options.git_branch && options.git_commit_id > 0 {
                print!(":");
            }
            if options.git_commit_id > 0 {
                if let Ok(commit) = head.peel_to_commit() {
                    let id = format!("{}", commit.id());
                    print!("{}", scheme[3].paint(&id[..options.git_commit_id]));
                }
            }
            if options.git_commit_id > 0 && options.git_state {
                print!(" ");
            }
            if options.git_state {
                if let Ok(statuses) = repo.statuses(None) {
                    let letters = ['N', 'D', 'M', 'R', 'T', 'C'];
                    let palette = [1, 3, 2, 0, 0, 3];
                    let mut counts = [0u32;6];
                    for status in statuses.iter() {
                        if status.status().is_wt_new() {
                            counts[0] += 1;
                        }
                        if status.status().bits() & git2::Status::WT_DELETED.bits() != 0 {
                            counts[1] += 1;
                        }
                        if status.status().is_wt_modified() {
                            counts[2] += 1;
                        }
                        if status.status().is_wt_renamed() {
                            counts[3] += 1;
                        }
                        if status.status().is_wt_typechange() {
                            counts[4] += 1;
                        }
                        if status.status().is_conflicted() {
                            counts[5] += 1;
                        }
                    }
                    for i in 0..counts.len() {
                        if counts[i] > 0 {
                            print!("{}", scheme[palette[i]].paint(format!("{}{}", letters[i], counts[i])));
                        }
                    }
                }
                match repo.state() {
                    git2::RepositoryState::Clean => {},
                    git2::RepositoryState::Merge => {
                        print!(" {}", scheme[2].paint("Merge"));
                    },
                    git2::RepositoryState::Revert => {
                        print!(" {}", scheme[2].paint("Revert"));
                    },
                    git2::RepositoryState::RevertSequence => {
                        print!(" {}", scheme[2].paint("Revert Sequence"));
                    },
                    git2::RepositoryState::CherryPick => {
                        print!(" {}", scheme[2].paint("Cherry-pick"));
                    },
                    git2::RepositoryState::CherryPickSequence => {
                        print!(" {}", scheme[2].paint("Cherry-pick Sequence"));
                    },
                    git2::RepositoryState::Bisect => {
                        print!(" {}", scheme[2].paint("Bisect"));
                    },
                    git2::RepositoryState::Rebase => {
                        print!(" {}", scheme[2].paint("Rebase"));
                    },
                    git2::RepositoryState::RebaseInteractive => {
                        print!(" {}", scheme[2].paint("Rebase Interactive"));
                    },
                    git2::RepositoryState::RebaseMerge => {
                        print!(" {}", scheme[2].paint("Rebase Merge"));
                    },
                    git2::RepositoryState::ApplyMailbox => {
                        print!(" {}", scheme[2].paint("Apply Mailbox"));
                    },
                    git2::RepositoryState::ApplyMailboxOrRebase => {
                        print!(" {}", scheme[2].paint("Apply Mailbox or Rebase"));
                    },
                }
            }
        }
        print!(")");
    }
}
