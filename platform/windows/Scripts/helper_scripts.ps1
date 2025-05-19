Function CheckoutRepoHash (
        [string]$repo_path,
        [string]$repo_hash) {
    Write-Host "Start executing method CheckoutRepoHash"
    Write-Host "Repo path: $repo_path";
    Write-Host "Hash: $repo_hash";
    git -C $repo_path fetch --all --tags
    if ($repo_hash -eq "latest") {
        $repo_hash = git -C $repo_path describe --tags --abbrev=0
    }

    git -C $repo_path checkout $repo_hash
}