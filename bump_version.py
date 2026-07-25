# type: ignore
import sys
import re
import subprocess

how = sys.argv[1]

with open("Cargo.toml", "r", encoding="utf-8") as f:
    content = f.read()
old_version = re.search(r'(?<! )version = "(.*)"', content).group(1)
version = old_version.split(".")
if how == "patch":
    version = ".".join(version[:-1] + [str(int(version[-1]) + 1)])
elif how == "minor":
    version = ".".join(version[:-2] + [str(int(version[-2]) + 1), "0"])
elif how == "major":
    version = ".".join([str(int(version[0]) + 1), "0", "0"])
else:
    sys.exit(1)
old_marker = f'version = "{old_version}"'
assert old_marker in content, f"{old_marker!r} not found in Cargo.toml"
content = content.replace(old_marker, f'version = "{version}"')
with open("Cargo.toml", "w", encoding="utf-8") as f:
    f.write(content)

with open("polars_reverse_geocode/__init__.py", "r", encoding="utf-8") as f:
    content = f.read()
old_marker = f'__version__ = "{old_version}"'
assert old_marker in content, (
    f"{old_marker!r} not found in polars_reverse_geocode/__init__.py "
    "(it may be out of sync with Cargo.toml)"
)
content = content.replace(old_marker, f'__version__ = "{version}"')
with open("polars_reverse_geocode/__init__.py", "w", encoding="utf-8") as f:
    f.write(content)

subprocess.run(["git", "commit", "-a", "-m", f"Bump version to {version}"])
subprocess.run(["git", "tag", "-a", version, "-m", version])
subprocess.run(["git", "push", "origin", "HEAD", "--follow-tags"])
