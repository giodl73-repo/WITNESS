"""Compare every original public frame byte and typed error, native vs WASM."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
STEPS = [*range(10), 31, 32, 255, 65535, 2147483647, 4294967295]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--site', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    output = Path(os.path.abspath(args.output))
    for parent in [output, *output.parents]:
        if parent.is_symlink() or getattr(parent.lstat() if parent.exists() else None, 'st_file_attributes', 0) & 0x400:
            raise ValueError('Output uses a link/junction')
    metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--locked', '--offline', '--no-deps', '--format-version', '1'], cwd=ROOT))
    target = Path(metadata['target_directory']).resolve()
    if output.exists() or any(output == p or output in p.parents or p in output.parents for p in (ROOT, target)):
        raise ValueError('Use a new output outside source and compiled target')
    output.mkdir(parents=True)
    controls = b''.join((str(step)+'\n').encode() for step in STEPS)
    (output/'controls.jsonl').write_bytes(controls)
    commands = []

    def run(name, command, data=None):
        result = subprocess.run([str(v) for v in command], cwd=ROOT, input=data, capture_output=True)
        (output/(name+'.stdout')).write_bytes(result.stdout)
        (output/(name+'.stderr')).write_bytes(result.stderr)
        commands.append(dict(name=name, command=[str(v) for v in command], exit_code=result.returncode))
        (output/'commands.json').write_text(json.dumps(commands, indent=2)+'\n', encoding='utf-8')
        if result.returncode:
            raise RuntimeError(name+' failed; raw evidence retained at '+str(output))
        return result.stdout

    run('build-native', ['cargo', 'build', '--locked', '--offline', '--release', '-p', 'witness-web', '--example', 'frame-control'])
    executable = target/'release/examples'/('frame-control.exe' if os.name == 'nt' else 'frame-control')
    native = run('native', [executable], controls)
    run('wasm', ['node', ROOT/'tools/check-frames.mjs', args.site.resolve(), output/'controls.jsonl', output/'wasm.jsonl', output/'raw-js-abi.json'])
    wasm = (output/'wasm.jsonl').read_bytes()
    assert native == wasm and native.endswith(b'\n'), 'Full native/WASM byte stream differs'
    lines = native.splitlines()
    assert len(lines) == len(STEPS)
    for step, line in zip(STEPS, lines):
        result = json.loads(line)
        if step <= 7:
            frame = json.loads(result['Ok'])
            assert frame['step'] == frame['event_count'] == frame['receipt_count'] == len(frame['events']) == step
            assert frame['total_steps'] == 7
            assert frame['validation_owed'] == (step == 4)
            assert frame['validation_attached'] == (step >= 5)
            assert frame['source_pointer_count'] == sum(event['source_pointer'] is not None for event in frame['events'])
            assert frame['checkpoint'] == ('sample:checkpoint' if step >= 6 else 'not yet checkpointed')
            assert frame['rehydrated_cut'] == ('sample:resumed' if step >= 7 else 'not yet resumed')
        else:
            assert result == {'Err':'Choose a replay step between 0 and 7'}
    receipt = dict(schema='witness.frame-parity.v1', controls=len(STEPS), valid_frames=8,
        all_native_wasm_bytes_exact=True, independent_prefix_expectations=True,
        native_sha256=hashlib.sha256(native).hexdigest(), wasm_sha256=hashlib.sha256(wasm).hexdigest(),
        raw_abi_observations=5, commands=commands,
        scope='Complete unchanged native frame strings/errors and whole Result NDJSON, not normalized objects. All eight public prefixes plus eight portable typed invalid controls. Raw JS coercions are separate observations. No live/provider/closure execution or full source/delivery claim.')
    (output/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({k:v for k,v in receipt.items() if k != 'commands'}))


if __name__ == '__main__':
    main()
