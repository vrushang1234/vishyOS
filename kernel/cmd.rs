use crate::framebuffer;

pub fn handle_command(buf: &[u8]) {
    if buf == b"help" {
        cmd_help();
    } else if buf == b"clear" {
        cmd_clear();
    } else {
        framebuffer::print("\nUnknown command\n", 0xFFFFFFFF);
    }
}

fn cmd_help() {
    framebuffer::print(
        "\nCommands:\n help - show this message\n clear - clear screen\n",
        0xFFFFFFFF,
    );
}

fn cmd_clear() {
    framebuffer::clear();
}
