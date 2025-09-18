import subprocess
import sys
import re
from pathlib import Path


def run_cargo_expand(binary_name, path_to_module):
    """Run cargo expand and return its output."""
    try:
        result = subprocess.run(
            ["cargo", "expand", "--bin", binary_name, path_to_module],
            capture_output=True,
            text=True,
            check=True,
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        print(f"Error running cargo expand: {e}", file=sys.stderr)
        print(f"stderr: {e.stderr}", file=sys.stderr)
        sys.exit(1)
    except FileNotFoundError:
        print(
            "Error: cargo expand command not found. Make sure cargo-expand is installed.",
            file=sys.stderr,
        )
        print("You can install it with: cargo install cargo-expand", file=sys.stderr)
        sys.exit(1)


def extract_functions_with_push_state(code):
    """Extract function names that have &mut PushState as their only parameter."""
    # Look for function definitions with pattern: pub fn name(state: &mut PushState)
    pattern = r"pub fn ([a-zA-Z0-9_]+)\s*\(\s*_?state\s*:\s*&mut\s+PushState\s*\)"
    matches = re.findall(pattern, code)
    return matches


def organize_functions(func_list) -> dict[str, list]:
    categorized: dict[str, list] = {}

    for func in func_list:
        parts: list[str] = func.split("_")

        if parts[0] == "vector":
            categorized[f"vector_{parts[1]}"] = []
        else:
            categorized[parts[0]] = []

    for func in func_list:
        for key in categorized.keys():
            if func.startswith(key):
                categorized[key].append(func)

    return categorized


if __name__ == "__main__":
    expanded_code: list = []

    for rs_file in Path("src/instructions/").iterdir():
        if rs_file.name not in ["mod.rs", "utils.rs", "list.rs"]:
            expanded_code.append(
                run_cargo_expand("rush", f"instructions::{rs_file.stem}")
            )

    function_names: list = []
    for code in expanded_code:
        function_names.extend(extract_functions_with_push_state(code))

    categorized_funcs: dict[str, list] = organize_functions(function_names)

    with open("src/instructions/list.rs", "w") as f:
        for rs_file in Path("src/instructions/").iterdir():
            if rs_file.name not in ["mod.rs", "utils.rs", "list.rs"]:
                f.write(f"use crate::instructions::{rs_file.stem}::*;\n")
        f.write("use crate::push::state::PushState;\n")
        f.write("use std::collections::HashMap;\n")
        f.write("use std::sync::LazyLock;\n")

        f.write("\n")

        for list_name in categorized_funcs.keys():
            f.write(
                "pub fn " + list_name + "_instructions() -> Vec<fn(&mut PushState)> {\n"
            )
            f.write("    vec![\n")
            for func in categorized_funcs[list_name]:
                f.write(f"        {func},\n")
            f.write("    ]\n")
            f.write("}\n")
            f.write("\n")

        f.write("pub fn all_instructions() -> Vec<fn(&mut PushState)> {\n")
        f.write("    let mut all_vec = vec![];\n")
        for list_name in categorized_funcs.keys():
            f.write(f"    all_vec.extend({list_name}_instructions().iter());\n")
        f.write("    all_vec\n")
        f.write("}\n")

        f.write("\n")

        f.write(
            "pub static INSTR_NAME_MAP: LazyLock<HashMap<usize, String>> = LazyLock::new(|| {\n"
        )
        f.write("   let mut temp_map = HashMap::default();\n")
        for list_name in categorized_funcs.keys():
            for func in categorized_funcs[list_name]:
                f.write(f'   temp_map.insert({func} as usize, "{func}".to_string());\n')
        f.write("   temp_map\n")
        f.write("});")
