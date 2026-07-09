# GNU AFFERO GENERAL PUBLIC LICENSE
# Version 3, 19 November 2007
#
# Copyright (C) 2026 Mathew Aloisio
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

import configparser
import glob
import os
import subprocess
import sys

# Define absolute paths for contracts and output destination.
CONTRACTS_DIR = os.path.dirname(os.path.abspath(__file__))
CSHARP_OUT = os.path.abspath(os.path.join(CONTRACTS_DIR, "../dot-net-apis/OcLink.API.Contracts/Scripts/Generated"))


def ensure_dir(path: str) -> None:
    """Create the directory if it does not already exist."""
    if not os.path.exists(path):
        os.makedirs(path)


def get_editorconfig_rules() -> dict:
    """Locate and parse the workspace root .editorconfig for [*.proto] rule overrides.
    
    Falls back to official Google Protocol Buffer baseline defaults if the rule
    manifest cannot be resolved or parsed.
    """
    defaults = {"indent_style": "space", "indent_size": 2, "end_of_line": "lf"}
    
    # Traverse upward from current module context to find the project '.editorconfig'.
    cursor = CONTRACTS_DIR
    while cursor and cursor != os.path.dirname(cursor):
        ec_path = os.path.join(cursor, ".editorconfig")
        if os.path.exists(ec_path):
            try:
                parser = configparser.ConfigParser(interpolation=None)
                parser.optionxform = str  # Preserve rule casing.
                parser.read(ec_path, encoding="utf-8")
                
                for section in parser.sections():
                    if "*.proto" in section:
                        sect = parser[section]
                        return {
                            "indent_style": sect.get("indent_style", defaults["indent_style"]),
                            "indent_size": int(sect.get("indent_size", defaults["indent_size"])),
                            "end_of_line": sect.get("end_of_line", defaults["end_of_line"])
                        }
            except Exception:
                break
        cursor = os.path.dirname(cursor)
    return defaults


def process_formatting(fix: bool = False) -> None:
    """Evaluate or auto-correct project .proto file schemas using active .editorconfig parameters.
    
    Guarantees strict, zero-dependency parity across cross-platform developer
    workstations and containerized CI/CD build environments.
    """
    rules = get_editorconfig_rules()
    indent_char = "\t" if rules["indent_style"] == "tab" else " "
    indent_size = 1 if rules["indent_style"] == "tab" else rules["indent_size"]
    eol = "\n" if rules["end_of_line"] == "lf" else "\r\n"
    
    proto_files = glob.glob(os.path.join(CONTRACTS_DIR, "**/*.proto"), recursive=True)
    failed = False
    
    for filepath in proto_files:
        rel_path = os.path.relpath(filepath, CONTRACTS_DIR)
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()
            
        lines = content.splitlines(keepends=True)
        processed_lines = []
        file_mutated = False
        
        for line_num, line in enumerate(lines, 1):
            stripped = line.lstrip(" \t")
            if not stripped.strip():
                processed_lines.append(line)
                continue
                
            # Determine block depth multiplier based on active rule specifications.
            current_indent = len(line) - len(stripped)
            nesting_level = round(current_indent / indent_size) if indent_char == " " else current_indent
            target_indent = (indent_char * indent_size) * nesting_level
            
            normalized_line = target_indent + stripped.rstrip(" \t\r\n") + eol
            
            if line != normalized_line:
                file_mutated = True
                if not fix:
                    print(f"{rel_path}:{line_num} -> Style deviation from .editorconfig constraints.", file=sys.stderr)
                    failed = True
            processed_lines.append(normalized_line)
            
        if fix and file_mutated:
            print(f"Normalizing layout constraints: {rel_path}")
            with open(filepath, "w", encoding="utf-8", newline="") as f:
                f.write("".join(processed_lines))

    if failed and not fix:
        print("\nStyle assertion failed! Run 'just format' to align changes locally.", file=sys.stderr)
        sys.exit(1)
        
    if not fix:
        print("All Protocol Buffer schemas align with current .editorconfig constraints.")


def compile_protos() -> None:
    """Find and compile all Protocol Buffer files into C# source code.
    
    Requires a global system installation of protoc to ensure environment
    parity with CI/CD build runners.
    """
    ensure_dir(CSHARP_OUT)
    proto_files = glob.glob(os.path.join(CONTRACTS_DIR, "**/*.proto"), recursive=True)
    
    if not proto_files:
        print("No .proto files found.")
        return

    print(f"Compiling {len(proto_files)} proto files to C#...")

    cmd = [
        "protoc",
        f"-I{CONTRACTS_DIR}",
        f"--csharp_out={CSHARP_OUT}"
    ] + proto_files

    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode != 0:
        print("Protoc Compilation Failed:", file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        sys.exit(result.returncode)
    
    print("C# Protobuf Compilation Successful.")


if __name__ == "__main__":
    if "--check-format" in sys.argv:
        process_formatting(fix=False)
        sys.exit(0)
    elif "--format" in sys.argv:
        process_formatting(fix=True)
        sys.exit(0)
        
    compile_protos()
