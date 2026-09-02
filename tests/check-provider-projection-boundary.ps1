$ErrorActionPreference = 'Stop'

function Assert-Contains {
  param(
    [string]$Path,
    [string]$Needle
  )

  $text = Get-Content -Raw -LiteralPath $Path
  if ($text.IndexOf($Needle, [StringComparison]::Ordinal) -lt 0) {
    throw "Missing expected text in ${Path}: ${Needle}"
  }
}

Assert-Contains 'docs/PROVIDER_PROJECTION.md' '`WITPUB-PF-03`'
Assert-Contains 'docs/PROVIDER_PROJECTION.md' 'witness.provider-projection.v1'
Assert-Contains 'docs/PROVIDER_PROJECTION.md' 'unsupported provider behavior'
Assert-Contains 'docs/PROVIDER_PROJECTION.md' 'ordering loss'
Assert-Contains 'README.md' 'provider-projection'
Assert-Contains '.pitfall/witness-public-pitfalls.md' 'WITPUB-PF-03'
Assert-Contains '.pitfall/witness-public-pitfalls.md' '**Status:** MITIGATED'
Assert-Contains '.pitfall/witness-public-invariants.md' 'WITPUB-I-06'
Assert-Contains 'crates/witness-core/src/lib.rs' 'ProviderProjectionReport'
Assert-Contains 'crates/witness-core/src/lib.rs' 'provider_projection_fixture'
Assert-Contains 'crates/witness-cli/src/main.rs' 'provider-projection'
Assert-Contains 'crates/witness-cli/tests/proof_surface.rs' 'projection.json_loss'
Assert-Contains 'crates/witness-cli/tests/fixtures/replay_cli_proof.txt' 'witness.provider-projection.v1'
Assert-Contains '.roles/ROLE.md' '`WITPUB-PF-03`'
Assert-Contains '.roles/ROLE.md' 'Provider Portability Reviewer; Session Safety Reviewer; Harness Runtime Reviewer'

Write-Host 'WITNESS provider projection boundary check passed.'
