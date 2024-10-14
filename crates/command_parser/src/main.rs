use luminol_data::rpg;

use std::fmt::Write;
use std::iter::Peekable;

fn parse_command_under(
    node: indextree::NodeId,
    command: rpg::EventCommand,
    arena: &mut indextree::Arena<rpg::EventCommand>,
    iter: &mut Peekable<impl Iterator<Item = rpg::EventCommand>>,
    config: &luminol_config::project::Config,
) {
    if command.code == 0 {
        return;
    }

    let desc = config.command_db.get(command.code);
    let mut child = node.append_value(command, arena);
    if let Some(desc) = desc {
        match &desc.kind {
            luminol_data::commands::CommandKind::Branch {
                parameters,
                branches,
                terminator,
                command_contains_branch,
            } => loop {
                let next = iter.next().unwrap();

                if next.code == terminator.code {
                    break;
                }

                if branches.iter().any(|branch| branch.code == next.code) {
                    child = node.append_value(next, arena);
                    continue;
                }

                parse_command_under(child, next, arena, iter, config);
            },
            luminol_data::commands::CommandKind::Multi(cont) => {
                let command = arena[child].get_mut();
                let text = command.parameters[0].as_string_mut().unwrap();
                while let Some(next) = iter.next_if(|next| next.code == *cont) {
                    let next_line = next.parameters[0].as_string().unwrap();
                    write!(text, "\n{next_line}").unwrap();
                }
            }
            luminol_data::commands::CommandKind::Regular { parameters } => todo!(),
            luminol_data::commands::CommandKind::MoveRoute(_) => todo!(),
            luminol_data::commands::CommandKind::Blank => todo!(),
        }
    }
}

fn main() {
    let mut filesystem = luminol_filesystem::project::FileSystem::new();
    // hardcoding the path for now
    let host = luminol_filesystem::host::FileSystem::new("/mnt/hdd/Git/OSFM-GitHub");

    let mut global_config = luminol_config::global::Config::new();
    let mut config = None;

    let _ = filesystem
        .load_project(host, &mut config, &mut global_config)
        .unwrap();

    let mut config = config.unwrap();

    let mut toasts = luminol_core::Toasts::default();
    let mut data_cache = luminol_core::Data::Unloaded;
    data_cache
        .load(&filesystem, &mut toasts, &mut config)
        .unwrap();

    let mut map = data_cache.get_or_load_map(1, &filesystem, &config);
    let event = map.events.remove(1);
    for page in event.pages {
        let mut arena = indextree::Arena::<rpg::EventCommand>::new();

        let root = arena.new_node(rpg::EventCommand::default());

        let mut iter = page.list.into_iter().peekable();
        while let Some(command) = iter.next() {
            parse_command_under(root, command, &mut arena, &mut iter, &config);
        }

        let printable = root.debug_pretty_print(&arena);
        println!("{printable:?}")
    }
}
