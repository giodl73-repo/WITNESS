# Original offline Cargo inputs

`inputs.zip` retains all20 original registry `.crate` archives selected by the
committed Cargo lock and all-features metadata:740 original files, including
files Cargo vendor may filter. There are no locked Git dependencies. No crate
manifest/source/grant bytes are rewritten. `inputs.sha256` anchors this capsule;
the verifier also checks the full lock/package/archive/file/mode/grant closure.

The standard-library receiver rejects unsafe paths, links, duplicate/aliased
names, archive resource violations, existing destinations and source/compiled
target overlap before writing. It creates20 additional Cargo checksum files and
an owned offline source-replacement config; these managed files are distinguished
from original input files. Full original source/config readback follows builds.

From a trusted complete source checkout/receipt, use new external directories:

```sh
python -I -S -B tools/input_custody.py --sha256 c4898b39483a85c7734e8ea66204f826afc8bd6b08182e9714efc488de63c749 --inputs /new/witness-inputs --cargo-home /new/witness-cargo-home --protect-target /compiled-target
```

`tools/build-pages.py` restores or verifies the same owned pair automatically
when both flags are supplied. All20 registry package identities must resolve
under that source directory. No default Cargo cache, registry download or
TRACKER-relative override is a build-correctness dependency. Installed Python,
Rust1.95.0, wasm-bindgen0.2.127 and optional Node/browser tools remain external
prerequisites; this is a source capsule, not an installer.

`tools/make-inputs.py` is an authoring tool only. It can construct a new capsule
from already installed, original checksum-verified locked archives. It requires
Git/Cargo and does not fetch or silently upgrade inputs. A dependency change
requires new original archives/notices, guards and full qualification.
