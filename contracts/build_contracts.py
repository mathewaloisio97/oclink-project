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

import os
import glob
import subprocess
import sys

# Define absolute paths for contracts and output destination.
CONTRACTS_DIR = os.path.dirname(os.path.abspath(__file__))
CSHARP_OUT = os.path.abspath(os.path.join(CONTRACTS_DIR, "../dot-net-apis/OcLink.API.Core/Generated"))


def ensure_dir(path: str) -> None:
    """Create the directory if it does not already exist."""
    if not os.path.exists(path):
        os.makedirs(path)


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
    compile_protos()