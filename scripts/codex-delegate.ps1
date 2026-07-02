param(
  [Parameter(Mandatory = $true)]
  [ValidateSet("read", "write")]
  [string]$Mode,

  [Parameter(Mandatory = $true)]
  [string]$Ticket
)

$ErrorActionPreference = "Stop"

if (!(Test-Path -LiteralPath $Ticket)) {
  throw "Ticket not found: $Ticket"
}

$Root = (git rev-parse --show-toplevel).Trim()

if (-not $Root) {
  throw "Could not determine git repository root."
}

$Model = if ($env:CODEX_DELEGATE_MODEL) {
  $env:CODEX_DELEGATE_MODEL
} else {
  "gpt-5.5"
}

$RunId = (Get-Date).ToUniversalTime().ToString("yyyyMMddTHHmmssZ") + "-" + (Get-Random)

$RunsDir = Join-Path $Root ".agent-runs"
$WorktreesDir = Join-Path $Root ".agent-worktrees"
$RunDir = Join-Path $RunsDir $RunId

New-Item -ItemType Directory -Force -Path $RunDir | Out-Null
New-Item -ItemType Directory -Force -Path $WorktreesDir | Out-Null

if ($Mode -eq "read") {
  $WorkDir = $Root
  $Sandbox = "read-only"
} else {
  git -C $Root rev-parse --verify HEAD *> $null
  if ($LASTEXITCODE -ne 0) {
    throw "write mode requires at least one git commit so an isolated worktree can be created."
  }

  $WorkDir = Join-Path $WorktreesDir $RunId
  $Branch = "agent/codex-$RunId"

  git -C $Root worktree add -b $Branch $WorkDir HEAD
  if ($LASTEXITCODE -ne 0) {
    throw "failed to create git worktree at $WorkDir"
  }

  $Sandbox = "workspace-write"
}

$TicketCopy = Join-Path $RunDir "ticket.md"
$PromptPath = Join-Path $RunDir "prompt.md"
$SummaryPath = Join-Path $RunDir "summary.md"
$EventsPath = Join-Path $RunDir "events.jsonl"
$StatusPath = Join-Path $RunDir "status.txt"
$DiffPath = Join-Path $RunDir "diff.patch"

Copy-Item -LiteralPath $Ticket -Destination $TicketCopy -Force

$TicketText = Get-Content -Raw -LiteralPath $Ticket

$Prompt = @"
You are running as a bounded Codex worker for the Magpie repository.

Follow AGENTS.md and the ticket exactly.

Do not broaden scope.
Do not commit.
Do not merge.
Do not access secrets.
Do not use the network unless explicitly allowed.
Do not delete unrelated files.

Native Windows execution constraint:

Shell execution may fail on native Windows with 'CreateProcessAsUserW failed: 5'.
Do not repeatedly retry failed shell commands.
Continue with patch or file edits only when the requested edits are safe and bounded from the repository contents.
Report any skipped validation commands in the final summary.
Do not claim tests passed unless they actually ran and passed.

Return a concise final summary with:

- files changed
- behaviour added
- tests run
- known risks
- suggested reviewer checks

TICKET:

$TicketText
"@

Set-Content -Path $PromptPath -Value $Prompt -Encoding UTF8

# Codex can write informational notices to stderr. In Windows PowerShell 5.1,
# merging stderr with stdout can create ErrorRecords when $ErrorActionPreference
# is Stop. Relax it only around the Codex call, stringify the stream, then
# restore strict error handling and check the exit code explicitly.
$PreviousErrorActionPreference = $ErrorActionPreference
$ErrorActionPreference = "Continue"

Get-Content -Raw -LiteralPath $PromptPath |
  codex exec `
    --cd $WorkDir `
    --model $Model `
    --sandbox $Sandbox `
    --json `
    --output-last-message $SummaryPath `
    "Run the piped Magpie worker ticket exactly." 2>&1 |
  ForEach-Object { "$_" } |
  Tee-Object -FilePath $EventsPath

$CodexExit = $LASTEXITCODE
$ErrorActionPreference = $PreviousErrorActionPreference

if ($CodexExit -ne 0) {
  throw "codex exec failed with exit code $CodexExit. Run dir: $RunDir"
}

if (!(Test-Path -LiteralPath $SummaryPath)) {
  throw "codex exec completed but did not produce summary file: $SummaryPath"
}

if ($Mode -eq "write") {
  git -C $WorkDir status --short | Set-Content -Path $StatusPath -Encoding UTF8
  git -C $WorkDir add -N . | Out-Null
  git -C $WorkDir diff --binary | Set-Content -Path $DiffPath -Encoding UTF8
}

Write-Host ""
Write-Host "Codex run complete:"
Write-Host "  run:     $RunDir"
Write-Host "  workdir: $WorkDir"
Write-Host "  summary: $SummaryPath"

if ($Mode -eq "write") {
  Write-Host "  status:  $StatusPath"
  Write-Host "  diff:    $DiffPath"
}
